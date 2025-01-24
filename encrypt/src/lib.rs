mod functions;
use aes_gcm::{
    aead::{Aead, AeadCore, OsRng}, Aes256Gcm, AesGcm, Error, Key, KeyInit, Nonce // Or `Aes128Gcm`
};
use functions::{decipher_price, decrypt_device_id, generate_cipher_to_connect, modify_connect_code};



pub fn handle_decipher_price(price_key: Vec<u8>, price: Vec<u8>) -> u64 {
    let mut key: [u8; 16] = [0; 16];   
    let mut msg: [u8; 16] = [0; 16];   
    for i in price_key.iter().enumerate() {
        key[i.0] = *i.1;
    }
    for i in price.iter().enumerate() {
        msg[i.0] = *i.1;
    }
    decipher_price(msg, key)
}

pub fn handle_decipher_device_id(key: Vec<u8>, ciphertext: Vec<u8>) -> Result<[u8; 24], Error> {
    let mut value = [0; 24];
    let result = match decrypt_device_id(&ciphertext[..52], &key) {
        Ok(v) => v,
        Err(err) => {
            return Err(err);
        },
    };
    for (i, v) in result.iter().enumerate() {
        value[i] = *v;
    }
    Ok(value)
} 



pub fn handle_cipher_connect(connect_key: Vec<u8>, connect_msg: String) -> [u8; 16] {
    let mut key: [u8; 16] = [0; 16];   
    let mut msg: [u8; 9] = [0; 9];   
    for i in connect_key.iter().enumerate() {
        key[i.0] = *i.1;
    }
    for i in connect_msg.as_bytes().iter().enumerate() {
        msg[i.0] = *i.1;
    }
    generate_cipher_to_connect(&msg, key)
}

pub fn handle_env_bytes(value: String) -> Vec<u8> {
    value.split(",").map(|x| x.trim().parse::<u8>().unwrap()).collect()
}



#[cfg(test)]
mod tests {
    use functions::{cipher_price, encrypt_device_id, 
        generate_random_values, generate_cipher_to_connect, 
        generate_decipher_to_connect, vec_to_string};

    use super::*;
    const ENCRYPTION_KEY_DEVICE_ID: [u8; 32] = [80, 52, 48, 85, 87, 99, 48, 56, 107, 48, 117, 105, 115, 56, 54, 116, 79, 103, 54, 86, 100, 56, 101, 90, 109, 104, 84, 99, 119, 90, 102, 120];
    const ENCRYPTION_KEY_PRICE: [u8; 16] = [0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x1A,0x1B,0x1C,0x1D,0x1E,0x1F];
    const ENCRYPTION_CONNECT_KEY: [u8; 16] = [0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A,0x0B,0x0C,0x0D,0x0E,0x0F];
    const CONNECT_MSG: [u8; 9] = *b"ozxlerfor";

    #[test]
    fn test_random_number_generator() {
        let key = generate_random_values(16);
        let mut good_num= false;
        let mut good_lowcase = false;
        let mut good_uppercase = false;
        for byte in key.iter() {
            if *byte < 10 {
                good_num = true;
            } else if byte.is_ascii_uppercase() {
                good_uppercase = true;
            } else if byte.is_ascii_lowercase() {
                good_lowcase = true;
            }
        }
        
        assert!(good_num && good_uppercase && good_lowcase);
    }
    #[test]
    fn test_api_connect() {
        let cipher = generate_cipher_to_connect(&CONNECT_MSG, ENCRYPTION_CONNECT_KEY);
        let decipher = generate_decipher_to_connect(cipher, ENCRYPTION_CONNECT_KEY);
        assert!(decipher.contains(&b'o') && decipher.contains(&b'l'));

    }
    #[test]
    fn test_price_key() {
        let mut price: [u8; 16] = [b'a'; 16];
        for i in 0..2 {
            price[i] = 9;
        } 
        let cipher = cipher_price(price, ENCRYPTION_KEY_PRICE);
        let decipher = decipher_price(cipher, ENCRYPTION_KEY_PRICE);
        assert_eq!(decipher, 99);
    }

    #[test]
    fn test_price_convert() {
        let value = vec![1,2,4];
        let result = value.into_iter().fold(0, |acc, digit| acc * 10 + digit as u64);
        assert!(result == 124)
    }

    #[test]
    fn test_device_uuid() {
        let device_uuid = [106, 119, 6, 88, 122, 1, 83, 88, 5, 7, 85, 110, 4, 90, 81, 110, 119, 112, 118, 85, 110, 79, 1, 65];
        let cipher = encrypt_device_id(&device_uuid, &ENCRYPTION_KEY_DEVICE_ID).unwrap();
        panic!("cipher{:?}", cipher);
        let decipher = decrypt_device_id(&cipher, &ENCRYPTION_KEY_DEVICE_ID).expect("nothing found");
        assert!(decipher == device_uuid);
    }

    #[test]
    fn handle_format_string() {
        panic!("{:?}", vec_to_string(&generate_random_values(16)));
        let mut new_data: Vec<u8> = Vec::new();
        let data: Vec<u8> = "80, 52, 48, 85, 87, 99, 48, 56, 107, 48, 117, 105, 115, 56, 54, 116, 79, 103, 54, 86, 100, 56, 101, 90, 109, 104, 84, 99, 119, 90, 102, 120".to_string().split(",").map(|x| x.trim().parse::<u8>().unwrap()).collect();
        panic!("{:?}", data)
    }

    // #[test]
    // fn test_device() {
    //     let key = b"very-secret-key-32-bytes-long!!!";
    //     let message = b"Hello, secure world!";
    //     let encrypted = encrypt(message, key).unwrap();
    //     let decrypted = decrypt(&encrypted, key).unwrap();
    //     assert_eq!(message, decrypted.as_slice());
    // }
}

