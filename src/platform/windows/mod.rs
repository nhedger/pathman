use crate::PathmanError::{UnableToOpenEnvironmentKey, UnableToWritePathEnvironmentKey};
use crate::UpdateType;
use crate::error::PathmanError;
use crate::platform::PathUpdater;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};

/// Path Updater for Windows
pub struct WindowsPathUpdater;

impl WindowsPathUpdater {
    /// Retrieves the user's current PATH environment variable from the Windows registry.
    fn get_path() -> Result<String, PathmanError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let env_key = match hkcu.open_subkey("Environment") {
            Ok(env_key) => env_key,
            Err(_) => return Err(UnableToOpenEnvironmentKey),
        };

        let path = env_key.get_value("Path").unwrap_or_else(|_| String::new());

        Ok(path)
    }

    /// Set the PATH value in the registry
    fn set_path(new_path: &str) -> Result<(), PathmanError> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let env_key = match hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE) {
            Ok(env_key) => env_key,
            Err(_) => return Err(UnableToOpenEnvironmentKey),
        };

        match env_key.set_value("Path", &new_path) {
            Ok(_) => {}
            Err(_) => return Err(UnableToWritePathEnvironmentKey),
        }

        Ok(())
    }

    /// Determines if the given path exists in the current PATH environment variable.
    ///
    /// When comparing the paths, this function will ignore trailing slashes so that
    /// if this is the only different we don't add the path twice.
    fn path_exists_in_path(path: &str, path_env: &str) -> bool {
        let normalized_path = path.trim_end_matches(&['/', '\\'][..]);

        path_env.split(';').any(|segment| {
            let normalized_segment = segment.trim_end_matches(&['/', '\\'][..]);

            normalized_path == normalized_segment
        })
    }
}

impl PathUpdater for WindowsPathUpdater {
    fn prepend<P: AsRef<Path>>(path: P, _: Option<&str>) -> Result<UpdateType, PathmanError> {
        let path = path.as_ref().to_string_lossy().to_string();
        let current_path = Self::get_path()?;

        // Check if the path already exists in PATH
        if Self::path_exists_in_path(&path, &current_path) {
            return Ok(UpdateType::AlreadyInPath);
        }

        // Prepend the new path to the beginning of PATH
        let new_path = if current_path.is_empty() {
            path
        } else {
            format!("{path};{current_path}")
        };

        Self::set_path(&new_path)?;
        Ok(UpdateType::Success)
    }

    fn append<P: AsRef<Path>>(path: P, _: Option<&str>) -> Result<UpdateType, PathmanError> {
        let path = path.as_ref().to_string_lossy().to_string();
        let current_path = Self::get_path()?;

        // Check if the path already exists in PATH
        if Self::path_exists_in_path(&path, &current_path) {
            return Ok(UpdateType::AlreadyInPath);
        }

        // Append the new path to the end of PATH
        let new_path = if current_path.is_empty() {
            path
        } else {
            format!("{current_path};{path}")
        };

        Self::set_path(&new_path)?;
        Ok(UpdateType::Success)
    }
}
