use crate::signing::SignatureSet;
use colored::*;
use serde::{Deserialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::process;

#[derive(Deserialize)]
pub struct VerifyPayload {
    message: String,
    signature: Option<String>,         // modo simples
    public_key: Option<String>,        // modo simples
    signatures: Option<Vec<String>>,   // modo multisig
    public_keys: Option<Vec<String>>,  // modo multisig
    m_required: Option<usize>,         // modo multisig
}

/// Execução tradicional via terminal (somente modo multisig)
pub fn run(message: String, m_required: usize, public_keys: Vec<String>, signatures: Vec<String>) {
    if public_keys.len() < m_required {
        eprintln!("{}", format!("❌ Chaves insuficientes para M = {}", m_required).red());
        process::exit(1);
    }
    if signatures.len() < m_required {
        eprintln!("{}", format!("❌ Assinaturas fornecidas < M = {}", m_required).red());
        process::exit(1);
    }

    let mut sigset = SignatureSet::new();
    let mut seen = HashSet::new();
    for (i, sig_hex) in signatures.iter().enumerate().take(public_keys.len()) {
        let pk = &public_keys[i];
        if seen.insert(pk.clone()) {
            sigset.add(pk.clone(), sig_hex.clone());
        }
    }

    let ok = sigset.verify(message.as_bytes(), m_required, &public_keys);
    if ok {
        println!("{}", format!("✅ Assinaturas válidas para M = {}", m_required).green());
    } else {
        println!("{}", "❌ Assinaturas insuficientes ou inválidas".red());
        process::exit(1);
    }
}

/// Execução programável via JSON/STDIN — modo simples ou multisig
pub fn run_json_from_value(payload: VerifyPayload) -> Result<Value, String> {
    let message = payload.message;

    let (signatures, public_keys, m_required): (Vec<String>, Vec<String>, usize) =
        if let (Some(sig), Some(pk)) = (payload.signature, payload.public_key) {
            // ✅ Modo simples
            (vec![sig], vec![pk], 1)
        } else if let (Some(sigs), Some(pks), Some(m)) = (payload.signatures, payload.public_keys, payload.m_required) {
            // ✅ Modo multisig
            (sigs, pks, m)
        } else {
            return Err("❌ Campos ausentes ou inválidos para verificação".into());
        };

    if public_keys.len() < m_required {
        return Err(format!("❌ Chaves insuficientes para M = {}", m_required));
    }

    if signatures.len() < m_required {
        return Err(format!("❌ Assinaturas fornecidas < M = {}", m_required));
    }

    let mut sigset = SignatureSet::new();
    let mut seen = HashSet::new();
    for (i, sig_hex) in signatures.iter().enumerate().take(public_keys.len()) {
        let pk = &public_keys[i];
        if seen.insert(pk.clone()) {
            sigset.add(pk.clone(), sig_hex.clone());
        }
    }

    let valid = sigset.verify(message.as_bytes(), m_required, &public_keys);
    Ok(json!({ "valid": valid }))
}
