use crate::multisig::MultisigWallet;
use colored::*;
use dialoguer::Confirm;
use prettytable::{Table, Row, Cell};
use std::fs::write;
use std::process;
use serde_json::{json, Value};

/// Execução tradicional via terminal
pub fn run(m_required: usize, public_keys: Vec<String>) {
    match MultisigWallet::new(m_required, public_keys.clone()) {
        Ok(ms) => {
            println!("{}", "🔐 Carteira Multisig criada:".bold());
            println!("{}", format!("Address: {}", ms.address).green());
            println!("{}", format!("M requerido: {}", ms.m_required).cyan());

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Participante #"),
                Cell::new("PublicKey"),
            ]));
            for (i, pk) in ms.public_keys.iter().enumerate() {
                table.add_row(Row::new(vec![
                    Cell::new(&format!("{}", i + 1)),
                    Cell::new(pk),
                ]));
            }
            table.printstd();

            let label = format!("multisig_{}of{}", ms.m_required, ms.public_keys.len());
            let filename = format!("{}.json", label);

            if !Confirm::new()
                .with_prompt(format!("Salvar '{}'? ", filename))
                .default(true)
                .interact()
                .unwrap() 
            {
                println!("{}", "⚠️ Cancelado.".yellow());
                return;
            }

            let json = serde_json::to_string_pretty(&ms)
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
        Err(e) => {
            eprintln!("{}", format!("❌ Erro ao criar multisig: {}", e).red());
            process::exit(1);
        }
    }
}

/// Execução programável via JSON/STDIN
pub fn run_json(m_required: usize, public_keys: Vec<String>) -> Result<Value, String> {
    let ms = MultisigWallet::new(m_required, public_keys.clone())
        .map_err(|e| format!("Erro ao criar multisig: {}", e))?;

    Ok(json!({
        "address": ms.address,
        "m_required": ms.m_required,
        "public_keys": ms.public_keys,
    }))
}
