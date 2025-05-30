
use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

use argon2::Argon2;
use password_hash::{PasswordHasher, SaltString}; // ✅ Trait necessário agora incluso

use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand_core::{RngCore, OsRng};

use crate::FireError;

/// 🔐 Criptografa dados com AES-256-GCM + HMAC-SHA256
pub fn encrypt(plaintext: &str, password: &str) -> Result<Vec<u8>, FireError> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let key = derive_key(password, &salt)?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| FireError::EncryptionError)?;

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce_obj = Nonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(nonce_obj, plaintext.as_bytes())
        .map_err(|_| FireError::EncryptionError)?;

    let mut output = vec![];
    output.extend_from_slice(&salt);     // 16 bytes
    output.extend_from_slice(&nonce);    // 12 bytes
    output.extend_from_slice(&ciphertext);

    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(&key).unwrap();
    mac.update(&output);
    let tag = mac.finalize().into_bytes();

    output.extend_from_slice(&tag); // 32 bytes

    Ok(output)
}

/// 🔓 Descriptografa dados criptografados com AES-256-GCM + HMAC-SHA256
pub fn decrypt(encrypted: &[u8], password: &str) -> Result<String, FireError> {
    if encrypted.len() < 60 {
        return Err(FireError::DecryptionError);
    }

    let (salt, rest) = encrypted.split_at(16);
    let (nonce, rest) = rest.split_at(12);
    let (ciphertext, tag) = rest.split_at(rest.len() - 32);

    let key = derive_key(password, salt)?;

    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(&key).unwrap();
    mac.update(&encrypted[..encrypted.len() - 32]);
    mac.verify_slice(tag).map_err(|_| FireError::InvalidPassword)?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| FireError::DecryptionError)?;
    let nonce_obj = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce_obj, ciphertext)
        .map_err(|_| FireError::DecryptionError)?;

    Ok(String::from_utf8(plaintext).map_err(|_| FireError::DecryptionError)?)
}

/// 🔑 Deriva uma chave de 256 bits via Argon2id
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32], FireError> {
    let salt_str = SaltString::encode_b64(salt).map_err(|_| FireError::KeyDerivationError)?;
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt_str)
        .map_err(|_| FireError::KeyDerivationError)?
        .hash
        .ok_or(FireError::KeyDerivationError)?;

    let mut key = [0u8; 32];
    key.copy_from_slice(&hash.as_bytes()[..32]);
    Ok(key)
}
