use crate::core::address::{generate_btc_address, generate_eth_address, generate_fire_address};
use crate::utils::crypto::aes::{encrypt, decrypt}; // ✅ Corrigido
use crate::FireError;
use rand::rngs::OsRng;
use secp256k1::{Secp256k1, PublicKey}; // ✅ SecretKey removido pois não está sendo usado diretamente
use sha2::{Sha256, Digest};
use hex;
use serde::{Serialize, Deserialize}; // ✅ Adicionado

#[derive(Debug, Clone, Serialize, Deserialize)] // ✅ Necessário para export/recover
pub struct Wallet {
    pub fingerprint: String,
    pub public_key: String,
    pub private_key: String,
    pub address_btc: String,
    pub address_eth: String,
    pub address_firechain: String,
}

impl Wallet {
    /// 🔧 Cria nova carteira com derivação de endereços
    pub fn new(private_key: String, public_key: String, fingerprint: String) -> Self {
        let pubkey_bytes = hex::decode(&public_key).expect("chave pública inválida");

        let address_btc = generate_btc_address(&pubkey_bytes);
        let address_eth = generate_eth_address(&pubkey_bytes);
        let address_firechain = generate_fire_address(&pubkey_bytes);

        Self {
            fingerprint,
            public_key,
            private_key,
            address_btc,
            address_eth,
            address_firechain,
        }
    }

    /// 🧠 Gera pubkey, privkey e fingerprint
    pub fn generate_wallet_identity() -> (String, String, String) {
        let secp = Secp256k1::new();
        let (secret_key, _public_key) = secp.generate_keypair(&mut OsRng);

        let private_key = hex::encode(secret_key.secret_bytes());
        let public_key = hex::encode(PublicKey::from_secret_key(&secp, &secret_key).serialize_uncompressed());

        // SHA256(pubkey) → primeiros 6 bytes como fingerprint
        let mut hasher = Sha256::new();
        hasher.update(&hex::decode(&public_key).unwrap());
        let result = hasher.finalize();
        let fingerprint = hex::encode(&result[..6]).to_uppercase();

        (private_key, public_key, fingerprint)
    }
}

/// 🔐 Criptografa carteira como JSON + AES-GCM
pub fn encrypt_wallet(wallet: &Wallet, password: &str) -> Result<Vec<u8>, FireError> {
    let json = serde_json::to_string(wallet)?; // ✅ Requer Serialize
    encrypt(&json, password)
}

/// 🔓 Descriptografa e reconstrói a carteira
pub fn decrypt_wallet(encrypted: &[u8], password: &str) -> Result<Wallet, FireError> {
    let decrypted = decrypt(encrypted, password)?;
    let wallet: Wallet = serde_json::from_str(&decrypted)?; // ✅ Requer Deserialize
    Ok(wallet)
}
