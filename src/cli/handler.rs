// 📂 src/cli/handler.rs

use crate::cli::parser::Commands;
use crate::cli::derive;
use crate::core::wallet::Wallet;
use crate::core::crypto::sha256;
use hex;

/// 🧠 Lógica dos comandos executáveis via CLI
pub fn handle_command(command: &Commands) {
    match command {
        Commands::New => {
            let wallet = Wallet::new();

            // Gera fingerprint curta da chave pública (primeiros 6 bytes do SHA256)
            let fingerprint_bytes = sha256(wallet.public_key.as_bytes());
            let fingerprint = hex::encode(&fingerprint_bytes[..6]);
            let wallet_id = &fingerprint[..12].to_uppercase();

            println!("\n🔥 FireChain CLI");
            println!("🧬 Segurança blockchain com modularidade, criptografia e elegância CLI-first\n");

            println!("🔐 Chave Pública : {}", wallet.public_key);
            println!("🔒 Chave Privada : {}", wallet.private_key);
            println!("🧬 Fingerprint    : {}", fingerprint);
            println!("🆔 Wallet ID      : FC-{}\n", wallet_id);

            println!("🎯 Use o comando `derive` para gerar endereços BTC, ETH e FireChain.");
        }

        Commands::Derive(args) => {
            derive::execute(args);
        }

        Commands::Help => {
            println!("ℹ️  Use `firechain-cli <comando>` para acessar funcionalidades:");
            println!("  • new      🔐 Gera uma nova carteira");
            println!("  • derive   📡 Deriva múltiplos endereços (BTC, ETH, F1R3)");
            println!("  • help     ℹ️ Mostra esta ajuda\n");
        }
    }
}
