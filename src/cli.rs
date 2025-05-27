//! 🧭 Interface de linha de comando da FireChain CLI
//! 🎯 Usa `clap` para oferecer subcomandos com experiência premium

use clap::{Parser, Subcommand};

/// 🔥 FireChain CLI - Carteira Web3 segura com experiência premium
#[derive(Parser, Debug)]
#[command(
    name = "firechain-cli",
    author = "Guilherme Lima <https://github.com/firechainmainnet>",
    version,
    about = "🧬 FireChain CLI — Geração e gestão de carteiras blockchain com segurança e elegância.",
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

/// 🎯 Subcomandos disponíveis na FireChain CLI
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 🔐 Gera uma nova carteira FireChain
    New,
}