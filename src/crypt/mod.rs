pub mod rsa;
pub mod aes;

use crypto::md5::Md5;
use std::str::{self, Utf8Error};
use crypto::digest::Digest;
use uuid::Uuid;

fn to_hex(x: u8) -> u8 {
    if x < 10 {
        (b'0' as u8) + x
    }
    else {
        (b'a' as u8) + x - 10
    }
}

pub fn create_md5(plaintext: String) -> Result<String, Utf8Error> {
    let mut md5 = Md5::new();
    let mut md5res = [0; 16];
    let mut output: [u8; 32] = [0; 32];
    md5.input(plaintext.as_bytes());
    md5.result(&mut md5res);
    for i in 0..16 {
        output[i * 2] = to_hex(md5res[i] >> 4);
        output[i * 2 + 1] = to_hex(md5res[i] & 0xF);
    }
    let result = str::from_utf8(&output)?.to_string();
    Ok(result)
}

pub fn create_uuid() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}