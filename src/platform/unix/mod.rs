mod shell;

use crate::error::PathmanError;
use crate::error::PathmanError::{
    UnableToCreateExportCommand, UnableToReadShellConfigFile, UnableToWriteShellConfigFile,
};
use crate::platform::unix::shell::CurrentShell;
use crate::platform::{PathUpdater, UpdateType};
use std::path::{Path, PathBuf};

/// Path Updater for macOS
pub struct UnixPathUpdater;

impl UnixPathUpdater {
    fn write_to_shell_config_file(
        config_file_path: PathBuf,
        export_line: &str,
        comment: Option<&str>,
    ) -> Result<UpdateType, PathmanError> {
        // Read the existing content of the shell configuration file
        let mut content = match std::fs::read_to_string(&config_file_path) {
            Ok(content) => content,
            Err(_) => {
                return Err(UnableToReadShellConfigFile(
                    config_file_path.to_string_lossy().to_string(),
                ));
            }
        };

        // Check if the export line is already present in the content
        if content.contains(export_line) {
            return Ok(UpdateType::AlreadyInPath);
        }

        let comment = match comment {
            Some(comment) => format!("\n# {comment}"),
            None => String::new(),
        };

        // Prepend the export line to the content
        content.push_str(&format!("{comment}\n{export_line}"));

        // Write the content back to the shell configuration file
        if std::fs::write(&config_file_path, content).is_err() {
            return Err(UnableToWriteShellConfigFile(
                config_file_path.to_string_lossy().to_string(),
            ));
        }

        Ok(UpdateType::Success)
    }
}

impl PathUpdater for UnixPathUpdater {
    fn prepend<P: AsRef<Path>>(path: P, comment: Option<&str>) -> Result<UpdateType, PathmanError> {
        // Retrieve the path to shell config file
        let shell = CurrentShell::detect()?;

        // Prepare the path export line
        let export_command = match shell.get_prepend_command(path) {
            Ok(line) => line,
            Err(_) => return Err(UnableToCreateExportCommand),
        };

        // Write the export line to the shell configuration file
        Self::write_to_shell_config_file(shell.config_file_path()?, &export_command, comment)
    }

    fn append<P: AsRef<Path>>(path: P, comment: Option<&str>) -> Result<UpdateType, PathmanError> {
        // Retrieve the path to shell config file
        let shell = CurrentShell::detect()?;

        // Prepare the path export line
        let export_command = match shell.get_append_command(path) {
            Ok(line) => line,
            Err(_) => return Err(UnableToCreateExportCommand),
        };

        // Write the export line to the shell configuration file
        Self::write_to_shell_config_file(shell.config_file_path()?, &export_command, comment)
    }
}
