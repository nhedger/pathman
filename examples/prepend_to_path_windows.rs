#[cfg(windows)]
use pathman::{UpdateType, prepend_to_path};

fn main() {
    #[cfg(windows)]
    match prepend_to_path("C:\\test\\prepended", None) {
        Ok(update_type) => match update_type {
            UpdateType::Success => println!("Success"),
            UpdateType::AlreadyInPath => println!("Already in Path"),
        },
        Err(e) => println!("Error: {e}"),
    }
}
