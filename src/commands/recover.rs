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
pub fn run(words: String, label: Option<String>, unsafe_dump: bool) {
    let mut wallet = match Wallet::restore_from_mnemonic(label.clone(), &words) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{}", format!("❌ Erro ao restaurar carteira: {}", e).red());
            return;
        }
    };

    wallet.ensure_seed_loaded();
    println!("{}", "🧠 Carteira Restaurada:".bold());
    print_wallet(&wallet, unsafe_dump);

    if let Some(base_label) = &label {
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
pub fn run_json(words: String, label: Option<String>, unsafe_dump: bool) -> Result<Value, String> {
    let mut wallet = Wallet::restore_from_mnemonic(label.clone(), &words)
        .map_err(|e| format!("Erro ao restaurar carteira: {}", e))?;

    wallet.ensure_seed_loaded();

    let mut wallet_json = serde_json::to_value(&wallet).map_err(|e| e.to_string())?;

    if !unsafe_dump {
        // Oculta chaves privadas
        if let Some(addrs) = wallet_json.get_mut("addresses").and_then(|v| v.as_array_mut()) {
            for addr in addrs {
                if let Some(obj) = addr.as_object_mut() {
                    obj.insert("private_key".to_string(), json!("[oculta]"));
                }
            }
        }

        // Oculta frase mnemônica
        if let Some(obj) = wallet_json.as_object_mut() {
            obj.insert("mnemonic_phrase".to_string(), json!("[oculta]"));
        }
    }

    Ok(wallet_json)
}

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
