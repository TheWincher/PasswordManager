use base64::Engine;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{
    app::Entry,
    crypto::{self, CryptoError},
};

pub enum VaultError {
    Io(std::io::Error),
    Crypto(CryptoError),
    Serialization(serde_json::Error),
    Deserialization(serde_json::Error),
    InvalidBase64,
}

#[derive(Serialize, Deserialize)]
pub struct VaultFile {
    salt: String,
    nonce: String,
    ciphertext: String,
}

pub fn save(entries: &[Entry], password: &str, path: &Path) -> Result<(), VaultError> {
    let serialized_entries = serde_json::to_vec(entries).map_err(VaultError::Serialization)?;

    let salt: [u8; 16] = rand::rng().random();
    let key = crypto::derive_key(password, &salt).map_err(VaultError::Crypto)?;

    let (ciphertext, nonce) =
        crypto::encrypt(&serialized_entries, &key).map_err(VaultError::Crypto)?;

    let vault_file = VaultFile {
        salt: STANDARD.encode(&salt),
        nonce: STANDARD.encode(&nonce),
        ciphertext: STANDARD.encode(&ciphertext),
    };

    let serialized_vault = serde_json::to_vec(&vault_file).map_err(VaultError::Serialization)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(VaultError::Io)?;
    }

    std::fs::write(path, serialized_vault).map_err(VaultError::Io)?;
    Ok(())
}

pub fn load(password: &str, path: &Path) -> Result<Vec<Entry>, VaultError> {
    let serialized_vault = std::fs::read(path).map_err(VaultError::Io)?;
    let vault_file = serde_json::from_slice::<VaultFile>(&serialized_vault)
        .map_err(VaultError::Deserialization)?;

    let salt = STANDARD
        .decode(vault_file.salt)
        .map_err(|_| VaultError::InvalidBase64)?;

    let nonce: [u8; 12] = STANDARD
        .decode(vault_file.nonce)
        .map_err(|_| VaultError::InvalidBase64)?
        .try_into()
        .map_err(|_| VaultError::InvalidBase64)?;

    let ciphertext = STANDARD
        .decode(vault_file.ciphertext)
        .map_err(|_| VaultError::InvalidBase64)?;

    let key = crypto::derive_key(password, &salt).map_err(VaultError::Crypto)?;

    let data = crypto::decrypt(&ciphertext, &key, &nonce).map_err(VaultError::Crypto)?;
    let entries =
        serde_json::from_slice::<Vec<Entry>>(&data).map_err(VaultError::Deserialization)?;

    Ok(entries)
}
