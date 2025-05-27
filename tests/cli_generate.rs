//! 🧪 Teste de execução real do binário da CLI (firechain-cli)
//! 🔍 Valida o comportamento como se fosse o usuário final
//! 🎯 Foco em black-box, segurança e experiência visual

use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_cli_execution_outputs_wallet_data() {
    println!("🔧 Iniciando teste de execução CLI da FireChain...\n");

    Command::cargo_bin("firechain-cli")
        .expect("❌ Binário 'firechain-cli' não foi encontrado.")
        .args(&["new"])
        .assert()
        .success()
        .stdout(contains("Carteira gerada com sucesso"))
        .stdout(contains("Private Key (hex)"))
        .stdout(contains("Public  Key (hex)"))
        .stdout(contains("Endereço (Fire)"))
        .stdout(contains("Endereço (Ethereum)"))
        .stdout(contains("Endereço (Bitcoin)"))
        .stdout(contains("Fingerprint SHA256"))
        .stdout(contains("Hash de Derivação (Keccak)"));

    println!("\n✅ Teste CLI executado com sucesso!");
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
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}
