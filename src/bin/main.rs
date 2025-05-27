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

    // ✅ Geração segura da carteira
    let (privkey, pubkey, fire, eth, btc, fingerprint, derivation_hash) = generate_keypair();

    println!("✅ Carteira gerada com sucesso!\n");

    // 🔑 CHAVES
    println!("━━━━━━━━━━━━━━━━━━━━ 🔑 CHAVES ━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 Private Key (hex) : {}", privkey);
    println!("🔓 Public  Key (hex) : {}", pubkey);

    // 📬 ENDEREÇOS
    println!("\n━━━━━━━━━━━━━━━━━━ 📬 ENDEREÇOS ━━━━━━━━━━━━━━━━━━");
    println!("📬 Endereço (Fire)     : {}", fire); // ✅ Já vem com `f1r3` + base58check
    println!("🌐 Endereço (Ethereum) : {}", eth);
    println!("₿  Endereço (Bitcoin)  : {}", btc);

    // 🧬 METADADOS
    println!("\n━━━━━━━━━━━━━━━━━━ 🧬 METADADOS ━━━━━━━━━━━━━━━━━━");
    println!("🧬 Fingerprint SHA256        : {}", fingerprint);
    println!("🔗 Hash de Derivação (Keccak): {}", derivation_hash);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 🔐 Aviso de segurança
    println!("\n🔒 Use com segurança. Backup é sua responsabilidade.");
}
