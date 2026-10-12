# Storage

Contracts persist state with `env.storage()`. There are three kinds:

| Kind | Use for | When TTL expires |
|---|---|---|
| `instance()` | small, contract-wide config (admin, token address) | archived with the contract |
| `persistent()` | important per-user data (balances) | archived, can be restored |
| `temporary()` | short-lived, cheap data (caches, nonces, oracle prices) | deleted forever |

All three share the same API: `get`, `set`, `has`, `remove`, `update`, `extend_ttl`.

Storage is **rented**. Every entry has a TTL in ledgers (~5 s each). Extend it with
`extend_ttl(&key, threshold, extend_to)`.

A common pattern is a `DataKey` enum used as the key for every entry:

```rust
#[contracttype]
#[derive(Clone)]
pub enum DataKey { Admin, Balance(Address) }
```

## Further reading

- [Storing data](https://developers.stellar.org/docs/build/smart-contracts/getting-started/storing-data)
- [State archival](https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival)
