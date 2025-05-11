use crate::address::Address;
use bip39::{Mnemonic, Language};
use hmac::{Hmac, Mac};
use rand::RngCore;
use secp256k1::SecretKey;
use sha2::Sha512;
use zeroize::Zeroize;
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};

type HmacSha512 = Hmac<Sha512>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletType {
    Random,
    Mnemonic12,
    Mnemonic24,
    Multisig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub wallet_type: WalletType,
    pub addresses: Vec<Address>,
    pub label: Option<String>,
    pub mnemonic_phrase: Option<String>,
    pub hd_index: usize,

    #[serde(skip_serializing, skip_deserializing)]
    #[serde(default)]
    pub seed: Vec<u8>,

    #[serde(rename = "seed_enc", skip_serializing_if = "Option::is_none")]
    pub seed_encoded: Option<String>,
}

impl Wallet {
    pub fn new_random(label: Option<String>) -> Self {
        let addr = Address::generate_random();
        Self {
            wallet_type: WalletType::Random,
            addresses: vec![addr],
            label,
            mnemonic_phrase: None,
            seed: vec![],
            seed_encoded: None,
            hd_index: 0,
        }
    }

    pub fn new_mnemonic_12(label: Option<String>) -> Self {
        Self::new_mnemonic(label, 12).expect("Falha ao gerar carteira com 12 palavras")
    }

    pub fn new_mnemonic_24(label: Option<String>) -> Self {
        Self::new_mnemonic(label, 24).expect("Falha ao gerar carteira com 24 palavras")
    }

    // ✅ Torna pública para uso externo
    pub fn new_mnemonic(label: Option<String>, word_count: usize) -> Result<Self, String> {
        let entropy_bytes = match word_count {
            12 => 16,
            24 => 32,
            _ => return Err("Somente 12 ou 24 palavras são suportadas".into()),
        };

        let mut entropy = vec![0u8; entropy_bytes];
        rand::thread_rng().fill_bytes(&mut entropy);

        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)
            .map_err(|e| format!("Erro ao gerar mnemônico: {}", e))?;
        let phrase = mnemonic.to_string();
        let seed = mnemonic.to_seed("");

        let secret_key = Self::secret_key_from_seed(&seed)?;
        let address = Address::from_secret_key(secret_key);

        Ok(Self {
            wallet_type: if word_count == 12 { WalletType::Mnemonic12 } else { WalletType::Mnemonic24 },
            addresses: vec![address],
            label,
            mnemonic_phrase: Some(phrase),
            seed: seed.to_vec(),
            seed_encoded: Some(general_purpose::STANDARD.encode(seed)),
            hd_index: 1,
        })
    }

    pub fn restore_from_mnemonic(label: Option<String>, phrase: &str) -> Result<Self, String> {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase)
            .map_err(|e| format!("Frase mnemônica inválida: {}", e))?;
        let seed = mnemonic.to_seed("");

        let secret_key = Self::secret_key_from_seed(&seed)?;
        let address = Address::from_secret_key(secret_key);

        Ok(Self {
            wallet_type: if phrase.split_whitespace().count() == 12 {
                WalletType::Mnemonic12
            } else {
                WalletType::Mnemonic24
            },
            addresses: vec![address],
            label,
            mnemonic_phrase: Some(phrase.to_string()),
            seed: seed.to_vec(),
            seed_encoded: Some(general_purpose::STANDARD.encode(seed)),
            hd_index: 1,
        })
    }

    fn secret_key_from_seed(seed: &[u8]) -> Result<SecretKey, String> {
        if seed.len() < 32 {
            return Err("Seed com comprimento insuficiente".into());
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&seed[..32]);
        let key = SecretKey::from_slice(&bytes)
            .map_err(|_| "Chave inválida derivada da seed".to_string())?;
        bytes.zeroize();
        Ok(key)
    }

    pub fn derive_address(&self, index: usize) -> Result<Address, String> {
        if self.seed.is_empty() {
            return Err("Seed ausente. Derivação HD indisponível.".into());
        }

        let path = format!("m/77/{}", index);
        let mut mac = HmacSha512::new_from_slice(&self.seed)
            .map_err(|_| "Erro ao inicializar HMAC-SHA512".to_string())?;
        mac.update(path.as_bytes());
        let result = mac.finalize().into_bytes();

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&result[..32]);
        let sk = SecretKey::from_slice(&key_bytes)
            .map_err(|_| "Erro ao derivar chave privada".to_string())?;
        key_bytes.zeroize();

        Ok(Address::from_secret_key(sk))
    }

    pub fn derive_next_address(&mut self) -> Result<Address, String> {
        let addr = self.derive_address(self.hd_index)?;
        self.addresses.push(addr.clone());
        self.hd_index += 1;
        Ok(addr)
    }

    pub fn add_address(&mut self, addr: Address) {
        self.addresses.push(addr);
    }

    pub fn ensure_seed_loaded(&mut self) {
        if self.seed.is_empty() {
            if let Some(enc) = &self.seed_encoded {
                if let Ok(decoded) = general_purpose::STANDARD.decode(enc) {
                    self.seed = decoded;
                }
            }
        }
    }
}
