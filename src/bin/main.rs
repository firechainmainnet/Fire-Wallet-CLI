use clap::Parser;
use colored::Colorize; // ✅ Importa o trait que habilita `.dimmed()`
use firechain_cli::cli::parser::{Cli, Commands};
use firechain_cli::cli::handler::handle_new_command;
use firechain_cli::cli::derive::handle_derive_command;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New(ref args) => {
            handle_new_command(args);
        }
        Commands::Derive(ref args) => {
            handle_derive_command(args);
        }
        Commands::Help => {
            println!("{}", "🆘 Use `firechain-cli --help` para consultar os comandos disponíveis.".dimmed());
        }
    }
}
