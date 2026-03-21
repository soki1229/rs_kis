/*
use base64::{Engine as _, engine::general_purpose};
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use cipher::block_padding::Pkcs7;

pub fn aes_cbc_base64_dec(key: &str, iv: &str, cipher_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decode the base64 cipher text
    let cipher_data = general_purpose::STANDARD.decode(cipher_text)?;

    // Create cipher instance
    let cipher = cbc::Decryptor::<aes::Aes256>::new(key.as_bytes().into(), iv.as_bytes().into());

    // Decrypt
    let mut buf = cipher_data.to_vec();
    let decrypted_data = cipher.decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| format!("Decryption error: {:?}", e))?;

    // Convert to string
    let decrypted_string = String::from_utf8(decrypted_data.to_vec())?;

    Ok(decrypted_string)
}
*/