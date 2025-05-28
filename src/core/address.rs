// 📂 src/core/address.rs

use crate::core::crypto::{sha256, ripemd160, keccak256};
use bs58;
use hex;

/// 🔗 Gera endereço BTC (formato P2PKH clássico, prefixo 0x00)
pub fn generate_btc_address(pub_key_bytes: &[u8]) -> String {
    let sha256_hash = sha256(pub_key_bytes);
    let ripemd = ripemd160(&sha256_hash);

    let mut extended = vec![0x00]; // prefixo padrão BTC (P2PKH)
    extended.extend(&ripemd);

    let checksum = &sha256(&sha256(&extended))[..4];
    extended.extend(checksum);

    bs58::encode(extended).into_string()
}

/// ⛓️ Gera endereço Ethereum (últimos 20 bytes do Keccak256 da pubkey sem 0x04)
pub fn generate_eth_address(pub_key_bytes: &[u8]) -> String {
    let keccak = keccak256(&pub_key_bytes[1..]); // remove o 0x04 do início
    format!("0x{}", hex::encode(&keccak[12..])) // últimos 20 bytes
}

/// 🔥 Gera endereço FireChain com prefixo `f1r3`, base58 e checksum SHA256
pub fn generate_fire_address(pub_key_bytes: &[u8]) -> String {
    let base_hash = ripemd160(&keccak256(&pub_key_bytes[1..]));
    let checksum = &sha256(&base_hash)[..4];

    let mut full = base_hash.to_vec();
    full.extend(checksum);

    format!("f1r3{}", bs58::encode(full).into_string())
}
