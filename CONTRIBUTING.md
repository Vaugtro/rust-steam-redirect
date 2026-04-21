# Development Dependencies and Testing

This document outlines the minimum requirements for developing this project.

## Requirements

- Rust 1.70 or later (see `Cargo.toml` for `rust-version`)
- Cargo (comes with Rust)

## Development Setup

### Install Rust

If you don't have Rust installed, install it from https://rustup.rs/

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Development Tools

```bash
# Format checker
rustup component add rustfmt

# Linter
rustup component add clippy
```

## Building

### Debug Build
```bash
cargo build
# or
make build
```

### Release Build
```bash
cargo build --release
# or
make release
```

### Check Without Building
```bash
cargo check
# or
make check
```

## Testing

### Run All Tests
```bash
cargo test
# or
make test
```

### Run Specific Test
```bash
cargo test test_name
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Integration Tests Only
```bash
cargo test --test integration_tests
```

## Code Quality

### Format Check
```bash
cargo fmt -- --check
# or
make fmt-check
```

### Auto Format
```bash
cargo fmt
# or
make fmt
```

### Run Linter (Clippy)
```bash
cargo clippy -- -D warnings
# or
make lint
```

### Generate Documentation
```bash
cargo doc --no-deps --open
# or
make doc
```

## Continuous Integration

All checks run on:
- Linux (ubuntu-latest)
- Windows (windows-latest)
- macOS (macos-latest)

Using both stable and beta Rust toolchains.

See [.github/workflows/ci.yml](.github/workflows/ci.yml) for details.

## Complete Workflow

Run all checks locally before pushing:

```bash
make all
```

This command:
1. Formats code
2. Runs linter
3. Runs tests
4. Builds release binary

## Troubleshooting

### "cargo: command not found"
Install Rust from https://rustup.rs/

### Tests failing
Ensure you're using a recent version of Rust:
```bash
rustup update stable
```

### Formatting issues
Run `cargo fmt` to auto-fix formatting issues.

### Clippy warnings
Address warnings as suggested. Use `cargo clippy -- -D warnings` to see all issues.
