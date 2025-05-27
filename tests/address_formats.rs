//! 🧪 Teste de formatação de endereços a partir de chave pública fixa
//! 🎯 Garante consistência determinística nos formatos Fire, Ethereum e Bitcoin

use firechain_cli::utils::address::{
    public_key_to_address_firechain, public_key_to_address_eth, public_key_to_address_btc,
};

#[test]
fn test_address_format_generation_from_pubkey() {
    let pub_key_hex = "04a34f5d8a5a16b927a0a537dbb5b5c5b162404ce3c7e55dfb7d1990e0bc1a77a02149f524eab5e24e003d1e8c4bb6e2aefc9138702899249bc3c4798e7f7e2a6b";

    let fire = public_key_to_address_firechain(pub_key_hex);
    let eth  = public_key_to_address_eth(pub_key_hex);
    let btc  = public_key_to_address_btc(pub_key_hex);

    println!("\n🔍 Teste direto de formatação de endereços:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📬 FireChain : {}", fire);
    println!("🌐 Ethereum  : {}", eth);
    println!("₿  Bitcoin   : {}", btc);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Valida prefixo personalizado
    assert!(fire.starts_with("f1r3:"));

    // Valida padrão 0x + 40 hex
    assert!(eth.starts_with("0x") && eth.len() == 42);

    // Valida padrão base58 com prefixo 1
    assert!(btc.starts_with('1') && btc.len() >= 26 && btc.len() <= 35);
}
