use clap::Parser;
use colored::Colorize;

use firechain_cli::cli::parser::Cli;
use firechain_cli::cli::handler::handle_cli;

fn main() {
    let cli = Cli::parse();

    // 🔁 Roteia e executa o comando via handler central
    if let Err(e) = handle_cli(cli) {
        eprintln!("{} {}", "❌ Erro ao executar comando:".red(), e.to_string().dimmed());
        std::process::exit(1);
    }
}
