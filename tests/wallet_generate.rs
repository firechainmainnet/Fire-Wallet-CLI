//! 🧪 Teste direto da geração de carteira (generate_keypair)

use firechain_cli::wallet::generate_keypair;
use firechain_cli::utils::address::{
    public_key_to_fire_address,
    public_key_to_eth_address,
    public_key_to_btc_address,
};

use secp256k1::Secp256k1;
use sha2::{Sha256, Digest};
use rand::rngs::OsRng; // ✅ necessário para compilar
use hex;

#[test]
fn test_generate_keypair_all_formats_valid() {
    let (priv_key, pub_key, fire_addr, eth_addr, btc_addr, fingerprint, hash) = generate_keypair();

    println!("\n🔎 [FireChain Teste] Resultado da geração de carteira:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 Chave privada (hex)      : {priv_key}");
    println!("🔓 Chave pública (hex)      : {pub_key}");
    println!("📬 Endereço (Fire)          : {fire_addr}");
    println!("🌐 Endereço (Ethereum)      : {eth_addr}");
    println!("₿  Endereço (Bitcoin)       : {btc_addr}");
    println!("🧬 Fingerprint SHA256       : {fingerprint}");
    println!("🧪 Hash de Derivação Keccak : {hash}");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Validação da private key (64 caracteres hex)
    assert!(priv_key.len() == 64 && priv_key.chars().all(|c| c.is_ascii_hexdigit()));

    // Validação da public key (130 caracteres uncompressed)
    assert!(pub_key.len() == 130 && pub_key.starts_with("04"));

    // FireChain: prefixo + base58
    assert!(fire_addr.starts_with("f1r3:") && fire_addr.len() > 20);

    // Ethereum: 0x + 40 hex
    assert!(eth_addr.starts_with("0x") && eth_addr.len() == 42);

    // Bitcoin: começa com 1 e entre 26 e 35 chars
    assert!(btc_addr.starts_with('1') && (26..=35).contains(&btc_addr.len()));

    // SHA256 fingerprint: 64 hex
    assert!(fingerprint.len() == 64 && fingerprint.chars().all(|c| c.is_ascii_hexdigit()));

    // Keccak hash: 64 hex
    assert!(hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()));
}
