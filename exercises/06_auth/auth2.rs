// auth2.rs
//
// The admin pattern: the deployer chooses an admin in the constructor, and
// privileged functions require the admin's authorization.
//
// Soroban only runs a constructor if the function is named exactly
// `__constructor`. It runs once, during deployment, so nobody can call it
// later to make themselves admin.
//
// Two things to fix:
//   1. The admin is never stored at deployment.
//   2. `set_fee` must only work when the stored admin authorizes it.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Fee,
}

#[contract]
pub struct FeeConfig;

#[contractimpl]
impl FeeConfig {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn set_fee(env: Env, fee: u32) {
        // TODO: only the admin may change the fee.
        env.storage().instance().set(&DataKey::Fee, &fee);
    }

    pub fn fee(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Fee).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn admin_sets_fee() {
        let env = Env::default();
        let admin = Address::generate(&env);
        // Constructor arguments are passed as a tuple.
        let client = FeeConfigClient::new(&env, &env.register(FeeConfig, (&admin,)));

        assert_eq!(client.admin(), admin);

        env.mock_all_auths();
        client.set_fee(&25);

        // Exactly one authorization was required, and it came from the admin.
        // (`env.auths()` only covers the most recent contract call.)
        let auths = env.auths();
        assert_eq!(auths.len(), 1);
        assert_eq!(auths[0].0, admin);
        assert_eq!(client.fee(), 25);
    }

    #[test]
    fn others_cannot_set_fee() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let client = FeeConfigClient::new(&env, &env.register(FeeConfig, (&admin,)));

        assert!(client.try_set_fee(&99).is_err());
        assert_eq!(client.fee(), 0);
    }
}
