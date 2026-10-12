// transactions1.rs
//
// Everything that changes the ledger is a *transaction*: a list of up to 100
// *operations* that succeed or fail together (atomically), plus:
//
//   source account   who pays the fee and whose sequence number is used
//   sequence number  must be exactly the account's current sequence + 1,
//                    which orders transactions and prevents replays
//   fee              the maximum the source will pay, in stroops:
//                    base fee (100 stroops) × number of operations
//   preconditions    e.g. time bounds: valid only within a time window
//   memo             a short note; text memos are at most 28 bytes
//
// Transactions are encoded as XDR, Stellar's binary format. `soroban_sdk::xdr`
// has every XDR type, the same ones the network and the Stellar CLI use.
//
// Build a transaction that pays several accounts in XLM in one go.

use soroban_sdk::xdr::{
    Asset, Memo, MuxedAccount, Operation, OperationBody, PaymentOp, Preconditions,
    SequenceNumber, StringM, TimeBounds, TimePoint, Transaction, TransactionExt, Uint256, VecM,
};

/// Stroops per operation.
pub const BASE_FEE: u32 = 100;

#[derive(Debug, PartialEq, Eq)]
pub enum TxError {
    NonPositiveAmount,
    MemoTooLong,
    TooManyOperations,
}

pub struct Payment {
    /// Destination account's ed25519 public key.
    pub destination: [u8; 32],
    pub stroops: i64,
}

pub fn build_payments(
    source: [u8; 32],
    current_sequence: i64,
    payments: &[Payment],
    memo: &str,
    valid_until: u64,
) -> Result<Transaction, TxError> {
    let memo: StringM<28> = memo.try_into().unwrap();

    let operations: Vec<Operation> = payments
        .iter()
        .map(|p| Operation {
            source_account: None,
            body: OperationBody::Payment(PaymentOp {
                destination: MuxedAccount::Ed25519(Uint256(p.destination)),
                asset: Asset::Native,
                amount: p.stroops,
            }),
        })
        .collect();
    let fee = BASE_FEE;
    let operations: VecM<Operation, 100> =
        operations.try_into().map_err(|_| TxError::TooManyOperations)?;

    Ok(Transaction {
        source_account: MuxedAccount::Ed25519(Uint256(source)),
        fee,
        seq_num: SequenceNumber(current_sequence),
        cond: Preconditions::Time(TimeBounds {
            min_time: TimePoint(0),
            max_time: TimePoint(valid_until),
        }),
        memo: Memo::Text(memo),
        operations,
        ext: TransactionExt::V0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALICE: [u8; 32] = [1; 32];
    const BOB: [u8; 32] = [2; 32];
    const CAROL: [u8; 32] = [3; 32];

    fn pay(destination: [u8; 32], stroops: i64) -> Payment {
        Payment { destination, stroops }
    }

    #[test]
    fn batches_payments() {
        let tx = build_payments(ALICE, 41, &[pay(BOB, 50_000_000), pay(CAROL, 7)], "payroll", 1_800_000_000).unwrap();

        assert_eq!(tx.seq_num, SequenceNumber(42));
        assert_eq!(tx.fee, 200);
        assert_eq!(tx.operations.len(), 2);
        let OperationBody::Payment(first) = &tx.operations[0].body else { panic!("not a payment") };
        assert_eq!(first.destination, MuxedAccount::Ed25519(Uint256(BOB)));
        assert_eq!(first.amount, 50_000_000);
        assert_eq!(tx.memo, Memo::Text("payroll".try_into().unwrap()));
        assert_eq!(
            tx.cond,
            Preconditions::Time(TimeBounds { min_time: TimePoint(0), max_time: TimePoint(1_800_000_000) })
        );
    }

    #[test]
    fn rejects_bad_amounts() {
        let result = build_payments(ALICE, 1, &[pay(BOB, 10), pay(CAROL, 0)], "", 0);
        assert_eq!(result.err(), Some(TxError::NonPositiveAmount));
    }

    #[test]
    fn rejects_long_memos() {
        let memo = "this memo is definitely too long";
        let result = build_payments(ALICE, 1, &[pay(BOB, 10)], memo, 0);
        assert_eq!(result.err(), Some(TxError::MemoTooLong));
    }

    #[test]
    fn at_most_100_operations() {
        let payments: Vec<Payment> = (0..101).map(|_| pay(BOB, 1)).collect();
        assert_eq!(build_payments(ALICE, 1, &payments, "", 0).err(), Some(TxError::TooManyOperations));
        assert_eq!(build_payments(ALICE, 1, &payments[..100], "", 0).unwrap().fee, 10_000);
    }
}
