use ring::aead;
use ring::rand::{SystemRandom, SecureRandom};
use ring::pbkdf2;
use base64::engine::general_purpose::{self, STANDARD as BASE64_STANDARD};
use base64::Engine;
use std::num::NonZeroU32;
use rand::{rngs::OsRng, RngCore};


const PBKDF2_ITERATIONS: Option<NonZeroU32> = NonZeroU32::new(600_000);


// ############### GENERATE SALT ###############
pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32]; // 32 bytes for the salt
    OsRng.fill_bytes(&mut salt); // Fill the array with secure random bytes
    salt
}


// ############### GENERATE UNIQUE NONCE ###############
pub fn generate_unique_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    SystemRandom::new()
        .fill(&mut nonce)
        .expect("Failed to generate nonce");
    nonce
}


// ############### DERIVE MASTER KEY ###############
pub fn derive_master_key(master_password: &str, salt: &[u8; 32]) -> Result<[u8; 32], &'static str> {
    let mut master_key = [0u8; 32]; // 32-byte master key

    // Use PBKDF2 with HMAC-SHA256 to derive the master key
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS.unwrap(),
        salt,
        master_password.as_bytes(),
        &mut master_key,
    );

    Ok(master_key)
}


// ############### ENCRYPT PASSWORD ###############
pub fn encrypt_password(
    password: &str,
    master_key: &[u8; 32],
    nonce: &[u8; 12],
    salt: &[u8; 32]
) -> Result<String, &'static str> {
    // Generate the encryption key for the service using PBKDF2
    let mut encryption_key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS.unwrap(),
        salt,
        master_key,
        &mut encryption_key,
    );

    // Create the encryption key for AES-GCM
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
        .map_err(|_| "Failed to create encryption key")?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Prepare the password for encryption
    let aad = aead::Aad::empty();
    let mut buffer = password.as_bytes().to_vec();
    buffer.resize(buffer.len() + aead::AES_256_GCM.tag_len(), 0);

    // Encrypt the password
    key.seal_in_place_append_tag(
        aead::Nonce::assume_unique_for_key(*nonce), 
        aad,
        &mut buffer,
    )
    .map_err(|_| "Encryption failed")?;

    // Encode the encrypted password and nonce to base64
    let encrypted_password = BASE64_STANDARD.encode(&buffer);

    Ok(encrypted_password)
}


// ############### DECRYPT PASSWORD ###############
pub fn decrypt_password(
    master_key: &[u8; 32], 
    encoded_nonce: &str, 
    encoded_encrypted_password: &str, 
    salt: &[u8; 32]
) -> Result<String, &'static str> {
    // Derive the decryption key using PBKDF2 with the service as the key material
    let mut decryption_key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        PBKDF2_ITERATIONS.unwrap(),
        salt,
        master_key,
        &mut decryption_key,
    );

    // Create the encryption key for AES-GCM from the derived decryption key
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &decryption_key)
        .map_err(|_| "Failed to create decryption key")?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Decode the encoded nonce and encrypted password from Base64
    let nonce = general_purpose::STANDARD.decode(encoded_nonce).map_err(|_| "Invalid nonce data")?;
    let encrypted_password = general_purpose::STANDARD.decode(encoded_encrypted_password).map_err(|_| "Invalid password data")?;

    // Prepare the decryption process
    let mut buffer = encrypted_password;
    let aad = aead::Aad::empty();  

    // Decrypt the password
    let decrypted = key
        .open_in_place(aead::Nonce::try_assume_unique_for_key(&nonce).map_err(|_| "Invalid nonce")?, aad, &mut buffer)
        .map_err(|_| "Decryption failed")?;

    // Remove trailing null bytes from the decrypted password
    let trimmed = decrypted
        .split(|&byte| byte == 0)  // Split at null bytes (`\0`)
        .next()  // Take the first part (everything before the null bytes)
        .ok_or("Failed to trim null bytes")?;

    // Convert the decrypted bytes back to a String
    Ok(String::from_utf8_lossy(trimmed).to_string())
}
