use clap::{Parser, Subcommand, Args};

/// 🧬 FireChain CLI — Carteira Web3 com foco em segurança, modularidade e UX premium.
/// CLI profissional para geração, derivação e gestão de identidades blockchain.
/// Ideal para devs Web3, apps self-custodial, validadores e operações seguras.
#[derive(Parser)]
#[command(name = "firechain-cli")]
#[command(author = "Guilherme Lima")]
#[command(version = "0.1.4")] // 🔄 Atualizado para versão com export/recover
#[command(
    about = "🔥 FireChain CLI — Carteira Web3 com foco em segurança e modularidade.",
    long_about = r#"
🧬 CLI premium para geração, derivação e gestão de identidades blockchain.

Comandos disponíveis:
  🔐 new      → Gera uma nova carteira criptografada (.wallet)
  🧬 derive   → Deriva múltiplos endereços compatíveis (BTC, ETH, FireChain)
  📤 export   → Exporta carteira descriptografada (.wallet → JSON seguro)
  ♻️ recover  → Restaura carteira criptografada a partir de um JSON exportado

Exemplo de uso:
  firechain-cli new --password "minhaSenha"
  firechain-cli derive --input minha.wallet --password "minhaSenha" --all
  firechain-cli export --input minha.wallet --password "minhaSenha" --json
  firechain-cli recover --input carteira.json --password "novaSenha"

100% local, seguro e auditável.
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
    #[command(about = "🔐 Gera uma nova carteira com fingerprint único")]
    New(NewArgs),

    /// 🧬 Deriva múltiplos endereços (BTC, ETH, FireChain)
    #[command(about = "🧬 Deriva endereços compatíveis a partir de uma carteira criptografada")]
    Derive(DeriveArgs),

    /// 📤 Exporta uma carteira `.wallet` descriptografada localmente
    #[command(about = "📤 Exporta uma carteira de forma segura para JSON legível")]
    Export(ExportArgs),

    /// ♻️ Recupera uma carteira `.wallet` a partir de um JSON exportado
    #[command(about = "♻️ Restaura carteira 🔥 a partir de backup JSON exportado")]
    Recover(RecoverArgs),

    /// ℹ️ Mostra ajuda detalhada da FireChain CLI
    #[command(hide = true)]
    Help,
}

/// 🛠️ Argumentos para o comando `new`
#[derive(Args, Debug)]
pub struct NewArgs {
    /// 🔑 Senha obrigatória para criptografar a carteira
    #[arg(long, help = "Senha utilizada para proteger o arquivo .wallet")]
    pub password: String,

    /// 📄 Caminho para salvar o arquivo .wallet. Se omitido, será salvo como <fingerprint>.wallet
    #[arg(long, help = "Caminho de saída personalizado (opcional)")]
    pub out: Option<String>,
}

/// ⚙️ Argumentos para o comando `derive`
#[derive(Args)]
pub struct DeriveArgs {
    /// Gera endereço BTC (Base58)
    #[arg(long, help = "Gera endereço Bitcoin (legacy base58)")]
    pub btc: bool,

    /// Gera endereço Ethereum (0x-prefixed)
    #[arg(long, help = "Gera endereço Ethereum (prefixado com 0x...)")]
    pub eth: bool,

    /// Gera endereço FireChain personalizado
    #[arg(long, help = "Gera endereço FireChain (prefixo f1r3...)")]
    pub f1r3: bool,

    /// Gera todos os formatos disponíveis (BTC, ETH, FIRE)
    #[arg(long, help = "Gera todos os endereços (BTC, ETH e FireChain)")]
    pub all: bool,

    /// 📥 Caminho para o arquivo .wallet criptografado
    #[arg(long, help = "Arquivo .wallet de origem (obrigatório para derivar)")]
    pub input: Option<String>,

    /// 🔑 Senha usada para descriptografar o conteúdo
    #[arg(long, help = "Senha usada na geração da carteira (obrigatória)")]
    pub password: Option<String>,
}

/// 📤 Argumentos para o comando `export`
#[derive(Args)]
pub struct ExportArgs {
    /// 📥 Caminho para o arquivo .wallet criptografado
    #[arg(long, help = "Caminho do arquivo .wallet que será descriptografado")]
    pub input: String,

    /// 🔑 Senha usada para descriptografar o conteúdo do arquivo
    #[arg(long, help = "Senha original usada na geração da carteira")]
    pub password: String,

    /// 📦 Exibe os dados como JSON estruturado (com chave privada)
    #[arg(long, help = "Exporta os dados como JSON estruturado")]
    pub json: bool,
}

/// ♻️ Argumentos para o comando `recover`
#[derive(Args)]
pub struct RecoverArgs {
    /// 📥 Caminho para o arquivo JSON exportado anteriormente
    #[arg(long, help = "Arquivo JSON contendo os dados exportados da carteira")]
    pub input: String,

    /// 🔑 Nova senha que será usada para recriptografar o arquivo `.wallet`
    #[arg(long, help = "Nova senha de proteção do arquivo .wallet restaurado")]
    pub password: String,

    /// 📄 Caminho de saída para o novo arquivo .wallet
    #[arg(long, help = "Caminho de saída personalizado (opcional)")]
    pub out: Option<String>,
}
