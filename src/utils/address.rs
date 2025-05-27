//! 🔐 Módulo de geração de endereços (FireChain, Ethereum, Bitcoin)
//! 📦 Compatível com BIP-44 e múltiplas redes
//! 🔍 Padrão enterprise: seguro, modular e interoperável

use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use secp256k1::PublicKey as SecpPublicKey;
use tiny_keccak::{Hasher, Keccak};
use bs58;
use hex;

/// 🔥 FireChain Address (nova versão):
/// SHA256 → RIPEMD160 → prefixo 0x77 → checksum → base58check → prefixo visual `f1r3`
/// Resultado: f1r3<base58check>
pub fn public_key_to_fire_address(pubkey: &SecpPublicKey) -> String {
    let pubkey_bytes = pubkey.serialize_uncompressed();

    // 🔐 1. SHA256 da public key
    let sha256 = Sha256::digest(&pubkey_bytes);

    // 🔐 2. RIPEMD160 do SHA256
    let ripemd160 = Ripemd160::digest(sha256);

    // 🔥 3. Prefixo FireChain: 0x77
    let mut payload = vec![0x77];
    payload.extend_from_slice(&ripemd160);

    // 🔍 4. Checksum = SHA256(SHA256(payload))
    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend_from_slice(&checksum[..4]); // 4 bytes

    // 🧬 5. Base58Check encoding
    let encoded = bs58::encode(payload).into_string();

    // ✅ 6. Prefixo visual `f1r3` (fora do base58check)
    format!("f1r3{}", encoded)
}

/// 🌐 Ethereum: keccak256(pubkey)[12..] → hex
pub fn public_key_to_eth_address(pubkey: &SecpPublicKey) -> String {
    let pubkey_bytes = pubkey.serialize_uncompressed();

    let mut hasher = Keccak::v256();
    let mut hash = [0u8; 32];
    hasher.update(&pubkey_bytes[1..]); // remove prefixo 0x04
    hasher.finalize(&mut hash);

    format!("0x{}", hex::encode(&hash[12..])) // últimos 20 bytes (20 x 2 = 40 hex chars)
}

/// ₿ Bitcoin: sha256 → ripemd160 → base58check com prefixo 0x00
pub fn public_key_to_btc_address(pubkey: &SecpPublicKey) -> String {
    let pubkey_bytes = pubkey.serialize();
    let sha256 = Sha256::digest(&pubkey_bytes);
    let ripemd = Ripemd160::digest(&sha256);

    let mut payload = vec![0x00]; // prefixo mainnet BTC
    payload.extend_from_slice(&ripemd);

    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend_from_slice(&checksum[..4]);

    bs58::encode(payload).into_string()
}
