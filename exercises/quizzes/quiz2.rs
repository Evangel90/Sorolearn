// quiz2.rs
//
// Final quiz! Build a time-locked savings vault. It combines everything:
// constructors, storage, errors, auth, events, tokens and time.
//
//   __constructor(token, unlock_time)
//     - Store the token address and the UNIX timestamp after which
//       withdrawals are allowed.
//
//   deposit(from, amount) -> Result<(), VaultError>
//     - `from` must authorize the call.
//     - `amount` must be positive, otherwise `VaultError::InvalidAmount`.
//     - Transfer `amount` tokens from `from` into the vault, add it to
//       `from`'s savings and publish a `Deposited` event.
//
//   withdraw(owner) -> Result<i128, VaultError>
//     - `owner` must authorize the call.
//     - Before `unlock_time`: `VaultError::StillLocked`.
//     - With no savings: `VaultError::NothingToWithdraw`.
//     - Otherwise send all of `owner`'s savings back, set them to zero and
//       return the amount withdrawn.
//
//   savings(owner) -> i128
//
// Good luck!

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InvalidAmount = 1,
    StillLocked = 2,
    NothingToWithdraw = 3,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Token,
    UnlockTime,
    Savings(Address),
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposited {
    #[topic]
    pub from: Address,
    pub amount: i128,
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    pub fn __constructor(env: Env, token: Address, unlock_time: u64) {
        todo!()
    }

    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), VaultError> {
        todo!()
    }

    pub fn withdraw(env: Env, owner: Address) -> Result<i128, VaultError> {
        todo!()
    }

    pub fn savings(env: Env, owner: Address) -> i128 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Events as _, Ledger as _};
    use soroban_sdk::Event;

    const UNLOCK: u64 = 1_800_000_000;

    struct Setup {
        env: Env,
        vault_id: Address,
        vault: VaultClient<'static>,
        token: token::TokenClient<'static>,
        user: Address,
    }

    fn setup() -> Setup {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_timestamp(UNLOCK - 1_000);

        let asset = env.register_stellar_asset_contract_v2(Address::generate(&env));
        let token = token::TokenClient::new(&env, &asset.address());
        let user = Address::generate(&env);
        token::StellarAssetClient::new(&env, &asset.address()).mint(&user, &1_000);

        let vault_id = env.register(Vault, (&asset.address(), UNLOCK));
        let vault = VaultClient::new(&env, &vault_id);
        Setup { env, vault_id, vault, token, user }
    }

    #[test]
    fn deposits_and_emits_event() {
        let s = setup();

        s.vault.deposit(&s.user, &400);

        // Events (like auths) are recorded for the most recent contract call.
        let expected = Deposited { from: s.user.clone(), amount: 400 };
        assert_eq!(
            s.env.events().all().filter_by_contract(&s.vault_id),
            [expected.to_xdr(&s.env, &s.vault_id)]
        );

        assert_eq!(s.vault.savings(&s.user), 400);
        assert_eq!(s.token.balance(&s.user), 600);
        assert_eq!(s.token.balance(&s.vault_id), 400);
    }

    #[test]
    fn rejects_invalid_amounts() {
        let s = setup();

        assert_eq!(s.vault.try_deposit(&s.user, &0), Err(Ok(VaultError::InvalidAmount)));
        assert_eq!(s.vault.try_deposit(&s.user, &-5), Err(Ok(VaultError::InvalidAmount)));
    }

    #[test]
    fn locked_until_unlock_time() {
        let s = setup();
        s.vault.deposit(&s.user, &400);

        assert_eq!(s.vault.try_withdraw(&s.user), Err(Ok(VaultError::StillLocked)));

        s.env.ledger().set_timestamp(UNLOCK);
        assert_eq!(s.vault.withdraw(&s.user), 400);
        assert_eq!(s.token.balance(&s.user), 1_000);
        assert_eq!(s.vault.savings(&s.user), 0);

        assert_eq!(s.vault.try_withdraw(&s.user), Err(Ok(VaultError::NothingToWithdraw)));
    }

    #[test]
    fn requires_auth() {
        let s = setup();
        s.vault.deposit(&s.user, &100);

        let auths = s.env.auths();
        assert_eq!(auths[0].0, s.user);
    }
}
