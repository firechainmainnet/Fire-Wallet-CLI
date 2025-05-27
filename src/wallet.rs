//! 🔐 Geração segura de carteiras FireChain
//! 🎯 Derivação de múltiplos formatos de endereço + metadados avançados

use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use tiny_keccak::{Hasher, Keccak};
use crate::utils::address::{
    public_key_to_fire_address,
    public_key_to_eth_address,
    public_key_to_btc_address,
};

/// ✅ Gera todos os dados fundamentais da carteira FireChain
/// 🔁 Retorna: (private_key_hex, public_key_hex, fire_addr, eth_addr, btc_addr, fingerprint_sha256, hash_keccak)
pub fn generate_keypair() -> (String, String, String, String, String, String, String) {
    // 🔒 Inicializa o contexto de segurança com entropia de sistema
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // 🔐 Geração de chave privada e pública
    let private_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // 🔑 Representações hexadecimais
    let priv_hex = hex::encode(private_key.secret_bytes());
    let pub_uncompressed = public_key.serialize_uncompressed();
    let pub_hex = hex::encode(&pub_uncompressed);

    // 📬 Geração de endereços
    let fire_addr = public_key_to_fire_address(&public_key); // ✅ NOVO PADRÃO f1r3<base58check>
    let eth_addr  = public_key_to_eth_address(&public_key);
    let btc_addr  = public_key_to_btc_address(&public_key);

    // 🧬 Fingerprint: SHA256 da public key (uncompressed)
    let fingerprint_bytes = Sha256::digest(&pub_uncompressed);
    let fingerprint = hex::encode(fingerprint_bytes);

    // 🔗 Hash de derivação: Keccak256 da public key
    let mut keccak = Keccak::v256();
    let mut keccak_out = [0u8; 32];
    keccak.update(&pub_uncompressed);
    keccak.finalize(&mut keccak_out);
    let derivation_hash = hex::encode(keccak_out);

    // 🎯 Retorno completo
    (
        priv_hex,
        pub_hex,
        fire_addr,
        eth_addr,
        btc_addr,
        fingerprint,
        derivation_hash,
    )
}
