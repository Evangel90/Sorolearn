mod info;
mod runner;
mod state;
mod ui;
mod watch;

use info::Info;
use runner::Source;
use state::State;
use std::process::ExitCode;

const USAGE: &str = "\
sorolearn - learn Soroban smart contracts by fixing small exercises

USAGE:
    sorolearn                 Start watch mode (recommended)
    sorolearn list            List all exercises and your progress
    sorolearn run [NAME]      Compile and test one exercise (default: current)
    sorolearn hint [NAME]     Show the hint for an exercise (default: current)
    sorolearn reset NAME      Restore an exercise to its original state
    sorolearn verify          Check every exercise, in order
    sorolearn setup           Download + pre-build everything (runs automatically
                              the first time; afterwards sorolearn works offline)
    sorolearn help            Show this message

DEVELOPER:
    sorolearn dev check       Verify all solutions pass and all exercises fail
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let info = match Info::load() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("{}", ui::red(&e));
            return ExitCode::FAILURE;
        }
    };
    if let Err(e) = runner::ensure_workspace(&info, Source::Exercises)
        .and_then(|_| runner::ensure_originals(&info))
    {
        eprintln!("{}", ui::red(&format!("Failed to prepare .sorolearn/: {e}")));
        return ExitCode::FAILURE;
    }
    let mut state = State::load(&info);

    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let ok = match args.as_slice() {
        [] | ["watch"] => watch::watch(&info, &mut state),
        ["list"] => {
            list(&info, &state);
            true
        }
        ["run"] => {
            let current = state.current;
            run_one(&info, &mut state, current)
        }
        ["run", name] => with_exercise(&info, name, |i| run_one(&info, &mut state, i)),
        ["hint"] => {
            print_hint(&info.exercises[state.current]);
            true
        }
        ["hint", name] => with_exercise(&info, name, |i| {
            print_hint(&info.exercises[i]);
            true
        }),
        ["reset", name] => with_exercise(&info, name, |i| reset(&info, &mut state, i)),
        ["verify"] => verify(&info, &mut state),
        ["setup"] => runner::setup(),
        ["dev", "check"] => dev_check(&info),
        ["help"] | ["--help"] | ["-h"] => {
            print!("{USAGE}");
            true
        }
        _ => {
            eprint!("{}\n\n{USAGE}", ui::red("Unknown command."));
            false
        }
    };
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn with_exercise(info: &Info, name: &str, f: impl FnOnce(usize) -> bool) -> bool {
    match info.find(name) {
        Some(i) => f(i),
        None => {
            eprintln!("{}", ui::red(&format!("No exercise named `{name}`. Try `sorolearn list`.")));
            false
        }
    }
}

pub fn list(info: &Info, state: &State) {
    println!("{}", ui::bold(&format!("  {:<4} {:<22} {:<20} {}", "#", "NAME", "TOPIC", "STATUS")));
    for (i, ex) in info.exercises.iter().enumerate() {
        let marker = if i == state.current { ui::cyan(">") } else { " ".into() };
        let status = if state.is_done(&ex.name) { ui::green("done") } else { ui::yellow("pending") };
        println!("{marker} {:<4} {:<22} {:<20} {status}", i + 1, ex.name, ex.dir);
    }
    println!("\n{}", ui::progress_bar(state.done.len(), info.exercises.len()));
}

pub fn print_hint(ex: &info::Exercise) {
    println!("{} {}\n", ui::bold("Hint for"), ui::cyan(&ex.name));
    println!("{}", ex.hint.trim());
}

fn run_one(info: &Info, state: &mut State, i: usize) -> bool {
    let ex = &info.exercises[i];
    println!("Compiling and testing {} ...", ui::cyan(&ex.path().display().to_string()));
    let outcome = runner::run(ex, Source::Exercises);
    if outcome.success {
        println!("{}", ui::green(&format!("✓ {} passed!", ex.name)));
        state.done.insert(ex.name.clone());
    } else {
        println!("{}", outcome.output);
        println!("{}", ui::red(&format!("✗ {} is not solved yet.", ex.name)));
        println!("{}", ui::dim(&format!("Stuck? Run `sorolearn hint {}`.", ex.name)));
        state.done.remove(&ex.name);
    }
    state.save(info);
    outcome.success
}

fn reset(info: &Info, state: &mut State, i: usize) -> bool {
    let ex = &info.exercises[i];
    // Read + write rather than `fs::copy`, which can preserve the old mtime and
    // make cargo (and watch mode) think the file did not change.
    match std::fs::read(runner::original_path(ex)).and_then(|orig| std::fs::write(ex.path(), orig)) {
        Ok(()) => {
            state.done.remove(&ex.name);
            state.save(info);
            println!("{}", ui::green(&format!("Reset {} to its original state.", ex.path().display())));
            true
        }
        Err(e) => {
            eprintln!("{}", ui::red(&format!("Could not reset {}: {e}", ex.name)));
            false
        }
    }
}

fn verify(info: &Info, state: &mut State) -> bool {
    for (i, ex) in info.exercises.iter().enumerate() {
        print!("[{}/{}] {:<22} ", i + 1, info.exercises.len(), ex.name);
        let _ = std::io::Write::flush(&mut std::io::stdout());
        let outcome = runner::run(ex, Source::Exercises);
        if outcome.success {
            println!("{}", ui::green("ok"));
            state.done.insert(ex.name.clone());
        } else {
            println!("{}\n\n{}", ui::red("failed"), outcome.output);
            state.done.remove(&ex.name);
            state.current = i;
            state.save(info);
            println!("Continue with {}", ui::cyan(&ex.path().display().to_string()));
            return false;
        }
    }
    state.save(info);
    println!("\n{}", info.final_message.trim());
    true
}

/// Maintainer check: every solution must pass and every exercise must fail.
fn dev_check(info: &Info) -> bool {
    if let Err(e) = runner::ensure_workspace(info, Source::Solutions) {
        eprintln!("{}", ui::red(&format!("Failed to generate solutions workspace: {e}")));
        return false;
    }
    let mut ok = true;
    for ex in &info.exercises {
        for (label, path) in [("exercise", ex.path()), ("solution", ex.solution_path()), ("readme", ex.readme_path())] {
            if !path.exists() {
                println!("{} missing {label}: {}", ui::red("✗"), path.display());
                ok = false;
            }
        }
        let solution = runner::run(ex, Source::Solutions);
        if !solution.success {
            println!("{} solution for {} fails:\n{}", ui::red("✗"), ex.name, solution.output);
            ok = false;
            continue;
        }
        if !ex.skip_check_unsolved && runner::run(ex, Source::Exercises).success {
            println!("{} exercise {} already passes without changes", ui::red("✗"), ex.name);
            ok = false;
            continue;
        }
        println!("{} {}", ui::green("✓"), ex.name);
    }
    ok
}
