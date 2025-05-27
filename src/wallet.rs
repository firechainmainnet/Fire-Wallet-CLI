//! 🔐 Geração segura de carteiras FireChain
//! 🎯 Com derivação de múltiplos formatos de endereço + metadados avançados

use rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use tiny_keccak::{Hasher, Keccak};
use bs58;
use hex;

/// ✅ Gera todos os dados fundamentais da carteira
/// Retorna: (priv_hex, pub_hex, fire_addr, eth_addr, btc_addr, fingerprint, derivation_hash)
pub fn generate_keypair() -> (String, String, String, String, String, String, String) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // 🔐 Chaves privada e pública
    let private_key = SecretKey::new(&mut rng);
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    let priv_hex = hex::encode(private_key.secret_bytes());
    let pub_uncompressed = public_key.serialize_uncompressed();
    let pub_hex = hex::encode(pub_uncompressed);

    // 📬 Endereço FireChain: SHA256 -> RIPEMD160 -> base58 -> prefixo
    let sha_result = Sha256::digest(&pub_uncompressed);
    let ripemd_result = Ripemd160::digest(sha_result);
    let fire_addr = format!("f1r3:{}", bs58::encode(ripemd_result).into_string());

    // 🌐 Endereço Ethereum: Keccak256(pubkey[1..]) -> últimos 20 bytes
    let mut keccak = Keccak::v256();
    keccak.update(&pub_uncompressed[1..]); // remove prefixo 0x04
    let mut keccak_out = [0u8; 32];
    keccak.finalize(&mut keccak_out);
    let eth_addr = format!("0x{}", hex::encode(&keccak_out[12..]));

    // ₿ Endereço Bitcoin: SHA256 -> RIPEMD160 -> prefixo 0x00 -> checksum -> base58check
    let pub_compressed = public_key.serialize(); // formato comprimido (33 bytes)
    let sha_btc = Sha256::digest(&pub_compressed);
    let ripemd_btc = Ripemd160::digest(sha_btc);

    let mut payload = vec![0x00]; // prefixo da rede mainnet BTC
    payload.extend_from_slice(&ripemd_btc);

    let checksum = Sha256::digest(&Sha256::digest(&payload));
    payload.extend_from_slice(&checksum[..4]);

    let btc_addr = bs58::encode(payload).into_string(); // ✅ base58check completo

    // 🧬 Fingerprint SHA256
    let fingerprint = hex::encode(Sha256::digest(&pub_uncompressed));

    // 🔗 Hash de derivação Keccak256
    let mut keccak_der = Keccak::v256();
    keccak_der.update(&pub_uncompressed);
    let mut deriv_out = [0u8; 32];
    keccak_der.finalize(&mut deriv_out);
    let derivation_hash = hex::encode(deriv_out);

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
