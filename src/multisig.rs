use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use bs58;
use hex::FromHex;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};

/// Representa uma carteira multisig (M de N)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigWallet {
    pub address: String,
    pub public_keys: Vec<String>,
    pub m_required: usize,
}

impl MultisigWallet {
    /// Cria uma nova carteira multisig com M de N public keys
    pub fn new(m_required: usize, mut public_keys: Vec<String>) -> Result<Self, String> {
        let n_total = public_keys.len();

        // Validar M/N
        if n_total < 2 {
            return Err("Multisig exige pelo menos 2 participantes.".to_string());
        }
        if m_required == 0 || m_required > n_total {
            return Err(format!("M inválido: {} (deve ser entre 1 e {})", m_required, n_total));
        }

        // Verificar duplicatas
        let unique: HashSet<_> = public_keys.iter().collect();
        if unique.len() != n_total {
            return Err("Chaves públicas duplicadas detectadas.".to_string());
        }

        // Ordenar para gerar endereço determinístico
        public_keys.sort();

        // Concatenar M (em 1 byte) + todas as pubkeys em binário
        let mut raw = vec![m_required as u8];
        for hex_key in &public_keys {
            let bytes = <[u8; 33]>::from_hex(hex_key)
                .map_err(|_| format!("Chave pública inválida: {}", hex_key))?;
            raw.extend_from_slice(&bytes);
        }

        // Hash160 (RIPEMD160(SHA256()))
        let sha = Sha256::digest(&raw);
        let ripemd = Ripemd160::digest(&sha);

        // Prefixo da Fire Chain
        let mut data = vec![0x77];
        data.extend(&ripemd);

        // Checksum
        let checksum = Sha256::digest(&Sha256::digest(&data));
        data.extend(&checksum[..4]);

        // Gerar string do endereço
        let address = format!("f1r3{}", bs58::encode(&data).into_string());

        Ok(MultisigWallet {
            address,
            public_keys,
            m_required,
        })
    }

    /// Exibe de forma legível para debug
    pub fn display(&self) {
        println!("🔐 Multisig {} de {}:", self.m_required, self.public_keys.len());
        println!("  Address: {}", self.address);
        for (i, pk) in self.public_keys.iter().enumerate() {
            println!("  Public Key {}: {}", i + 1, pk);
        }
    }
}
