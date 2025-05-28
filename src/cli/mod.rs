// 📂 src/cli/mod.rs

/// 🧭 Parser de argumentos com `clap` (estrutura dos comandos)
pub mod parser;

/// 🧠 Executor dos comandos CLI definidos no parser
pub mod handler;

/// 🧬 Lógica do comando `derive` (multi-endereço: BTC, ETH, F1R3)
pub mod derive;
