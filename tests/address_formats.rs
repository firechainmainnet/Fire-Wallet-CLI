use firechain_cli::utils::address::{
    public_key_to_fire_address,
    public_key_to_eth_address,
    public_key_to_btc_address,
};

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;

#[test]
fn test_address_format_generation_from_pubkey() {
    // ✅ Gerar uma chave pública válida em tempo real
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    // ✅ Testar os formatos de endereço
    let fire = public_key_to_fire_address(&public_key);
    let eth  = public_key_to_eth_address(&public_key);
    let btc  = public_key_to_btc_address(&public_key);

    println!("\n🔍 Teste direto de formatação de endereços:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📬 FireChain : {}", fire);
    println!("🌐 Ethereum  : {}", eth);
    println!("₿  Bitcoin   : {}", btc);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    assert!(fire.starts_with("f1r3:") && fire.len() > 20);
    assert!(eth.starts_with("0x") && eth.len() == 42);
    assert!(btc.starts_with('1') && (26..=35).contains(&btc.len()));
}
