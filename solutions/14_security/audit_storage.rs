// audit_storage.rs  -  Review the AI #4
//
// An AI assistant wrote this contract from the prompt:
//   "users lock credits into savings and can check how much they saved"
//
// There's no attacker this time. The bug is quieter, and it is one of the
// most common mistakes in AI-written Soroban code: the contract forgets that
// storage on Stellar is *rented*.
//
// The `exploit_` test fast-forwards the ledger by 30 days. Make the savings
// survive. Don't edit the tests.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// ~1 day of ledgers (5 seconds each).
pub const DAY_IN_LEDGERS: u32 = 17_280;
/// Keep savings alive for at least 30 days after every update...
pub const SAVINGS_TTL: u32 = 30 * DAY_IN_LEDGERS;
/// ...extending whenever less than 7 days remain.
pub const SAVINGS_TTL_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Savings(Address),
}

#[contract]
pub struct Savings;

#[contractimpl]
impl Savings {
    pub fn save(env: Env, user: Address, amount: i128) {
        user.require_auth();
        let key = DataKey::Savings(user.clone());
        let total = Self::savings(env.clone(), user) + amount;

        // FIX 1: temporary storage is for data that may disappear (caches,
        // nonces). When its TTL runs out it is deleted for good. Savings
        // must live in persistent storage.
        env.storage().persistent().set(&key, &total);
        // FIX 2: persistent entries also have a TTL. Extend it on every
        // write so active savings never get archived.
        env.storage()
            .persistent()
            .extend_ttl(&key, SAVINGS_TTL_THRESHOLD, SAVINGS_TTL);
    }

    pub fn savings(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Savings(user))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::storage::Persistent as _;
    use soroban_sdk::testutils::{Address as _, Ledger as _};

    #[test]
    fn happy_path_save() {
        let env = Env::default();
        env.mock_all_auths();
        let savings = SavingsClient::new(&env, &env.register(Savings, ()));
        let alice = Address::generate(&env);

        savings.save(&alice, &50);
        savings.save(&alice, &25);

        assert_eq!(savings.savings(&alice), 75);
    }

    #[test]
    fn exploit_savings_vanish_after_30_days() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(Savings, ());
        let savings = SavingsClient::new(&env, &id);
        let alice = Address::generate(&env);

        savings.save(&alice, &500);

        // The entry must be rented for (at least) 30 days...
        let ttl = env.as_contract(&id, || {
            env.storage().persistent().get_ttl(&DataKey::Savings(alice.clone()))
        });
        assert!(ttl >= SAVINGS_TTL, "savings are only kept for {ttl} ledgers");

        // ...and still be there after 30 days have passed.
        let now = env.ledger().sequence();
        env.ledger().set_sequence_number(now + SAVINGS_TTL - 1);
        assert_eq!(savings.savings(&alice), 500, "alice's savings disappeared!");
    }
}
