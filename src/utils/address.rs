//! 🔐 Módulo de geração de endereços (FireChain, Ethereum, Bitcoin)
//! 📦 Compatível com BIP-44 e múltiplas redes
//! 🔍 Padrão enterprise: seguro, modular e interoperável

use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use secp256k1::PublicKey as SecpPublicKey;
use tiny_keccak::{Hasher, Keccak};
use bs58;
use hex;

/// 🔥 FireChain: sha256 → ripemd160 → base58 → "f1r3:..."
pub fn public_key_to_fire_address(pubkey: &SecpPublicKey) -> String {
    let pubkey_bytes = pubkey.serialize_uncompressed();
    let sha256 = Sha256::digest(&pubkey_bytes[1..]);
    let ripemd = Ripemd160::digest(&sha256);
    format!("f1r3:{}", bs58::encode(ripemd).into_string())
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