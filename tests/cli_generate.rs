//! 🔧 Teste de execução real da CLI FireChain (`firechain-cli new`)
//! 📋 Garante que a interface exibe todas as seções e dados esperados

use assert_cmd::Command;

#[test]
fn test_cli_execution_outputs_wallet_data() {
    let mut cmd = Command::cargo_bin("firechain-cli").unwrap();
    let output = cmd.arg("new").assert().get_output().stdout.clone();
    let stdout = String::from_utf8_lossy(&output);

    println!("🔧 Iniciando teste de execução CLI da FireChain...\n");

    // 📋 Verificações esperadas na saída
    assert!(stdout.contains("Carteira gerada com sucesso"));
    assert!(stdout.contains("Private Key"));
    assert!(stdout.contains("Public  Key"));
    assert!(stdout.contains("Endereço (Fire)"));
    assert!(stdout.contains("Endereço (Ethereum)"));
    assert!(stdout.contains("Endereço (Bitcoin)"));
    assert!(stdout.contains("Fingerprint SHA256"));
    assert!(stdout.contains("Hash de Derivação (Keccak)"));

    // ✅ Novo formato do endereço FireChain
    assert!(stdout.contains("f1r3"));
    assert!(!stdout.contains("f1r3:")); // ❌ ':' não permitido mais
    assert!(!stdout.contains("🔥"));    // ❌ emoji não permitido

    println!("✅ Teste CLI executado com sucesso!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Todas as mensagens esperadas foram exibidas:");
    println!("   • ✅ Título de sucesso");
    println!("   • ✅ Chave privada (hex)");
    println!("   • ✅ Chave pública (hex)");
    println!("   • ✅ Endereço FireChain");
    println!("   • ✅ Endereço Ethereum");
    println!("   • ✅ Endereço Bitcoin");
    println!("   • ✅ Fingerprint SHA256");
    println!("   • ✅ Hash de derivação (Keccak)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
