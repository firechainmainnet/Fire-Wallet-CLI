use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use hex::ToHex;

/// Representa uma assinatura junto com a chave pública correspondente
pub struct SignatureEntry {
    pub public_key: String,
    pub signature: String,
}

/// Coleção de assinaturas para verificação multisig
pub struct SignatureSet {
    pub signatures: Vec<SignatureEntry>,
}

impl SignatureSet {
    pub fn new() -> Self {
        SignatureSet {
            signatures: Vec::new(),
        }
    }

    pub fn add(&mut self, public_key: String, signature: String) {
        self.signatures.push(SignatureEntry { public_key, signature });
    }

    /// Verifica se ao menos `m_required` assinaturas são válidas
    pub fn verify(&self, message: &[u8], m_required: usize, allowed_pubkeys: &[String]) -> bool {
        let secp = Secp256k1::new();
        let digest = Sha256::digest(message);
        let msg = match Message::from_slice(&digest) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let mut valid_count = 0;
        let mut seen = HashMap::new();

        for entry in &self.signatures {
            if !allowed_pubkeys.contains(&entry.public_key) {
                continue;
            }
            if seen.contains_key(&entry.public_key) {
                continue; // evitar duplicatas
            }

            let pk_bytes = match hex::decode(&entry.public_key) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let pubkey = match PublicKey::from_slice(&pk_bytes) {
                Ok(pk) => pk,
                Err(_) => continue,
            };

            let sig_bytes = match hex::decode(&entry.signature) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };
            let signature = match Signature::from_der(&sig_bytes) {
                Ok(sig) => sig,
                Err(_) => continue,
            };

            if secp.verify_ecdsa(&msg, &signature, &pubkey).is_ok() {
                seen.insert(entry.public_key.clone(), true);
                valid_count += 1;
            }

            if valid_count >= m_required {
                return true;
            }
        }

        false
    }
}

/// Assina uma mensagem com uma chave privada
pub fn sign_message(secret_key: &SecretKey, message: &[u8]) -> String {
    let secp = Secp256k1::new();
    let digest = Sha256::digest(message);
    let msg = Message::from_slice(&digest).expect("Mensagem inválida");
    let sig = secp.sign_ecdsa(&msg, secret_key);
    sig.serialize_der().encode_hex::<String>()
}
/// Assina uma mensagem a partir de uma chave privada em HEX
pub fn sign_message_from_hex(privkey_hex: &str, message: &str) -> Result<String, String> {
    let bytes = hex::decode(privkey_hex)
        .map_err(|_| "Chave privada inválida (não é hex)".to_string())?;
    let secret_key = SecretKey::from_slice(&bytes)
        .map_err(|_| "Chave privada inválida (tamanho incorreto)".to_string())?;
    Ok(sign_message(&secret_key, message.as_bytes()))
}
