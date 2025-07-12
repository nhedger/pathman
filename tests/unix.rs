#![cfg(unix)]

use assert_fs::prelude::*;
use pathman::PathmanError::{
    UnableToFindShellConfigFile, UnableToReadShellConfigFile, UnableToWriteShellConfigFile,
};
use pathman::{UpdateType, append_to_path, prepend_to_path};
use predicates::prelude::*;
use std::fs::{Permissions, create_dir_all, set_permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

// --- Bash ---

#[test]
fn it_prepends_a_path_to_the_path_in_bashrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("export PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_appends_a_path_to_the_path_in_bashrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("export PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_in_bashrc_if_bashrc_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_in_bashrc_if_bashrc_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepends_a_path_to_the_path_if_the_export_command_is_already_present_in_bashrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory with an existing export command
    let bashrc = home.child(".bashrc");
    bashrc.write_str("export PATH=\"/test:$PATH\"\n").unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("export PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_the_export_command_is_already_present_in_bashrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory with an existing export command
    let bashrc = home.child(".bashrc");
    bashrc.write_str("export PATH=\"$PATH:/test\"\n").unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("export PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_bashrc_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory but make it unreadable
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&bashrc, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    bashrc.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_bashrc_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory but make it unreadable
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&bashrc, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    bashrc.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_bashrc_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory but make it unwritable
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();
    // Set permissions to unwritable
    set_permissions(&bashrc, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    bashrc.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_bashrc_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory but make it unwritable
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();
    // Set permissions to unwritable
    set_permissions(&bashrc, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    bashrc.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_export_command_in_bashrc_when_prepending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the prepend function with a test path and a comment
            match prepend_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nexport PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .bashrc with comment");
                }
            }
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_export_command_in_bashrc_when_appending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".bashrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/bash".to_string())),
        ],
        || {
            // Call the prepend function with a test path and a comment
            match append_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nexport PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .bashrc with comment");
                }
            }
        },
    );
}

// --- Zsh ---

#[test]
fn it_prepends_a_path_to_the_path_in_zshrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".zshrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("export PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_appends_a_path_to_the_path_in_zshrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .bashrc file in the virtual home directory
    let bashrc = home.child(".zshrc");
    bashrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("export PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(bashrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to .bashrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_in_zshrc_if_zshrc_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_in_zshrc_if_zshrc_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_the_export_command_is_already_present_in_zshrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory with an existing export command
    let zshrc = home.child(".zshrc");
    zshrc.write_str("export PATH=\"/test:$PATH\"\n").unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("export PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(zshrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .zshrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_the_export_command_is_already_present_in_zshrc() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory with an existing export command
    let zshrc = home.child(".zshrc");
    zshrc.write_str("export PATH=\"$PATH:/test\"\n").unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("export PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(zshrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to .zshrc");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_zshrc_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory but make it unreadable
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&zshrc, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    zshrc.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_zshrc_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory but make it unreadable
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&zshrc, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    zshrc.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_zshrc_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory but make it unwritable
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();
    // Set permissions to unwritable
    set_permissions(&zshrc, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    zshrc.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_zshrc_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory but make it unwritable
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();
    // Set permissions to unwritable
    set_permissions(&zshrc, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    zshrc.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_export_command_in_zshrc_when_prepending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the prepend function with a test path and a comment
            match prepend_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nexport PATH=\"/test:$PATH\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(zshrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .zshrc with comment");
                }
            }
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_export_command_in_zshrc_when_appending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .zshrc file in the virtual home directory
    let zshrc = home.child(".zshrc");
    zshrc.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/zsh".to_string())),
        ],
        || {
            // Call the prepend function with a test path and a comment
            match append_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nexport PATH=\"$PATH:/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(zshrc.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to .zshrc with comment");
                }
            }
        },
    );
}

// --- Fish ---

#[test]
fn it_prepends_a_path_to_the_path_in_fish_config() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("set -gx PATH \"/test\" $PATH")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to config.fish");
                }
            }
        },
    );
}

#[test]
fn it_appends_a_path_to_the_path_in_fish_config() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("set -gx PATH $PATH \"/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to config.fish");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_in_fish_config_if_fish_config_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_in_fish_config_if_fish_config_does_not_exist() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToFindShellConfigFile),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_the_set_command_is_already_present_in_fish_config() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory with an existing set command
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config
        .write_str("set -gx PATH \"/test\" $PATH\n")
        .unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the prepend function with a test path
            match prepend_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("set -gx PATH \"/test\" $PATH")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to config.fish");
                }
            }
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_the_set_command_is_already_present_in_fish_config() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory with an existing set command
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config
        .write_str("set -gx PATH $PATH \"/test\"\n")
        .unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the append function with a test path
            match append_to_path(PathBuf::from("/test"), None) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::AlreadyInPath);
                    assert!(
                        predicate::str::contains("set -gx PATH $PATH \"/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to append path to config.fish");
                }
            }
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_fish_config_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&fish_config, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    fish_config.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_fish_config_cannot_be_read_from() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&fish_config, Permissions::from_mode(0o000)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToReadShellConfigFile(
                    fish_config.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_fish_config_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&fish_config, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    fish_config.path().to_string_lossy().to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_fish_config_is_not_writable() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();
    // Set permissions to unreadable
    set_permissions(&fish_config, Permissions::from_mode(0o444)).unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            assert_eq!(
                Err(UnableToWriteShellConfigFile(
                    fish_config.path().to_string_lossy().to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_set_command_in_fish_config_when_prepending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the prepend function with a test path and a comment
            match prepend_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nset -gx PATH \"/test\" $PATH")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to config.fish with comment");
                }
            }
        },
    );
}

#[test]
fn it_adds_a_comment_before_the_set_command_in_fish_config_when_appending() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    // Create .config/fish/config.fish file in the virtual home directory
    let fish_config = home.child(".config/fish/config.fish");
    create_dir_all(fish_config.parent().unwrap()).unwrap();
    fish_config.touch().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/fish".to_string())),
        ],
        || {
            // Call the append function with a test path and a comment
            match append_to_path(PathBuf::from("/test"), Some("Test comment")) {
                Ok(result) => {
                    assert_eq!(result, UpdateType::Success);
                    assert!(
                        predicate::str::contains("# Test comment\nset -gx PATH $PATH \"/test\"")
                            .from_utf8()
                            .from_file_path()
                            .eval(fish_config.path())
                    );
                }
                Err(e) => {
                    println!("Error {e}");
                    panic!("Failed to prepend path to config.fish with comment");
                }
            }
        },
    );
}

// --- Non-detected Shell ---

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_shell_is_not_detected() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", None),
        ],
        || {
            assert_eq!(
                Err(pathman::PathmanError::UnableToDetectShell),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_shell_is_not_detected() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", None),
        ],
        || {
            assert_eq!(
                Err(pathman::PathmanError::UnableToDetectShell),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

// --- Unsupported Shell ---

#[test]
fn it_does_not_prepend_a_path_to_the_path_if_shell_is_not_supported() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/unsupported_shell".to_string())),
        ],
        || {
            assert_eq!(
                Err(pathman::PathmanError::UnsupportedShell(
                    "/bin/unsupported_shell".to_string()
                )),
                prepend_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}

#[test]
fn it_does_not_append_a_path_to_the_path_if_shell_is_not_supported() {
    // Create the virtual home directory
    let home = assert_fs::TempDir::new().unwrap();

    temp_env::with_vars(
        [
            ("HOME", Some(home.path().to_string_lossy().to_string())),
            ("SHELL", Some("/bin/unsupported_shell".to_string())),
        ],
        || {
            assert_eq!(
                Err(pathman::PathmanError::UnsupportedShell(
                    "/bin/unsupported_shell".to_string()
                )),
                append_to_path(PathBuf::from("/test"), None)
            );
        },
    );
}
