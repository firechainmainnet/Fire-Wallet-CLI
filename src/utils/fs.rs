// src/utils/fs.rs
use std::fs::write;
use std::path::Path;
use rand::{thread_rng, Rng};
use chrono::Local;

/// Salva arquivo de forma segura com timestamp + id aleatório.
/// Evita sobrescrita mesmo com labels iguais.
///
/// Retorna o nome final salvo, ex: "carteira_label_20240502_003012_ab12.json"
pub fn save_unique_file(
    base_label: &str,
    extension: &str,
    contents: &str,
    max_attempts: usize,
) -> Result<String, String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let sanitized = base_label.replace(" ", "_").to_lowercase();

    for _ in 0..max_attempts {
        let rand_id: u16 = thread_rng().gen_range(0..=9999);
        let id_str = format!("{:04x}", rand_id); // Ex: "a2f3"

        let filename = format!("{}_{}_{}.{}", sanitized, timestamp, id_str, extension);
        if !Path::new(&filename).exists() {
            if let Err(e) = write(&filename, contents) {
                return Err(format!("Erro ao salvar '{}': {}", filename, e));
            }
            return Ok(filename);
        }
    }

    Err(format!(
        "Falha ao salvar arquivo: tentativas esgotadas para '{}'",
        base_label
    ))
}
