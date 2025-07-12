#[cfg(windows)]
use pathman::{UpdateType, append_to_path};

fn main() {
    #[cfg(windows)]
    match append_to_path("C:\\test\\appended", None) {
        Ok(update_type) => match update_type {
            UpdateType::Success => println!("Success"),
            UpdateType::AlreadyInPath => println!("Already in Path"),
        },
        Err(e) => println!("Error: {e}"),
    }
}
