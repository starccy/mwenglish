use openssl::rsa::{Rsa, Padding};
use openssl::pkey::{Private, Public};

#[derive(Debug, Clone)]
pub struct ReaKeyPair {
    pub public_key: Option<Rsa<Public>>,
    pub private_key: Option<Rsa<Private>>,
}

impl ReaKeyPair {
    pub fn new() -> Self {
        let rsa = Rsa::generate(2048).unwrap();
        let public_key = Rsa::public_key_from_pem(&rsa.public_key_to_pem().unwrap()).unwrap();
        let private_key = Rsa::private_key_from_pem(&rsa.private_key_to_pem().unwrap()).unwrap();
        Self {
            public_key: Some(public_key),
            private_key: Some(private_key),
        }
    }

    pub fn new_with_params(private_key: Option<String>, public_key: Option<String>) -> Self {
        let private_key = if private_key.is_some() {
            let raw = private_key.unwrap();
            Some(Rsa::private_key_from_pem(raw.as_bytes()).unwrap())
        } else {
            None
        };

        let public_key = if public_key.is_some() {
            let raw = public_key.unwrap();
            let de_raw = base64::decode(&raw).unwrap();
            Some(Rsa::public_key_from_pem(raw.as_bytes()).unwrap())
        } else {
            None
        };

        Self {
            public_key,
            private_key,
        }
    }

    pub fn decrypt_with_private_key(&self, encrypted_content: String) -> String {
        let content = base64::decode(&encrypted_content).unwrap();
        let private_key = self.private_key.as_ref().unwrap();
        let mut result = vec![0u8; private_key.size() as usize];
        let _ = private_key.private_decrypt(&content, result.as_mut_slice(), Padding::PKCS1).unwrap();
        for i in (0..result.len()).rev() {
            if result[i] != 0 {
                break;
            }
            result.remove(i);
        }
        String::from_utf8_lossy(&result).to_string()
    }

    pub fn encrypt_with_public_key(&self, original_content: String) -> String {
        let content = original_content.as_bytes();
        let public_key = self.public_key.as_ref().unwrap();
        let mut result = vec![0u8; public_key.size() as usize];
        let _ = public_key.public_encrypt(content, result.as_mut_slice(), Padding::PKCS1).unwrap();
        base64::encode(&result)
    }

    pub fn public_key_string(&self) -> String {
        let public_key = self.public_key.as_ref().unwrap();
        let result = public_key.public_key_to_pem().unwrap();
        base64::encode(&result)
    }

    pub fn private_key_string(&self) -> String {
        let private_key = self.private_key.as_ref().unwrap();
        let result = private_key.private_key_to_pem().unwrap();
        base64::encode(&result)
    }
}
