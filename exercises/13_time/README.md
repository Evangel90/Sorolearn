# Ledger & time

`env.ledger()` describes the ledger the transaction runs in:

- `timestamp()`: close time in UNIX seconds, for real-world deadlines,
- `sequence()`: ledger number (one every ~5 s), for durations in ledgers such as TTLs.

In tests (with `soroban_sdk::testutils::Ledger as _`):

```rust
env.ledger().set_timestamp(1_700_000_000);
env.ledger().set_sequence_number(100);
```
