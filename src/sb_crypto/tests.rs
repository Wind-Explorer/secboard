#[test]
fn test_generate() {
    let key = crate::sb_crypto::key::generate();
    assert_eq!(key.len(), 32);
}

#[test]
fn test_encrypt() {
    let key = [0u8; 32];
    let data = "hello world uwu";
    let encrypted_obj: (Vec<u8>, [u8; 12]) = crate::sb_crypto::data::encrypt(data, &key).unwrap();
    assert_ne!(data.as_bytes(), encrypted_obj.0);

    let decrypted_data =
        crate::sb_crypto::data::decrypt(&encrypted_obj.0, &key, &encrypted_obj.1).unwrap();
    assert_eq!(data, decrypted_data.as_str());
}

#[test]
fn test_decrypt() {
    let key = [0u8; 32];
    let data = "hello world :3";
    let encrypted_obj: (Vec<u8>, [u8; 12]) = crate::sb_crypto::data::encrypt(data, &key).unwrap();

    let decrypted_data =
        crate::sb_crypto::data::decrypt(&encrypted_obj.0, &key, &encrypted_obj.1).unwrap();
    assert_eq!(data, decrypted_data.as_str());
}
