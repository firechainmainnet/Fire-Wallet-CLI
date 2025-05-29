use colored::Colorize;
use hex;
use crate::core::wallet::Wallet;
use crate::core::address::{generate_btc_address, generate_eth_address, generate_fire_address};
use crate::cli::parser::NewArgs;
use crate::utils::crypto::aes::encrypt_wallet;
use crate::cli::format::print_new_wallet_summary;

pub fn handle_new_command(args: &NewArgs) {
    println!("{}", "🔐 Iniciando criação da carteira FireChain...\n".bold());

    let (private_key, public_key, fingerprint) = Wallet::generate_wallet_identity();
    let wallet = Wallet::new(private_key.clone(), public_key.clone(), fingerprint.clone());

    let pubkey_bytes = hex::decode(&public_key).unwrap();

    let eth_address = generate_eth_address(&pubkey_bytes);
    let btc_address = generate_btc_address(&pubkey_bytes);
    let fire_address = generate_fire_address(&pubkey_bytes);

    print_new_wallet_summary(&wallet, &eth_address, &btc_address, &fire_address);

    // 📁 Se --out for None, usamos <fingerprint>.wallet
    let output_path = match &args.out {
        Some(path) if !path.trim().is_empty() => path.clone(),
        _ => format!("{}.wallet", wallet.fingerprint),
    };

    match encrypt_wallet(&wallet, &args.password) {
        Ok(encrypted_data) => {
            match std::fs::write(&output_path, encrypted_data) {
                Ok(_) => {
                    println!("{}", "✅ Carteira criada e criptografada com sucesso!\n".green().bold());
                    println!(
                        "{} {}",
                        "📁 Arquivo salvo em:".dimmed(),
                        output_path.as_str().bold().yellow()
                    );
                }
                Err(e) => {
                    eprintln!("{} {}", "❌ Erro ao salvar a carteira:".red(), e);
                }
            }
        }
        Err(e) => {
            eprintln!("{} {}", "❌ Erro ao criptografar a carteira:".red(), e);
        }
    }
}
