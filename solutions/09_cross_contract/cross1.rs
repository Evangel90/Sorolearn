// cross1.rs
//
// Contracts can call other contracts. `#[contractimpl]` generates a client
// (`<Name>Client`) for every contract, and the same client works both in
// tests and *inside another contract*.
//
// For a contract that lives in a different project you would normally import
// its compiled WASM with `soroban_sdk::contractimport!`, which generates the
// same kind of client. Here both contracts live in the same file, so we can
// use the client directly.
//
// Implement `Caller::add_three` by calling the Calculator contract.

#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

pub mod calculator {
    use soroban_sdk::{contract, contractimpl, Env};

    #[contract]
    pub struct Calculator;

    #[contractimpl]
    impl Calculator {
        pub fn add(_env: Env, a: u32, b: u32) -> u32 {
            a + b
        }
    }
}

#[contract]
pub struct Caller;

#[contractimpl]
impl Caller {
    /// Returns `a + b + c`, computed by the Calculator deployed at `calculator`.
    pub fn add_three(env: Env, calculator: Address, a: u32, b: u32, c: u32) -> u32 {
        let client = calculator::CalculatorClient::new(&env, &calculator);
        let ab = client.add(&a, &b);
        client.add(&ab, &c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn calls_calculator() {
        let env = Env::default();
        let calculator_id = env.register(calculator::Calculator, ());
        let caller = CallerClient::new(&env, &env.register(Caller, ()));

        assert_eq!(caller.add_three(&calculator_id, &1, &2, &3), 6);
    }

    #[test]
    fn fails_without_a_calculator() {
        let env = Env::default();
        let caller = CallerClient::new(&env, &env.register(Caller, ()));
        // An address with no contract deployed: the cross-contract call fails.
        let nobody = Address::generate(&env);

        assert!(caller.try_add_three(&nobody, &1, &2, &3).is_err());
    }
}
