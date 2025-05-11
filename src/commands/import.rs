use atty::Stream;
use colored::*;
use dialoguer::Password;
use prettytable::{Table, Row, Cell};
use std::path::Path;
use std::process;

use crate::secure::import_wallet_from_file;
use crate::wallet::Wallet;
use serde_json::{json, Value};

/// Execução tradicional via terminal
pub fn run(password: String, path: String, _unsafe_dump: bool) {
    let pw = if password == "-" {
        Password::new()
            .with_prompt("🔑 Digite a senha de importação")
            .interact()
            .unwrap_or_else(|e| {
                eprintln!("{}", format!("❌ Falha ao ler senha: {}", e).red());
                process::exit(1)
            })
    } else {
        password
    };

    match import_wallet_from_file(&pw, &path) {
        Ok(mut wallet) => {
            wallet.ensure_seed_loaded();
            println!("{}", "✅ Carteira importada com sucesso:".green());

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
                if _unsafe_dump {
                    println!("    PrivateKey: {}", addr.private_key);
                } else {
                    println!("    PrivateKey: [oculta]");
                }
            }

            if let Some(phrase) = &wallet.mnemonic_phrase {
                if _unsafe_dump {
                    println!("\n{}", format!("🧠 Frase: {}", phrase).cyan());
                } else {
                    println!("\n{}", "🧠 Frase: [oculta]".cyan());
                }
            }
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Falha na importação da carteira: {}", e).red());
            process::exit(1);
        }
    }
}

/// Execução programável via JSON/STDIN
pub fn run_json(password: String, path: String, unsafe_dump: bool) -> Result<Value, String> {
    let wallet_result = import_wallet_from_file(&password, &path);

    match wallet_result {
        Ok(mut wallet) => {
            wallet.ensure_seed_loaded();

            let mut result = json!({
                "addresses": [],
                "mnemonic_phrase": null
            });

            if let Some(addrs) = result.get_mut("addresses").and_then(|v| v.as_array_mut()) {
                for addr in wallet.addresses.iter() {
                    addrs.push(json!({
                        "address": addr.address,
                        "public_key": addr.public_key,
                        "private_key": if unsafe_dump { addr.private_key.clone() } else { "[oculta]".to_string() }
                    }));
                }
            }

            if let Some(phrase) = &wallet.mnemonic_phrase {
                result["mnemonic_phrase"] = if unsafe_dump {
                    json!(phrase)
                } else {
                    json!("[oculta]")
                };
            }

            Ok(result)
        }
        Err(e) => Err(format!("Falha na importação da carteira: {}", e)),
    }
}
