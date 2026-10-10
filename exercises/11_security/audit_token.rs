// audit_token.rs  -  Review the AI #5
//
// An AI assistant wrote this contract from the prompt:
//   "a vault for our USDC token: users deposit and withdraw"
//
// Deposits and withdrawals use the real token interface and the happy-path
// test passes. But the `exploit_` test drains the vault using a token the
// attacker deployed themselves. Find out how, and fix it.
// Don't edit the tests.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InsufficientBalance = 1,
    UnsupportedToken = 2,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Token,
    Balance(Address),
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    pub fn __constructor(env: Env, token: Address) {
        env.storage().instance().set(&DataKey::Token, &token);
    }

    pub fn deposit(env: Env, from: Address, token: Address, amount: i128) -> Result<(), VaultError> {
        from.require_auth();

        token::TokenClient::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        let balance = Self::balance(env.clone(), from.clone());
        env.storage().persistent().set(&DataKey::Balance(from), &(balance + amount));
        Ok(())
    }

    pub fn withdraw(env: Env, to: Address, amount: i128) -> Result<(), VaultError> {
        to.require_auth();
        let balance = Self::balance(env.clone(), to.clone());
        if amount <= 0 || amount > balance {
            return Err(VaultError::InsufficientBalance);
        }
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance - amount));

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::TokenClient::new(&env, &token).transfer(&env.current_contract_address(), &to, &amount);
        Ok(())
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(user)).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    /// The attacker's "token": its transfer function happily does nothing.
    mod fake {
        use soroban_sdk::{contract, contractimpl, Address, Env};

        #[contract]
        pub struct FakeToken;

        #[contractimpl]
        impl FakeToken {
            pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {}
        }
    }

    struct Setup {
        env: Env,
        usdc: token::TokenClient<'static>,
        vault: VaultClient<'static>,
        vault_id: Address,
    }

    fn setup() -> Setup {
        let env = Env::default();
        env.mock_all_auths();
        let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
        let usdc = token::TokenClient::new(&env, &asset.address());
        let vault_id = env.register(Vault, (&asset.address(),));
        let vault = VaultClient::new(&env, &vault_id);
        Setup { env, usdc, vault, vault_id }
    }

    fn mint(s: &Setup, to: &Address, amount: i128) {
        token::StellarAssetClient::new(&s.env, &s.usdc.address).mint(to, &amount);
    }

    #[test]
    fn happy_path_deposit_withdraw() {
        let s = setup();
        let alice = Address::generate(&s.env);
        mint(&s, &alice, 1_000);

        s.vault.deposit(&alice, &s.usdc.address, &600);
        assert_eq!(s.usdc.balance(&s.vault_id), 600);

        s.vault.withdraw(&alice, &200);
        assert_eq!(s.usdc.balance(&alice), 600);
        assert_eq!(s.vault.balance(&alice), 400);
    }

    #[test]
    fn exploit_drain_with_fake_token() {
        let s = setup();
        let victim = Address::generate(&s.env);
        let attacker = Address::generate(&s.env);
        mint(&s, &victim, 1_000);
        s.vault.deposit(&victim, &s.usdc.address, &1_000);

        // The attacker deploys their own contract and "deposits" with it.
        let fake_token = s.env.register(fake::FakeToken, ());
        let _ = s.vault.try_deposit(&attacker, &fake_token, &1_000);
        // Then withdraws real USDC against the fake deposit.
        let _ = s.vault.try_withdraw(&attacker, &1_000);

        assert_eq!(s.usdc.balance(&attacker), 0, "the attacker drained the vault!");
        assert_eq!(s.usdc.balance(&s.vault_id), 1_000);
        assert_eq!(s.vault.balance(&victim), 1_000);
    }
}
