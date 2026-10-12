//! Watch mode: re-run the current exercise whenever its file is saved.
//!
//! File changes are detected by polling the modification time. Keyboard
//! commands are read on a background thread: single keypresses in a terminal
//! (see `keys`), or whole lines when stdin is piped or on platforms without
//! single-key support.

use crate::info::Info;
use crate::keys::{KeyDecoder, KeyMode};
use crate::runner::{self, Source};
use crate::state::State;
use crate::ui;
use std::io::{BufRead, Read};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

enum Event {
    Command(String),
    StdinClosed,
}

pub fn watch(info: &Info, state: &mut State) -> bool {
    // Restores the terminal when watch mode ends (including on `q` / Ctrl-C).
    let key_mode = KeyMode::enable();
    let single_keys = key_mode.is_some();

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        if single_keys {
            let mut decoder = KeyDecoder::default();
            let mut byte = [0u8; 1];
            let mut stdin = std::io::stdin().lock();
            while let Ok(1) = stdin.read(&mut byte) {
                if let Some(cmd) = decoder.feed(byte[0]) {
                    if tx.send(Event::Command(cmd)).is_err() {
                        return;
                    }
                }
            }
        } else {
            for line in std::io::stdin().lock().lines() {
                let Ok(line) = line else { break };
                if tx.send(Event::Command(line.trim().to_lowercase())).is_err() {
                    return;
                }
            }
        }
        let _ = tx.send(Event::StdinClosed);
    });

    if state.done.is_empty() {
        ui::clear_screen();
        println!("{}", ui::banner(info.welcome_message.trim_end()));
        let start = if single_keys { "Press any key to start..." } else { "Press Enter to start..." };
        println!("{}", ui::dim(start));
        match rx.recv() {
            Ok(Event::Command(_)) => {}
            _ => return true,
        }
    }

    let mut passed = check(info, state, single_keys);
    let mut last_mtime = mtime(info, state);

    loop {
        match rx.recv_timeout(Duration::from_millis(300)) {
            Ok(Event::Command(cmd)) => match cmd.as_str() {
                "q" | "quit" => return true,
                "h" | "hint" => {
                    println!();
                    crate::print_hint(&info.exercises[state.current]);
                    prompt(passed, single_keys);
                }
                "l" | "list" => {
                    println!();
                    crate::list(info, state);
                    prompt(passed, single_keys);
                }
                "r" | "run" => passed = check(info, state, single_keys),
                "n" | "next" if passed => match state.next_pending(info, state.current) {
                    Some(next) => {
                        state.current = next;
                        state.save(info);
                        last_mtime = mtime(info, state);
                        passed = check(info, state, single_keys);
                    }
                    None => {
                        ui::clear_screen();
                        println!("{}", info.final_message.trim());
                        return true;
                    }
                },
                "n" | "next" => {
                    println!("{}", ui::yellow("Solve this exercise first (or press h for a hint)."));
                    prompt(passed, single_keys);
                }
                // Enter or space in single-key mode: nothing to do.
                "" if single_keys => {}
                "" => prompt(passed, single_keys),
                // A stray key shouldn't print anything; a mistyped line deserves feedback.
                _ if single_keys => {}
                other => {
                    println!("{}", ui::yellow(&format!("Unknown command `{other}`.")));
                    prompt(passed, single_keys);
                }
            },
            Ok(Event::StdinClosed) => return true,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let now = mtime(info, state);
                if now != last_mtime {
                    last_mtime = now;
                    passed = check(info, state, single_keys);
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
fn check(info: &Info, state: &mut State, single_keys: bool) -> bool {
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
    prompt(outcome.success, single_keys);
    outcome.success
}

fn prompt(passed: bool, single_keys: bool) {
    let next = if passed { format!("{}:next  ", ui::bold("n")) } else { String::new() };
    println!(
        "\n{next}{}:hint  {}:list  {}:rerun  {}:quit   {}",
        ui::bold("h"),
        ui::bold("l"),
        ui::bold("r"),
        ui::bold("q"),
        ui::dim(if single_keys { "(press a key)" } else { "(type a letter + Enter)" })
    );
}
