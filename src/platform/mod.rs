use crate::error::PathmanError;
use std::path::Path;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
pub(crate) mod windows;

#[cfg(unix)]
pub use unix::UnixPathUpdater as PlatformPathUpdater;

#[cfg(windows)]
pub use windows::WindowsPathUpdater as PlatformPathUpdater;

pub trait PathUpdater {
    /// Prepends the given path to the PATH environment variable.
    fn prepend<P: AsRef<Path>>(path: P, comment: Option<&str>) -> Result<UpdateType, PathmanError>;

    /// Appends the given path to the PATH environment variable.
    fn append<P: AsRef<Path>>(path: P, comment: Option<&str>) -> Result<UpdateType, PathmanError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum UpdateType {
    /// Indicates that the path was successfully added to the PATH environment variable.
    Success,

    /// Indicates that the path was already present in the PATH environment variable.
    AlreadyInPath,
}
