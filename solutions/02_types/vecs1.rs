// vecs1.rs
//
// Contracts cannot use `std::vec::Vec`. Instead they use `soroban_sdk::Vec`,
// whose data lives in the Soroban host. That's why creating one needs the
// environment: `Vec::new(&env)` or `vec![&env, 1, 2, 3]`.
//
// Implement `sum` and fix `doubled`.

#![no_std]

use soroban_sdk::{contract, contractimpl, vec, Env, Vec};

#[contract]
pub struct VecContract;

#[contractimpl]
impl VecContract {
    /// Adds up every value.
    pub fn sum(_env: Env, values: Vec<u32>) -> u32 {
        let mut total = 0;
        for value in values.iter() {
            total += value;
        }
        total
    }

    /// Returns a new Vec with every value multiplied by two.
    pub fn doubled(env: Env, values: Vec<u32>) -> Vec<u32> {
        let mut out = Vec::new(&env);
        for value in values.iter() {
            out.push_back(value * 2);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        let env = Env::default();
        let client = VecContractClient::new(&env, &env.register(VecContract, ()));

        assert_eq!(client.sum(&vec![&env, 1, 2, 3, 4]), 10);
        assert_eq!(client.sum(&Vec::new(&env)), 0);
    }

    #[test]
    fn doubles() {
        let env = Env::default();
        let client = VecContractClient::new(&env, &env.register(VecContract, ()));

        assert_eq!(client.doubled(&vec![&env, 1, 5, 10]), vec![&env, 2, 10, 20]);
    }
}
