use crate::info::Info;
use std::collections::BTreeSet;
use std::fs;

const STATE_FILE: &str = ".sorolearn/state.txt";

/// Learner progress, persisted as a tiny line-based text file.
pub struct State {
    pub current: usize,
    pub done: BTreeSet<String>,
}

impl State {
    pub fn load(info: &Info) -> Self {
        let mut state = State { current: 0, done: BTreeSet::new() };
        let Ok(raw) = fs::read_to_string(STATE_FILE) else {
            return state;
        };
        for line in raw.lines() {
            if let Some(name) = line.strip_prefix("current=") {
                if let Some(i) = info.find(name) {
                    state.current = i;
                }
            } else if let Some(name) = line.strip_prefix("done=") {
                if info.find(name).is_some() {
                    state.done.insert(name.to_string());
                }
            }
        }
        state
    }

    pub fn save(&self, info: &Info) {
        let mut out = format!("current={}\n", info.exercises[self.current].name);
        for name in &self.done {
            out.push_str(&format!("done={name}\n"));
        }
        let _ = fs::create_dir_all(".sorolearn");
        let _ = fs::write(STATE_FILE, out);
    }

    pub fn is_done(&self, name: &str) -> bool {
        self.done.contains(name)
    }

    /// The first exercise after `from` (wrapping around) that is not done yet.
    pub fn next_pending(&self, info: &Info, from: usize) -> Option<usize> {
        let n = info.exercises.len();
        (1..=n)
            .map(|k| (from + k) % n)
            .find(|&i| !self.is_done(&info.exercises[i].name))
    }
}
