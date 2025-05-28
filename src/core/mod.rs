// 📂 src/core/mod.rs

/// 🔐 Módulo responsável pela geração e gerenciamento de carteiras FireChain.
///
/// Inclui chaves privadas, públicas e o endereço criptografado.
pub mod wallet;

/// 🔗 Módulo responsável por gerar endereços FireChain a partir da chave pública.
///
/// Encapsula o processo de hashing e codificação base58.
pub mod address;

/// ♻️ Módulo de funções criptográficas de baixo nível (SHA256, KECCAK256, RIPEMD160).
///
/// Utilizado para fingerprinting, geração de endereços, verificação de integridade e assinaturas.
pub mod crypto;
