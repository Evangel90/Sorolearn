# Graduation: Stellar testnet

Everything so far ran in Soroban's simulated environment. Now put your code on a real network
and earn an **on-chain graduation record**.

The Sorolearn **registry** contract lives on testnet at:

```
CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY
```

When you call `graduate`, the registry:

1. deploys a **fresh copy** of your `quiz1` poll contract from the code you uploaded,
2. **examines it with cross-contract calls**: votes, tries to vote twice, checks the tally and
   `has_voted`,
3. if it passes, records your address as a graduate and emits a `Graduated` event.

Completion is checked on-chain by a contract, not self-reported.

## The quick way

```bash
sorolearn graduate            # uses (and creates) the testnet identity `sorolearn`
sorolearn graduate alice      # or pick your own identity name
```

It prints every command it runs, which are the same steps as below.

## The manual way (recommended once!)

You need the [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli)
(`brew install stellar-cli` or `cargo install --locked stellar-cli`) and the wasm target:

```bash
rustup target add wasm32v1-none
```

**1. Create a testnet account**, funded with free test XLM by Friendbot:

```bash
stellar keys generate me --network testnet --fund
stellar keys address me
```

**2. Build your contract to WebAssembly.** Sorolearn generates a small crate for the exercise in
`.sorolearn/wasm/quiz1/` and runs `stellar contract build` on it (it prints the exact command):

```bash
sorolearn build-wasm quiz1
WASM=.sorolearn/wasm/quiz1/out/quiz1.wasm
```

In your own projects, `stellar contract init` scaffolds a contract crate and `stellar contract build`
builds it.

**3. Deploy and play with your poll** (optional, but this is how real contracts ship):

```bash
stellar contract deploy --wasm $WASM \
  --source-account me --network testnet --alias my-poll
stellar contract invoke --id my-poll --source-account me --network testnet \
  -- vote --voter me --option pizza
stellar contract invoke --id my-poll --source-account me --network testnet \
  -- votes --option pizza
```

**4. Upload the code and graduate:**

```bash
HASH=$(stellar contract upload --wasm $WASM \
  --source-account me --network testnet)
stellar contract invoke --id CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY \
  --source-account me --network testnet \
  -- graduate --learner me --wasm_hash $HASH
```

## Exam error codes

| Code | Meaning |
|---|---|
| `#1` | this address already graduated |
| `#10` | the first vote was rejected, or your contract trapped |
| `#11` | the same address could vote twice |
| `#12` | a second vote did not fail with `PollError::AlreadyVoted` (#1) |
| `#13` | `votes` returned the wrong tally |
| `#14` | `has_voted` returned the wrong answer, or trapped |

## Good to know

- Testnet is **reset periodically**, which wipes all contracts and accounts. If the registry is
  gone, a maintainer redeploys it (`registry/README.md`). Point sorolearn at the new one with
  `SOROLEARN_REGISTRY=<contract id>`.
- Testnet XLM has no value. Never reuse a testnet key on mainnet.

## Further reading

- [Deploy to testnet](https://developers.stellar.org/docs/build/smart-contracts/getting-started/deploy-to-testnet)
- [Stellar CLI manual](https://developers.stellar.org/docs/tools/cli/stellar-cli)
