# Deploy to Stellar testnet

Everything so far ran offline, against Soroban's simulated environment. Now ship it: deploy the
contracts **you** wrote to Stellar testnet, call them from real accounts, watch their events,
connect them to a classic Stellar asset, and finally earn an **on-chain graduation record**.

Each lab uses an exercise you have already solved. `sorolearn build-wasm <name>` compiles it to a
deployable `.wasm` (and prints the `stellar contract build` command it runs).

## Setup

Install the [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli)
(`brew install stellar-cli` or `cargo install --locked stellar-cli`; it's pre-installed in
Codespaces) and the wasm target:

```bash
rustup target add wasm32v1-none
```

Create two testnet accounts, funded with free test XLM by Friendbot (`11_stellar_accounts`
explains what you just made):

```bash
stellar keys generate me --network testnet --fund
stellar keys generate friend --network testnet --fund
stellar keys address me
```

## Lab 1: your first deployment (`storage1`)

Deploy the counter, then call it from two different accounts. Each `invoke` is a real
transaction (`12_stellar_transactions`): signed by the source account, paid for in XLM.

```bash
sorolearn build-wasm storage1
stellar contract deploy --wasm .sorolearn/wasm/storage1/out/storage1.wasm \
  --source-account me --network testnet --alias counter
stellar contract invoke --id counter --source-account me --network testnet -- increment
# => 1
stellar contract invoke --id counter --source-account friend --network testnet -- increment
# => 2  (instance storage is shared by everyone who calls the contract)
stellar contract invoke --id counter --source-account me --network testnet --send=no -- get
# => 2  (--send=no simulates: a free read, no transaction submitted)
```

Paste the contract ID into [stellar.expert](https://stellar.expert/explorer/testnet) to see its
transactions and storage.

## Lab 2: constructors and auth for real (`auth2`)

Constructor arguments go after `--`. Then see `require_auth` protect the contract on a real
network:

```bash
sorolearn build-wasm auth2
stellar contract deploy --wasm .sorolearn/wasm/auth2/out/auth2.wasm \
  --source-account me --network testnet --alias fees -- --admin me
stellar contract invoke --id fees --source-account me --network testnet -- set_fee --fee 25
stellar contract invoke --id fees --source-account friend --network testnet -- set_fee --fee 99
# => error: Missing signing key for account G...  (the admin, `me`, never signed)
stellar contract invoke --id fees --source-account me --network testnet --send=no -- fee
# => 25
```

## Lab 3: events you can watch (`events1`)

```bash
sorolearn build-wasm events1
START=$(stellar ledger latest --network testnet | awk '/^Sequence/{print $2}')
stellar contract deploy --wasm .sorolearn/wasm/events1/out/events1.wasm \
  --source-account me --network testnet --alias tipjar
stellar contract invoke --id tipjar --source-account me --network testnet \
  -- tip --from me --to friend --amount 100
stellar events --start-ledger $START --id tipjar --network testnet
# => Event: Tipped (tipped)  from: "G…"  to: "G…"  amount: "100"
```

This is exactly what wallets and indexers consume.

## Lab 4: your own asset, held by your contract (`assets1`, `tokens1`)

Issue a classic Stellar asset (`13_stellar_assets`), give it a contract interface, and let your
piggy bank from `08_tokens` hold it:

```bash
ISSUER=$(stellar keys address me)

# friend opens a trustline to SORO issued by `me`, then receives 100 SORO
# (amounts are in stroops: 100 × 10,000,000)
stellar tx new change-trust --source-account friend --line "SORO:$ISSUER" --network testnet
stellar tx new payment --source-account me --destination friend \
  --asset "SORO:$ISSUER" --amount 1000000000 --network testnet

# The asset's Stellar Asset Contract: the same balances, now callable from Soroban
stellar contract asset deploy --asset "SORO:$ISSUER" \
  --source-account me --network testnet --alias soro

# Your piggy bank, configured with the SORO contract, receives a deposit from friend
sorolearn build-wasm tokens1
stellar contract deploy --wasm .sorolearn/wasm/tokens1/out/tokens1.wasm \
  --source-account me --network testnet --alias piggy -- --token soro
stellar contract invoke --id piggy --source-account friend --network testnet \
  -- deposit --from friend --amount 300000000
stellar contract invoke --id piggy --source-account friend --network testnet --send=no -- total
# => "300000000"
stellar contract invoke --id soro --source-account friend --network testnet --send=no \
  -- balance --id friend
# => "700000000"
```

The same SORO moved by a classic payment and by a contract call: one asset, two interfaces.

## Graduation

The Sorolearn **registry** contract lives on testnet at:

```
CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY
```

When you call `graduate`, the registry:

1. deploys a **fresh copy** of your `quiz1` poll contract from the code you uploaded,
2. **examines it with cross-contract calls**: votes, tries to vote twice, checks the tally and
   `has_voted`,
3. if it passes, records your address as a graduate and emits a `Graduated` event.

Completion is checked on-chain by a contract, not self-reported. Graduates appear on the
[graduates page](https://evangel90.github.io/Sorolearn/graduates/).

### The quick way

```bash
sorolearn graduate            # uses (and creates) the testnet identity `sorolearn`
sorolearn graduate me         # or use an identity you already have
```

It prints every command it runs: the same steps as below.

### By hand

Upload your poll's code (just the code, not an instance: the registry deploys its own copy) and
call the registry:

```bash
sorolearn build-wasm quiz1
HASH=$(stellar contract upload --wasm .sorolearn/wasm/quiz1/out/quiz1.wasm \
  --source-account me --network testnet)
stellar contract invoke --id CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY \
  --source-account me --network testnet \
  -- graduate --learner me --wasm_hash $HASH
```

### Exam error codes

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
- Clean up when you're done: `stellar keys rm <name>` and `stellar contract alias remove <alias> --network testnet`.

## Further reading

- [Deploy to testnet](https://developers.stellar.org/docs/build/smart-contracts/getting-started/deploy-to-testnet)
- [Stellar CLI manual](https://developers.stellar.org/docs/tools/cli/stellar-cli)
- [Events](https://developers.stellar.org/docs/learn/fundamentals/contract-development/events)
