use serde::Deserialize;
use serde_json::{Value, from_value};

use crate::commands::{
    new, mnemonic, derive, export, import, multisig, sign, verify, recover,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum JsonCommandRequest {
    New { label: Option<String>, unsafe_dump: bool },
    Mnemonic { words: u8, label: Option<String>, unsafe_dump: bool },
    Derive { input_wallet: String, password: String, output: Option<String>, unsafe_dump: bool },
    Export { input_json: String, password: String, output: String },
    Import { password: String, path: String, unsafe_dump: bool },
    Multisig { m_required: usize, public_keys: Vec<String> },
    Sign { privkey: String, message: String },
    // ❌ REMOVIDO: Verify
    Recover { words: String, label: Option<String>, unsafe_dump: bool },
}

pub fn execute_from_json(input: &str) -> Result<Value, String> {
    let input_json: Value = serde_json::from_str(input)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))?;

    // 🔍 Suporte especial para comando Verify com modo simples ou multisig
    if let Some(action) = input_json.get("action").and_then(|v| v.as_str()) {
        if action == "Verify" {
            let payload: verify::VerifyPayload = from_value(input_json)
                .map_err(|e| format!("Erro ao ler payload Verify: {}", e))?;
            return verify::run_json_from_value(payload);
        }
    }

    // 🎮 Dispatcher padrão para os demais comandos
    let parsed: JsonCommandRequest = serde_json::from_str(input)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))?;

    match parsed {
        JsonCommandRequest::New { label, unsafe_dump } => {
            new::run_json(label, unsafe_dump)
        }
        JsonCommandRequest::Mnemonic { words, label, unsafe_dump } => {
            mnemonic::run_json(words, label, unsafe_dump)
        }
        JsonCommandRequest::Derive { input_wallet, password, output, unsafe_dump } => {
            derive::run_json(input_wallet, password, output, unsafe_dump)
        }
        JsonCommandRequest::Export { input_json, password, output } => {
            export::run_json(input_json, password, output)
        }
        JsonCommandRequest::Import { password, path, unsafe_dump } => {
            import::run_json(password, path, unsafe_dump)
        }
        JsonCommandRequest::Multisig { m_required, public_keys } => {
            multisig::run_json(m_required, public_keys)
        }
        JsonCommandRequest::Sign { privkey, message } => {
            sign::run_json(privkey, message)
        }
        JsonCommandRequest::Recover { words, label, unsafe_dump } => {
            recover::run_json(words, label, unsafe_dump)
        }
    }
}
