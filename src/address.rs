use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::Sha256;
use ripemd::Ripemd160;
use digest::Digest;
use bs58;
use zeroize::Zeroize;
use hex::ToHex;
use serde::{Serialize, Deserialize};

/// Representa um endereço com suas chaves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}

impl Address {
    /// Gera um endereço aleatório com chaves ECDSA
    pub fn generate_random() -> Address {
        let mut rng = OsRng;
        let secret_key = SecretKey::new(&mut rng);
        Self::from_secret_key(secret_key)
    }

    /// Gera um endereço a partir de uma chave privada fornecida
    pub fn from_secret_key(secret_key: SecretKey) -> Address {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // Hash160 da public key (SHA256 + RIPEMD160)
        let sha256_hash = Sha256::digest(&public_key.serialize());
        let ripemd_hash = Ripemd160::digest(&sha256_hash);

        // Prefixo da Fire Chain + hash
        let mut data = Vec::with_capacity(1 + ripemd_hash.len() + 4);
        data.push(0x77); // Prefixo da Fire Chain
        data.extend(&ripemd_hash);

        // Checksum: SHA256(SHA256(data))[0..4]
        let checksum = Sha256::digest(&Sha256::digest(&data));
        data.extend(&checksum[..4]);

        // Base58 final com prefixo "f1r3"
        let address_str = format!("f1r3{}", bs58::encode(&data).into_string());

        // Clona bytes da private key antes de apagar
        let private_bytes = secret_key.secret_bytes();
        let addr = Address {
            address: address_str,
            public_key: public_key.serialize().encode_hex::<String>(),
            private_key: private_bytes.encode_hex::<String>(),
        };

        // Zeroiza memória da chave privada
        let mut zeroize_bytes = private_bytes;
        zeroize_bytes.zeroize();

        addr
    }
}
