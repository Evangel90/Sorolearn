# Security: review the AI

AI assistants write Soroban contracts that compile, pass the obvious tests, and *look*
right, but sometimes they are exploitable. The cost of those bugs is real money, and you
can only catch them if you understand the platform.

Every exercise in this track is a plausible, AI-style contract with **one planted
vulnerability**:

- `happy_path_*` tests already pass: the contract "works".
- `exploit_*` tests reproduce an attack and currently **fail**.

Fix the contract (not the tests!) so the attack no longer works.

## Checklist for reviewing any Soroban contract

1. **Auth**: does every function that spends or changes a user's assets call
   `require_auth()` on *that* user (not the recipient, not "someone")?
2. **Initialization**: can `initialize`-style functions be called twice, or by anyone?
   Prefer `__constructor`.
3. **Amounts**: are signed amounts (`i128`) checked to be positive? Is arithmetic checked?
4. **Storage**: is each piece of data in the right tier (instance / persistent /
   temporary)? Are TTLs extended for data that must survive?
5. **External contracts**: are token / contract addresses supplied by the caller trusted
   blindly? Validate them against stored, trusted addresses.

## Further reading

- [Soroban security best practices](https://developers.stellar.org/docs/build/security-docs)
- [Authorization](https://developers.stellar.org/docs/learn/fundamentals/contract-development/authorization)
- [State archival](https://developers.stellar.org/docs/learn/fundamentals/contract-development/storage/state-archival)
