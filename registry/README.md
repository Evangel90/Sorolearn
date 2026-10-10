# Sorolearn registry

The on-chain graduation registry used by `sorolearn graduate`.

`graduate(learner, wasm_hash)` deploys a fresh instance of the uploaded Wasm (the learner's
`quiz1` poll), examines it with cross-contract calls, and records the learner on success. See
`src/lib.rs` for the exam and `src/test.rs` for its tests. `testdata/` holds Wasm builds of a correct
poll, the unsolved exercise and a poll that allows double voting.

Current testnet deployment: `CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY`

## Test

```bash
cargo test --manifest-path registry/Cargo.toml
```

## Redeploy (e.g. after a testnet reset)

```bash
cd registry
stellar keys generate sorolearn-deployer --network testnet --fund   # once
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/sorolearn_registry.wasm \
  --source-account sorolearn-deployer --network testnet --alias sorolearn-registry
```

Then update `REGISTRY_ID` in `src/graduate.rs`, the docs, and `docs/index.html`.
