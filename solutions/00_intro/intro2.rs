// intro2.rs
//
// This contract does not compile yet! The tests try to use `GreeterClient`,
// but nothing generates it.
//
// Fix the contract so its functions are exported and the client exists.

#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct Greeter;

#[contractimpl]
impl Greeter {
    pub fn greet(env: Env, name: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), name]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_by_name() {
        let env = Env::default();
        let contract_id = env.register(Greeter, ());
        let client = GreeterClient::new(&env, &contract_id);

        assert_eq!(
            client.greet(&symbol_short!("Stellar")),
            vec![&env, symbol_short!("Hello"), symbol_short!("Stellar")]
        );
    }
}
