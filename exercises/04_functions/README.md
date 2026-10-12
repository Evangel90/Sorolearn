# Contract functions

Every `pub fn` inside a `#[contractimpl]` block becomes a function that anyone can invoke.

- The first parameter is usually `env: Env`, which gives access to storage, events,
  auth, the ledger and more. Prefix it with `_` if you don't use it.
- Parameters and return values must be types Soroban can encode: integers
  (`u32`, `i32`, `u64`, `i64`, `u128`, `i128`, ...), `bool`, `Symbol`, `String`,
  `Bytes`, `Address`, `Vec`, `Map`, and your own `#[contracttype]`s.
- `usize`, `std::string::String` and references are **not** allowed.
- Private functions (no `pub`), or functions outside the block, are internal helpers.

## Further reading

- [Contract functions & the Env](https://developers.stellar.org/docs/learn/fundamentals/contract-development/environment-concepts)
