// accounts3.rs
//
// Multisig is built into every Stellar account, no contract needed.
//
// An account has a list of signers, each with a *weight*. Its own key (the
// "master key") is a signer too, with `master_weight`; setting it to 0
// disables the master key. Every operation belongs to a threshold level:
//
//   Low     BumpSequence, AllowTrust
//   High    SetOptions (changes signers/thresholds), AccountMerge
//   Medium  everything else: Payment, CreateAccount, ChangeTrust, ...
//
// A transaction may perform an operation when the total weight of the keys
// that signed it is at least the threshold for that operation's level (and
// greater than zero: somebody must sign).
//
// Make the tests pass.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Payment,
    CreateAccount,
    ChangeTrust,
    BumpSequence,
    AllowTrust,
    SetOptions,
    AccountMerge,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Low,
    Medium,
    High,
}

pub struct Account<'a> {
    pub master_key: &'a str,
    pub master_weight: u8,
    /// Additional signers as (key, weight).
    pub signers: &'a [(&'a str, u8)],
    pub low: u8,
    pub medium: u8,
    pub high: u8,
}

pub fn level(op: Op) -> Level {
    match op {
        // TODO: not every operation is Medium.
        _ => Level::Medium,
    }
}

/// Can a transaction signed by `signed_by` perform `op` on `account`?
pub fn can_submit(account: &Account, op: Op, signed_by: &[&str]) -> bool {
    let threshold = account.medium;
    // u32: several u8 weights can add up to more than 255.
    let mut weight: u32 = 0;
    for key in signed_by {
        if let Some((_, w)) = account.signers.iter().find(|(signer, _)| signer == key) {
            weight += *w as u32;
        }
    }
    weight > 0 && weight >= threshold as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A company treasury: master key disabled, 3 officers, "2 of 3" for
    /// payments, all 3 to change the account's configuration.
    fn treasury() -> Account<'static> {
        Account {
            master_key: "GTREASURY",
            master_weight: 0,
            signers: &[("GALICE", 1), ("GBOB", 1), ("GCAROL", 1)],
            low: 1,
            medium: 2,
            high: 3,
        }
    }

    #[test]
    fn classifies_operations() {
        assert_eq!(level(Op::Payment), Level::Medium);
        assert_eq!(level(Op::BumpSequence), Level::Low);
        assert_eq!(level(Op::SetOptions), Level::High);
        assert_eq!(level(Op::AccountMerge), Level::High);
    }

    #[test]
    fn two_of_three_for_payments() {
        let t = treasury();
        assert!(!can_submit(&t, Op::Payment, &["GALICE"]));
        assert!(can_submit(&t, Op::Payment, &["GALICE", "GBOB"]));
        assert!(can_submit(&t, Op::BumpSequence, &["GCAROL"]));
    }

    #[test]
    fn all_three_to_reconfigure() {
        let t = treasury();
        assert!(!can_submit(&t, Op::SetOptions, &["GALICE", "GBOB"]));
        assert!(can_submit(&t, Op::SetOptions, &["GALICE", "GBOB", "GCAROL"]));
    }

    #[test]
    fn disabled_master_key_and_strangers_count_for_nothing() {
        let t = treasury();
        assert!(!can_submit(&t, Op::BumpSequence, &["GTREASURY"]));
        assert!(!can_submit(&t, Op::Payment, &["GALICE", "GMALLORY"]));
    }

    #[test]
    fn default_account_signs_alone() {
        // A fresh account: master weight 1, all thresholds 0.
        let solo = Account { master_key: "GSOLO", master_weight: 1, signers: &[], low: 0, medium: 0, high: 0 };
        assert!(can_submit(&solo, Op::AccountMerge, &["GSOLO"]));
        assert!(!can_submit(&solo, Op::Payment, &[]));
    }
}
