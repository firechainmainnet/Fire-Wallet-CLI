use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_mnemonic_generates_json() {
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    let label = "testemnemonic";
    let file = dir.path().join(format!("{}.json", label));

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["mnemonic", "--words", "12", "--label", label])
        .assert()
        .success()
        .stdout(contains("🧠 Carteira Mnemônica"))
        .stdout(contains("PrivateKey: [oculta]"));

    assert!(file.exists(), "Arquivo JSON não foi gerado");
}

#[test]
fn test_mnemonic_unsafe_dump_shows_private_key() {
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    let label = "dump";
    let file = dir.path().join(format!("{}.json", label));

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["--unsafe-dump", "mnemonic", "--words", "12", "--label", label])
        .assert()
        .success()
        .stdout(contains("PrivateKey:"))
        .stdout(contains("PrivateKey: [oculta]").not());

    assert!(file.exists(), "Arquivo JSON não foi gerado");
}

#[test]
fn test_new_wallet_safe_and_unsafe() {
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    let label = "aleatoria_cli";

    // Seguro
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["new", "--label", label])
        .assert()
        .success()
        .stdout(contains("PrivateKey: [oculta]"));

    // Inseguro
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["--unsafe-dump", "new", "--label", label])
        .assert()
        .success()
        .stdout(contains("PrivateKey:").and(contains("PrivateKey: [oculta]").not()));
}

#[test]
fn test_derive_hd_address_and_save() {
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    let label = "derive_cli";
    let json = dir.path().join(format!("{label}.json"));
    let out  = dir.path().join(format!("{label}_out.json"));

    // 1) mnemonic
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["mnemonic", "--words", "12", "--label", label])
        .assert().success();
    assert!(json.exists(), "JSON não foi gerado");

    // 2) derive
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args([
            "derive",
            "--input-wallet", json.to_str().unwrap(),
            "--password", "123",
            "--output", out.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(contains("Novo endereço derivado"));

    assert!(out.exists(), "Arquivo de saída não foi gerado");
}

#[test]
fn test_recover_wallet_from_phrase() {
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    let label = "restaurada_cli";
    let json = dir.path().join(format!("{label}.json"));
    let phrase = "alarm suffer below human prepare weekend task kitten slender apart duty pizza";

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["recover", "--words", phrase, "--label", label])
        .assert()
        .success()
        .stdout(contains("🧠 Carteira Restaurada"));

    assert!(json.exists(), "Arquivo restaurado não encontrado");
}

#[test]
fn test_export_and_import_wallet() {
    let dir = TempDir::new().unwrap();
    let label = "exportavel";
    let json = dir.path().join(format!("{label}.json"));
    let wal  = dir.path().join(format!("{label}.wallet"));

    // mnemonic
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["mnemonic", "--words", "12", "--label", label])
        .assert().success();

    // export
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args([
            "export",
            "--input-json", json.to_str().unwrap(),
            "--password", "senha123",
            "--output", wal.to_str().unwrap(),
        ])
        .assert().success()
        .stdout(contains("Exportada"));

    assert!(wal.exists(), "Arquivo .wallet não foi gerado");

    // import
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["--unsafe-dump", "import", "--password", "senha123", "--path", wal.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Carteira importada com sucesso"));
}

#[test]
fn test_import_corrupted_wallet_fails() {
    let dir = TempDir::new().unwrap();
    let wal = dir.path().join("bad.wallet");
    fs::write(&wal, "dados inválidos").unwrap();

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["import", "--password", "senha", "--path", wal.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("❌ Falha na importação da carteira"));
}

#[test]
fn test_import_nonexistent_file_fails() {
    let dir = TempDir::new().unwrap();
    let wal = dir.path().join("nope.wallet");

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["import", "--password", "senha", "--path", wal.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("Erro ao ler arquivo"));
}

#[test]
fn test_import_with_wrong_password_fails() {
    let dir = TempDir::new().unwrap();
    let label = "exportpw";
    let json = dir.path().join(format!("{label}.json"));
    let wal  = dir.path().join(format!("{label}.wallet"));

    // mnemonic + export
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["mnemonic", "--words", "12", "--label", label])
        .assert().success();
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args([
            "export",
            "--input-json", json.to_str().unwrap(),
            "--password", "senha123",
            "--output", wal.to_str().unwrap(),
        ])
        .assert().success();

    // import com senha errada
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["import", "--password", "wrongpw", "--path", wal.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("Senha incorreta"));
}

#[test]
fn test_recover_invalid_mnemonic_shows_error() {
    let dir = TempDir::new().unwrap();
    let label = "badrecover";
    let phrase = "palavras inválidas aqui";

    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["recover", "--words", phrase, "--label", label])
        .assert()
        .success()
        .stderr(contains("❌ Erro ao restaurar carteira"));
}

#[test]
fn test_derive_without_mnemonic_fails() {
    let dir = TempDir::new().unwrap();
    let label = "newwallet";
    let json = dir.path().join(format!("{label}.json"));
    let out  = dir.path().join(format!("{label}_out.json"));

    // new
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args(["new", "--label", label])
        .assert().success();

    // derive sem mnemonic
    Command::cargo_bin("firechain_wallet").unwrap()
        .current_dir(dir.path())
        .args([
            "derive",
            "--input-wallet", json.to_str().unwrap(),
            "--password", "pw",
            "--output", out.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(contains("❌ Esta carteira não possui frase mnemônica."));
}
