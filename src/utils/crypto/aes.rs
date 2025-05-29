use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::Argon2; // ✅ Apenas o necessário
use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand_core::RngCore;
use zeroize::Zeroizing;

use crate::core::wallet::Wallet;

pub fn encrypt_wallet(wallet: &Wallet, password: &str) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);

    let mut key = Zeroizing::new([0u8; 64]);
    let argon2 = Argon2::default();

    argon2
        .hash_password_into(password.as_bytes(), &salt, key.as_mut_slice())
        .map_err(|_| "Erro ao derivar chave com Argon2.".to_string())?;

    let mut aes_key = Zeroizing::new([0u8; 32]);
    let mut hmac_key = Zeroizing::new([0u8; 32]);

    aes_key.as_mut_slice().copy_from_slice(&key[..32]);
    hmac_key.as_mut_slice().copy_from_slice(&key[32..]);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(aes_key.as_slice()));
    let nonce = Nonce::from_slice(&salt[..12]);
    let data = wallet.to_encrypted_bytes()?;

    let encrypted = cipher
        .encrypt(nonce, data.as_ref())
        .map_err(|_| "Erro ao criptografar dados.".to_string())?;

    let mut mac: Hmac<Sha256> = Mac::new_from_slice(hmac_key.as_slice())
        .map_err(|_| "Erro ao criar HMAC.".to_string())?;

    mac.update(&salt);
    mac.update(&encrypted);
    let tag = mac.finalize().into_bytes();

    let mut result = vec![];
    result.extend_from_slice(&salt);
    result.extend_from_slice(&tag);
    result.extend_from_slice(&encrypted);
    Ok(result)
}

pub fn decrypt_wallet(data: &[u8], password: &str) -> Result<Wallet, String> {
    if data.len() < 48 {
        return Err("Dados criptografados estão incompletos.".to_string());
    }

    let salt = &data[0..16];
    let tag = &data[16..48];
    let encrypted = &data[48..];

    let mut key = Zeroizing::new([0u8; 64]);
    let argon2 = Argon2::default();

    argon2
        .hash_password_into(password.as_bytes(), salt, key.as_mut_slice())
        .map_err(|_| "Erro ao derivar chave com Argon2.".to_string())?;

    let mut aes_key = Zeroizing::new([0u8; 32]);
    let mut hmac_key = Zeroizing::new([0u8; 32]);

    aes_key.as_mut_slice().copy_from_slice(&key[..32]);
    hmac_key.as_mut_slice().copy_from_slice(&key[32..]);

    let mut mac: Hmac<Sha256> = Mac::new_from_slice(hmac_key.as_slice())
        .map_err(|_| "Erro ao criar HMAC.".to_string())?;

    mac.update(salt);
    mac.update(encrypted);
    mac.verify_slice(tag)
        .map_err(|_| "❌ Integridade comprometida: senha incorreta ou dados alterados.".to_string())?;

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(aes_key.as_slice()));
    let nonce = Nonce::from_slice(&salt[..12]);

    let decrypted = cipher
        .decrypt(nonce, encrypted.as_ref())
        .map_err(|_| "Erro ao descriptografar dados.".to_string())?;

    Wallet::from_decrypted_bytes(&decrypted)
}
