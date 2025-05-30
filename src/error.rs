use std::fmt;
use std::io;
use serde_json;
use aes_gcm::Error as AesError;
use argon2::Error as ArgonError;
use hex::FromHexError;

/// ⚠️ Tipo de erro principal da FireChain CLI
#[derive(Debug)]
pub enum FireError {
    /// Falha ao criptografar os dados (AES-GCM)
    EncryptionError,

    /// Falha ao descriptografar os dados
    DecryptionError,

    /// Senha incorreta (falha no HMAC)
    InvalidPassword,

    /// Falha ao derivar chave com Argon2id
    KeyDerivationError,

    /// Falha ao serializar ou desserializar JSON
    SerdeError,

    /// Falha ao ler ou escrever arquivos
    IoError(io::Error),

    /// Arquivo não encontrado
    FileNotFound(String),

    /// Erro de parsing de JSON incompleto ou inválido
    ParseError,

    /// Campo obrigatório ausente
    MissingArgument,

    /// Outro erro descritivo (fallback)
    Custom(String),
}

impl fmt::Display for FireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FireError::EncryptionError => write!(f, "erro ao criptografar os dados"),
            FireError::DecryptionError => write!(f, "erro ao descriptografar os dados"),
            FireError::InvalidPassword => write!(f, "senha inválida ou HMAC incorreto"),
            FireError::KeyDerivationError => write!(f, "falha ao derivar chave com Argon2id"),
            FireError::SerdeError => write!(f, "erro de serialização ou desserialização JSON"),
            FireError::IoError(e) => write!(f, "erro de I/O: {}", e),
            FireError::FileNotFound(p) => write!(f, "arquivo não encontrado: {}", p),
            FireError::ParseError => write!(f, "erro ao interpretar estrutura JSON"),
            FireError::MissingArgument => write!(f, "argumento obrigatório ausente"),
            FireError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for FireError {}

/// Integrações com erros comuns da stack
impl From<io::Error> for FireError {
    fn from(e: io::Error) -> Self {
        FireError::IoError(e)
    }
}

impl From<serde_json::Error> for FireError {
    fn from(_: serde_json::Error) -> Self {
        FireError::SerdeError
    }
}

impl From<argon2::Error> for FireError {
    fn from(_: ArgonError) -> Self {
        FireError::KeyDerivationError
    }
}

impl From<aes_gcm::Error> for FireError {
    fn from(_: AesError) -> Self {
        FireError::DecryptionError
    }
}

impl From<FromHexError> for FireError {
    fn from(_: FromHexError) -> Self {
        FireError::ParseError
    }
}
