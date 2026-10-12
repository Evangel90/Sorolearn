# Custom types

`#[contracttype]` lets you use your own structs and enums as arguments, return values
and storage keys/values. They become part of the contract's public interface (its *spec*),
so clients and tools know how to encode them.

```rust
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Order { pub id: u64, pub amount: i128 }

#[contracttype]
#[derive(Clone)]
pub enum Status { Open, Filled(i128) }
```

## Further reading

- [Custom types](https://developers.stellar.org/docs/learn/fundamentals/contract-development/types/custom-types)
