use crate::core::wallet::decrypt_wallet;
use crate::utils::format::{bold, green, yellow, dimmed};
use crate::core::wallet::Wallet;
use serde_json::json;

use std::fs;
use std::path::Path;

use crate::FireError;

/// Argumentos esperados para o subcomando `export`
pub struct ExportArgs {
    pub input: String,     // Caminho do arquivo `.wallet`
    pub password: String,  // Senha de descriptografia
    pub as_json: bool,     // Se true, saída será em JSON estruturado
}

/// Handler principal do comando `export`
pub fn handle_export(args: ExportArgs) -> Result<(), FireError> {
    // 🚨 Validação inicial do caminho
    if !Path::new(&args.input).exists() {
        return Err(FireError::FileNotFound(args.input.clone()));
    }

    // 📥 Leitura do conteúdo criptografado
    let encrypted_bytes = fs::read(&args.input)
        .map_err(|_| FireError::ReadError(args.input.clone()))?;

    // 🔐 Descriptografar utilizando a senha fornecida
    let wallet: Wallet = decrypt_wallet(&encrypted_bytes, &args.password)?;

    // 🔐 Impressão bonita no terminal OU saída JSON
    if args.as_json {
        let export_json = json!({
            "version": "0.1.4",
            "fingerprint": wallet.fingerprint,
            "pubkey": wallet.public_key,
            "privkey": wallet.private_key, // ⚠️ Exibido apenas em modo JSON
            "addresses": {
                "BTC": wallet.address_btc,
                "ETH": wallet.address_eth,
                "FIRECHAIN": wallet.address_firechain,
            }
        });

        println!("{}", serde_json::to_string_pretty(&export_json)?);
    } else {
        println!("{}", bold("🔓 Carteira descriptografada com sucesso:\n"));
        println!("{} {}", bold("🔐 Fingerprint:"), green(&wallet.fingerprint));
        println!("{} {}", bold("🧠 Chave Pública:"), &wallet.public_key);
        println!("{} {}", bold("🔥 FireChain:"), &wallet.address_firechain);
        println!("{} {}", bold("⛓️ Ethereum:"), &wallet.address_eth);
        println!("{} {}", bold("₿ Bitcoin:"), &wallet.address_btc);
        println!("\n{} {}", dimmed("Versão:"), yellow("0.1.4"));
        println!("⚠️  A chave privada não é exibida no modo padrão.");
    }

    Ok(())
}
