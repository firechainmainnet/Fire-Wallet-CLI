use serde::{Deserialize, Serialize}; // ✅ Serialização da struct Wallet
use secp256k1::{Secp256k1, SecretKey, PublicKey}; // ✅ Criptografia
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub fingerprint: String,
}

impl Wallet {
    /// 🔧 Construtor principal da Wallet FireChain
    pub fn new(private_key: String, public_key: String, fingerprint: String) -> Self {
        Wallet {
            private_key,
            public_key,
            fingerprint,
        }
    }

    /// 🔐 Serializa a carteira para JSON (armazenamento seguro)
    pub fn to_encrypted_bytes(&self) -> Result<Vec<u8>, String> {
        let json_data = serde_json::to_vec_pretty(&self)
            .map_err(|_| "❌ Falha ao serializar a carteira.".to_string())?;
        Ok(json_data)
    }

    /// 🔓 Desserializa a carteira a partir de bytes descriptografados
    pub fn from_decrypted_bytes(decrypted: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(&decrypted)
            .map_err(|_| "❌ Falha ao decodificar o conteúdo da wallet.".to_string())
    }

    /// 🧠 Gera uma identidade criptográfica (PK, PUB, Fingerprint) aleatória
    pub fn generate_wallet_identity() -> (String, String, String) {
        let secp = Secp256k1::new();
        let mut rng = OsRng;

        // 🔐 Gera chave privada
        let private_key = SecretKey::new(&mut rng);
        let private_key_hex = hex::encode(private_key.secret_bytes());

        // 📤 Gera chave pública
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let public_key_bytes = public_key.serialize_uncompressed();
        let public_key_hex = hex::encode(public_key_bytes);

        // 🧬 Calcula fingerprint (SHA256 -> RIPEMD160)
        let sha256 = Sha256::digest(&public_key_bytes);
        let ripemd160 = Ripemd160::digest(&sha256);
        let fingerprint_hex = hex::encode(ripemd160);

        (private_key_hex, public_key_hex, fingerprint_hex)
    }
}
