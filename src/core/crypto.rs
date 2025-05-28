// 📂 src/core/crypto.rs

use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use tiny_keccak::{Keccak, Hasher};

/// 🔒 Aplica SHA-256 sobre os dados fornecidos.
///
/// Muito utilizado para validar integridade e gerar entropia inicial.
pub fn sha256(input: &[u8]) -> Vec<u8> {
    let digest = Sha256::digest(input);
    digest.to_vec()
}

/// 🔒 Aplica KECCAK-256 sobre os dados fornecidos.
///
/// Usado amplamente em Ethereum e Web3 para fingerprints e endereços.
pub fn keccak256(input: &[u8]) -> Vec<u8> {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(input);
    hasher.finalize(&mut output);
    output.to_vec()
}

/// 🔒 Aplica RIPEMD-160 sobre os dados fornecidos.
///
/// Usado para compactar fingerprints longas (ex: em endereços).
pub fn ripemd160(input: &[u8]) -> Vec<u8> {
    let digest = Ripemd160::digest(input);
    digest.to_vec()
}
