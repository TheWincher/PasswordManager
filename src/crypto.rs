use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use argon2::Argon2;
use rand::RngExt;

pub enum CryptoError {
    EncryptFailed,
    DecryptFailed,
    DeriveKeyFailed,
}

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut key: [u8; 32] = [0; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|_| CryptoError::DeriveKeyFailed)?;

    Ok(key)
}

pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, [u8; 12]), CryptoError> {
    let aes = Aes256Gcm::new(key.into());
    let nonce: [u8; 12] = rand::rng().random();

    let ciphertext = aes
        .encrypt(Nonce::from_slice(&nonce), data)
        .map_err(|_| CryptoError::EncryptFailed)?;

    Ok((ciphertext, nonce))
}

pub fn decrypt(
    ciphertext: &[u8],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<Vec<u8>, CryptoError> {
    let aes = Aes256Gcm::new(key.into());
    aes.decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|_| CryptoError::DecryptFailed)
}
