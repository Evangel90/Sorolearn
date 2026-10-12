//! Single-keypress input for watch mode.
//!
//! On Unix terminals we switch stdin to "cbreak" mode: no line buffering and
//! no echo, so `n` or `h` act the moment they are pressed. Output processing
//! is left alone, so everything else prints normally. Ctrl-C arrives as a key
//! (byte 3) and is treated as quit, which lets the terminal be restored.
//!
//! Elsewhere (Windows, or stdin that is not a terminal, e.g. piped input) the
//! caller falls back to reading whole lines.

use std::io::IsTerminal;

/// Restores the original terminal settings when dropped.
pub struct KeyMode {
    #[cfg(unix)]
    original: libc::termios,
}

impl KeyMode {
    /// Enable single-keypress input, or `None` if it isn't available here.
    pub fn enable() -> Option<Self> {
        if !std::io::stdin().is_terminal() {
            return None;
        }
        Self::enable_unix()
    }

    #[cfg(unix)]
    fn enable_unix() -> Option<Self> {
        // SAFETY: tcgetattr/tcsetattr on fd 0 with a properly sized termios.
        unsafe {
            let mut original = std::mem::MaybeUninit::<libc::termios>::uninit();
            if libc::tcgetattr(libc::STDIN_FILENO, original.as_mut_ptr()) != 0 {
                return None;
            }
            let original = original.assume_init();
            let mut keys = original;
            keys.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG);
            keys.c_cc[libc::VMIN] = 1;
            keys.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &keys) != 0 {
                return None;
            }
            Some(KeyMode { original })
        }
    }

    #[cfg(not(unix))]
    fn enable_unix() -> Option<Self> {
        None
    }
}

#[cfg(unix)]
impl Drop for KeyMode {
    fn drop(&mut self) {
        // SAFETY: restores the settings captured in `enable_unix`.
        unsafe {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &self.original);
        }
    }
}

/// Turns raw input bytes into single-letter commands, skipping escape
/// sequences (arrow keys, Home, ...) so they don't trigger commands.
#[derive(Default)]
pub struct KeyDecoder {
    state: Escape,
}

#[derive(Default, PartialEq)]
enum Escape {
    #[default]
    None,
    /// Just saw ESC.
    Start,
    /// Inside `ESC [ ...` or `ESC O ...`, waiting for the final byte.
    Sequence,
}

impl KeyDecoder {
    /// The command for this byte, if it completes one.
    pub fn feed(&mut self, byte: u8) -> Option<String> {
        match self.state {
            Escape::Start => {
                self.state = if byte == b'[' || byte == b'O' { Escape::Sequence } else { Escape::None };
                return None;
            }
            Escape::Sequence => {
                if (0x40..=0x7e).contains(&byte) {
                    self.state = Escape::None;
                }
                return None;
            }
            Escape::None => {}
        }
        match byte {
            0x1b => {
                self.state = Escape::Start;
                None
            }
            // Ctrl-C and Ctrl-D quit.
            3 | 4 => Some("q".into()),
            b if b.is_ascii_alphabetic() => Some((b.to_ascii_lowercase() as char).to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(bytes: &[u8]) -> Vec<String> {
        let mut decoder = KeyDecoder::default();
        bytes.iter().filter_map(|b| decoder.feed(*b)).collect()
    }

    #[test]
    fn letters_become_commands() {
        assert_eq!(decode(b"nH"), ["n", "h"]);
    }

    #[test]
    fn escape_sequences_are_ignored() {
        // Up arrow, Home (ESC [ H, which must not trigger the hint), F1 (ESC O P).
        assert_eq!(decode(b"\x1b[A\x1b[H\x1bOPn"), ["n"]);
    }

    #[test]
    fn ctrl_c_quits_and_enter_is_ignored() {
        assert_eq!(decode(b"\r\n\x03"), ["q"]);
    }
}
