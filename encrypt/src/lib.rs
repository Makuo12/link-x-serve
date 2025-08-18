mod functions;
use aes_gcm::Error;
use functions::{basic_decipher, decrypt_device_id, generate_cipher_to_connect, price_decipher};

// const B: [u8; 16] = [9, 73, 120, 2, 107, 67, 83, 90, 89, 100, 88, 117, 119, 83, 4, 72, 79, 80, 81, 97, 79, 0, 1, 111];


pub fn handle_decipher_price(price_key: Vec<u8>, price: Vec<u8>) -> u64 {
    let mut key: [u8; 16] = [0; 16];   
    let mut msg: [u8; 16] = [0; 16];   
    for i in price_key.iter().enumerate() {
        key[i.0] = *i.1;
    }
    for i in price[0..16].iter().enumerate() {
        msg[i.0] = *i.1;
    }
    price_decipher(msg, key)
}

pub fn handle_decipher_device_id(cipher_key: Vec<u8>, ciphertext: Vec<u8>) -> Result<[u8; 16], Error> {
    let mut key: [u8; 16] = [0; 16];   
    let mut msg: [u8; 16] = [0; 16];   
    for i in cipher_key.iter().enumerate() {
        key[i.0] = *i.1;
    }
    for i in ciphertext[0..16].iter().enumerate() {
        msg[i.0] = *i.1;
    }
    let result = basic_decipher(msg, key);
    Ok(result)
} 

pub fn handle_decipher_device_id_max(key: Vec<u8>, ciphertext: Vec<u8>) -> Result<[u8; 24], Error> {
    let mut value = [0; 24];
    if ciphertext.len() < 52 {
        return Err(Error)
    }
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
    use functions::{encrypt_device_id, 
        generate_random_values, generate_cipher_to_connect, 
        generate_decipher_to_connect, vec_to_string};
    use crate::functions::aes_cipher;

    use super::*;
    const ENCRYPTION_DEVICE_ID_KEY: [u8; 16] = [122, 80, 122, 105, 115, 78, 53, 55, 122, 102, 72, 119, 119, 103, 50, 76];
    const ENCRYPTION_KEY_DEVICE_ID: [u8; 32] = [80, 52, 48, 85, 87, 99, 48, 56, 107, 48, 117, 105, 115, 56, 54, 116, 79, 103, 54, 86, 100, 56, 101, 90, 109, 104, 84, 99, 119, 90, 102, 120];
    const ENCRYPTION_KEY_PRICE: [u8; 16] = [0x10,0x11,0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x1A,0x1B,0x1C,0x1D,0x1E,0x1F];
    const ENCRYPTION_CONNECT_KEY: [u8; 16] = [3, 109, 87, 69, 7, 100, 107, 82, 79, 82, 112, 100, 7, 81, 0, 3];
    const CONNECT_MSG: [u8; 9] = *b"ozxlerfor";
    const DRIVER_CONNECT_MSG: [u8; 9] = *b"macfiller";
    #[test]
    fn test_random_number_generator() {
        let key = generate_random_values(16);
        let mut good_num= false;
        let mut good_lowcase = false;
        let mut good_uppercase = false;
        for byte in key.iter() {
            if byte.is_ascii_digit() {
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
        panic!("{:?}, {:?}", cipher, decipher.map(|c| c as char));
        assert!(decipher.contains(&b'l') && decipher.contains(&b'l'));
    }
    
    #[test]
    fn test_price_key() {
        let mut price: [u8; 16] = [b'a'; 16];
        for i in 0..2 {
            price[i] = b'9';
        } 
        let cipher = aes_cipher(price, ENCRYPTION_KEY_PRICE);
        let decipher = price_decipher(cipher, ENCRYPTION_KEY_PRICE);
        assert_eq!(decipher, 99);
    }

    #[test]
    fn test_price_convert() {
        let value = vec![1,2,4];
        let result = value.into_iter().fold(0, |acc, digit| acc * 10 + digit as u64);
        assert!(result == 124)
    }

    #[test]
    fn test_device_uuid_max() {
        let device_uuid = [106, 119, 6, 88, 122, 1, 83, 88, 5, 7, 85, 110, 4, 90, 81, 110, 119, 112, 118, 85, 110, 79, 1, 65];
        let cipher = encrypt_device_id(&device_uuid, &ENCRYPTION_KEY_DEVICE_ID).unwrap();
        let decipher = decrypt_device_id(&cipher, &ENCRYPTION_KEY_DEVICE_ID).expect("nothing found");
        assert!(decipher == device_uuid);
    }
    #[test]
    fn test_device_id() {
        let id: [u8; 16] = [66, 31, 57, 55, 137, 160, 33, 86, 251, 127, 189, 100, 216, 65, 216, 141];
        let decipher = handle_decipher_device_id(id.to_vec(), ENCRYPTION_DEVICE_ID_KEY.to_vec());
        
    }
    #[test]
    fn handle_format_string() {
        // panic!("generate {:?}", vec_to_string(&generate_random_values(16)));
        let mut new_data: Vec<u8> = Vec::new();
        let data: Vec<u8> = "80, 52, 48, 85, 87, 99, 48, 56, 107, 48, 117, 105, 115, 56, 54, 116, 79, 103, 54, 86, 100, 56, 101, 90, 109, 104, 84, 99, 119, 90, 102, 120".to_string().split(",").map(|x| x.trim().parse::<u8>().unwrap()).collect();
        assert!(true);
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

