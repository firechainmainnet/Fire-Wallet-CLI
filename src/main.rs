use clap::{Parser, Subcommand};
use env_logger;
use log::LevelFilter;

use firechain_wallet::commands::{
    new, mnemonic, derive, export, import, multisig, sign, verify, recover,
};

/// CLI da Fire Chain Wallet — agora com suporte a STDIN/JSON
#[derive(Parser)]
#[command(name = "firechain-cli", version, about = "🔥 Fire Chain Wallet CLI", long_about = None)]
struct Cli {
    /// Exibe chaves privadas e dados sensíveis
    #[arg(long)]
    unsafe_dump: bool,

    /// Ativa modo verboso (RUST_LOG=debug)
    #[arg(short, long)]
    verbose: bool,

    /// Ativa modo JSON via STDIN/STDOUT
    #[arg(long)]
    json: bool,

    /// Comando tradicional via CLI
    #[command(subcommand)]
    command: Option<Commands>, // agora opcional para suportar modo JSON puro
}

#[derive(Subcommand)]
enum Commands {
    /// Gera uma carteira aleatória
    New {
        #[arg(short, long)]
        label: Option<String>,
    },

    /// Gera carteira mnemônica com 12 ou 24 palavras
    Mnemonic {
        #[arg(short, long, default_value_t = 12)]
        words: u8,

        #[arg(short, long)]
        label: Option<String>,
    },

    /// Deriva o próximo endereço da carteira HD
    Derive {
        #[arg(short, long)]
        input_wallet: String,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: Option<String>,
    },

    /// Exporta carteira criptografada como `.wallet`
    Export {
        #[arg(short, long)]
        input_json: String,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: String,
    },

    /// Importa carteira criptografada `.wallet`
    Import {
        #[arg(short = 'p', long)]
        password: String,

        #[arg(short = 'f', long)]
        path: String,
    },

    /// Cria carteira multisig M-de-N
    Multisig {
        #[arg(short = 'm', long)]
        m_required: usize,

        #[arg(short = 'k', long, num_args = 1..)]
        public_keys: Vec<String>,
    },

    /// Assina mensagem com chave privada
    Sign {
        #[arg(short, long)]
        privkey: String,

        #[arg(short, long)]
        message: String,
    },

    /// Verifica assinatura multisig M-de-N
    Verify {
        #[arg(long)]
        message: String,

        #[arg(short = 'r', long)]
        m_required: usize,

        #[arg(short = 'k', long, num_args = 1..)]
        public_keys: Vec<String>,

        #[arg(short = 's', long, num_args = 1..)]
        signatures: Vec<String>,
    },

    /// Restaura carteira `.json` a partir de frase mnemônica
    Recover {
        #[arg(short, long)]
        words: String,

        #[arg(short, long)]
        label: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // 🧩 Execução via JSON (STDIN)
    if cli.json {
        use std::io::{self, Read};
        use firechain_wallet::commands::json_api;
        use firechain_wallet::commands::verify::{VerifyPayload, run_json_from_value};

        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Erro ao ler STDIN");

        let parsed: serde_json::Value = match serde_json::from_str(&buffer) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", serde_json::json!({ "error": format!("JSON inválido: {}", e) }));
                std::process::exit(1);
            }
        };

        // 🔍 Match inteligente por ação
        match parsed.get("action").and_then(|v| v.as_str()) {
            Some("Verify") => {
                match serde_json::from_value::<VerifyPayload>(parsed) {
                    Ok(payload) => match run_json_from_value(payload) {
                        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).unwrap()),
                        Err(e) => {
                            eprintln!("{}", serde_json::json!({ "error": e }));
                            std::process::exit(1);
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", serde_json::json!({ "error": format!("Erro ao parsear payload Verify: {}", e) }));
                        std::process::exit(1);
                    }
                }
            }

            // 👉 fallback para comandos genéricos
            _ => {
                match json_api::execute_from_json(&buffer) {
                    Ok(result) => println!("{}", serde_json::to_string_pretty(&result).unwrap()),
                    Err(e) => {
                        eprintln!("{}", serde_json::json!({ "error": e }));
                        std::process::exit(1);
                    }
                }
            }
        }

        return;
    }

    // 🎮 CLI tradicional
    match cli.command {
        Some(Commands::New { label }) => new::run(label, cli.unsafe_dump),
        Some(Commands::Mnemonic { words, label }) => mnemonic::run(words, label, cli.unsafe_dump),
        Some(Commands::Derive { input_wallet, password, output }) => {
            derive::run(input_wallet, password, output, cli.unsafe_dump)
        }
        Some(Commands::Export { input_json, password, output }) => {
            export::run(input_json, password, output)
        }
        Some(Commands::Import { password, path }) => import::run(password, path, cli.unsafe_dump),
        Some(Commands::Multisig { m_required, public_keys }) => {
            multisig::run(m_required, public_keys)
        }
        Some(Commands::Sign { privkey, message }) => sign::run(privkey, message),
        Some(Commands::Verify { message, m_required, public_keys, signatures }) => {
            verify::run(message, m_required, public_keys, signatures)
        }
        Some(Commands::Recover { words, label }) => {
            recover::run(words, label, cli.unsafe_dump)
        }
        None => {
            eprintln!("❌ Nenhum comando fornecido. Use --help para ver as opções.");
            std::process::exit(1);
        }
    }
}
