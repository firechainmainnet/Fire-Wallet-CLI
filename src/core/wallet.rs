// 📂 src/core/wallet.rs

use secp256k1::Secp256k1;
use rand::rngs::OsRng;

/// 🔐 Representa uma carteira FireChain gerada localmente.
///
/// Contém chave privada e chave pública derivada.
/// A derivação dos endereços é feita separadamente pelo comando `derive`.
pub struct Wallet {
    /// Chave privada no formato hexadecimal (NÃO deve ser exposta em produção sem criptografia)
    pub private_key: String,

    /// Chave pública derivada da chave privada, também no formato hexadecimal
    pub public_key: String,
}

impl Wallet {
    /// 🧬 Gera uma nova carteira FireChain com chaves aleatórias usando `secp256k1`.
    ///
    /// A carteira é gerada localmente com base em um gerador de entropia seguro (`OsRng`).
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let private_key_hex = hex::encode(secret_key.secret_bytes());
        let public_key_hex = hex::encode(public_key.serialize_uncompressed());

        Wallet {
            private_key: private_key_hex,
            public_key: public_key_hex,
        }
    }
}
