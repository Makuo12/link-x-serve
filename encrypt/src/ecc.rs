use base64::engine::general_purpose;
use base64::Engine as _;
use k256::pkcs8::DecodePrivateKey;
use p256::{NistP256, SecretKey};
use p256::pkcs8::{EncodePrivateKey, LineEnding};
use rand_core::OsRng;
use serde::Serialize;
use std::fs;
use p256::PublicKey ;
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, Payload}};
use p256::ecdh::diffie_hellman;
use hkdf::Hkdf;
use sha2::Sha256;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json as AppJson,
};
use crate::functions::generate_randome_names;

pub struct KeyPair {
    pub private_key: String,
    pub public_key: String,
    pub file_name: String,

}

pub enum CryptError {
    KeyGenerationError(String),
    FileWriteError(String),
    FileReadError(String),
    KeyDecodingError(String),
    KeyAgreementError(String),
    DecryptionError(String),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for CryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CryptError::KeyGenerationError(err) => write!(f, "Key generation error: {}", err),
            CryptError::FileWriteError(err) => write!(f, "File write error: {}", err),
            CryptError::FileReadError(err) => write!(f, "File read error: {}", err),
            CryptError::KeyDecodingError(err) => write!(f, "Key decoding error: {}", err),
            CryptError::KeyAgreementError(err) => write!(f, "Key agreement error: {}", err),
            CryptError::DecryptionError(err) => write!(f, "Decryption error: {}", err),
        }
    }
}

impl IntoResponse for CryptError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            CryptError::KeyGenerationError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Key generation failed: {}", err),
            ),
            CryptError::FileWriteError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("File write failed: {}", err),
            ),
            CryptError::FileReadError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("File read failed: {}", err),
            ),
            CryptError::KeyDecodingError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid key data: {}", err),
            ),
            CryptError::KeyAgreementError(err) => (
                StatusCode::BAD_REQUEST,
                format!("Key agreement failed: {}", err),
            ),
            CryptError::DecryptionError(err) => (
                StatusCode::UNAUTHORIZED,
                format!("Decryption failed: {}", err),
            ),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}


pub fn generate_keys() -> Result<KeyPair, CryptError> {
    let server_private_key = SecretKey::random(&mut OsRng);
    let server_public_key = server_private_key.public_key();
    
    // To be able to use the key later, you should serialize it.
    let private_key_pem = server_private_key.to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| CryptError::KeyGenerationError(format!("Failed to encode private key: {}", e)))?;
    // Define the file path
    let file_name = generate_randome_names(10)+".pem";
    // Write the key to the file
    fs::write(&file_name, private_key_pem.as_bytes()).map_err(|e| CryptError::FileWriteError(format!("Failed to write private key to file: {}", e)))?;
    let result = fs::read_to_string(&file_name).map_err(|e| CryptError::FileReadError(format!("Failed to read private key from file: {}", e)))?;
    fs::remove_file(&file_name).map_err(|e| CryptError::FileWriteError(format!("Failed to remove private key file: {}", e)))?;
    let public_key_pem = server_public_key.to_sec1_bytes();
    let public_key_base64 = base64::engine::general_purpose::STANDARD.encode(&public_key_pem);
    return Ok(KeyPair {
        private_key: result,
        public_key: public_key_base64,
        file_name: file_name,
    });
}


pub fn ecc_decrypt_key(key: &String, private_key: String) -> Result<String, CryptError> {
    let base64_string = key;
    let combined_data_bytes = general_purpose::STANDARD.decode(base64_string).map_err(|e| CryptError::KeyDecodingError(e.to_string()))?;

    let server_private_key = SecretKey::from_pkcs8_pem(&private_key).map_err(|e| CryptError::KeyDecodingError(format!("Failed to decode private key: {}", e)))?;

    // Separate ephemeral public key and sealed box
    const EPHEMERAL_PUBLIC_KEY_LEN: usize = 65;
    let ephemeral_public_key_bytes = &combined_data_bytes[0..EPHEMERAL_PUBLIC_KEY_LEN];
    let sealed_box_bytes = &combined_data_bytes[EPHEMERAL_PUBLIC_KEY_LEN..];

    // Perform key agreement
    let ephemeral_public_key = PublicKey::from_sec1_bytes(ephemeral_public_key_bytes).map_err(|e| CryptError::KeyDecodingError(format!("Failed to decode ephemeral public key: {}", e)))?;
    let shared_secret = diffie_hellman::<NistP256>(
        &server_private_key.to_nonzero_scalar(),
        ephemeral_public_key.as_affine()
    );

    // Derive symmetric key
    let hkdf = Hkdf::<Sha256>::new(None, shared_secret.raw_secret_bytes());
    let mut key = [0u8; 32];
    hkdf.expand(&[], &mut key).unwrap();
    let symmetric_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key);

    // Decrypt the sealed box
    let cipher = Aes256Gcm::new(symmetric_key);
    let nonce_len = 12;

    let nonce = aes_gcm::Nonce::from_slice(&sealed_box_bytes[0..nonce_len]);
    let ciphertext_bytes = &sealed_box_bytes[nonce_len..];

    let decrypted_bytes = cipher.decrypt(
        nonce,
        Payload {
            msg: ciphertext_bytes,
            aad: &[],
        },
    ).map_err(|e| CryptError::DecryptionError(format!("Decryption failed: {}", e)))?;

    let decrypted_message = String::from_utf8(decrypted_bytes).map_err(|e| CryptError::DecryptionError(format!("Failed to convert decrypted bytes to string: {}", e)))?;
    Ok(decrypted_message)
}