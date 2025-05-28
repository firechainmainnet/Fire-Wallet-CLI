// 📂 src/lib.rs

/// 📦 Módulo da linha de comando (CLI)
///
/// Contém o parser com `clap` (`parser.rs`) e o handler de execução de comandos (`handler.rs`)
pub mod cli;

/// 🔐 Núcleo lógico da FireChain
///
/// Inclui carteiras, endereços e funções criptográficas reutilizáveis.
pub mod core;

/// 🎨 Utilitários visuais para a CLI
///
/// Funções de formatação de terminal, mensagens com cor, banners e UX premium.
pub mod utils;
