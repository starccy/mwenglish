use rand::Rng;
use crypto::buffer::{WriteBuffer, ReadBuffer};


pub struct AesKey([u8; 16]);

const IV_KEY: [u8; 16] = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'1', b'2', b'3', b'4', b'5', b'6'];

impl AesKey {
    pub fn gen_key() -> Self {
        let key: [u8; 16] = rand::thread_rng().gen();
        Self(key)
    }

    pub fn new(key: [u8; 16]) -> Self {
        Self(key)
    }

    pub fn get_key_to_base64(&self) -> String {
        base64::encode(&self.0)
    }

    pub fn set_key_from_base64(key: String) -> Result<Self, String> {
        let key = base64::decode(&key).map_err(|e| e.to_string())?;
        let mut result: [u8; 16] = [0; 16];
        let bytes = key.as_slice();
        let bytes = &bytes[..result.len()];
        result.copy_from_slice(bytes);
        Ok(Self(result))
    }

    pub fn encrypt(&self, data: String) -> Result<String, String> {
        let mut encryptor = crypto::aes::cbc_encryptor(crypto::aes::KeySize::KeySize128, &self.0, &IV_KEY, crypto::blockmodes::PkcsPadding);
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(data.as_bytes());
        let mut buffer = [0; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).map_err(|_| "crypto::symmetriccipher::SymmetricCipherError".to_string() )?;
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                crypto::buffer::BufferResult::BufferUnderflow => break,
                crypto::buffer::BufferResult::BufferOverflow => {}
            }
        }
        let raw_result = (&*String::from_utf8_lossy(&final_result)).to_string();
        Ok(base64::encode(&raw_result))
    }

    pub fn decrypt(&self, encrypted_data: String) -> Result<String, String> {
        let encrypted_data = base64::decode(&encrypted_data).map_err(|e| e.to_string())?;
        let mut decryptor = crypto::aes::cbc_decryptor(crypto::aes::KeySize::KeySize128, &self.0, &IV_KEY, crypto::blockmodes::PkcsPadding);
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(&encrypted_data);
        let mut buffer = [0; 4096];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
        loop {
            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).map_err(|_| "crypto::symmetriccipher::SymmetricCipherError".to_string())?;
//            let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
            final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                crypto::buffer::BufferResult::BufferUnderflow => break,
                crypto::buffer::BufferResult::BufferOverflow => {}
            }
        }
        Ok((&*String::from_utf8_lossy(&final_result)).to_string())
    }
}