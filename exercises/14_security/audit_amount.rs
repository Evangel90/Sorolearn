// audit_amount.rs  -  Review the AI #3
//
// An AI assistant wrote this contract from the prompt:
//   "an internal credits ledger with transfers; reject transfers larger
//    than the sender's balance"
//
// It checks auth. It checks the balance. The happy-path test passes.
// The `exploit_` test still lets an attacker print free credits. Fix it.
// Don't edit the tests.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CreditsError {
    InsufficientBalance = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
}

#[contract]
pub struct Credits;

#[contractimpl]
impl Credits {
    /// Gives every new user 100 starter credits (once).
    pub fn join(env: Env, user: Address) {
        user.require_auth();
        let key = DataKey::Balance(user);
        if !env.storage().persistent().has(&key) {
            env.storage().persistent().set(&key, &100_i128);
        }
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), CreditsError> {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(CreditsError::InsufficientBalance);
        }
        let to_balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to), &(to_balance + amount));
        Ok(())
    }

    pub fn balance(env: Env, who: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(who)).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn happy_path_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let credits = CreditsClient::new(&env, &env.register(Credits, ()));
        let (alice, bob) = (Address::generate(&env), Address::generate(&env));

        credits.join(&alice);
        credits.transfer(&alice, &bob, &30);

        assert_eq!(credits.balance(&alice), 70);
        assert_eq!(credits.balance(&bob), 30);
        assert_eq!(credits.try_transfer(&alice, &bob, &1_000), Err(Ok(CreditsError::InsufficientBalance)));
    }

    #[test]
    fn exploit_print_money() {
        let env = Env::default();
        env.mock_all_auths();
        let credits = CreditsClient::new(&env, &env.register(Credits, ()));
        let attacker = Address::generate(&env);
        let victim = Address::generate(&env);
        credits.join(&attacker);
        credits.join(&victim);

        // The attacker legitimately signs a transfer of... -1,000,000 credits.
        let attack = credits.try_transfer(&attacker, &victim, &-1_000_000);

        assert!(attack.is_err(), "the attacker minted free credits!");
        assert_eq!(credits.balance(&attacker), 100);
        assert_eq!(credits.balance(&victim), 100);
    }
}
