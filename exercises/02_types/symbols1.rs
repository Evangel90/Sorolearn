// symbols1.rs
//
// `Symbol` is Soroban's small, cheap identifier type (letters, digits and `_`).
//
// - `symbol_short!("...")` builds one at compile time, but only for up to
//   9 characters.
// - `Symbol::new(&env, "...")` works for up to 32 characters and is created
//   at runtime, so it needs the `Env`.
//
// Make this compile.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Actions;

#[contractimpl]
impl Actions {
    pub fn short_action(_env: Env) -> Symbol {
        symbol_short!("transfer")
    }

    pub fn long_action(_env: Env) -> Symbol {
        symbol_short!("transfer_from")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_actions() {
        let env = Env::default();
        let client = ActionsClient::new(&env, &env.register(Actions, ()));

        assert_eq!(client.short_action(), Symbol::new(&env, "transfer"));
        assert_eq!(client.long_action(), Symbol::new(&env, "transfer_from"));
    }
}
