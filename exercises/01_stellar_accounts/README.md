# Stellar: accounts

Soroban contracts run on **Stellar**, so it pays to know the network itself. These lessons
are plain Rust (no contract) and run fully offline.

- **Accounts are key pairs.** An ed25519 secret seed signs; the public key identifies the
  account. Keys are shown as *StrKeys*: `G…` accounts, `S…` secrets, `C…` contracts,
  `M…` muxed accounts. Each has a version byte and a checksum, so typos are caught.
- **Lumens (XLM)** are the native asset. On-ledger amounts are integers in **stroops**
  (1 XLM = 10,000,000 stroops).
- **Reserves:** every account keeps a minimum balance of `(2 + subentries) × 0.5 XLM`.
  Subentries are trustlines, offers, signers and data entries.
- **Multisig is built in:** signers have weights, and each operation needs the low,
  medium or high threshold to be met.

## Further reading

- [Accounts](https://developers.stellar.org/docs/learn/fundamentals/stellar-data-structures/accounts)
- [Lumens & reserves](https://developers.stellar.org/docs/learn/fundamentals/lumens)
- [Signatures and multisig](https://developers.stellar.org/docs/learn/encyclopedia/security/signatures-multisig)
