    use assert_cmd::Command;
    use predicates::str::contains;
    use serde_json::Value;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_full_export_recover_flow() -> Result<(), Box<dyn std::error::Error>> {
        let test_wallet = "tests/tmp/test.wallet";
        let test_json = "tests/tmp/test.json";
        let restored_wallet = "tests/tmp/restored.wallet";

        // 🔐 1. Gera uma nova carteira
        Command::cargo_bin("firechain-cli")?
            .args(&["new", "--password", "test123", "--out", test_wallet])
            .assert()
            .success()
            .stdout(contains("✅ Carteira criada e criptografada com sucesso"));

        assert!(Path::new(test_wallet).exists(), "Arquivo .wallet não foi criado");

        // 📤 2. Exporta como JSON
        let export_output = Command::cargo_bin("firechain-cli")?
            .args(&["export", "--input", test_wallet, "--password", "test123", "--json"])
            .output()?;

        assert!(export_output.status.success(), "Falha ao exportar .wallet");

        let export_stdout = String::from_utf8(export_output.stdout)?;
        let export_json: Value = serde_json::from_str(&export_stdout)?;
        fs::write(test_json, serde_json::to_string_pretty(&export_json)?)?;

        assert_eq!(export_json["version"], "0.1.4");
        assert!(export_json["fingerprint"].as_str().unwrap().len() >= 12);

        // ♻️ 3. Recupera a carteira com nova senha
        Command::cargo_bin("firechain-cli")?
            .args(&[
                "recover",
                "--input", test_json,
                "--password", "novaSenha123",
                "--out", restored_wallet
            ])
            .assert()
            .success()
            .stdout(contains("✅ Carteira restaurada com sucesso"));

        assert!(Path::new(restored_wallet).exists(), "Arquivo .wallet restaurado não foi criado");

        // 📤 4. Exporta a restaurada e compara os dados
        let reexport_output = Command::cargo_bin("firechain-cli")?
            .args(&["export", "--input", restored_wallet, "--password", "novaSenha123", "--json"])
            .output()?;

        assert!(reexport_output.status.success(), "Falha ao exportar carteira restaurada");

        let reexport_json: Value = serde_json::from_str(&String::from_utf8(reexport_output.stdout)?)?;

        assert_eq!(reexport_json["fingerprint"], export_json["fingerprint"]);
        assert_eq!(reexport_json["pubkey"], export_json["pubkey"]);
        assert_eq!(reexport_json["privkey"], export_json["privkey"]);
        assert_eq!(reexport_json["addresses"], export_json["addresses"]);

        // 🧬 5. Valida saída do comando derive (placeholder)
        Command::cargo_bin("firechain-cli")?
            .args(&["derive", "--all"])
            .assert()
            .stdout(contains("⏳ Comando `derive` ainda não implementado"));

        // 🧹 6. Cleanup
        fs::remove_file(test_wallet)?;
        fs::remove_file(test_json)?;
        fs::remove_file(restored_wallet)?;

        Ok(())
    }