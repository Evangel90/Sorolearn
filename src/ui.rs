//! Minimal ANSI styling, so we do not need a terminal crate.

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}
pub fn green(s: &str) -> String {
    format!("\x1b[32m{s}\x1b[0m")
}
pub fn red(s: &str) -> String {
    format!("\x1b[31m{s}\x1b[0m")
}
pub fn yellow(s: &str) -> String {
    format!("\x1b[33m{s}\x1b[0m")
}
pub fn cyan(s: &str) -> String {
    format!("\x1b[36m{s}\x1b[0m")
}
pub fn dim(s: &str) -> String {
    format!("\x1b[2m{s}\x1b[0m")
}

/// Clear the screen *and* the scrollback, so output from a previous exercise
/// can't be scrolled back to and mistaken for the current one.
pub fn clear_screen() {
    print!("\x1b[H\x1b[2J\x1b[3J");
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

pub fn progress_bar(done: usize, total: usize) -> String {
    const WIDTH: usize = 40;
    let filled = (done * WIDTH).checked_div(total).unwrap_or(0);
    format!(
        "Progress: [{}{}] {done}/{total}",
        green(&"#".repeat(filled)),
        "-".repeat(WIDTH - filled)
    )
}
