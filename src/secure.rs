use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, generic_array::GenericArray};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng as CryptoOsRng};
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::{write, read};

use crate::wallet::Wallet;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedWallet {
    pub label: Option<String>,
    pub encrypted_data: Vec<u8>,
    pub nonce: [u8; 12],
    pub salt: String,
    pub hash: String,
}

/// Exporta uma carteira para um arquivo `.wallet` (criptografado)
pub fn export_wallet_to_file(wallet: &Wallet, password: &str, path: &str) -> Result<(), String> {
    let encrypted = encrypt_wallet(wallet, password)?;
    let json = serde_json::to_string_pretty(&encrypted).map_err(|e| e.to_string())?;
    write(path, json).map_err(|e| format!("Erro ao salvar arquivo: {:?}", e))?;
    Ok(())
}

/// Importa e descriptografa um arquivo `.wallet` protegido por senha
pub fn import_wallet_from_file(password: &str, path: &str) -> Result<Wallet, String> {
    let file = read(path).map_err(|e| format!("Erro ao ler arquivo: {:?}", e))?;
    let parsed: EncryptedWallet = serde_json::from_slice(&file).map_err(|e| format!("Erro no JSON: {:?}", e))?;
    decrypt_wallet(password, &parsed)
}

/// Criptografa a carteira serializada usando AES-256-GCM + Argon2id
fn encrypt_wallet(wallet: &Wallet, password: &str) -> Result<EncryptedWallet, String> {
    let json = serde_json::to_vec(wallet).map_err(|e| e.to_string())?;

    let salt = SaltString::generate(&mut CryptoOsRng);
    let mut nonce = [0u8; 12];
    CryptoOsRng.fill_bytes(&mut nonce);

    let argon2 = Argon2::default();
    let hashed = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Erro na derivação: {:?}", e))?;
    let key = hashed.hash.ok_or("Hash ausente ao derivar chave")?;

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key.as_bytes()[..32]));
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), json.as_ref())
        .map_err(|e| format!("Erro na criptografia: {:?}", e))?;

    Ok(EncryptedWallet {
        label: wallet.label.clone(),
        encrypted_data: ciphertext,
        nonce,
        salt: salt.to_string(),
        hash: hashed.to_string(),
    })
}

/// Descriptografa e restaura a carteira protegida
fn decrypt_wallet(password: &str, enc: &EncryptedWallet) -> Result<Wallet, String> {
    let parsed_hash = PasswordHash::new(&enc.hash).map_err(|e| format!("Hash inválido: {:?}", e))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Senha incorreta".to_string())?;

    let key = parsed_hash.hash.ok_or("Hash ausente ao derivar chave")?;
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key.as_bytes()[..32]));

    let plaintext = cipher.decrypt(Nonce::from_slice(&enc.nonce), enc.encrypted_data.as_ref())
        .map_err(|_| "Falha ao descriptografar. Senha incorreta ou dados corrompidos.".to_string())?;

    let wallet: Wallet = serde_json::from_slice(&plaintext)
        .map_err(|e| format!("Erro ao restaurar carteira: {:?}", e))?;
    Ok(wallet)
}
