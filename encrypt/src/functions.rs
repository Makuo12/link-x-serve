
use aes::{cipher::{consts::{B0, B1}, generic_array::GenericArray, typenum::{UInt, UTerm}, BlockDecrypt, BlockEncrypt}, Aes128};
use rand::{rngs::StdRng, Rng, SeedableRng};
use aes_gcm::{
    aead::Aead, Aes128Gcm, Key, KeyInit // Or `Aes128Gcm`
};
use aes_gcm::Nonce as GcmNonce;




pub(crate) fn modify_connect_code(code: &mut [u8; 16], connect_msg: &[u8; 9]) {
    let mut rng = StdRng::from_entropy();
    for i in 0..16 {
        if i > 3 && i < 13 {
            // middle
            code[i] = connect_msg[i - 4];
        } else {
            // end
            code[i] = rng.gen_range(b'a'..=b'z');
        } 
    }
}
pub(crate) fn generate_randome_names(count: usize) -> String {
    let mut key: String =String::new();
    let mut rng = StdRng::from_entropy();
    for _ in 0..count {
        key.push(char::from(rng.gen_range(b'a'..=b'z')));
    }
    key
}
pub(crate) fn generate_random_values(count: usize) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    let mut rng = StdRng::from_entropy();
    for _ in 0..count {
        let decider = rng.gen_range(0..=2);
        if decider == 0 {
            key.push(rng.gen_range(b'A'..=b'Z'));
        } else if decider == 1 {
            key.push(rng.gen_range(b'a'..=b'z'));
        } else {
            key.push(rng.gen_range(b'0'..=b'9'));
        }
    }
    key
}

// pub(crate) fn vec_to_string(vec: &[u8]) -> String {
//     let mut result = String::new();
//     for i in vec {
//         result.push(*i as char);
//     }
//     return result;
// }

// aes_decipher means it uses the Aes128 algorithm to decrypt (16 bits long)
fn aes_decipher(ciphertext: [u8; 16], key: [u8; 16]) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>>{
    let mut block = GenericArray::from(ciphertext);
    let key = GenericArray::from(key);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    cipher.decrypt_block(&mut block);
    block
}

pub(crate) fn price_decipher(price: [u8; 16], price_key: [u8; 16]) -> u64{
    let mut code: Vec<u32> = Vec::new();
    let block = aes_decipher(price, price_key); 
    for i in block.iter() {
        if let Some(value) = (*i as char).to_digit(10) {
            code.push(value);
        }
    }
    return code.iter().fold(0, |acc, value| acc * 10 + *value as u64);
}

pub(crate) fn basic_decipher(ciphertext: [u8; 16], key: [u8; 16]) -> [u8; 16] { 
    let mut code: [u8; 16] = [0; 16];
    let block = aes_decipher(ciphertext, key); 
    for i in block.iter().enumerate() {
        code[i.0] = *i.1;
    }
    code
}
// basic_cipher means it uses the Aes128 algorithm to encrypt (16 bits long)
// pub(crate) fn aes_cipher(price: [u8; 16], price_key: [u8; 16]) -> [u8;16] {
//     let mut code: [u8; 16] = [0; 16];
//     let mut block = GenericArray::from(price);
//     let key = GenericArray::from(price_key);
//     // Initialize cipher
//     let cipher = Aes128::new(&key);
//     cipher.encrypt_block(&mut block);
//     for i in block.iter().enumerate() {
//         code[i.0] = *i.1;
//     }
//     return code;
// }
pub(crate) fn generate_cipher_to_connect(connect_msg: &[u8; 9], connect_key: [u8; 16]) -> [u8; 16] {
    // Encrypt the message
    let mut code: [u8; 16] = [0; 16];
    modify_connect_code(&mut code, connect_msg);
    let mut block = GenericArray::from(code);
    let key = GenericArray::from(connect_key);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    for i in block.iter().enumerate() {
        code[i.0] = *i.1;
    }
    return code;
}
// pub(crate) fn generate_decipher_to_connect(cipher: [u8; 16], connect_key: [u8; 16]) -> [u8; 16] {
//     // Encrypt the message
//     let mut block = GenericArray::from(cipher);
//     let key = GenericArray::from(connect_key);
//     // Initialize cipher
//     let cipher = Aes128::new(&key);
//     cipher.decrypt_block(&mut block);
//     let mut code: [u8; 16] = [0; 16];
//     for i in block.iter().enumerate() {
//         code[i.0] = *i.1;
//     }
//     return code;
// }

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    // Key type changed to Aes128Gcm
    let key = Key::<Aes128Gcm>::from_slice(key);
    let cipher = Aes128Gcm::new(key);
    
    let nonce = GcmNonce::from_slice(&[0u8; 12]);
    let ciphertext = cipher.encrypt(&nonce, data)?;
    
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

pub fn decrypt(encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    // Key type changed to Aes128Gcm
    let key = Key::<Aes128Gcm>::from_slice(key);
    let cipher = Aes128Gcm::new(key);
    
    let (nonce, ciphertext_with_tag) = encrypted_data.split_at(12);
    let nonce = GcmNonce::from_slice(nonce);
    
    cipher.decrypt(nonce, ciphertext_with_tag)
}