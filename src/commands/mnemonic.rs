use crate::wallet::Wallet;
use atty::Stream;
use colored::*;
use dialoguer::Confirm;
use prettytable::{Table, Row, Cell};
use std::fs::write;
use std::path::Path;
use std::process;
use serde_json::{json, Value};

/// Execução tradicional via terminal
pub fn run(words: u8, label: Option<String>, unsafe_dump: bool) {
    let wallet = match words {
        12 => Wallet::new_mnemonic(label.clone(), 12).unwrap_or_else(|e| {
            eprintln!("{}", format!("❌ Erro ao gerar carteira: {}", e).red());
            process::exit(1)
        }),
        24 => Wallet::new_mnemonic(label.clone(), 24).unwrap_or_else(|e| {
            eprintln!("{}", format!("❌ Erro ao gerar carteira: {}", e).red());
            process::exit(1)
        }),
        _ => {
            eprintln!("{}", "❌ Apenas 12 ou 24 palavras são suportadas.".red());
            process::exit(1);
        }
    };

    println!("{}", format!("🧠 Carteira Mnemônica ({} palavras):", words).bold());
    print_wallet(&wallet, unsafe_dump);

    if let Some(base_label) = label {
        let filename = format!("{}.json", base_label);

        if Path::new(&filename).exists() && atty::is(Stream::Stdin) {
            if !Confirm::new()
                .with_prompt(format!("Arquivo '{}' já existe. Sobrescrever?", filename))
                .interact()
                .unwrap()
            {
                println!("{}", "⚠️ Operação cancelada.".yellow());
                return;
            }
        }

        let json = serde_json::to_string_pretty(&wallet)
            .unwrap_or_else(|e| {
                eprintln!("{}", format!("❌ Erro ao serializar: {}", e).red());
                process::exit(1)
            });

        write(&filename, &json)
            .unwrap_or_else(|e| {
                eprintln!("{}", format!("❌ Erro ao salvar '{}': {}", filename, e).red());
                process::exit(1)
            });

        println!("{}", format!("💾 Salvo como '{}'", filename).green());
    }
}

/// Execução programável via JSON/STDIN
pub fn run_json(words: u8, label: Option<String>, unsafe_dump: bool) -> Result<Value, String> {
    let wallet = match words {
        12 => Wallet::new_mnemonic(label.clone(), 12)
            .map_err(|e| format!("Erro ao gerar carteira: {}", e))?,
        24 => Wallet::new_mnemonic(label.clone(), 24)
            .map_err(|e| format!("Erro ao gerar carteira: {}", e))?,
        _ => return Err("Somente 12 ou 24 palavras são suportadas".to_string()),
    };

    // Salva .json se label estiver presente
    if let Some(base_label) = &label {
        let filename = format!("{}.json", base_label);
        let json_str = serde_json::to_string_pretty(&wallet)
            .map_err(|e| format!("Erro ao serializar carteira: {}", e))?;

        std::fs::write(&filename, &json_str)
            .map_err(|e| format!("Erro ao salvar '{}': {}", filename, e))?;
    }

    let mut wallet_json = serde_json::to_value(&wallet).map_err(|e| e.to_string())?;

    if !unsafe_dump {
        if let Some(addrs) = wallet_json.get_mut("addresses").and_then(|v| v.as_array_mut()) {
            for addr in addrs {
                if let Some(obj) = addr.as_object_mut() {
                    obj.insert("private_key".to_string(), json!("[oculta]"));
                }
            }
        }

        if let Some(obj) = wallet_json.as_object_mut() {
            obj.insert("mnemonic_phrase".to_string(), json!("[oculta]"));
        }
    }

    Ok(wallet_json)
}

/// Utilitário de impressão para CLI
fn print_wallet(wallet: &Wallet, unsafe_dump: bool) {
    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("Índice"),
        Cell::new("Address"),
        Cell::new("PublicKey"),
    ]));
    for (i, addr) in wallet.addresses.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{}", i + 1)),
            Cell::new(&addr.address),
            Cell::new(&addr.public_key),
        ]));
    }
    table.printstd();

    for addr in wallet.addresses.iter() {
        if unsafe_dump {
            println!("    PrivateKey: {}", addr.private_key);
        } else {
            println!("    PrivateKey: [oculta]");
        }
    }
}
