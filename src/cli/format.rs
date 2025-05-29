use colored::*;
use crate::core::wallet::Wallet;

/// 🎨 Exibe a carteira recém-criada com estilo premium FireChain
pub fn print_new_wallet_summary(
    wallet: &Wallet,
    eth_address: &str,
    btc_address: &str,
    fire_address: &str,
) {
    println!("\n{}", "📦 Carteira Gerada com Sucesso:".bold().underline());

    println!(
        "{} {}",
        "🔐 Fingerprint:".dimmed(),
        wallet.fingerprint.bold().yellow()
    );
    println!(
        "{} {}",
        "🧠 Chave Pública:".dimmed(),
        wallet.public_key.bold().blue()
    );
    println!(
        "{} {}",
        "🔒 Chave Privada:".dimmed(),
        wallet.private_key.bold().red()
    );
    println!(
        "{} {}",
        "🔥 Endereço FireChain:".dimmed(),
        fire_address.bold().green()
    );
    println!(
        "{} {}",
        "⛓️ Endereço Ethereum:".dimmed(),
        eth_address.dimmed()
    );
    println!(
        "{} {}",
        "₿ Endereço Bitcoin:".dimmed(),
        btc_address.dimmed()
    );
    println!();
}

/// 🎯 Usado pelo comando `derive` para exibir identidade derivada sem salvar
pub fn print_wallet_summary_derive(
    fingerprint: &str,
    public_key: &str,
    fire_address: &str,
) {
    println!("\n{}", "🎯 Carteira Derivada".bold().underline());

    println!(
        "{} {}",
        "🔐 Fingerprint:".dimmed(),
        fingerprint.bold().yellow()
    );
    println!(
        "{} {}",
        "🧠 Chave Pública:".dimmed(),
        public_key.bold().blue()
    );
    println!(
        "{} {}",
        "🔥 Endereço FireChain:".dimmed(),
        fire_address.bold().green()
    );
    println!();
}
