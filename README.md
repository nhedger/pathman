# Pathman

> [!WARNING]
> This is a work in progress.

**Pathman** is a _cross-platform_ Rust library for prepending and appending
directories to the user's `PATH` environment variable.

- 🚀 **Cross-platform**. Works on Windows, macOS, and Linux
- 1️⃣ **Idempotent**. Does not duplicate entries in the `PATH`
- 💬 **Comments**. Supports adding custom comments in shell config files

## Installation

Run the following command to add **Pathman** to your project's dependencies.

```bash
cargo add pathman
```

## Usage

### Prepending a directory to the `PATH`

Use the `prepend_to_path` function to add a directory to the beginning of the 
`PATH` environment variable, which will ensure that the specified directory
will be searched first when executing commands. An optional comment can be
specified, and it will be added to the shell configuration file on macOS and
Linux.

```rust
use pathman::prepend_to_path;

prepend_to_path("/Users/nicolas/.biome/bin", Some("Biome installation directory"));
```

### Appending a directory to the `PATH`

Use the `append_to_path` function to add a directory to the end of the
`PATH` environment variable. This is useful for adding directories that contain
executables that you want to be available system-wide, but not necessarily
searched first. An optional comment can also be specified, which will be added 
to the shell configuration file on macOS and Linux.

```rust
use pathman::append_to_path;

append_to_path("/Users/nicolas/.cargo/bin", Some("Cargo installation directory"));
```

## License

Pathman is licensed under either of:

- The [Apache License Version 2.0](LICENSE-APACHE)
- The [MIT License](LICENSE-MIT)

at your option.

### Contributions

<small>Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this crate by you, as defined in the [Apache-2.0 license](LICENSE-APACHE),
shall be dual licensed as above, without any additional terms or conditions.</small>