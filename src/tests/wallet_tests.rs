// 📂 tests/wallet_tests.rs

use firechain_cli::core::wallet::Wallet;

#[test]
fn test_wallet_generation() {
    let wallet = Wallet::new();

    assert_eq!(wallet.private_key.len(), 64, "Chave privada deve ter 64 caracteres hexadecimais");
    assert_eq!(wallet.public_key.len(), 130, "Chave pública deve ter 130 caracteres (0x04 + X/Y)");

    // Verifica se começa com 0x04 (indicador de pubkey não comprimida)
    assert!(wallet.public_key.starts_with("04"), "Chave pública deve começar com 04");
}
