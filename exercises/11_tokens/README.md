# Tokens

Every token on Stellar, including XLM and classic assets like USDC (through the
*Stellar Asset Contract*), implements the same interface ([SEP-41](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md)).
Talk to any of them with:

```rust
let token = soroban_sdk::token::TokenClient::new(&env, &token_address);
token.balance(&who);
token.transfer(&from, &to, &amount);
```

Contracts can own tokens: their address is `env.current_contract_address()`.

In tests, create a token with `env.register_stellar_asset_contract_v2(admin)` and mint
with `token::StellarAssetClient::new(&env, &address).mint(&to, &amount)`.

## Further reading

- [Tokens](https://developers.stellar.org/docs/tokens)
- [Stellar Asset Contract](https://developers.stellar.org/docs/tokens/stellar-asset-contract)
