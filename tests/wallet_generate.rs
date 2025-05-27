//! 🧪 Teste direto da geração de carteira (generate_keypair)
//! 🔒 Valida todos os formatos e metadados criptográficos
//! 🎯 Foco em segurança, entropia e formatação

use firechain_cli::wallet::generate_keypair;
use predicates::prelude::*;
use regex::Regex;

/// ✅ Testa se a função de geração retorna chaves válidas e endereços formatados corretamente
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

    // 🔐 Chave privada: hex com 64 caracteres
    assert!(priv_key.len() == 64 && priv_key.chars().all(|c| c.is_ascii_hexdigit()));

    // 🔓 Chave pública: começa com 04, tem 130 caracteres
    assert!(pub_key.len() == 130 && pub_key.starts_with("04"));

    // 📬 Endereço FireChain: prefixo + base58
    assert!(fire_addr.starts_with("f1r3:") && fire_addr.len() > 20);

    // 🌐 Ethereum: prefixo 0x e 40 hex chars
    assert!(eth_addr.starts_with("0x") && eth_addr.len() == 42);

    // ₿ Bitcoin: começa com 1, mínimo 26, máximo 35
    assert!(btc_addr.starts_with('1') && (26..=35).contains(&btc_addr.len()));

    // 🧬 Fingerprint SHA256: 64 caracteres hex
    assert!(fingerprint.len() == 64 && fingerprint.chars().all(|c| c.is_ascii_hexdigit()));

    // 🧪 Hash de derivação (Keccak): 64 caracteres hex
    assert!(hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()));

    println!("✅ Todos os critérios de validação foram atendidos.");
}