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

/// Paint the ASCII-art banner (the lines before the first blank line) in a
/// purple-to-cyan gradient, matching the website. The rest prints as-is.
pub fn banner(text: &str) -> String {
    const STOPS: [(f32, f32, f32); 3] = [(139.0, 123.0, 255.0), (94.0, 106.0, 210.0), (56.0, 189.0, 248.0)];
    let art_lines = text.lines().take_while(|l| !l.trim().is_empty()).count();
    let mut out = String::new();
    for (i, line) in text.lines().enumerate() {
        if i < art_lines {
            let t = if art_lines > 1 { i as f32 / (art_lines - 1) as f32 } else { 0.0 };
            let (a, b, local) = if t < 0.5 { (STOPS[0], STOPS[1], t * 2.0) } else { (STOPS[1], STOPS[2], (t - 0.5) * 2.0) };
            let mix = |x: f32, y: f32| (x + (y - x) * local).round() as u8;
            let (r, g, bl) = (mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2));
            out.push_str(&format!("\x1b[1;38;2;{r};{g};{bl}m{line}\x1b[0m\n"));
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
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
