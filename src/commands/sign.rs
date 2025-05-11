use crate::signing::sign_message_from_hex;
use colored::*;
use std::process;
use serde_json::{json, Value};

/// Execução tradicional via terminal
pub fn run(privkey_hex: String, message: String) {
    match sign_message_from_hex(&privkey_hex, &message) {
        Ok(sig) => {
            println!("{}", "🖋️ Assinatura gerada:".green());
            println!("{}", sig);
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Erro ao assinar mensagem: {}", e).red());
            process::exit(1);
        }
    }
}

/// Execução programável via JSON/STDIN
pub fn run_json(privkey_hex: String, message: String) -> Result<Value, String> {
    match sign_message_from_hex(&privkey_hex, &message) {
        Ok(signature) => Ok(json!({
            "signature": signature
        })),
        Err(e) => Err(format!("Erro ao assinar: {}", e)),
    }
}
