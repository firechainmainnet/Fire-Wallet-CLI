use firechain_wallet::utils::fs::save_unique_file;
use tempfile::TempDir;
use std::fs::read_to_string;

#[test]
fn test_save_unique_file_creates_distinct_files() {
    // Workspace isolado
    let dir = TempDir::new().expect("Falha ao criar TempDir");
    std::env::set_current_dir(dir.path()).expect("Falha ao mudar current_dir");

    let content = "conteúdo de teste";
    let label = "teste_wallet";
    let ext = "txt";

    let filename1 = save_unique_file(label, ext, content, 3)
        .expect("Falha ao salvar primeiro arquivo");
    let filename2 = save_unique_file(label, ext, content, 3)
        .expect("Falha ao salvar segundo arquivo");

    assert_ne!(
        filename1, filename2,
        "Arquivos gerados com o mesmo nome"
    );

    let content1 = read_to_string(&filename1).expect("Erro ao ler primeiro arquivo");
    let content2 = read_to_string(&filename2).expect("Erro ao ler segundo arquivo");

    assert_eq!(content1, content, "Conteúdo do primeiro arquivo difere");
    assert_eq!(content2, content, "Conteúdo do segundo arquivo difere");

    // Cleanup automático ao dropar TempDir
}
