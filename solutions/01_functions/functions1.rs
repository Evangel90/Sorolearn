// functions1.rs
//
// Contract functions are ordinary Rust functions inside a `#[contractimpl]`
// block. Replace each `todo!()` with a real implementation.

#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MathContract;

#[contractimpl]
impl MathContract {
    /// Returns the sum of `a` and `b`.
    pub fn add(_env: Env, a: u32, b: u32) -> u32 {
        a + b
    }

    /// Returns `true` when `n` is even.
    pub fn is_even(_env: Env, n: u32) -> bool {
        n % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        let env = Env::default();
        let client = MathContractClient::new(&env, &env.register(MathContract, ()));

        assert_eq!(client.add(&2, &3), 5);
        assert_eq!(client.add(&0, &0), 0);
    }

    #[test]
    fn checks_parity() {
        let env = Env::default();
        let client = MathContractClient::new(&env, &env.register(MathContract, ()));

        assert!(client.is_even(&4));
        assert!(!client.is_even(&7));
    }
}
