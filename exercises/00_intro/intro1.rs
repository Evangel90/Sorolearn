// intro1.rs
//
// Welcome to Sorolearn!
//
// Every exercise is a tiny Soroban smart contract with tests at the bottom.
// Your job is to make the code compile and the tests pass. Sorolearn watches
// the file you are working on and re-runs the tests every time you save.
//
// This first exercise already passes, so you can see what success looks like.
// Read through it, then go back to the terminal and press `n` to move on to
// the next exercise.

// Soroban contracts do not use Rust's standard library: they compile to tiny
// WebAssembly binaries that run inside the Stellar network. `no_std` keeps
// them small.
#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

// `#[contract]` marks the type that represents your contract.
#[contract]
pub struct HelloContract;

// `#[contractimpl]` exports every `pub fn` in this block as a contract
// function that anyone can invoke on-chain.
#[contractimpl]
impl HelloContract {
    // `Env` is your window to the blockchain: storage, events, auth, the
    // ledger... Every contract function may take it as its first argument.
    pub fn hello(_env: Env) -> Symbol {
        symbol_short!("hello")
    }
}

// Tests run natively on your machine against a simulated Soroban environment.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn says_hello() {
        // A fresh simulated blockchain.
        let env = Env::default();
        // Deploy ("register") the contract and get back its address.
        let contract_id = env.register(HelloContract, ());
        // `#[contractimpl]` generated a typed client for calling the contract.
        let client = HelloContractClient::new(&env, &contract_id);

        assert_eq!(client.hello(), symbol_short!("hello"));
    }
}
