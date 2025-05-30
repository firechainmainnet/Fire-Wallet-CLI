use colored::*;
use crate::core::wallet::Wallet;
use crate::cli::parser::DeriveArgs;

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

/// 🎯 Exibe endereços derivados com base nas flags de derivação (--btc, --eth, --f1r3, --all)
pub fn print_wallet_summary_derive(wallet: &Wallet, args: &DeriveArgs) {
    println!("\n{}", "🎯 Carteira Derivada".bold().underline());

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

    if args.f1r3 || args.all {
        println!(
            "{} {}",
            "🔥 Endereço FireChain:".dimmed(),
            wallet.address_firechain.bold().green()
        );
    }

    if args.eth || args.all {
        println!(
            "{} {}",
            "⛓️ Endereço Ethereum:".dimmed(),
            wallet.address_eth.dimmed()
        );
    }

    if args.btc || args.all {
        println!(
            "{} {}",
            "₿ Endereço Bitcoin:".dimmed(),
            wallet.address_btc.dimmed()
        );
    }

    println!();
}
