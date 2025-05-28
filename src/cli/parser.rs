// 📂 src/cli/parser.rs

use clap::{Parser, Subcommand, Args};

/// 🧬 FireChain CLI — Carteira Web3 com foco em segurança, modularidade e UX premium.
///
/// CLI profissional para geração, derivação e gestão de identidades blockchain.
/// Ideal para devs Web3, apps self-custodial, validadores e operações seguras.
#[derive(Parser)]
#[command(name = "firechain-cli")]
#[command(author = "Guilherme Lima")]
#[command(version = "0.1.2")]
#[command(
    about = "🔥 FireChain CLI — Carteira Web3 com foco em segurança e modularidade.",
    long_about = r#"
🧬 CLI premium para geração, derivação e gestão de identidades blockchain.

Comandos disponíveis:
  🔐 new      → Gera uma nova carteira (priv/pub/address base58)
  🧬 derive   → Deriva múltiplos endereços compatíveis (BTC, ETH, FireChain)

Exemplo de uso:
  firechain-cli new
  firechain-cli derive --all
  firechain-cli derive --btc

Desenvolvido com segurança client-side e UX profissional.
"#
)]
pub struct Cli {
    /// Subcomando a ser executado
    #[command(subcommand)]
    pub command: Commands,
}

/// 📦 Subcomandos disponíveis na FireChain CLI
#[derive(Subcommand)]
pub enum Commands {
    /// 🔐 Gera uma nova carteira FireChain
    New,

    /// 🧬 Deriva múltiplos endereços (BTC, ETH, FireChain)
    Derive(DeriveArgs),

    /// ℹ️ Mostra ajuda detalhada da FireChain CLI
    #[command(hide = true)]
    Help,
}

/// ⚙️ Flags disponíveis para o comando `derive`
#[derive(Args)]
pub struct DeriveArgs {
    /// Gera endereço BTC (Base58)
    #[arg(long)]
    pub btc: bool,

    /// Gera endereço Ethereum (0x-prefixed)
    #[arg(long)]
    pub eth: bool,

    /// Gera endereço FireChain personalizado
    #[arg(long)]
    pub f1r3: bool,

    /// Gera todos os formatos disponíveis
    #[arg(long)]
    pub all: bool,
}
