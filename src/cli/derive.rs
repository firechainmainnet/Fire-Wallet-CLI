// 📂 src/cli/derive.rs

use crate::cli::parser::DeriveArgs;
use crate::core::wallet::Wallet;
use crate::core::crypto::sha256;
use crate::core::address::{generate_btc_address, generate_eth_address, generate_fire_address};
use hex;

/// 🧬 Executa o comando `derive` com base nas flags fornecidas.
///
/// Gera múltiplos tipos de endereço (BTC, ETH, FireChain) com base na chave pública gerada.
pub fn execute(args: &DeriveArgs) {
    let wallet = Wallet::new();

    // 🧬 Fingerprint e ID da carteira
    let fingerprint_bytes = sha256(wallet.public_key.as_bytes());
    let fingerprint = hex::encode(&fingerprint_bytes[..6]);
    let wallet_id = &fingerprint[..12].to_uppercase();

    // 📍 Identidade base da carteira
    println!("🔐 Chave Pública  : {}", wallet.public_key);
    println!("🔒 Chave Privada  : {}", wallet.private_key);
    println!("🧬 Fingerprint     : {}", fingerprint);
    println!("🆔 Wallet ID       : FC-{}\n", wallet_id);

    // 🔎 Decodifica a pubkey para bytes (removendo "0x04" na função de destino)
    let pub_key_bytes = hex::decode(&wallet.public_key).expect("❌ Chave pública inválida!");

    // ₿ Endereço BTC
    if args.all || args.btc {
        let btc_address = generate_btc_address(&pub_key_bytes);
        println!("₿ Endereço BTC    : {}", btc_address);
    }

    // ⛓️ Endereço Ethereum
    if args.all || args.eth {
        let eth_address = generate_eth_address(&pub_key_bytes);
        println!("⛓️ Endereço ETH    : {}", eth_address);
    }

    // 🔥 Endereço FireChain
    if args.all || args.f1r3 {
        let fire_address = generate_fire_address(&pub_key_bytes);
        println!("🔥 Endereço F1R3   : {}", fire_address);
    }

    println!("\n✅ Derivação concluída com sucesso!");
    println!("⚠️  Nunca compartilhe sua chave privada. Guarde em local seguro.\n");
}
