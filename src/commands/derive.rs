use crate::wallet::Wallet;
use colored::*;
use std::fs::{read_to_string, write};
use std::process;
use serde_json::{json, Value};

/// Execução tradicional via terminal
pub fn run(input_wallet: String, _password: String, output: Option<String>, unsafe_dump: bool) {
    let contents = read_to_string(&input_wallet).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Falha ao ler '{}': {}", input_wallet, e).red());
        process::exit(1)
    });

    let mut wallet: Wallet = serde_json::from_str(&contents).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Erro ao carregar carteira: {}", e).red());
        process::exit(1)
    });

    wallet.ensure_seed_loaded();
    if wallet.mnemonic_phrase.is_none() {
        eprintln!("{}", "❌ Esta carteira não possui frase mnemônica.".red());
        process::exit(1);
    }

    let addr = wallet.derive_next_address().unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Erro na derivação HD: {}", e).red());
        process::exit(1)
    });

    println!("{}", "➕ Novo endereço derivado:".green());
    println!("    Address:    {}", addr.address);
    println!("    PublicKey:  {}", addr.public_key);
    if unsafe_dump {
        println!("    PrivateKey: {}", addr.private_key);
    } else {
        println!("    PrivateKey: [oculta]");
    }
    println!("{}", format!("🔢 Índice HD atual: {}", wallet.hd_index - 1).cyan());

    let json = serde_json::to_string_pretty(&wallet).unwrap_or_else(|e| {
        eprintln!("{}", format!("❌ Erro ao serializar carteira: {}", e).red());
        process::exit(1)
    });

    if let Some(path) = output {
        write(&path, &json).unwrap_or_else(|e| {
            eprintln!("{}", format!("❌ Erro ao salvar '{}': {}", path, e).red());
            process::exit(1)
        });
        println!("{}", format!("💾 Atualizado: '{}'", path).green());
    } else {
        eprintln!("{}", "❌ Parâmetro --output é obrigatório.".red());
        process::exit(1);
    }
}

/// Execução programável via JSON/STDIN
pub fn run_json(input_wallet: String, _password: String, output: Option<String>, unsafe_dump: bool) -> Result<Value, String> {
    let contents = read_to_string(&input_wallet)
        .map_err(|e| format!("Erro ao ler '{}': {}", input_wallet, e))?;

    let mut wallet: Wallet = serde_json::from_str(&contents)
        .map_err(|e| format!("Erro ao carregar carteira: {}", e))?;

    wallet.ensure_seed_loaded();
    if wallet.mnemonic_phrase.is_none() {
        return Err("Esta carteira não possui frase mnemônica.".to_string());
    }

    let addr = wallet.derive_next_address()
        .map_err(|e| format!("Erro na derivação HD: {}", e))?;

    // Se `output` estiver presente, salva o novo estado da carteira com hd_index atualizado
    if let Some(path) = output {
        let json = serde_json::to_string_pretty(&wallet)
            .map_err(|e| format!("Erro ao serializar: {}", e))?;

        write(&path, &json)
            .map_err(|e| format!("Erro ao salvar '{}': {}", path, e))?;
    }

    let mut response = json!({
        "address": addr.address,
        "public_key": addr.public_key,
        "hd_index": wallet.hd_index - 1,
    });

    if unsafe_dump {
        response.as_object_mut().unwrap().insert("private_key".to_string(), json!(addr.private_key));
    } else {
        response.as_object_mut().unwrap().insert("private_key".to_string(), json!("[oculta]"));
    }

    Ok(response)
}
