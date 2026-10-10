//! `sorolearn graduate`: put your quiz1 poll on Stellar testnet and get it
//! examined by the on-chain Sorolearn registry.
//!
//! Every step shells out to the Stellar CLI and prints the exact command, so
//! the learner sees (and can repeat) the real deployment workflow.

use crate::info::{Exercise, Info, SOROBAN_SDK_VERSION};
use crate::runner::{self, Source, TARGET_DIR};
use crate::ui;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The registry deployed on testnet. Override with SOROLEARN_REGISTRY, e.g.
/// after a testnet reset.
pub const REGISTRY_ID: &str = "CD23Z2JCHQHLJOXWAWOV4NFMAL3N5YRDCGLBGHLKD62X5QUCHDYPFWYY";
const EXAM_EXERCISE: &str = "quiz1";
const NETWORK: &str = "testnet";
const WASM_DIR: &str = ".sorolearn/wasm";

pub fn graduate(info: &Info, identity: &str) -> bool {
    let registry = std::env::var("SOROLEARN_REGISTRY").unwrap_or_else(|_| REGISTRY_ID.to_string());
    println!("{}\n", ui::bold("🎓 Sorolearn graduation (Stellar testnet)"));

    // 1. The exam contract must pass locally first.
    let Some(ex) = info.find(EXAM_EXERCISE).map(|i| &info.exercises[i]) else {
        eprintln!("{}", ui::red("info.toml has no quiz1 exercise."));
        return false;
    };
    step(1, &format!("Check that {} passes locally", ex.path().display()));
    if !runner::run(ex, Source::Exercises).success {
        eprintln!("{}", ui::red("quiz1 is not solved yet. Run `sorolearn run quiz1` and fix it first."));
        return false;
    }

    // 2. Tooling.
    step(2, "Check the Stellar CLI and the wasm target");
    if !quiet(Command::new("stellar").arg("--version")) {
        eprintln!(
            "{}\n  macOS:  brew install stellar-cli\n  other:  cargo install --locked stellar-cli\n  docs:   https://developers.stellar.org/docs/tools/cli/install-cli",
            ui::red("The Stellar CLI is not installed.")
        );
        return false;
    }
    if !show(Command::new("rustup").args(["target", "add", "wasm32v1-none"])) {
        return false;
    }

    // 3. A testnet account, funded by Friendbot.
    step(3, &format!("Use the testnet account `{identity}`"));
    if !quiet(Command::new("stellar").args(["keys", "address", identity])) {
        let ok = show(Command::new("stellar").args(["keys", "generate", identity, "--network", NETWORK, "--fund"]));
        if !ok {
            return false;
        }
    }
    let Some(address) = capture(Command::new("stellar").args(["keys", "address", identity])) else {
        return false;
    };

    // 4. Compile quiz1 to WebAssembly.
    step(4, "Build quiz1 as a WebAssembly contract");
    let Some(wasm) = build_wasm(ex) else {
        return false;
    };

    // 5. Upload the code (not an instance: the registry deploys its own copy).
    step(5, "Upload the contract code to testnet");
    let Some(wasm_hash) = capture(
        Command::new("stellar")
            .args(["contract", "upload", "--wasm"])
            .arg(&wasm)
            .args(["--source-account", identity, "--network", NETWORK]),
    ) else {
        return false;
    };
    println!("  wasm hash: {}", ui::cyan(&wasm_hash));

    // 6. The exam.
    step(6, "Ask the registry to examine your contract and record you");
    let result = Command::new("stellar")
        .args(["contract", "invoke", "--id", &registry, "--source-account", identity, "--network", NETWORK])
        .args(["--", "graduate", "--learner", identity, "--wasm_hash", &wasm_hash])
        .output();
    let out = match result {
        Ok(out) => out,
        Err(e) => {
            eprintln!("{}", ui::red(&format!("Failed to run stellar: {e}")));
            return false;
        }
    };
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() {
        println!("{}", stderr.trim_end());
        println!("\n{}", ui::red(&format!("✗ {}", explain_failure(&stderr))));
        return false;
    }

    let number = stdout.trim().trim_matches('"');
    println!();
    println!("{}", ui::green(&format!("✓ You are Sorolearn graduate #{number}!")));
    println!("  Your account:  https://stellar.expert/explorer/testnet/account/{address}");
    println!("  The registry:  https://stellar.expert/explorer/testnet/contract/{registry}");
    println!("\nAnyone can verify it on-chain:");
    println!(
        "  stellar contract invoke --id {registry} --network testnet --source-account {address} --send=no -- is_graduate --learner {address}"
    );
    true
}

/// Turn registry error codes into advice.
fn explain_failure(stderr: &str) -> String {
    let codes: [(&str, &str); 6] = [
        ("Error(Contract, #1)", "This account already graduated. Use another one: `sorolearn graduate <identity>`."),
        ("Error(Contract, #10)", "Exam failed: the registry's first vote was rejected (or your contract trapped)."),
        ("Error(Contract, #11)", "Exam failed: the same address was allowed to vote twice."),
        ("Error(Contract, #12)", "Exam failed: a second vote must fail with PollError::AlreadyVoted (error #1)."),
        ("Error(Contract, #13)", "Exam failed: `votes` returned the wrong tally."),
        ("Error(Contract, #14)", "Exam failed: `has_voted` returned the wrong answer (or trapped)."),
    ];
    codes
        .iter()
        .find(|(code, _)| stderr.contains(code))
        .map(|(_, msg)| msg.to_string())
        .unwrap_or_else(|| "The graduation transaction failed. See the output above.".into())
}

/// `sorolearn build-wasm NAME`: compile an exercise to a deployable `.wasm`.
pub fn build_wasm_command(ex: &Exercise) -> bool {
    if !quiet(Command::new("stellar").arg("--version")) {
        eprintln!("{}", ui::red("The Stellar CLI is not installed. See exercises/12_testnet/README.md."));
        return false;
    }
    if !show(Command::new("rustup").args(["target", "add", "wasm32v1-none"])) {
        return false;
    }
    match build_wasm(ex) {
        Some(wasm) => {
            println!("\n{} {}", ui::green("✓ Built"), ui::cyan(&wasm.display().to_string()));
            true
        }
        None => false,
    }
}

/// Generate a cdylib crate for the exercise and build it with the Stellar CLI.
fn build_wasm(ex: &Exercise) -> Option<PathBuf> {
    let dir = Path::new(WASM_DIR).join(&ex.name);
    if let Err(e) = write_wasm_crate(ex, &dir) {
        eprintln!("{}", ui::red(&format!("Could not prepare {}: {e}", dir.display())));
        return None;
    }
    let out_dir = dir.join("out");
    let built = show(
        Command::new("stellar")
            .args(["contract", "build", "--manifest-path"])
            .arg(dir.join("Cargo.toml"))
            .arg("--out-dir")
            .arg(&out_dir)
            .env("CARGO_TARGET_DIR", TARGET_DIR),
    );
    built.then(|| out_dir.join(format!("{}.wasm", ex.name)))
}

/// A tiny cdylib crate whose lib is the learner's exercise file.
fn write_wasm_crate(ex: &Exercise, dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let lib = std::env::current_dir()?.join(ex.path());
    let manifest = format!(
        r#"# Generated by sorolearn graduate.
[package]
name = "{name}"
version = "0.0.0"
edition = "2021"
publish = false

[lib]
path = '{lib}'
crate-type = ["cdylib"]
doctest = false

[dependencies]
soroban-sdk = "={SOROBAN_SDK_VERSION}"

[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true

[workspace]
"#,
        name = ex.name,
        lib = lib.display()
    );
    fs::write(dir.join("Cargo.toml"), manifest)?;
    let lock = dir.join("Cargo.lock");
    if !lock.exists() && Path::new("exercises.lock").exists() {
        fs::copy("exercises.lock", lock)?;
    }
    Ok(())
}

fn step(n: u32, title: &str) {
    println!("{} {}", ui::cyan(&format!("[{n}/6]")), ui::bold(title));
}

fn print_cmd(cmd: &Command) {
    let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
    println!("  {} {}", ui::dim("$"), ui::dim(&format!("{} {}", cmd.get_program().to_string_lossy(), args.join(" "))));
}

/// Run, streaming output; report success.
fn show(cmd: &mut Command) -> bool {
    print_cmd(cmd);
    match cmd.status() {
        Ok(s) if s.success() => true,
        Ok(_) => false,
        Err(e) => {
            eprintln!("{}", ui::red(&format!("Failed to run {}: {e}", cmd.get_program().to_string_lossy())));
            false
        }
    }
}

/// Run silently; report success.
fn quiet(cmd: &mut Command) -> bool {
    cmd.output().is_ok_and(|o| o.status.success())
}

/// Run, printing stderr live, and return trimmed stdout's last line.
fn capture(cmd: &mut Command) -> Option<String> {
    print_cmd(cmd);
    let out = cmd.stderr(std::process::Stdio::inherit()).output().ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8_lossy(&out.stdout).lines().last().map(|l| l.trim().to_string())
}
