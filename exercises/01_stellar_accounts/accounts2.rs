// accounts2.rs
//
// A Stellar account *is* an ed25519 key pair. The 32-byte secret seed signs
// transactions; the public key derived from it identifies the account.
//
// Humans never see raw bytes. Stellar encodes keys as *StrKeys*: base32 text
// with a version byte (which becomes the first letter) and a checksum:
//
//   G...  account (ed25519 public key)    S...  secret seed
//   C...  contract                        M...  muxed account (G + a 64-bit id)
//
// The `stellar_strkey` crate does the encoding; `ed25519_dalek` does the math.
//
// Make the tests pass.

use ed25519_dalek::SigningKey;
use stellar_strkey::{ed25519, Strkey};

#[derive(Debug, PartialEq, Eq)]
pub enum AddressKind {
    Account,
    Contract,
    Muxed,
    Invalid,
}

/// The `G...` address of the account controlled by `seed`.
pub fn address_from_seed(seed: &[u8; 32]) -> String {
    format!("{}", ed25519::PublicKey(*seed))
}

/// The `S...` secret for `seed`. Treat it like a password.
pub fn secret_from_seed(seed: &[u8; 32]) -> String {
    todo!()
}

/// What kind of address is this? Bad checksums and unknown prefixes are `Invalid`.
pub fn kind(address: &str) -> AddressKind {
    match Strkey::from_string(address) {
        Ok(Strkey::PublicKeyEd25519(_)) => AddressKind::Account,
        _ => AddressKind::Invalid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: [u8; 32] = [7; 32];

    #[test]
    fn derives_the_account_address() {
        let address = address_from_seed(&SEED);
        assert!(address.starts_with('G') && address.len() == 56);
        // Cross-checked with the Stellar CLI (`stellar keys add` + `stellar keys address`).
        assert_eq!(address, "GDVEU3DD4KOFECV66VIHWEZOYX4ZKR3WV27L464SIIPOU2IUI3JCZA57");
    }

    #[test]
    fn encodes_the_secret() {
        let secret = secret_from_seed(&SEED);
        assert!(secret.starts_with('S') && secret.len() == 56);
        assert_eq!(secret, "SADQOBYHA4DQOBYHA4DQOBYHA4DQOBYHA4DQOBYHA4DQOBYHA4DQP54X");
    }

    #[test]
    fn tells_addresses_apart() {
        assert_eq!(kind(&address_from_seed(&SEED)), AddressKind::Account);
        // The Sorolearn registry contract on testnet.
        assert_eq!(kind("CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY"), AddressKind::Contract);
        let muxed = format!("{}", ed25519::MuxedAccount { ed25519: [7; 32], id: 42 });
        assert_eq!(kind(&muxed), AddressKind::Muxed);
    }

    #[test]
    fn rejects_typos() {
        // One changed character breaks the checksum.
        let mut address = address_from_seed(&SEED);
        let last = if address.ends_with('A') { "B" } else { "A" };
        address.replace_range(55.., last);
        assert_eq!(kind(&address), AddressKind::Invalid);
        assert_eq!(kind("hello"), AddressKind::Invalid);
    }
}
