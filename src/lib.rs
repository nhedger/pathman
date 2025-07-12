use crate::platform::PathUpdater;
use crate::platform::PlatformPathUpdater;
use std::path::Path;

mod error;
mod platform;

pub use error::PathmanError;
pub use platform::UpdateType;

/// Prepends the given path to the PATH environment variable
///
/// This function provides a cross-platform interface for prepending a given
/// path to the user's PATH environment variable.
///
/// It is generally necessary for the end user to restart their shell or to
/// source their shell configuration file for the changes to take effect.
///
/// # OS-specific behavior
///
/// This function behaves differently depending on the operating system,
/// because the method of modifying the PATH environment variable varies
/// across platforms.
///
/// # macOS and Linux
///
/// On macOS and Linux, PATH is modified by adding a command to the user's
/// shell configuration file (e.g., `.bashrc`, `.zshrc`, etc.).
///
/// # Windows
///
/// On Windows, the user's PATH is modified by updating the registry.
pub fn prepend_to_path<P: AsRef<Path>>(
    path: P,
    comment: Option<&str>,
) -> Result<UpdateType, PathmanError> {
    PlatformPathUpdater::prepend(path, comment)
}

/// Appends the given path to the PATH environment variable
///
/// This function provides a cross-platform interface for appending a given
/// path to the user's PATH environment variable.
///
/// It is generally necessary for the end user to restart their shell or to
/// source their shell configuration file for the changes to take effect.
///
/// # OS-specific behavior
///
/// This function behaves differently depending on the operating system,
/// because the method of modifying the PATH environment variable varies
/// across platforms.
///
/// # macOS and Linux
///
/// On macOS and Linux, PATH is modified by adding a command to the user's
/// shell configuration file (e.g., `.bashrc`, `.zshrc`, etc.).
///
/// # Windows
///
/// On Windows, the user's PATH is modified by updating the registry.
pub fn append_to_path<P: AsRef<Path>>(
    path: P,
    comment: Option<&str>,
) -> Result<UpdateType, PathmanError> {
    PlatformPathUpdater::append(path, comment)
}
