// bytes1.rs
//
// Soroban has its own host-managed byte and text types:
//
//   Bytes      - a growable byte array
//   BytesN<N>  - a fixed-size byte array (great for hashes and keys)
//   String     - UTF-8 text (not `std::string::String`!)
//
// Make the contract compile and implement `text_len`.

#![no_std]

use soroban_sdk::{contract, contractimpl, Bytes, Env, String};

#[contract]
pub struct BytesContract;

#[contractimpl]
impl BytesContract {
    /// Appends `b` to the end of `a`.
    pub fn join(_env: Env, mut a: Bytes, b: Bytes) -> Bytes {
        a.append(&b);
        a
    }

    /// Returns the length of `text` in bytes.
    pub fn text_len(_env: Env, text: String) -> u32 {
        text.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_bytes() {
        let env = Env::default();
        let client = BytesContractClient::new(&env, &env.register(BytesContract, ()));

        let a = Bytes::from_slice(&env, b"Soro");
        let b = Bytes::from_slice(&env, b"ban");
        assert_eq!(client.join(&a, &b), Bytes::from_slice(&env, b"Soroban"));
    }

    #[test]
    fn measures_text() {
        let env = Env::default();
        let client = BytesContractClient::new(&env, &env.register(BytesContract, ()));

        assert_eq!(client.text_len(&String::from_str(&env, "Stellar")), 7);
    }
}
