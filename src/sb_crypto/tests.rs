#[test]
fn test_generate() {
    let key = crate::sb_crypto::key::generate();
    assert_eq!(key.len(), 32);
}
