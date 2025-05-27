//! 🧪 Teste de geração completa de uma carteira FireChain
//! 🔍 Valida chaves, endereços e metadados com padrão enterprise

use firechain_cli::wallet::generate_keypair;

#[test]
fn test_generate_keypair_all_formats_valid() {
    let (priv_hex, pub_hex, fire_addr, eth_addr, btc_addr, fingerprint, hash_keccak) = generate_keypair();

    println!("\n🔎 [FireChain Teste] Resultado da geração de carteira:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 Chave privada (hex)      : {}", priv_hex);
    println!("🔓 Chave pública (hex)      : {}", pub_hex);
    println!("📬 Endereço (Fire)          : {}", fire_addr);
    println!("🌐 Endereço (Ethereum)      : {}", eth_addr);
    println!("₿  Endereço (Bitcoin)       : {}", btc_addr);
    println!("🧬 Fingerprint SHA256       : {}", fingerprint);
    println!("🧪 Hash de Derivação Keccak : {}", hash_keccak);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // ✅ FireChain Address
    assert!(fire_addr.starts_with("f1r3"));
    let base58_part = &fire_addr[4..];
    assert!((26..=35).contains(&base58_part.len()), "Endereço FireChain com base58 inválido");

    // 🌐 Ethereum
    assert!(eth_addr.starts_with("0x"));
    assert_eq!(eth_addr.len(), 42);

    // ₿ Bitcoin
    assert!(btc_addr.starts_with('1') || btc_addr.starts_with('m') || btc_addr.starts_with('n'));
    assert!((26..=35).contains(&btc_addr.len()), "Comprimento do endereço BTC inválido");

    // 🧬 Fingerprint SHA256
    assert_eq!(fingerprint.len(), 64);

    // 🔗 Hash derivação Keccak
    assert_eq!(hash_keccak.len(), 64);
}
