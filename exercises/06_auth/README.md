# Authorization

Anyone can call any contract function with any arguments. Whenever a function acts on
behalf of an address (moves its funds, changes its data), call:

```rust
user.require_auth();
```

The Soroban host checks that `user` authorized *this exact call with these arguments*,
either by signing (for accounts) or by being the calling contract (for contracts).
There's no `msg.sender`: you name the address that must authorize the call.

The **admin pattern**: store an admin address in the `__constructor`, then call
`admin.require_auth()` in privileged functions.

In tests:

- `env.mock_all_auths()` makes every `require_auth` succeed,
- `env.auths()` shows which authorizations the last call required.

## Further reading

- [Authorization](https://developers.stellar.org/docs/learn/fundamentals/contract-development/authorization)
- [Auth example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/auth)
