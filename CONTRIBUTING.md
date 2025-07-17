# Contributing to RustVim

Thank you for your interest in contributing to RustVim! This guide covers development setup, build instructions, configuration, and best practices for contributors.

## Installation & Build

- **Rust 1.85+ required** ([Install Rust](https://rustup.rs/))
- Clone and build:
```bash
git clone <repository-url>
cd rustvim
cargo build --release
```
- Run:
```bash
cargo run [filename]
# Or run the built binary
./target/release/rustvim [filename]
```

## Configuration Setup
1. Copy the example config:
   ```bash
   cp .rustvimrc.example ~/.rustvimrc
   ```
2. Edit your preferences in `~/.rustvimrc` (TOML format)
3. Settings auto-load at startup; change options anytime with `:set` commands

#### Example `.rustvimrc`
```toml
tab_size = 4
show_line_numbers = true
word_wrap = false
auto_save = false
search_case_sensitive = false
search_highlight = true
```

#### Common `:set` Commands
- `:set number` / `:set nonumber` — Show/hide line numbers
- `:set tabsize=8` — Set tab width
- `:set wrap` / `:set nowrap` — Enable/disable word wrap
- `:set auto_save` / `:set noauto_save` — Enable/disable auto-save
- `:set hlsearch` / `:set nohlsearch` — Highlight search results

## Development Workflow
- Fork the repository and create a feature branch
- Make your changes with comprehensive tests
- Run quality checks: `cargo test && ./coverage.sh && cargo clippy`
- Submit a pull request

### Adding Plugin Commands
- Create new plugin files in `src/plugins/`
- Export modules in `src/plugins/mod.rs`
- Register commands in the `register_builtin_plugins()` function
- See **[docs/PLUGIN_SYSTEM.md](docs/PLUGIN_SYSTEM.md)** for detailed plugin development guide

## Quality Checks
- Run all tests: `cargo test`
- Code coverage: `./coverage.sh html` (see `target/llvm-cov/html/index.html`)
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Pre-commit hooks: `./scripts/install-pre-commit-hook.sh`

## Additional Resources
- **[ARCHITECTURE.md](ARCHITECTURE.md)** — System design overview
- **[docs/PLUGIN_SYSTEM.md](docs/PLUGIN_SYSTEM.md)** — Plugin development guide
- **[docs/daily-summaries/](docs/daily-summaries/)** — Day-by-day progress
- **[.rustvimrc.example](.rustvimrc.example)** — Example config file
- **Vim Documentation** — [vimhelp.org](https://vimhelp.org/)
- **Kilo Editor Tutorial** — [viewsourcecode.org/snaptoken/kilo/](https://viewsourcecode.org/snaptoken/kilo/)

## License
MIT License — see [LICENSE](LICENSE)

## Acknowledgments
- Inspired by Vim and modal editing philosophy
- Built with Rust for safety and performance
- Terminal interface uses ANSI escape codes for compatibility
