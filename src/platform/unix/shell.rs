use crate::error::PathmanError;
use crate::error::PathmanError::{
    UnableToConvertPathToString, UnableToDetectShell, UnableToFindHomeDirectory,
    UnableToFindShellConfigFile, UnsupportedShell,
};
use home::home_dir;
use std::env::var_os;
use std::path::{Path, PathBuf};

pub struct CurrentShell {
    pub shell: Shell,
    pub home: PathBuf,
}

/// The supported shells
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl CurrentShell {
    /// Detects the current shell
    pub fn detect() -> Result<Self, PathmanError> {
        // Retrieve the home directory
        let home = match home_dir() {
            Some(path) => path,
            None => return Err(UnableToFindHomeDirectory),
        };

        // Determine the shell from the SHELL environment variable
        let shell = match var_os("SHELL") {
            Some(shell) => shell.to_string_lossy().to_string(),
            None => return Err(UnableToDetectShell),
        };

        // Match the shell string to supported shell kinds
        match shell {
            s if s.contains("bash") => Ok(CurrentShell {
                shell: Shell::Bash,
                home,
            }),
            s if s.contains("zsh") => Ok(CurrentShell {
                shell: Shell::Zsh,
                home,
            }),
            s if s.contains("fish") => Ok(CurrentShell {
                shell: Shell::Fish,
                home,
            }),
            _ => Err(UnsupportedShell(shell)),
        }
    }

    /// Returns the first shell configuration file that exists
    pub fn config_file_path(&self) -> Result<PathBuf, PathmanError> {
        let files = match self.shell {
            Shell::Bash => vec![
                self.home.join(".bashrc"),
                self.home.join(".bash_profile"),
                self.home.join(".profile"),
            ],
            Shell::Zsh => vec![self.home.join(".zshrc")],
            Shell::Fish => vec![self.home.join(".config/fish/config.fish")],
        };

        match files.into_iter().find(|f| f.exists()) {
            Some(file) => Ok(file),
            None => Err(UnableToFindShellConfigFile),
        }
    }

    /// Builds the shell command for prepending to the PATH environment variable
    pub fn get_prepend_command<P: AsRef<Path>>(&self, path: P) -> Result<String, PathmanError> {
        let path = match path.as_ref().to_str() {
            Some(p) => p,
            None => return Err(UnableToConvertPathToString),
        };

        let command = match self.shell {
            Shell::Bash | Shell::Zsh => {
                format!("export PATH=\"{path}:$PATH\"")
            }
            Shell::Fish => {
                format!("set -gx PATH \"{path}\" $PATH")
            }
        };

        Ok(command)
    }

    /// Builds the shell command for appending to the PATH environment variable
    pub fn get_append_command<P: AsRef<Path>>(&self, path: P) -> Result<String, PathmanError> {
        let path = match path.as_ref().to_str() {
            Some(p) => p,
            None => return Err(UnableToConvertPathToString),
        };

        let command = match self.shell {
            Shell::Bash | Shell::Zsh => {
                format!("export PATH=\"$PATH:{path}\"")
            }
            Shell::Fish => {
                format!("set -gx PATH $PATH \"{path}\"")
            }
        };

        Ok(command)
    }
}
