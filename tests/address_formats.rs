//! 🔍 Testes de validação direta dos formatos de endereço gerado a partir da chave pública

use firechain_cli::utils::address::{
    public_key_to_fire_address,
    public_key_to_eth_address,
    public_key_to_btc_address,
};

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;

#[test]
fn test_address_format_generation_from_pubkey() {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let (_secret_key, public_key) = secp.generate_keypair(&mut rng);

    let fire = public_key_to_fire_address(&public_key);
    let eth  = public_key_to_eth_address(&public_key);
    let btc  = public_key_to_btc_address(&public_key);

    println!("\n🔍 Teste direto de formatação de endereços:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📬 FireChain : {}", fire);
    println!("🌐 Ethereum  : {}", eth);
    println!("₿  Bitcoin   : {}", btc);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // ✅ FireChain: prefixo correto e comprimento entre 30–40 (f1r3 + base58)
    assert!(fire.starts_with("f1r3"));
    let base58_part = &fire[4..];
    assert!((26..=35).contains(&base58_part.len()), "Comprimento base58 inválido");

    // 🌐 Ethereum: 42 chars e começa com 0x
    assert!(eth.starts_with("0x"));
    assert_eq!(eth.len(), 42);

    // ₿ Bitcoin: prefixo '1' e comprimento base58 típico
    assert!(btc.starts_with('1') || btc.starts_with('m') || btc.starts_with('n'));
    assert!((26..=35).contains(&btc.len()), "Comprimento Bitcoin inválido");
}
