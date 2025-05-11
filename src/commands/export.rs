use colored::*;
use dialoguer::Password;
use std::fs::read_to_string;
use std::process;

use crate::secure::export_wallet_to_file;
use serde_json::Value;
use crate::wallet::Wallet;

/// Execução tradicional via terminal
pub fn run(input_json: String, password: String, output: String) {
    let pw = if password == "-" {
        Password::new()
            .with_prompt("🔑 Digite a senha de exportação")
            .with_confirmation("🔁 Confirme a senha", "As senhas não coincidem")
            .interact()
            .unwrap()
    } else {
        password
    };

    // (opcional) força de senha
    #[cfg(feature = "password_strength")]
    {
        let score = zxcvbn::zxcvbn(&pw, &[]).unwrap().score();
        if score < 3 {
            eprintln!("{}", format!("❌ Senha fraca (pontuação {}). Use uma senha mais forte.", score).red());
            process::exit(1);
        }
    }

    let content = read_to_string(&input_json).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Erro ao ler '{}': {}", input_json, e).red());
        process::exit(1)
    });

    let mut wallet: Wallet = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Erro ao parsear JSON: {}", e).red());
        process::exit(1)
    });

    wallet.ensure_seed_loaded();

    export_wallet_to_file(&wallet, &pw, &output).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Falha ao exportar: {}", e).red());
        process::exit(1)
    });

    println!("{}", format!("✅ Exportada como '{}'", output).green());
}

/// Execução programável via JSON/STDIN
pub fn run_json(input_json: String, password: String, output: String) -> Result<Value, String> {
    let content = read_to_string(&input_json)
        .map_err(|e| format!("Erro ao ler '{}': {}", input_json, e))?;

    let mut wallet: Wallet = serde_json::from_str(&content)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))?;

    wallet.ensure_seed_loaded();

    export_wallet_to_file(&wallet, &password, &output)
        .map_err(|e| format!("Falha ao exportar: {}", e))?;

    Ok(serde_json::json!({
        "status": "ok",
        "exported_to": output
    }))
}
