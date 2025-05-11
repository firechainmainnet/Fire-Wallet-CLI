use firechain_wallet::signing::{sign_message, SignatureSet};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex::encode;

#[test]
fn test_signature_set_validates_correct_signatures() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[7u8; 32]).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_hex = encode(pk.serialize());

    let msg = b"teste multisig";
    let sig_hex = sign_message(&sk, msg);

    let mut set = SignatureSet::new();
    set.add(pk_hex.clone(), sig_hex.clone());

    assert!(
        set.verify(msg, 1, &[pk_hex.clone()]),
        "Assinatura válida não foi reconhecida"
    );
}

#[test]
fn test_signature_set_insufficient_signatures() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[8u8; 32]).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_hex = encode(pk.serialize());

    let msg = b"mensagem qualquer";
    let sig_hex = sign_message(&sk, msg);

    let mut set = SignatureSet::new();
    set.add(pk_hex.clone(), sig_hex);

    assert!(
        !set.verify(msg, 2, &[pk_hex.clone()]),
        "Assinatura insuficiente deveria falhar"
    );
}

#[test]
fn test_signature_set_ignores_invalid_pubkey() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[9u8; 32]).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_hex = encode(pk.serialize());

    let msg = b"teste invalido";
    let sig_hex = sign_message(&sk, msg);

    let mut set = SignatureSet::new();
    set.add("deadbeef".into(), sig_hex.clone()); // chave não listada
    set.add(pk_hex.clone(), sig_hex.clone());

    assert!(
        set.verify(msg, 1, &[pk_hex.clone()]),
        "Assinatura válida não foi reconhecida quando chave inválida presente"
    );
}

#[test]
fn test_signature_set_ignores_duplicate_signatures() {
    let secp = Secp256k1::new();
    let sk = SecretKey::from_slice(&[10u8; 32]).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    let pk_hex = encode(pk.serialize());

    let msg = b"mensagem duplicada";
    let sig_hex = sign_message(&sk, msg);

    let mut set = SignatureSet::new();
    set.add(pk_hex.clone(), sig_hex.clone());
    set.add(pk_hex.clone(), sig_hex.clone()); // duplicata

    assert!(
        set.verify(msg, 1, &[pk_hex.clone()]),
        "Duplicata não deveria impedir validação"
    );
}
