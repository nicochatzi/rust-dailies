# 🦀 Rust Dailies! 🦀

## Commands

```sh
rustc --version
```

```sh
rustup show
rustup update stable
```

```sh
cargo new crab_walk // --bin is default for an app, --lib for library
cargo build
cargo test
cargo run
cargo run --release
```

## Cargo Extensions

[cargo-watch](https://github.com/passcod/cargo-watch)

When run inside a crate, i.e. directory with a top-level Cargo.toml file,
the watch command will run a given command whenever a file is changed. But
default that excludes `.git*` files and directories and the `target/` directory.

```sh
cargo install cargo-watch
cargo watch -c -x test
cargo watch -c -s 'python serve.py' --ignore '*.rs'
```

Add and remove dependencies from your crate.

[cargo-edit](https://github.com/killercup/cargo-edit)

```sh
cargo install cargo-edit
cargo add tui
cargo add nannou
cargo rm tui
```