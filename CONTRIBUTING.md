# Contributing to Sorolearn

Thanks for helping people learn to build on Stellar! Sorolearn is small on purpose, so most
contributions are one of these:

- **A new exercise** (or a better hint, comment or topic README)
- **A bug fix** in an exercise, the `sorolearn` CLI or the registry contract
- **Website** improvements

Not sure where to start? Open an issue describing the idea or the problem first. For anything bigger
than a typo, that saves everyone time.

## Setup

You need [Rust](https://rustup.rs) 1.91 or newer.

```bash
git clone https://github.com/Evangel90/Sorolearn.git
cd Sorolearn
cargo build
cargo run -- setup      # downloads and pre-builds the Soroban SDK (once)
```

Or open the repo in GitHub Codespaces: Rust, the Stellar CLI and a compiled SDK are ready to go.

Run the CLI from the repo root with `cargo run -- <command>`, for example `cargo run -- list`.

## Adding an exercise

Every exercise is one `.rs` file with tests at the bottom. The **solution is written first**, then broken.

1. **Write the solution** in `solutions/<topic>/<name>.rs`:
   - Contracts are `#![no_std]` with a `#[cfg(test)] mod tests` at the bottom.
   - The comment at the top explains the concept and states the task in one or two sentences.
   - Tests are the spec: they should fail with a clear message when the learner is wrong.
2. **Copy it** to `exercises/<topic>/<name>.rs` and **break it in an instructive way**: a missing
   line, a `todo!()`, a wrong type, a subtle bug. One or two problems per exercise, each teaching
   something. The learner should never need to edit the tests.
3. **Add an entry** to `info.toml`, in the position where it should appear in the course:

   ```toml
   [[exercises]]
   name = "storage4"
   dir = "04_storage"
   hint = """
   What to look at first, then (if needed) the exact fix."""
   ```

   Names are unique and become Cargo package names, so use lowercase letters, digits and `_`.
4. **Document it**: if the exercise needs background the topic's `README.md` doesn't cover, add it
   there, with links to the [Stellar docs](https://developers.stellar.org/docs).
5. **Check it**: `cargo run -- dev check` (see below). Then solve it yourself in watch mode once,
   reading only the comments and hints, to make sure it's fair.

### Security exercises (`14_security`)

These are plausible, AI-style contracts with **one planted vulnerability**:

- Name the tests `happy_path_*` (pass as given) and `exploit_*` (reproduce the attack, fail until fixed).
  `dev check` enforces this: the unsolved file must compile, the happy path must pass, and only the
  exploit tests may fail.
- Don't leave the answer lying around: no unused error variant or import that gives away the fix.
- In the solution, mark the change with a `// FIX:` comment explaining the vulnerability.

### Stellar network lessons (`11`-`13`)

These are plain Rust (no contract). Set `stellar = true` in `info.toml` to give the exercise the
`stellar-strkey`, `ed25519-dalek` and `sha2` crates; XDR types come from `soroban_sdk::xdr`.
When a test pins a real value (an address, a hash), **cross-check it with the Stellar CLI** and say
so in a comment, as `accounts2` and `transactions2` do.

## Checks

CI runs these on every push. Run them before opening a pull request:

```bash
cargo run -- dev check                                # every solution passes, every exercise fails
cargo test                                            # CLI unit tests
cargo clippy -- -D warnings                           # CLI lints
cargo test --locked --manifest-path registry/Cargo.toml   # registry contract tests
```

`dev check` compiles every exercise and solution, so the first run takes a few minutes.

## The registry contract

`registry/` holds the on-chain graduation registry. Its tests use Wasm builds of the `quiz1` poll in
`registry/testdata/`. If you change the exam, rebuild those and keep the error codes in
`exercises/15_testnet/README.md` and `src/graduate.rs` in sync. Redeploying is a maintainer task
(see `registry/README.md`).

## The website

The site in `docs/` is static and deployed to Vercel. Most pages are generated:

```bash
python3 scripts/site/build.py
```

Edit `scripts/site/build.py` (content) or `scripts/site/layout.py` (header and footer), then run
the generator and commit both the script and the generated `docs/` pages. Styles live in
`docs/assets/site.css`. The graduates page (`docs/graduates/index.html`) is hand-written.

## Upgrading the Soroban SDK

1. Change `SOROBAN_SDK_VERSION` in `src/info.rs`.
2. Delete `exercises.lock`, run `cargo run -- setup`, then copy `.sorolearn/exercises/Cargo.lock` to
   `exercises.lock`.
3. Run `cargo run -- dev check` and fix anything the new SDK breaks. Read the SDK's `_migrating`
   notes for the versions you skipped.
4. Keep the SDK on the protocol that testnet and mainnet run.

## Pull requests

- **Small and focused**: one exercise, fix or feature per PR.
- **Commit messages** follow [Conventional Commits](https://www.conventionalcommits.org):
  `feat(exercises): add storage4, TTL for instance storage`, `fix(cli): ...`, `docs: ...`.
- Describe what the learner should take away (for exercises) or what was broken (for fixes).
- All checks above pass.

## License

By contributing, you agree that your contributions are licensed under the [MIT License](LICENSE).
