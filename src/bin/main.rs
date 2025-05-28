// 📂 src/bin/main.rs

use firechain_cli::cli::parser::Cli;
use firechain_cli::cli::handler::handle_command;
use clap::Parser;

/// 🚀 Ponto de entrada principal da FireChain CLI
fn main() {
    let cli = Cli::parse();
    handle_command(&cli.command);
}
