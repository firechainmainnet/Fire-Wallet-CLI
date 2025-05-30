// 📂 tests/derive_tests.rs

use firechain_cli::core::address::{generate_btc_address, generate_eth_address, generate_fire_address};
use hex;

#[test]
fn test_generate_btc_address_format() {
    let pub_key_hex = "0456e26d8289cba0e05b20928ff7184095a72cc1c16c7905d0d26dd273c5a02fd97525c57f9cefc48c7f28953288e0938facafb3e5251902c88af578b7df404090";
    let pub_key_bytes = hex::decode(pub_key_hex).unwrap();
    let btc_address = generate_btc_address(&pub_key_bytes);

    assert!(btc_address.starts_with('1'), "BTC address should start with 1");
    assert!(btc_address.len() >= 26 && btc_address.len() <= 35, "BTC address length should be valid");
}

#[test]
fn test_generate_eth_address_format() {
    let pub_key_hex = "0456e26d8289cba0e05b20928ff7184095a72cc1c16c7905d0d26dd273c5a02fd97525c57f9cefc48c7f28953288e0938facafb3e5251902c88af578b7df404090";
    let pub_key_bytes = hex::decode(pub_key_hex).unwrap();
    let eth_address = generate_eth_address(&pub_key_bytes);

    assert!(eth_address.starts_with("0x"), "ETH address should start with 0x");
    assert_eq!(eth_address.len(), 42, "ETH address should be 42 characters long");
}

#[test]
fn test_generate_fire_address_format() {
    let pub_key_hex = "0456e26d8289cba0e05b20928ff7184095a72cc1c16c7905d0d26dd273c5a02fd97525c57f9cefc48c7f28953288e0938facafb3e5251902c88af578b7df404090";
    let pub_key_bytes = hex::decode(pub_key_hex).unwrap();
    let fire_address = generate_fire_address(&pub_key_bytes);

    assert!(fire_address.starts_with("f1r3"), "FireChain address should start with f1r3");
    assert!(fire_address.len() > 10, "FireChain address should not be empty or malformed");
}