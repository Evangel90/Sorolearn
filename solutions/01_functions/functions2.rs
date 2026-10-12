// functions2.rs
//
// Two rules about contract functions:
//
//   1. Only `pub` functions in a `#[contractimpl]` block are exported. A
//      private function is just a helper the outside world cannot call.
//   2. Arguments and return values cross the contract boundary, so they must
//      be types Soroban knows how to encode: u32, i32, u64, i64, u128, i128,
//      bool, Symbol, Address, Vec, Map, ... Platform-dependent types like
//      `usize` are not allowed.
//
// Make the tests compile and pass.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Calculator;

#[contractimpl]
impl Calculator {
    pub fn square(_env: Env, x: u64) -> u64 {
        multiply(x, x)
    }

    pub fn cube(_env: Env, x: u64) -> u64 {
        multiply(multiply(x, x), x)
    }
}

// A private helper, outside the contractimpl block. Not callable on-chain.
fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_and_cubes() {
        let env = Env::default();
        let client = CalculatorClient::new(&env, &env.register(Calculator, ()));

        assert_eq!(client.square(&4_u64), 16);
        assert_eq!(client.cube(&3_u64), 27);
    }
}
