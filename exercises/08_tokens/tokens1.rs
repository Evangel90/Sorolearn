// tokens1.rs
//
// Tokens on Stellar (including XLM and USDC) are contracts that follow a
// standard interface, so you talk to any of them with `token::TokenClient`:
//
//   let token = token::TokenClient::new(&env, &token_address);
//   token.balance(&who);
//   token.transfer(&from, &to, &amount);
//
// A contract has its own address, `env.current_contract_address()`, so it
// can hold tokens just like a user.
//
// Implement `deposit`: move `amount` tokens from `from` into this contract.
//
// Note: `transfer` itself calls `from.require_auth()`. Checking auth in your
// own contract as well makes your intent explicit.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Token,
}

#[contract]
pub struct PiggyBank;

#[contractimpl]
impl PiggyBank {
    pub fn __constructor(env: Env, token: Address) {
        env.storage().instance().set(&DataKey::Token, &token);
    }

    pub fn deposit(env: Env, from: Address, amount: i128) {
        todo!()
    }

    /// How many tokens the piggy bank holds.
    pub fn total(env: Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::TokenClient::new(&env, &token_address).balance(&env.current_contract_address())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn deposits_tokens() {
        let env = Env::default();
        env.mock_all_auths();

        // Create a test token (a "Stellar Asset Contract") and mint some to a user.
        let issuer = Address::generate(&env);
        let asset = env.register_stellar_asset_contract_v2(issuer);
        let token = token::TokenClient::new(&env, &asset.address());
        let user = Address::generate(&env);
        token::StellarAssetClient::new(&env, &asset.address()).mint(&user, &1_000);

        let piggy_id = env.register(PiggyBank, (&asset.address(),));
        let piggy = PiggyBankClient::new(&env, &piggy_id);

        piggy.deposit(&user, &300);

        assert_eq!(token.balance(&user), 700);
        assert_eq!(token.balance(&piggy_id), 300);
        assert_eq!(piggy.total(), 300);
    }
}
