// transactions2.rs
//
// A transaction is authorized by *signatures* over its hash. But what exactly
// gets hashed matters:
//
//     network id = sha256(network passphrase)
//     hash       = sha256(XDR of TransactionSignaturePayload {
//                      network_id,
//                      tagged_transaction: Tx(transaction),
//                  })
//
// Mixing in the network id means a transaction signed for testnet can never
// be replayed on mainnet (or vice versa).
//
// The signature itself is ed25519 over that hash. It is attached as a
// `DecoratedSignature`, together with a *hint*: the last 4 bytes of the
// signer's public key, so validators can quickly find which key signed.
//
// Make the tests pass.

use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};
use soroban_sdk::xdr::{
    DecoratedSignature, Hash, Limits, Signature, SignatureHint, Transaction, TransactionEnvelope,
    TransactionSignaturePayload, TransactionSignaturePayloadTaggedTransaction,
    TransactionV1Envelope, WriteXdr,
};

pub const TESTNET: &str = "Test SDF Network ; September 2015";
pub const MAINNET: &str = "Public Global Stellar Network ; September 2015";

pub fn network_id(passphrase: &str) -> [u8; 32] {
    Sha256::digest(passphrase.as_bytes()).into()
}

pub fn transaction_hash(tx: &Transaction, passphrase: &str) -> [u8; 32] {
    let payload = TransactionSignaturePayload {
        network_id: Hash(network_id(passphrase)),
        tagged_transaction: TransactionSignaturePayloadTaggedTransaction::Tx(tx.clone()),
    };
    let bytes = payload.to_xdr(Limits::none()).unwrap();
    Sha256::digest(bytes).into()
}

pub fn sign(tx: &Transaction, passphrase: &str, seed: &[u8; 32]) -> DecoratedSignature {
    let key = SigningKey::from_bytes(seed);
    let signature = key.sign(&transaction_hash(tx, passphrase));
    let public_key = key.verifying_key().to_bytes();
    DecoratedSignature {
        hint: SignatureHint(public_key[28..].try_into().unwrap()),
        signature: Signature(signature.to_bytes().to_vec().try_into().unwrap()),
    }
}

/// The signed transaction, ready to submit to the network.
pub fn envelope(tx: Transaction, signatures: Vec<DecoratedSignature>) -> TransactionEnvelope {
    TransactionEnvelope::Tx(TransactionV1Envelope {
        tx,
        signatures: signatures.try_into().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Verifier, VerifyingKey};
    use soroban_sdk::xdr::{
        Asset, Memo, MuxedAccount, Operation, OperationBody, PaymentOp, Preconditions,
        SequenceNumber, TransactionExt, Uint256,
    };

    const SEED: [u8; 32] = [7; 32];

    /// A 10 XLM payment from the account of SEED, sequence 1001.
    fn sample_tx() -> Transaction {
        let source = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
        Transaction {
            source_account: MuxedAccount::Ed25519(Uint256(source)),
            fee: 100,
            seq_num: SequenceNumber(1001),
            cond: Preconditions::None,
            memo: Memo::None,
            operations: vec![Operation {
                source_account: None,
                body: OperationBody::Payment(PaymentOp {
                    destination: MuxedAccount::Ed25519(Uint256([2; 32])),
                    asset: Asset::Native,
                    amount: 100_000_000,
                }),
            }]
            .try_into()
            .unwrap(),
            ext: TransactionExt::V0,
        }
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn matches_the_stellar_cli() {
        // `stellar tx hash --network testnet` computes the same hash for this transaction.
        assert_eq!(hex(&transaction_hash(&sample_tx(), TESTNET)), "958532119ca8e9ac2c72f2010f1f55d77e46b7a76e4d154ab318c080cef51dd9");
    }

    #[test]
    fn hashes_are_network_specific() {
        assert_ne!(transaction_hash(&sample_tx(), TESTNET), transaction_hash(&sample_tx(), MAINNET));
    }

    #[test]
    fn signature_verifies() {
        let tx = sample_tx();
        let signed = sign(&tx, TESTNET, &SEED);

        let public_key = SigningKey::from_bytes(&SEED).verifying_key();
        let signature = ed25519_dalek::Signature::from_slice(&signed.signature.0).unwrap();
        let verifying_key = VerifyingKey::from_bytes(&public_key.to_bytes()).unwrap();
        assert!(verifying_key.verify(&transaction_hash(&tx, TESTNET), &signature).is_ok());
        // The same signature is useless on mainnet.
        assert!(verifying_key.verify(&transaction_hash(&tx, MAINNET), &signature).is_err());
    }

    #[test]
    fn hint_is_last_four_bytes_of_public_key() {
        let signed = sign(&sample_tx(), TESTNET, &SEED);
        let public_key = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
        assert_eq!(signed.hint.0, public_key[28..32]);
    }

    #[test]
    fn builds_an_envelope() {
        let tx = sample_tx();
        let env = envelope(tx.clone(), vec![sign(&tx, TESTNET, &SEED)]);
        let TransactionEnvelope::Tx(v1) = &env else { panic!("expected a v1 envelope") };
        assert_eq!(v1.signatures.len(), 1);
        // Base64 XDR is what you would submit to the network (or paste into Stellar Lab).
        println!("{}", env.to_xdr_base64(Limits::none()).unwrap());
    }
}
