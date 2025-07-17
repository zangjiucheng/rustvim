[<img src="https://img.shields.io/badge/lang-中文-red?style=for-the-badge">](README_zh.md)

# RustVim - A Vim-like Text Editor in Rust

A comprehensive vim-like text editor implemented in Rust, featuring modal editing, visual selection (including block mode), file operations, search functionality, robust undo/redo, and a flexible configuration system.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-vim--like-green?style=for-the-badge)
[![codecov](https://codecov.io/gh/zangjiucheng/rustvim/branch/main/graph/badge.svg)](https://codecov.io/gh/zangjiucheng/rustvim)
[![Build Status](https://github.com/zangjiucheng/rustvim/workflows/Build%20RustVim/badge.svg)](https://github.com/zangjiucheng/rustvim/actions)

## Features

- **Modal Editing**: Normal, Insert, Visual (character/line/block), Command, and Search modes
- **Visual Block Mode**: Rectangular selection and block operations
- **Composite Undo/Redo**: Undo/redo complex operations as single units
- **Yank/Paste**: Full register system for copy/paste
- **Pattern Search**: Forward/backward search with highlighting
- **Multi-file Support**: Edit multiple files in one session
- **Status Line**: File info, mode display, and cursor position
- **Bell/Flash Feedback**: Immediate feedback for invalid keys
- **Configurable Keymap**: Foundation for custom keybindings
- **TOML Configuration**: Human-readable `~/.rustvimrc` config file, runtime `:set` commands

## Quick Start

### Installation
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
target/release/rustvim [filename]
```

### Configuration Setup
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

## Usage

### Modal Editing & Navigation
| Key         | Action                       |
|-------------|-----------------------------|
| `ESC`       | Normal mode                  |
| `i`         | Insert mode                  |
| `v`         | Visual mode                  |
| `V`         | Visual line mode             |
| `Ctrl+V`    | Visual block mode            |
| `:`         | Command mode                 |
| `/`         | Search mode                  |
| `h/j/k/l`   | Move left/down/up/right      |
| `w/b`       | Word forward/backward        |
| `0/$`       | Line start/end               |
| `gg/G`      | File start/end               |

### Text Operations
| Key         | Action                       |
|-------------|-----------------------------|
| `o/O`       | Insert new line below/above  |
| `dd`        | Delete line                  |
| `yy`        | Yank line                    |
| `p`         | Paste                        |
| `u`         | Undo                         |
| `Ctrl+R`    | Redo                         |

### File Commands
| Command     | Action                       |
|-------------|-----------------------------|
| `:w`        | Save                         |
| `:q`        | Quit                         |
| `:wq`       | Save and quit                |
| `:e filename`| Edit new file               |

### Search
| Command     | Action                       |
|-------------|-----------------------------|
| `/pattern`  | Search forward               |
| `?pattern`  | Search backward              |
| `n/N`       | Next/previous match          |
| `ESC`       | Exit search                  |

## Architecture Overview

```
src/
├── main.rs           # Entry point
├── editor.rs         # Core editor logic
├── buffer.rs         # Text buffer management
├── terminal.rs       # Terminal control
├── input.rs          # Key input handling
├── commands.rs       # Command processing
├── keymap.rs         # Key mapping system
├── io.rs             # File I/O
├── history.rs        # Undo/redo system
└── config.rs         # Configuration system

tests/                # 160+ tests for reliability
```

## Testing & Quality
- Run all tests: `cargo test`
- Code coverage: `./coverage.sh html` (see `target/llvm-cov/html/index.html`)
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Pre-commit hooks: `./scripts/install-pre-commit-hook.sh`

## Documentation & Resources
- **[ARCHITECTURE.md](ARCHITECTURE.md)** — System design overview
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
