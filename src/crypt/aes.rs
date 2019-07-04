use rand::Rng;
use crypto::aessafe::{AesSafe128Encryptor, AesSafe128Decryptor};
use aesstream::{AesWriter, AesReader};
use std::io::{Write, Cursor, Read};


pub struct AesKey([u8; 16]);

impl AesKey {
    pub fn gen_key() -> Self {
        let key: [u8; 16] = rand::thread_rng().gen();
        Self(key)
    }

    pub fn new(key: [u8; 16]) -> Self {
        Self(key)
    }

    pub fn get_key(&self) -> String {
        let key = &self.0;
        base64::encode(key)
    }

    pub fn set_key_from_string(key: String) -> Self {
        let key = base64::decode(&key).unwrap();
        let mut result = [0u8; 16];
        let bytes = key.as_slice();
        let bytes = &bytes[..result.len()];
        result.copy_from_slice(bytes);
        Self(result)
    }

    pub fn encrypt(&self, plaintext: String) -> String {
        let encryptor = AesSafe128Encryptor::new(&self.0);
        let mut encrypted = Vec::new();
        {
            let mut writer = AesWriter::new(&mut encrypted, encryptor).unwrap();
            writer.write_all(plaintext.as_bytes()).unwrap();
        }
        base64::encode(&encrypted)
    }

    pub fn decrypt(&self, encrypted_content: String) -> String {
        let decryptor = AesSafe128Decryptor::new(&self.0);
        let encrypted_content = base64::decode(&encrypted_content).unwrap();
        let mut decrypted = String::new();
        {
            let mut reader = AesReader::new(Cursor::new(encrypted_content), decryptor).unwrap();
            reader.read_to_string(&mut decrypted).unwrap();
        }
        decrypted
    }
}