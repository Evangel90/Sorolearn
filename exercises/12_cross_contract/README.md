# Cross-contract calls

Contracts call each other through generated clients:

```rust
let client = other::OtherContractClient::new(&env, &other_address);
let result = client.some_fn(&arg);
```

For a contract from another project, generate the client from its wasm:

```rust
mod other {
    soroban_sdk::contractimport!(file = "other_contract.wasm");
}
```

If the callee fails, the whole call fails, unless you use the `try_` variant and handle
the error.

## Further reading

- [Cross-contract calls](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/cross-contract-call)
