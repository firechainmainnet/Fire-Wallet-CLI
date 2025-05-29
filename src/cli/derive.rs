use colored::Colorize;
use hex;
use crate::core::wallet::Wallet;
use crate::core::address::generate_fire_address;
use crate::cli::format::print_wallet_summary_derive;

pub fn handle_derive_command(_args: &crate::cli::parser::DeriveArgs) {
    println!("{}", "🔐 Iniciando criação da carteira FireChain...\n".bold());

    let (_private_key, public_key, fingerprint) = Wallet::generate_wallet_identity();

    let pubkey_bytes = hex::decode(&public_key).unwrap();
    let fire_address = generate_fire_address(&pubkey_bytes);

    print_wallet_summary_derive(
        &hex::encode(&fingerprint),
        &hex::encode(&public_key),
        &fire_address,
    );
}
