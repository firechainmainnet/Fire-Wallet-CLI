use firechain_wallet::wallet::Wallet;
use firechain_wallet::signing::{sign_message_from_hex, SignatureSet};
use firechain_wallet::secure::{export_wallet_to_file, import_wallet_from_file};

use tempfile::NamedTempFile;
use std::io::Write;

// Import necessário para lidar com chaves públicas
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex::encode;

#[test]
fn test_generate_and_serialize_wallet() {
    let wallet = Wallet::new_random(Some("test_wallet".into()));
    let json = serde_json::to_string(&wallet).expect("Erro ao serializar");
    let deserialized: Wallet = serde_json::from_str(&json).expect("Erro ao desserializar");
    assert_eq!(wallet.addresses[0].address, deserialized.addresses[0].address);
}

#[test]
fn test_hd_derivation_increments_index() {
    let mut wallet = Wallet::new_mnemonic(Some("hd_wallet".into()), 12).unwrap();
    let index_before = wallet.hd_index;
    let addr1 = wallet.derive_next_address().unwrap();
    let addr2 = wallet.derive_next_address().unwrap();
    assert_ne!(addr1.address, addr2.address);
    assert_eq!(wallet.hd_index, index_before + 2);
}

#[test]
fn test_wallet_export_and_import() {
    let wallet = Wallet::new_mnemonic(Some("secure".into()), 12).unwrap();

    let tmp = NamedTempFile::new().expect("Falha ao criar NamedTempFile");
    let path = tmp.path().to_str().unwrap();

    export_wallet_to_file(&wallet, "senha123", path).expect("Erro ao exportar");
    let imported = import_wallet_from_file("senha123", path).expect("Erro ao importar");

    assert_eq!(
        wallet.addresses[0].address,
        imported.addresses[0].address,
        "Endereço deve permanecer igual após export/import"
    );
}

#[test]
fn test_signature_set_verification() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[42u8; 32]).unwrap();

    // Prepara privkey hex corretamente
    let secret_bytes = sk.secret_bytes();
    let priv_hex = encode(secret_bytes);

    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_hex = encode(pk.serialize());

    let message_bytes = b"mensagem de teste";
    let message_str = std::str::from_utf8(message_bytes).unwrap();

    // Gera assinatura usando sign_message_from_hex
    let signature = sign_message_from_hex(&priv_hex, message_str)
        .expect("Falha ao assinar");

    let mut sigset = SignatureSet::new();
    sigset.add(pk_hex.clone(), signature);

    let is_valid = sigset.verify(message_bytes, 1, &[pk_hex.clone()]);
    assert!(is_valid, "Assinatura deveria ser válida");
}
