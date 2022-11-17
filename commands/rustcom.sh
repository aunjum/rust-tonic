# Create a new binary executable crate
cargo new

# New binary executable with the name "my_binary".
cargo new my_binary

# Create a new library crate
cargo new --lib

# New library with the name "my_library"
cargo new --lib my_library

# Compiles our crate
cargo build

# Compiles our crate with optimizations
cargo build --release

# Compiles our crate and runs the compiled executable
cargo run

# Run all tests in a crate
cargo test

# Build and open our crate's documentation in a web browser
cargo doc --open

# Cleans up temporary files created during compilation
cargo clean

# Publishes your crate to `crates.io`
cargo publish

# Installs a binary directly from crates.io
cargo install