// time1.rs
//
// The ledger tells a contract "when" it is running:
//
//   env.ledger().sequence()  - the ledger number (increments every ~5 seconds)
//   env.ledger().timestamp() - the ledger close time, in UNIX seconds
//
// Use the timestamp for deadlines in real-world time, and the sequence number
// for things measured in ledgers (like storage TTLs).
//
// This time lock compares the wrong value. Fix `is_unlocked`.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const UNLOCK_AT: Symbol = symbol_short!("UNLOCK");

#[contract]
pub struct TimeLock;

#[contractimpl]
impl TimeLock {
    /// `unlock_time` is a UNIX timestamp in seconds.
    pub fn __constructor(env: Env, unlock_time: u64) {
        env.storage().instance().set(&UNLOCK_AT, &unlock_time);
    }

    pub fn is_unlocked(env: Env) -> bool {
        let unlock_time: u64 = env.storage().instance().get(&UNLOCK_AT).unwrap();
        env.ledger().timestamp() >= unlock_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Ledger as _;

    #[test]
    fn unlocks_at_the_right_time() {
        let env = Env::default();
        let client = TimeLockClient::new(&env, &env.register(TimeLock, (1_700_000_000_u64,)));

        // Tests can move time around freely.
        env.ledger().set_timestamp(1_699_999_999);
        assert!(!client.is_unlocked());

        env.ledger().set_timestamp(1_700_000_000);
        assert!(client.is_unlocked());
    }
}
