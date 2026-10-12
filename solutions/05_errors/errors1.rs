// errors1.rs
//
// Contracts report expected failures with error codes. Declare them with
// `#[contracterror]` on a `u32`-backed enum, and return them from functions
// as `Result<T, YourError>`.
//
// Callers then receive a clean, typed error instead of a crashed contract.
//
// Make `divide` and `add` return errors instead of panicking.

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MathError {
    DivisionByZero = 1,
    Overflow = 2,
}

#[contract]
pub struct SafeMath;

#[contractimpl]
impl SafeMath {
    pub fn divide(_env: Env, a: u32, b: u32) -> Result<u32, MathError> {
        if b == 0 {
            return Err(MathError::DivisionByZero);
        }
        Ok(a / b)
    }

    pub fn add(_env: Env, a: u32, b: u32) -> Result<u32, MathError> {
        a.checked_add(b).ok_or(MathError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides() {
        let env = Env::default();
        let client = SafeMathClient::new(&env, &env.register(SafeMath, ()));

        // The plain client method unwraps the Ok value for you...
        assert_eq!(client.divide(&10, &2), 5);
        // ...while `try_` methods let you inspect errors.
        assert_eq!(client.try_divide(&1, &0), Err(Ok(MathError::DivisionByZero)));
    }

    #[test]
    fn adds_without_overflowing() {
        let env = Env::default();
        let client = SafeMathClient::new(&env, &env.register(SafeMath, ()));

        assert_eq!(client.add(&1, &2), 3);
        assert_eq!(client.try_add(&u32::MAX, &1), Err(Ok(MathError::Overflow)));
    }
}
