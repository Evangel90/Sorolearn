// storage1.rs
//
// Contracts keep state in storage. Soroban has three kinds:
//
//   instance()   - small config/state tied to the contract itself
//   persistent() - long-lived data, typically one entry per user
//   temporary()  - cheap data that is allowed to expire
//
// Every kind works like a key-value store: `get(&key)`, `set(&key, &value)`,
// `has(&key)`, `remove(&key)`.
//
// This counter never seems to go past 1. Fix it.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Increments the counter and returns the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        count
    }

    pub fn get(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_up() {
        let env = Env::default();
        let client = CounterClient::new(&env, &env.register(Counter, ()));

        assert_eq!(client.get(), 0);
        assert_eq!(client.increment(), 1);
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);
        assert_eq!(client.get(), 3);
    }
}
