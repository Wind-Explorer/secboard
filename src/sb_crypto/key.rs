use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use uuid::Uuid;

///
/// Generate a new key from two UUIDs
/// ## Returns
/// * `[u8; 32]` - Generated key
///
pub fn generate() -> [u8; 32] {
    // Generate two UUIDs
    let uuid = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();

    // Return the key derived from the two UUIDs
    return pbkdf2_hmac_array::<Sha256, 32>(uuid.as_bytes(), uuid2.as_bytes(), 100_000);
}
