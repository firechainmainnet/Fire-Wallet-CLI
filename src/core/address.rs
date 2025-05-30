// 📂 src/core/address.rs

use crate::core::crypto::{sha256, ripemd160, keccak256};
use bs58;
use hex;

/// ₿ Gera endereço Bitcoin (formato clássico P2PKH, prefixo 0x00)
///
/// Processo:
/// 1. SHA256 da chave pública
/// 2. RIPEMD160 do hash
/// 3. Prefixo de rede (0x00 para mainnet)
/// 4. Checksum (primeiros 4 bytes de SHA256(SHA256(payload)))
/// 5. Base58 para o endereço final
pub fn generate_btc_address(pubkey_bytes: &[u8]) -> String {
    let sha256_hash = sha256(pubkey_bytes);
    let ripemd = ripemd160(&sha256_hash);

    let mut extended = vec![0x00]; // prefixo padrão BTC (mainnet P2PKH)
    extended.extend(&ripemd);

    let checksum = &sha256(&sha256(&extended))[..4];
    extended.extend(checksum);

    bs58::encode(extended).into_string()
}

/// ⛓️ Gera endereço Ethereum (compatível com carteiras 0x-prefixed)
///
/// Processo:
/// 1. Remove byte inicial 0x04 da chave pública (formato uncompressed)
/// 2. Calcula Keccak256 do restante
/// 3. Usa os últimos 20 bytes como endereço
pub fn generate_eth_address(pubkey_bytes: &[u8]) -> String {
    let keccak = keccak256(&pubkey_bytes[1..]); // remove o 0x04 do início
    format!("0x{}", hex::encode(&keccak[12..])) // últimos 20 bytes
}

/// 🔥 Gera endereço FireChain com prefixo `f1r3`, codificado em Base58 e com checksum
///
/// Processo:
/// 1. Remove 0x04 da chave pública
/// 2. Aplica Keccak256 + RIPEMD160
/// 3. Gera checksum SHA256 dos dados
/// 4. Codifica payload + checksum em Base58 com prefixo "f1r3"
pub fn generate_fire_address(pubkey_bytes: &[u8]) -> String {
    let base_hash = ripemd160(&keccak256(&pubkey_bytes[1..]));
    let checksum = &sha256(&base_hash)[..4];

    let mut full = base_hash.to_vec();
    full.extend(checksum);

    format!("f1r3{}", bs58::encode(full).into_string())
}
