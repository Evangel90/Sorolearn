# Built-in types

Contract values live inside the Soroban *host*, not in your wasm memory. That's why most
container types need the environment to be created:

| Type | Create | Notes |
|---|---|---|
| `Symbol` | `symbol_short!("abc")`, `Symbol::new(&env, "abc")` | identifiers, up to 32 chars `[a-zA-Z0-9_]` (9 for `symbol_short!`) |
| `Vec<T>` | `Vec::new(&env)`, `vec![&env, 1, 2]` | `push_back`, `get`, `len`, `iter` |
| `Map<K, V>` | `Map::new(&env)`, `map![&env, (k, v)]` | ordered by key; `get`, `set`, `remove` |
| `Bytes` / `BytesN<N>` | `Bytes::from_slice(&env, b"..")` | raw data, hashes, keys |
| `String` | `String::from_str(&env, "..")` | UTF-8 text |

Amounts of money use `i128` by convention.

## Further reading

- [Built-in types](https://developers.stellar.org/docs/learn/fundamentals/contract-development/types/built-in-types)
