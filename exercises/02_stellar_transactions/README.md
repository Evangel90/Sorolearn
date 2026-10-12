# Stellar: transactions

A **transaction** bundles up to 100 **operations** that succeed or fail together. It names a
source account, a **sequence number** (current + 1, which orders transactions and stops
replays), a **fee** (100 stroops × operations, as a maximum bid), optional
**preconditions** such as time bounds, and a **memo**.

Transactions are encoded as **XDR**, Stellar's binary format. `soroban_sdk::xdr` exposes the
same types the network and the Stellar CLI use.

**Signing:** the transaction hash is `sha256` of a `TransactionSignaturePayload` that includes
the **network ID** (`sha256(network passphrase)`), so a testnet signature can never be replayed
on mainnet. Signatures are ed25519 over that hash, attached with a 4-byte **hint**.

These lessons check their results against the real thing: the expected transaction hash in
`transactions2` was computed with `stellar tx hash`.

## Further reading

- [Transactions](https://developers.stellar.org/docs/learn/fundamentals/transactions)
- [Operations and transactions](https://developers.stellar.org/docs/learn/fundamentals/transactions/operations-and-transactions)
- [Fees](https://developers.stellar.org/docs/learn/fundamentals/fees-resource-limits-metering)
