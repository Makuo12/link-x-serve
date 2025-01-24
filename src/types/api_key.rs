use std::{collections::HashSet, fs::OpenOptions, io::Read};


#[derive(Debug, Clone)]
pub struct ApiKey {
    pub api_keys: HashSet<[u8; 16]>,
}

impl ApiKey {
    pub fn new() -> Self {
        let mut api_keys = HashSet::new();
        let mut file = OpenOptions::new().read(true).open("api_keys.txt").unwrap();
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        let mut key: [u8; 16] = [0; 16];
        let mut i = 0;
        for c in contents {
            if c == b',' || c == b';' {
                api_keys.insert(key);
                key = [0; 16];
                i = 0;
            } else {
                key[i] = c;
                i += 1;
            }
        }
        ApiKey {
            api_keys,
        }
        
    }
}