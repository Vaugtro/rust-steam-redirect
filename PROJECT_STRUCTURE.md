# Project Structure

This document describes the overall structure and organization of the steam-redirect project following Rust best practices.

## Directory Layout

```
steam-redirect/
├── src/                          # Source code
│   ├── main.rs                  # Binary entry point (thin CLI wrapper)
│   ├── lib.rs                   # Library root (public API)
│   ├── error.rs                 # Error types and Result alias
│   ├── config.rs                # Configuration file parsing
│   ├── path.rs                  # Path resolution utilities
│   └── executor.rs              # Program execution
├── tests/                        # Integration tests
│   └── integration_tests.rs      # Integration test suite
├── .github/
│   └── workflows/
│       └── ci.yml               # GitHub Actions CI/CD pipeline
├── Cargo.toml                    # Project manifest
├── Cargo.lock                    # Dependency lock file (generated)
├── rustfmt.toml                  # Code formatting configuration
├── .clippy.toml                  # Linter configuration
├── .editorconfig                 # Editor formatting standards
├── .gitignore                    # Git ignore rules
├── Makefile                      # Development task automation
├── README.md                     # Project overview and guide
├── CONTRIBUTING.md               # Development setup and guidelines
├── CHANGELOG.md                  # Version history
├── LICENSE                       # MIT License
├── redirect/config.cfg.example   # Configuration file template
└── target/                       # Build output (not in git)
    ├── debug/                    # Debug build
    └── release/                  # Release build
```

## Module Organization

### Public API (`src/lib.rs`)

The library exports these modules:
- `config` - Configuration file loading and parsing
- `error` - Error types (see `error::WrapperError`)
- `executor` - Program execution
- `path` - Path parsing and resolution

### src/main.rs
- Thin CLI entry point
- Delegates to library functions
- Handles program exit codes
- Error reporting to stderr

### src/error.rs
- `WrapperError` enum with all error variants
- `Result<T>` type alias for convenience
- Uses `thiserror` crate for error display

### src/config.rs
- `find_config_file()` - Searches for redirect/config.cfg
- `parse_program_entry()` - Parses program configuration
- `load_config()` - Unified config loading
- Contains unit tests for parsing logic

### src/path.rs
- `parse_command_line()` - Parses command line with quote handling
- `resolve_path()` - Resolves relative and absolute paths
- Contains unit tests for parsing
- Includes doc tests

### src/executor.rs
- `execute_program()` - Executes a program with arguments
- Returns the program's exit code
- Error handling for execution failures

## Testing Strategy

### Unit Tests
Located inline in module files (`#[cfg(test)]`):
- `src/config.rs` - Config parsing tests
- `src/path.rs` - Path parsing tests (with doc tests)

### Integration Tests
Located in `tests/`:
- `tests/integration_tests.rs` - Full feature integration tests
- Tests interaction between modules
- Real config parsing scenarios

### Doc Tests
Located in doc comments:
- `src/lib.rs` - Library examples
- `src/path.rs` - Path parsing examples

Run tests with: `cargo test` or `make test`

## Code Quality Tools

### Formatting (rustfmt)
- Configuration: `rustfmt.toml`
- Enforced by CI/CD
- Automatic fix: `cargo fmt` or `make fmt`

### Linting (clippy)
- Configuration: `.clippy.toml`
- Strict: All warnings are errors (`-D warnings`)
- Run: `cargo clippy` or `make lint`

### Editor Config (.editorconfig)
- Consistent settings across editors
- Tabs, spaces, line endings

## CI/CD Pipeline (.github/workflows/ci.yml)

Automated checks on push and pull requests:
1. **Test** - Runs on Linux, Windows, macOS; stable and beta Rust
2. **Format** - Verifies rustfmt compliance
3. **Clippy** - Runs linter, treats warnings as errors
4. **Documentation** - Builds docs, checks doc comments

## Development Workflow

### Build
```bash
make build          # Debug build
make release        # Optimized release build
make check          # Quick check without building
```

### Testing
```bash
make test           # All tests
cargo test test_name  # Specific test
```

### Code Quality
```bash
make fmt            # Auto-format code
make fmt-check      # Check formatting
make lint           # Run linter
```

### Documentation
```bash
make doc            # Generate and open docs
```

### Complete Workflow
```bash
make all            # Format → Lint → Test → Build
```

## Dependency Management

### Runtime Dependencies
- `thiserror` - Error handling and display

### Development Tools (handled by Rust toolchain)
- `rustfmt` - Code formatter
- `clippy` - Linter
- `cargo-test` - Testing framework

## Configuration Files

### Cargo.toml
- Project metadata and dependencies
- Build profiles (debug, release)
- Binary target definition

### rustfmt.toml
- Code formatting rules
- Max line width: 100
- Edition: 2021

### .clippy.toml
- Linter configuration
- Currently minimal (accepts defaults)

### .editorconfig
- Editor-independent formatting
- Handles tabs, line endings, charset

### .gitignore
- Standard Rust ignore patterns
- Build artifacts
- IDE files

## Release Configuration

Release builds include:
- `opt-level = 3` - Maximum optimization
- `lto = true` - Link-Time Optimization
- `codegen-units = 1` - Single codegen unit (slower build, better optimization)
- `strip = true` - Stripped binary (smaller size)

Result: ~500KB fully optimized binary

## Documentation

### User Documentation
- **README.md** - Overview, features, setup, and detailed usage examples
- **redirect/config.cfg.example** - Configuration template

### Developer Documentation
- **CONTRIBUTING.md** - Development setup and guidelines
- **CHANGELOG.md** - Version history (follows Keep a Changelog)
- **PROJECT_STRUCTURE.md** - This file
- **In-code comments** - Doc comments for all public items

## Versioning

- **Semantic Versioning**: Major.Minor.Patch
- **Current version**: 0.1.0 (initial release)
- **Minimum Rust version**: 1.70
- **Edition**: 2021

## Best Practices Implemented

✅ **Module Organization**
- Clear separation of concerns
- Public API through lib.rs
- Focused, single-responsibility modules

✅ **Testing**
- Unit tests co-located with code
- Integration tests in dedicated directory
- Doc tests for examples
- Good code coverage

✅ **Error Handling**
- Custom error types with thiserror
- Result type alias for ergonomics
- Clear error messages

✅ **Documentation**
- README with setup instructions
- Inline code comments
- Doc comments for public API
- Contributing guide

✅ **Code Quality**
- Strict clippy linting
- Consistent formatting
- CI/CD pipeline
- Multiple platform testing

✅ **Build Configuration**
- Optimized release profile
- Stripped binaries
- LTO enabled

✅ **Development Experience**
- Makefile for quick tasks
- Clear directory structure
- Configuration templates
- Example code

## Future Enhancements

Potential areas for improvement:
- Configuration file validation
- Environment variable substitution
- Logging/tracing support
- Windows-specific path handling
- Config file auto-generation
- Performance benchmarks
