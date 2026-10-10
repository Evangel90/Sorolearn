# Intro

Soroban is the smart contract platform of the [Stellar](https://stellar.org) network.
Contracts are written in Rust, compiled to WebAssembly, and executed by the network.

A minimal contract has three parts:

```rust
#![no_std]                       // no standard library: contracts are tiny wasm binaries
use soroban_sdk::{contract, contractimpl, Env};

#[contract]                      // the contract type
pub struct MyContract;

#[contractimpl]                  // its public functions
impl MyContract {
    pub fn hello(env: Env) -> u32 { 42 }
}
```

Contracts are tested natively with `Env::default()`, a simulated blockchain, plus
the `<Name>Client` that `#[contractimpl]` generates.

## Further reading

- [Getting started](https://developers.stellar.org/docs/build/smart-contracts/getting-started)
- [Hello World example](https://developers.stellar.org/docs/build/smart-contracts/getting-started/hello-world)
