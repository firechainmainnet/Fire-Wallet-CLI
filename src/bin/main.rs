//! 🚀 Entrada principal da FireChain CLI
//! 📦 Interpreta subcomandos e executa as ações correspondentes

use firechain_cli::cli::{CliArgs, Commands};
use firechain_cli::wallet::generate_keypair;
use clap::Parser;

fn main() {
    let args = CliArgs::parse();

    match args.command {
        Commands::New => run_generate_wallet(),
    }
}

/// 🧬 Executa a geração da carteira FireChain com UX visual
fn run_generate_wallet() {
    println!("🧬 FireChain Wallet Generator");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 Gerando carteira segura com padrões de produção...\n");

    let (privkey, pubkey, fire, eth, btc, fingerprint, derivation_hash) = generate_keypair();

    println!("✅ Carteira gerada com sucesso!\n");

    println!("━━━━━━━━━━━━━━━━━━━━ 🔑 CHAVES ━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 Private Key (hex) : {}", privkey);
    println!("🔓 Public  Key (hex) : {}", pubkey);

    println!("\n━━━━━━━━━━━━━━━━━━ 📬 ENDEREÇOS ━━━━━━━━━━━━━━━━━━");
    println!("📬 Endereço (Fire)     : {}", fire);
    println!("🌐 Endereço (Ethereum) : {}", eth);
    println!("₿  Endereço (Bitcoin)  : {}", btc);

    println!("\n━━━━━━━━━━━━━━━━━━ 🧬 METADADOS ━━━━━━━━━━━━━━━━━━");
    println!("🧬 Fingerprint SHA256        : {}", fingerprint);
    println!("🔗 Hash de Derivação (Keccak): {}", derivation_hash);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n🔒 Use com segurança. Backup é sua responsabilidade.");
}
