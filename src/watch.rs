//! Watch mode: re-run the current exercise whenever its file is saved.
//!
//! File changes are detected by polling the modification time, and keyboard
//! commands are read line-by-line from stdin on a background thread. This keeps
//! the tool dependency-free and portable.

use crate::info::Info;
use crate::runner::{self, Source};
use crate::state::State;
use crate::ui;
use std::io::BufRead;
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

enum Event {
    Command(String),
    StdinClosed,
}

pub fn watch(info: &Info, state: &mut State) -> bool {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            if tx.send(Event::Command(line.trim().to_lowercase())).is_err() {
                return;
            }
        }
        let _ = tx.send(Event::StdinClosed);
    });

    if state.done.is_empty() {
        ui::clear_screen();
        println!("{}\n", info.welcome_message.trim());
        println!("{}", ui::dim("Press Enter to start..."));
        match rx.recv() {
            Ok(Event::Command(_)) => {}
            _ => return true,
        }
    }

    let mut passed = check(info, state);
    let mut last_mtime = mtime(info, state);

    loop {
        match rx.recv_timeout(Duration::from_millis(300)) {
            Ok(Event::Command(cmd)) => match cmd.as_str() {
                "q" | "quit" => return true,
                "h" | "hint" => {
                    println!();
                    crate::print_hint(&info.exercises[state.current]);
                    prompt(passed);
                }
                "l" | "list" => {
                    println!();
                    crate::list(info, state);
                    prompt(passed);
                }
                "r" | "run" => passed = check(info, state),
                "n" | "next" if passed => match state.next_pending(info, state.current) {
                    Some(next) => {
                        state.current = next;
                        state.save(info);
                        last_mtime = mtime(info, state);
                        passed = check(info, state);
                    }
                    None => {
                        ui::clear_screen();
                        println!("{}", info.final_message.trim());
                        return true;
                    }
                },
                "n" | "next" => {
                    println!("{}", ui::yellow("Solve this exercise first (or press h for a hint)."));
                    prompt(passed);
                }
                "" => prompt(passed),
                other => {
                    println!("{}", ui::yellow(&format!("Unknown command `{other}`.")));
                    prompt(passed);
                }
            },
            Ok(Event::StdinClosed) => return true,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let now = mtime(info, state);
                if now != last_mtime {
                    last_mtime = now;
                    passed = check(info, state);
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return true,
        }
    }
}

fn mtime(info: &Info, state: &State) -> Option<SystemTime> {
    std::fs::metadata(info.exercises[state.current].path())
        .and_then(|m| m.modified())
        .ok()
}

/// Run the current exercise, render the result and return whether it passed.
fn check(info: &Info, state: &mut State) -> bool {
    let ex = &info.exercises[state.current];
    ui::clear_screen();
    println!("{} {}", ui::bold("Checking"), ui::cyan(&ex.path().display().to_string()));
    println!("{}", ui::dim("(compiling... the first build of the Soroban SDK can take a minute)"));
    let outcome = runner::run(ex, Source::Exercises);

    ui::clear_screen();
    if outcome.success {
        state.done.insert(ex.name.clone());
        println!("{}", ui::green(&format!("✓ Exercise {} passed!", ex.name)));
        println!();
        println!("Read through the code once more: the comments explain why it works.");
        println!("{}", ui::dim(&format!("Solution for comparison: {}", ex.solution_path().display())));
    } else {
        state.done.remove(&ex.name);
        println!("{}", outcome.output.trim_end());
        println!();
        println!("{}", ui::red(&format!("✗ Exercise {} is not solved yet.", ex.name)));
    }
    state.save(info);
    println!();
    println!("{}", ui::progress_bar(state.done.len(), info.exercises.len()));
    println!("Current exercise: {}", ui::cyan(&ex.path().display().to_string()));
    println!("Topic notes:      {}", ex.readme_path().display());
    prompt(outcome.success);
    outcome.success
}

fn prompt(passed: bool) {
    let next = if passed { format!("{}:next  ", ui::bold("n")) } else { String::new() };
    println!(
        "\n{next}{}:hint  {}:list  {}:rerun  {}:quit   {}",
        ui::bold("h"),
        ui::bold("l"),
        ui::bold("r"),
        ui::bold("q"),
        ui::dim("(type a letter + Enter)")
    );
}
