//! 🔐 Geração segura de carteiras FireChain
//! 🎯 Com derivação de múltiplos formatos de endereço + metadados avançados

use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use tiny_keccak::{Hasher, Keccak};
use bs58;
use hex;

/// Gera uma nova chave e todos os formatos de endereço associados
/// Retorna: (privkey, pubkey, fire_addr, eth_addr, btc_addr, fingerprint, derivation_hash)
pub fn generate_keypair() -> (String, String, String, String, String, String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let private_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    let priv_hex = hex::encode(private_key.secret_bytes());
    let pub_uncompressed = public_key.serialize_uncompressed();
    let pub_hex = hex::encode(pub_uncompressed);

    // 🔐 FireChain: SHA256 -> RIPEMD160 -> base58 -> prefix
    let mut hasher = Sha256::new();
    hasher.update(&pub_uncompressed);
    let sha_result = hasher.finalize();
    let ripemd_result = Ripemd160::digest(sha_result);
    let fire_addr = format!("f1r3:{}", bs58::encode(ripemd_result).into_string());

    // 🌐 Ethereum: Keccak256(pubkey[1..]) → últimos 20 bytes
    let mut keccak = Keccak::v256();
    keccak.update(&pub_uncompressed[1..]); // remove prefixo 0x04
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    let eth_addr = format!("0x{}", hex::encode(&output[12..])); // últimos 20 bytes

    // ₿ Bitcoin: SHA256 -> RIPEMD160 -> base58
    let mut sha_btc = Sha256::new();
    sha_btc.update(&pub_uncompressed);
    let result_sha_btc = sha_btc.finalize();
    let ripemd_btc = Ripemd160::digest(result_sha_btc);
    let btc_addr = bs58::encode(ripemd_btc).into_string();

    // 🧬 Fingerprint: SHA256 da public key (hex)
    let fingerprint = hex::encode(Sha256::digest(&pub_uncompressed));

    // 🔗 Hash de derivação: Keccak256 da public key completa
    let mut keccak_hash = Keccak::v256();
    keccak_hash.update(&pub_uncompressed);
    let mut output_derivation = [0u8; 32];
    keccak_hash.finalize(&mut output_derivation);
    let derivation_hash = hex::encode(output_derivation);

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
