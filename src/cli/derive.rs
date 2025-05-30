use hex;
use std::fs;
use std::path::Path;

use crate::cli::parser::DeriveArgs;
use crate::core::wallet::decrypt_wallet;
use crate::core::address::{generate_btc_address, generate_eth_address, generate_fire_address};
use crate::FireError;
use crate::core::wallet::Wallet;

/// 🧬 Lógica de derivação da carteira — retorna `Wallet` reconstituída com endereços
pub fn derive_addresses(args: &DeriveArgs) -> Result<Wallet, FireError> {
    // ✅ Validação das flags
    if !args.btc && !args.eth && !args.f1r3 && !args.all {
        return Err(FireError::MissingArgument); // será tratado visualmente no handler
    }

    // ✅ Extração de argumentos obrigatórios
    let input_path = args.input.as_ref().ok_or(FireError::MissingArgument)?;
    let password = args.password.as_ref().ok_or(FireError::MissingArgument)?;

    if !Path::new(input_path).exists() {
        return Err(FireError::FileNotFound(input_path.clone()));
    }

    // ✅ Leitura e descriptografia do arquivo .wallet
    let encrypted = fs::read(input_path)?;
    let mut wallet = decrypt_wallet(&encrypted, password)?;

    // ✅ Deriva novamente os endereços com base na pubkey
    let pubkey_bytes = hex::decode(&wallet.public_key).map_err(FireError::from)?;

    wallet.address_firechain = generate_fire_address(&pubkey_bytes);
    wallet.address_btc = generate_btc_address(&pubkey_bytes);
    wallet.address_eth = generate_eth_address(&pubkey_bytes);

    Ok(wallet)
}
