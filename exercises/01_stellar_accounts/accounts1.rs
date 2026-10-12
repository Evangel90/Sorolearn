// accounts1.rs
//
// Before writing contracts, meet the network they run on. The Stellar lessons
// are plain Rust: no contract, just the rules of the Stellar ledger.
//
// Accounts hold lumens (XLM), Stellar's native asset. On the network, amounts
// are integers counted in *stroops*: 1 XLM = 10,000,000 stroops. (Never use
// floating point for money.)
//
// Every account must also keep a minimum balance, called the reserve:
//
//     minimum balance = (2 + subentries) × base reserve
//
// The base reserve is 0.5 XLM. *Subentries* are things the account owns on the
// ledger: trustlines, offers, extra signers and data entries.
//
// Make the tests pass.

pub const STROOPS_PER_XLM: i64 = 10_000_000;
pub const BASE_RESERVE: i64 = STROOPS_PER_XLM / 2;

/// Whole lumens to stroops.
pub fn xlm_to_stroops(xlm: i64) -> i64 {
    todo!()
}

/// Stroops as lumens with all 7 decimal places, e.g. `12_500_000` -> `"1.2500000"`.
pub fn format_xlm(stroops: i64) -> String {
    format!("{}.{}", stroops / STROOPS_PER_XLM, stroops % STROOPS_PER_XLM)
}

/// The smallest balance an account with `subentries` subentries may hold.
pub fn minimum_balance(subentries: u32) -> i64 {
    subentries as i64 * BASE_RESERVE
}

/// How many stroops the account can actually send (never negative).
pub fn spendable(balance: i64, subentries: u32) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_lumens() {
        assert_eq!(xlm_to_stroops(1), 10_000_000);
        assert_eq!(xlm_to_stroops(250), 2_500_000_000);
    }

    #[test]
    fn formats_amounts() {
        assert_eq!(format_xlm(12_500_000), "1.2500000");
        assert_eq!(format_xlm(1), "0.0000001");
        assert_eq!(format_xlm(1_000_000_000), "100.0000000");
    }

    #[test]
    fn computes_reserves() {
        // A brand-new account needs 1 XLM.
        assert_eq!(minimum_balance(0), 10_000_000);
        // Add a trustline, an offer and a signer: 2.5 XLM.
        assert_eq!(minimum_balance(3), 25_000_000);
    }

    #[test]
    fn keeps_the_reserve() {
        assert_eq!(spendable(xlm_to_stroops(100), 2), xlm_to_stroops(98));
        assert_eq!(spendable(5_000_000, 0), 0);
    }
}
