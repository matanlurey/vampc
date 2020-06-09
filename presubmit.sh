# Exits when any command fails.
set -e

# Run rustfmt.
echo "Checking Formatting..."
cargo fmt -- --check

# Run clippy.
echo "Checking Lints..."
cargo clippy --all -- -D warnings

# Run cargo check.
echo "Checking Cargo..."
cargo check

# Run tests.
echo "Running Tests..."
cargo test
