use firechain_wallet::multisig::MultisigWallet;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex::encode;

#[test]
fn test_deterministic_address() {
    let secp = Secp256k1::new();
    let sk1 = SecretKey::from_slice(&[1u8; 32]).unwrap();
    let sk2 = SecretKey::from_slice(&[2u8; 32]).unwrap();
    let pk1 = encode(PublicKey::from_secret_key(&secp, &sk1).serialize());
    let pk2 = encode(PublicKey::from_secret_key(&secp, &sk2).serialize());

    let w1 = MultisigWallet::new(2, vec![pk1.clone(), pk2.clone()]).unwrap();
    let w2 = MultisigWallet::new(2, vec![pk2.clone(), pk1.clone()]).unwrap();
    assert_eq!(
        w1.address, w2.address,
        "Endereço deve ser determinístico independentemente da ordem das chaves"
    );
}

#[test]
fn test_m_greater_than_n_fails() {
    let secp = Secp256k1::new();
    // Agora usamos N = 2 participantes, mas pedimos M = 3
    let sk1 = SecretKey::from_slice(&[3u8; 32]).unwrap();
    let sk2 = SecretKey::from_slice(&[4u8; 32]).unwrap();
    let pk1 = encode(PublicKey::from_secret_key(&secp, &sk1).serialize());
    let pk2 = encode(PublicKey::from_secret_key(&secp, &sk2).serialize());

    let err = MultisigWallet::new(3, vec![pk1, pk2]).unwrap_err();
    assert!(
        err.contains("M inválido"),
        "Esperado erro sobre M inválido, mas obteve: {}",
        err
    );
}

#[test]
fn test_zero_m_fails() {
    let secp = Secp256k1::new();
    let sk1 = SecretKey::from_slice(&[5u8; 32]).unwrap();
    let sk2 = SecretKey::from_slice(&[6u8; 32]).unwrap();
    let pk1 = encode(PublicKey::from_secret_key(&secp, &sk1).serialize());
    let pk2 = encode(PublicKey::from_secret_key(&secp, &sk2).serialize());

    let err = MultisigWallet::new(0, vec![pk1, pk2]).unwrap_err();
    assert!(
        err.contains("M inválido"),
        "Esperado erro sobre M inválido para M=0, mas obteve: {}",
        err
    );
}

#[test]
fn test_duplicate_keys_fails() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[7u8; 32]).unwrap();
    let pk = encode(PublicKey::from_secret_key(&secp, &sk).serialize());

    let err = MultisigWallet::new(1, vec![pk.clone(), pk.clone()]).unwrap_err();
    assert!(
        err.contains("duplicadas"),
        "Esperado erro sobre chaves duplicadas, mas obteve: {}",
        err
    );
}

#[test]
fn test_invalid_hex_fails() {
    let err = MultisigWallet::new(1, vec!["zzzz".into(), "1234".into()]).unwrap_err();
    assert!(
        err.contains("Chave pública inválida"),
        "Esperado erro sobre formato HEX inválido, mas obteve: {}",
        err
    );
}
