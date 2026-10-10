// audit_init.rs  -  Review the AI #2
//
// An AI assistant wrote this contract from the prompt:
//   "a price oracle where the admin can update the price"
//
// The happy-path test passes. The `exploit_` test shows an attacker taking
// over the oracle. Find the vulnerability and fix it. Don't edit the tests.
//
// (In a brand-new contract you would use `__constructor`, which runs
// atomically with deployment. Many existing contracts and AI answers still use
// an `initialize` function, so you need to know how to make one safe.)

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OracleError {
    AlreadyInitialized = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Price,
}

#[contract]
pub struct Oracle;

#[contractimpl]
impl Oracle {
    pub fn initialize(env: Env, admin: Address) {
        // FIX: `initialize` is an ordinary public function, so anyone can call
        // it at any time. Without this check, a second call simply replaced
        // the admin, handing the oracle to whoever called it.
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, OracleError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn set_price(env: Env, price: i128) {
        let admin = Self::admin(env.clone());
        admin.require_auth();
        env.storage().instance().set(&DataKey::Price, &price);
    }

    pub fn price(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Price).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn happy_path_admin_updates_price() {
        let env = Env::default();
        env.mock_all_auths();
        let oracle = OracleClient::new(&env, &env.register(Oracle, ()));
        let admin = Address::generate(&env);

        oracle.initialize(&admin);
        oracle.set_price(&1_250);

        assert_eq!(oracle.price(), 1_250);
        assert_eq!(oracle.admin(), admin);
    }

    #[test]
    fn exploit_take_over_oracle() {
        let env = Env::default();
        env.mock_all_auths();
        let oracle = OracleClient::new(&env, &env.register(Oracle, ()));
        let admin = Address::generate(&env);
        let attacker = Address::generate(&env);
        oracle.initialize(&admin);

        // The attacker "initializes" the oracle again, naming themselves admin...
        let takeover = oracle.try_initialize(&attacker);

        assert!(takeover.is_err(), "the attacker became the oracle admin!");
        assert_eq!(oracle.admin(), admin);
    }
}
