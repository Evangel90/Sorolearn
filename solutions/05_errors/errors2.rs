// errors2.rs
//
// Sometimes a function should simply abort. A plain `panic!` works, but the
// caller only learns that "something went wrong". `panic_with_error!` aborts
// with one of your `#[contracterror]` codes instead, so callers can tell
// failures apart, even when the function does not return a `Result`.
//
// Make `withdraw` abort with `WalletError::InsufficientFunds`.
//
// This contract also uses a *constructor*: `__constructor` runs once, when the
// contract is deployed. Tests pass its arguments to `env.register`.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, panic_with_error, symbol_short, Env, Symbol};

const BALANCE: Symbol = symbol_short!("BALANCE");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WalletError {
    InsufficientFunds = 1,
}

#[contract]
pub struct Wallet;

#[contractimpl]
impl Wallet {
    pub fn __constructor(env: Env, initial_balance: i128) {
        env.storage().instance().set(&BALANCE, &initial_balance);
    }

    pub fn withdraw(env: Env, amount: i128) -> i128 {
        let balance: i128 = env.storage().instance().get(&BALANCE).unwrap();
        if amount > balance {
            panic_with_error!(&env, WalletError::InsufficientFunds);
        }
        let new_balance = balance - amount;
        env.storage().instance().set(&BALANCE, &new_balance);
        new_balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn withdraws() {
        let env = Env::default();
        let client = WalletClient::new(&env, &env.register(Wallet, (100_i128,)));

        assert_eq!(client.withdraw(&30), 70);
        assert_eq!(client.withdraw(&70), 0);
    }

    #[test]
    fn rejects_overdraft_with_error_code() {
        let env = Env::default();
        let client = WalletClient::new(&env, &env.register(Wallet, (100_i128,)));

        // `withdraw` does not return a Result, so errors come back as the
        // generic `soroban_sdk::Error`, which our error converts into.
        assert_eq!(client.try_withdraw(&500), Err(Ok(WalletError::InsufficientFunds.into())));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn overdraft_panics() {
        let env = Env::default();
        let client = WalletClient::new(&env, &env.register(Wallet, (100_i128,)));

        client.withdraw(&500);
    }
}
