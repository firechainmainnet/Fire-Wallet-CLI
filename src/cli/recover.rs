use crate::core::wallet::Wallet;
use crate::utils::crypto::aes::encrypt_wallet;
use crate::FireError;

use std::fs;
use std::path::Path;
use colored::Colorize;
use serde_json::Value;

/// 📥 Argumentos esperados para o comando `recover`
pub struct RecoverArgs {
    pub input: String,          // Caminho do JSON
    pub password: String,       // Nova senha para criptografar
    pub out: Option<String>,    // Caminho de saída opcional
}

/// ♻️ Executa o processo de recuperação da carteira
pub fn handle_recover(args: RecoverArgs) -> Result<(), FireError> {
    println!("{}", "♻️ Iniciando recuperação da carteira FireChain...\n".bold());

    // 🧾 Valida existência do JSON
    if !Path::new(&args.input).exists() {
        return Err(FireError::FileNotFound(args.input.clone()));
    }

    let raw = fs::read_to_string(&args.input)?;
    let json: Value = serde_json::from_str(&raw)?;

    // 🔍 Extração dos campos esperados
    let fingerprint = json["fingerprint"].as_str().ok_or(FireError::ParseError)?;
    let pubkey = json["pubkey"].as_str().ok_or(FireError::ParseError)?;
    let privkey = json["privkey"].as_str().ok_or(FireError::ParseError)?;
    let btc = json["addresses"]["BTC"].as_str().ok_or(FireError::ParseError)?;
    let eth = json["addresses"]["ETH"].as_str().ok_or(FireError::ParseError)?;
    let fire = json["addresses"]["FIRECHAIN"].as_str().ok_or(FireError::ParseError)?;

    // 🔁 Recriação da estrutura Wallet
    let wallet = Wallet {
        fingerprint: fingerprint.to_string(),
        public_key: pubkey.to_string(),
        private_key: privkey.to_string(),
        address_btc: btc.to_string(),
        address_eth: eth.to_string(),
        address_firechain: fire.to_string(),
    };

    // 🔐 Recriptografar com a nova senha
    let encrypted = encrypt_wallet(&wallet, &args.password)?;

    let path = match &args.out {
        Some(p) => p.to_string(),
        None => format!("{}.wallet", fingerprint),
    };

    fs::write(&path, encrypted)?;

    println!("{}", "✅ Carteira restaurada com sucesso!".green().bold());
    println!(
        "{} {}",
        "📁 Arquivo salvo em:".dimmed(),
        path.as_str().yellow().bold()
    );

    Ok(())
}
