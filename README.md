# learning-rust

## Installing rustup on Linux or macOS
``` rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

Check rust version
``` rust
rustc --version
```

Update Rust version
``` rust
rustc update
```

## Setup project manager
Cargo is Rust’s build system and package manager.

```rust
cargo install cargo-edit
```

cargo-edit package helps add packages in cargo add <any_package> format

```rust
cargo add <any_package>
```

## Building and Running a Cargo Project

```rust
cargo build # build your project and creates a executable on development
cargo run   # build your project, creates a executable on development and run code
cargo check # Quickly checks your code to make sure it compiles but doesn’t produce an executable
cargo build --release # Generates production artifacts
```

## Run  binary targets
cargo run --bin hello