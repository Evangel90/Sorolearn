# Errors

Declare errors with `#[contracterror]`:

```rust
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error { NotFound = 1, TooLarge = 2 }
```

Then either:

- return `Result<T, Error>` from a contract function, or
- abort with `panic_with_error!(&env, Error::NotFound)`.

Either way the call fails and **all state changes made during the call are rolled back**.

In tests, `client.foo(..)` panics on error, while `client.try_foo(..)` returns
`Err(Ok(error))` so you can assert on it.

Use checked arithmetic (`checked_add`, `checked_sub`, ...) whenever overflow is possible.

## Further reading

- [Errors example](https://developers.stellar.org/docs/build/smart-contracts/example-contracts/errors)
