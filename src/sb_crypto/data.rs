use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit};
use rand::thread_rng;
use rand::Rng;

///
/// Encrypt data with AES-GCM
/// ## Arguments
/// * `data` - Data to encrypt
/// * `key` - Encryption key
/// ## Returns
/// * `Result<(Vec<u8>, [u8; 12]), Box<dyn std::error::Error>>` - Encrypted data and nonce
///
pub fn encrypt(
    data: &str,
    key: &[u8; 32],
) -> Result<(Vec<u8>, [u8; 12]), Box<dyn std::error::Error>> {
    // Create an AES-GCM cipher instance
    let cipher = Aes256Gcm::new(&(*key).into());

    // Generate a random nonce
    let mut nonce = [0u8; 12];
    thread_rng().fill(&mut nonce);

    // Encrypt the data
    let payload: Payload<'_, '_> = Payload {
        msg: data.as_bytes(),
        aad: &[],
    };
    let ciphertext = match cipher.encrypt(&nonce.into(), payload) {
        Ok(ciphertext) => ciphertext,
        Err(_) => return Err("Encryption failed".into()),
    };

    Ok((ciphertext, nonce))
}

///
/// Decrypt data with AES-GCM
/// ## Arguments
/// * `data` - Encrypted data
/// * `key` - Encryption key
/// * `nonce` - Nonce used to encrypt the data
/// ## Returns
/// * `Result<String, Box<dyn std::error::Error>>` - Decrypted data
///
pub fn decrypt(
    data: &[u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<String, Box<dyn std::error::Error>> {
    // Create an AES-GCM cipher instance
    let cipher = Aes256Gcm::new(&(*key).into());

    // Decrypt the data
    let payload = Payload {
        msg: data,
        aad: &[],
    };
    let plaintext = match cipher.decrypt(&(*nonce).into(), payload) {
        Ok(plaintext) => plaintext,
        Err(_) => return Err("Decryption failed".into()),
    };

    Ok(String::from_utf8(plaintext)?)
}
