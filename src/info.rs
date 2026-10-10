use serde::Deserialize;
use std::path::{Path, PathBuf};

/// The SDK version every exercise crate is compiled against.
pub const SOROBAN_SDK_VERSION: &str = "29.0.0";

#[derive(Debug, Deserialize)]
pub struct Info {
    pub welcome_message: String,
    pub final_message: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Exercise {
    /// Unique name, also used as the Cargo package name.
    pub name: String,
    /// Topic directory inside `exercises/` and `solutions/`.
    pub dir: String,
    pub hint: String,
    /// Set for exercises that already pass before the learner touches them.
    #[serde(default)]
    pub skip_check_unsolved: bool,
}

impl Exercise {
    pub fn path(&self) -> PathBuf {
        Path::new("exercises").join(&self.dir).join(format!("{}.rs", self.name))
    }

    pub fn solution_path(&self) -> PathBuf {
        Path::new("solutions").join(&self.dir).join(format!("{}.rs", self.name))
    }

    pub fn readme_path(&self) -> PathBuf {
        Path::new("exercises").join(&self.dir).join("README.md")
    }
}

impl Info {
    pub fn load() -> Result<Self, String> {
        let raw = std::fs::read_to_string("info.toml").map_err(|_| {
            "Could not find `info.toml`. Run sorolearn from the root of the sorolearn repository."
                .to_string()
        })?;
        let info: Info = toml::from_str(&raw).map_err(|e| format!("Invalid info.toml: {e}"))?;
        if info.exercises.is_empty() {
            return Err("info.toml contains no exercises".into());
        }
        Ok(info)
    }

    pub fn find(&self, name: &str) -> Option<usize> {
        self.exercises.iter().position(|e| e.name == name)
    }
}
