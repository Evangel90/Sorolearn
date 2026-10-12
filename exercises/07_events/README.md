# Events

Events tell the outside world what happened, so wallets, indexers and frontends can react.

```rust
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    #[topic] pub from: Address,   // topics: indexable
    #[topic] pub to: Address,
    pub amount: i128,             // data
}

Transfer { from, to, amount }.publish(&env);
```

The struct name in snake_case (`"transfer"`) is automatically the first topic.

In tests, compare `env.events().all()` with `event.to_xdr(&env, &contract_id)`.

## Further reading

- [Events](https://developers.stellar.org/docs/learn/fundamentals/contract-development/events)
