use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PathmanError {

    #[error("Unable to find the home directory")]
    UnableToFindHomeDirectory,

    #[error("Unable to detect the current shell")]
    UnableToDetectShell,

    #[error("Unable to convert the path to a string")]
    UnableToConvertPathToString,

    #[error("Unsupported shell: {0}")]
    UnsupportedShell(String),

    #[error("Unable to update the PATH environment variable")]
    UnableToUpdatePath,

    #[error("Shell configuration file not found")]
    UnableToFindShellConfigFile,

    #[error("Unable to read the shell configuration file: {0}")]
    UnableToReadShellConfigFile(String),

    #[error("Unable to write to the shell configuration file: {0}")]
    UnableToWriteShellConfigFile(String),

    #[error("Unable to create export line for the shell configuration file")]
    UnableToCreateExportCommand,

    #[error("Unable to read the environment registry subkey")]
    UnableToOpenEnvironmentKey,

    #[error("Unable to read the PATH environment variable from the registry")]
    UnableToReadPathEnvironmentKey,
    
    #[error("Unable to write the PATH environment variable to the registry")]
    UnableToWritePathEnvironmentKey,
}