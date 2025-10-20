# LazyAnki

A Rust workspace for Anki automation and parsing tools.

## Project Structure

This project uses a Cargo workspace to manage multiple related crates:

- **`lazyanki`** - Main application crate
- **`anki`** - Anki API client library
- **`parser`** - Web scraping and parsing utilities

## Workspace Benefits

The project has been optimized using Cargo workspace features:

### Centralized Dependency Management
- All common dependencies are defined in the workspace `Cargo.toml`
- Version consistency across all crates
- Easier maintenance and updates
- Reduced compilation time through shared artifacts

### Common Dependencies
- **Error Handling**: `anyhow` for error management
- **Async Runtime**: `tokio` with full features
- **HTTP Client**: `reqwest` with JSON support
- **Serialization**: `serde` and `serde_json`
- **Web Scraping**: `scraper`, `regex`, `url`
- **CLI**: `clap` with derive features

## Building

Build the entire workspace:
```bash
cargo build --all
```

Build a specific crate:
```bash
cargo build -p anki
cargo build -p parser
cargo build -p lazyanki
```

## Testing

Run tests for all crates:
```bash
cargo test --all
```

## Workspace Configuration

The workspace is configured with:
- Rust edition 2021
- Shared metadata (version, authors, license)
- Centralized dependency versions
- Consistent feature flags

This structure makes it easy to:
- Add new related crates
- Maintain dependency versions
- Share code between crates
- Build and test everything together