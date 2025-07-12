#[cfg(unix)]
use pathman::{UpdateType, append_to_path};

fn main() {
    #[cfg(unix)]
    match append_to_path("/home/user/test/appended", Some("Managed by Pathman")) {
        Ok(update_type) => match update_type {
            UpdateType::Success => println!("Success"),
            UpdateType::AlreadyInPath => println!("Already in Path"),
        },
        Err(e) => println!("Error: {e}"),
    }
}
