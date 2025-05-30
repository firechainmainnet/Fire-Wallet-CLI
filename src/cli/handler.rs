use colored::Colorize;
use std::fs;
use std::path::Path;
use serde_json::{json, Value};

use crate::cli::parser::{NewArgs, DeriveArgs, ExportArgs, RecoverArgs, Commands};
use crate::core::wallet::{Wallet, decrypt_wallet, encrypt_wallet};
use crate::cli::format::{print_new_wallet_summary, print_wallet_summary_derive};
use crate::cli::derive::derive_addresses; // ✅ novo
use crate::FireError;

/// 🧠 Roteador principal da CLI FireChain
pub fn handle_cli(cli: crate::cli::parser::Cli) -> Result<(), FireError> {
    match cli.command {
        Commands::New(args) => {
            handle_new_command(&args)?;
            Ok(())
        },
        Commands::Derive(args) => {
            handle_derive_command(&args)?;
            Ok(())
        },
        Commands::Export(args) => {
            handle_export_command(&args)?;
            Ok(())
        },
        Commands::Recover(args) => {
            handle_recover_command(&args)?;
            Ok(())
        },
        Commands::Help => {
            println!("{}", "ℹ️ Use --help para ver os comandos disponíveis.".blue());
            Ok(())
        }
    }
}

//
// 🔐 Comandos Individuais
//

/// 🔐 Geração de nova carteira com exportação automática
pub fn handle_new_command(args: &NewArgs) -> Result<(), FireError> {
    println!("{}", "🔐 Iniciando criação da carteira FireChain...\n".bold());

    let (private_key, public_key, fingerprint) = Wallet::generate_wallet_identity();
    let wallet = Wallet::new(private_key.clone(), public_key.clone(), fingerprint.clone());

    let eth_address = &wallet.address_eth;
    let btc_address = &wallet.address_btc;
    let fire_address = &wallet.address_firechain;

    print_new_wallet_summary(&wallet, eth_address, btc_address, fire_address);

    let output_path = match &args.out {
        Some(path) if !path.trim().is_empty() => path.clone(),
        _ => format!("{}.wallet", wallet.fingerprint),
    };

    let encrypted_data = encrypt_wallet(&wallet, &args.password)?;
    fs::write(&output_path, encrypted_data)?;

    println!("{}", "✅ Carteira criada e criptografada com sucesso!\n".green().bold());
    println!(
        "{} {}",
        "📁 Arquivo salvo em:".dimmed(),
        output_path.as_str().bold().yellow()
    );

    Ok(())
}

/// 🧬 Deriva endereços (BTC, ETH, FIRECHAIN) a partir de uma carteira `.wallet`
pub fn handle_derive_command(args: &DeriveArgs) -> Result<(), FireError> {
    // ✅ Retorna erro se nenhuma flag estiver ativa (exibido aqui com UX)
    if !args.btc && !args.eth && !args.f1r3 && !args.all {
        println!(
            "{}",
            "⚠️ Nenhuma flag de derivação foi passada. Use --btc, --eth, --f1r3 ou --all.".yellow()
        );
        return Ok(());
    }

    // 🧠 Lógica agora delegada ao derive.rs
    let wallet = derive_addresses(args)?;

    // 🎨 Impressão feita aqui no handler (UX separada da lógica)
    print_wallet_summary_derive(&wallet, args);

    Ok(())
}

/// 📤 Exporta uma carteira `.wallet` descriptografando localmente
pub fn handle_export_command(args: &ExportArgs) -> Result<(), FireError> {
    if !args.json {
        println!("{}", "📤 Iniciando exportação da carteira FireChain...\n".bold());
    }

    if !Path::new(&args.input).exists() {
        return Err(FireError::FileNotFound(args.input.clone()));
    }

    let encrypted_bytes = fs::read(&args.input)?;
    let wallet = decrypt_wallet(&encrypted_bytes, &args.password)?;

    if args.json {
        let export_json = json!({
            "version": "0.1.4",
            "fingerprint": wallet.fingerprint,
            "pubkey": wallet.public_key,
            "privkey": wallet.private_key,
            "addresses": {
                "BTC": wallet.address_btc,
                "ETH": wallet.address_eth,
                "FIRECHAIN": wallet.address_firechain,
            }
        });

        println!("{}", serde_json::to_string_pretty(&export_json)?);
        return Ok(());
    }

    println!("{}", "🔓 Carteira descriptografada com sucesso!\n".green().bold());
    println!("{} {}", "🔐 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧠 Chave Pública:".bold(), wallet.public_key);
    println!("{} {}", "🔥 FireChain:".bold(), wallet.address_firechain);
    println!("{} {}", "⛓️ Ethereum:".bold(), wallet.address_eth);
    println!("{} {}", "₿ Bitcoin:".bold(), wallet.address_btc);
    println!("\n{} {}", "🔖 Versão:".dimmed(), "0.1.4".yellow());
    println!("{}", "⚠️  A chave privada não é exibida no modo padrão.".dimmed());

    Ok(())
}

/// ♻️ Restaura uma carteira FireChain a partir de um JSON exportado
pub fn handle_recover_command(args: &RecoverArgs) -> Result<(), FireError> {
    println!("{}", "♻️ Iniciando recuperação da carteira FireChain...\n".bold());

    if !Path::new(&args.input).exists() {
        return Err(FireError::FileNotFound(args.input.clone()));
    }

    let raw = fs::read_to_string(&args.input)?;
    let json: Value = serde_json::from_str(&raw)?;

    let fingerprint = json["fingerprint"].as_str().unwrap_or_default();
    let pubkey = json["pubkey"].as_str().unwrap_or_default();
    let privkey = json["privkey"].as_str().unwrap_or_default();

    if fingerprint.is_empty() || pubkey.is_empty() || privkey.is_empty() {
        return Err(FireError::ParseError);
    }

    let wallet = Wallet::new(privkey.to_string(), pubkey.to_string(), fingerprint.to_string());
    let encrypted = encrypt_wallet(&wallet, &args.password)?;

    let output_path = match &args.out {
        Some(p) => p.clone(),
        None => format!("{}.wallet", wallet.fingerprint),
    };

    fs::write(&output_path, encrypted)?;

    println!("{}", "✅ Carteira restaurada com sucesso!\n".green().bold());
    println!(
        "{} {}",
        "📁 Arquivo salvo em:".dimmed(),
        output_path.as_str().bold().yellow()
    );

    Ok(())
}
