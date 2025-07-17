[<img src="https://img.shields.io/badge/lang-English-blue?style=for-the-badge">](README.md) [<img src="https://img.shields.io/badge/lang-中文-red?style=for-the-badge">](README_zh.md)
# RustVim - A Vim-like Text Editor in Rust

A comprehensive vim-like text editor implemented in Rust, featuring multiple editing modes, visual selection (including block mode), file operations, search functionality, and a robust undo/redo system.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-vim--like-green?style=for-the-badge)
[![codecov](https://codecov.io/gh/zangjiucheng/rustvim/branch/main/graph/badge.svg)](https://codecov.io/gh/zangjiucheng/rustvim)
[![Build Status](https://github.com/zangjiucheng/rustvim/workflows/Build%20RustVim/badge.svg)](https://github.com/zangjiucheng/rustvim/actions)

## Features

### 🎯 Modal Editing
- **Normal Mode**: Navigation and command execution
- **Insert Mode**: Text insertion with full editing capabilities  
- **Visual Mode**: Character-wise, line-wise, and block-wise text selection
- **Command Mode**: Ex-command execution (`:w`, `:q`, `:wq`, `:e`, etc.)
- **Search Mode**: Forward and backward text search with highlighting

### 📝 Text Operations
- **Visual Block Mode**: Rectangular text selection with Ctrl+V
- **Block Operations**: Copy, delete, and manipulate rectangular text blocks
- **Composite Undo**: Complex operations undo as single units
- **Yank/Paste**: Full register system for copy/paste operations
- **Line Operations**: Insert, delete, and manipulate entire lines

### 🔍 Search & Navigation
- **Pattern Search**: Forward (`/`) and backward (`?`) search
- **Search Highlighting**: Visual feedback for matches
- **Word Navigation**: Forward (`w`) and backward (`b`) word movement
- **Line Navigation**: Beginning (`0`, `^`) and end (`$`) of line
- **File Navigation**: Go to beginning (`gg`) and end (`G`) of file

### 💾 File Management
- **Multi-file Support**: Edit multiple files in one session
- **File Operations**: Load, save, save-as, and create new files
- **Change Detection**: Prevents data loss with unsaved change warnings
- **Newline Preservation**: Maintains original file formatting

### ⚡ Advanced Features
- **Undo/Redo**: Complete history tracking with `u` and `Ctrl+R`
- **Count Prefixes**: Numeric multipliers for commands (`5j`, `3dd`, etc.)
- **Operator-Motion**: Combine operators with motions (`d3w`, `y5j`)
- **Status Line**: File info, mode display, and cursor position
- **Error Handling**: Comprehensive error messages and recovery

## Installation

### Prerequisites
- Rust 1.85+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source
```bash
git clone <repository-url>
cd rustvim
cargo build --release
```

### Run
```bash
# Start with empty buffer
cargo run

# Load a specific file
cargo run filename.txt

# Or run the built binary
./target/release/rustvim [filename]
```

## Usage

### Basic Commands

#### Mode Switching
| Key | Action |
|-----|--------|
| `ESC` | Return to Normal mode |
| `i` | Enter Insert mode |
| `v` | Enter Visual mode (character-wise) |
| `V` | Enter Visual Line mode |
| `Ctrl+V` | Enter Visual Block mode |
| `:` | Enter Command mode |
| `/` | Enter Search mode |

#### Navigation
| Key | Action |
|-----|--------|
| `h/j/k/l` | Move left/down/up/right |
| `w/b` | Word forward/backward |
| `0` | Beginning of line |
| `$` | End of line |
| `gg` | Go to first line |
| `G` | Go to last line |

#### Text Operations
| Key | Action |
|-----|--------|
| `o/O` | Insert new line below/above |
| `dd` | Delete current line |
| `yy` | Yank (copy) current line |
| `p` | Paste after cursor |
| `u` | Undo |
| `Ctrl+R` | Redo |

### Visual Mode Operations

#### Character-wise Selection (`v`)
```
v           # Start selection
<movement>  # Extend selection
d           # Delete selected text
y           # Copy selected text
ESC         # Exit visual mode
```

#### Line-wise Selection (`V`)
```
V           # Start line selection
<movement>  # Extend to full lines
d           # Delete selected lines
y           # Copy selected lines
```

#### Block Selection (`Ctrl+V`)
```
Ctrl+V      # Start block selection
<movement>  # Create rectangular selection
d           # Delete block
y           # Copy block
ESC         # Exit visual mode
```

### File Commands
| Command | Action |
|---------|--------|
| `:w` | Save current file |
| `:w filename` | Save as filename |
| `:q` | Quit (with change detection) |
| `:q!` | Force quit (discard changes) |
| `:wq` | Save and quit |
| `:e filename` | Edit new file |

### Search Operations
| Command | Action |
|---------|--------|
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Next match |
| `N` | Previous match |
| `ESC` | Exit search |

## Examples

### Basic Editing Workflow
```bash
# Open a file
cargo run example.txt

# Navigate to a word and enter visual mode
w v 3w    # Select 3 words

# Copy the selection
y

# Move somewhere else and paste
G p

# Save and quit
:wq
```

### Visual Block Operations
```bash
# Select a rectangular block
Ctrl+V    # Enter visual block mode
3j 5l     # Select 4 rows × 6 columns

# Delete the block
d         # Removes rectangular selection

# Undo the entire operation
u         # Restores the complete block
```

### Multi-file Editing
```bash
# Open multiple files
cargo run file1.txt file2.txt

# Edit first file
i "Hello World" ESC

# Switch to second file
:bn

# Make changes and save both
:w
:bp
:w
```

## Architecture

### Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs              # Module exports  
├── editor.rs           # Core editor state and event loop
├── buffer.rs           # Text buffer management
├── terminal.rs         # Terminal control and rendering
├── input.rs            # Key input handling
├── commands.rs         # Command processing
├── keymap.rs           # Key mapping system
├── io.rs              # File I/O operations
└── history.rs          # Undo/redo system

tests/                   # Comprehensive test suite
├── visual_block_mode_tests.rs
├── visual_mode_tests.rs
├── buffer_tests.rs
└── ... (148+ tests)
```

### Key Design Principles
- **Modal Architecture**: Clean separation between editing modes
- **Composable Commands**: Operators combine with motions naturally
- **Memory Safety**: Rust's ownership system prevents common editor bugs
- **Comprehensive Testing**: 148+ tests ensure reliability
- **Vim Compatibility**: Faithful reproduction of vim behavior

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test visual_block_mode_tests
cargo test --test buffer_tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

## Code Coverage

Generate and view code coverage reports to ensure comprehensive testing:

### Prerequisites
The LLVM coverage tool is included in dev dependencies, or install manually:
```bash
cargo install cargo-llvm-cov
```

### Quick Coverage Analysis
Use the included coverage script for convenient analysis:
```bash
# Make the script executable (first time only)
chmod +x coverage.sh

# Generate coverage summary (default)
./coverage.sh

# Generate HTML report and open in browser
./coverage.sh html

# Generate all report formats
./coverage.sh all

# Show help and available options
./coverage.sh help
```

### Manual Coverage Commands
```bash
# Generate HTML coverage report (recommended)
cargo llvm-cov --html

# View summary in terminal
cargo llvm-cov --summary-only

# Show detailed line-by-line coverage
cargo llvm-cov --show-missing-lines

# Generate LCOV format for CI/CD integration
cargo llvm-cov --lcov --output-path coverage.lcov
```

### View Coverage Reports
- **HTML Report**: Open `target/llvm-cov/html/index.html` in your browser
- **Terminal**: Coverage percentages displayed directly in console
- **CI Integration**: Use LCOV format for services like Codecov

### Automated Coverage (CI/CD)
- **GitHub Actions**: Coverage runs automatically on every push and PR
- **Codecov Integration**: Detailed coverage tracking and trending
- **PR Comments**: Coverage summaries posted directly on pull requests
- **Coverage Artifacts**: Reports archived for download and analysis

## Performance

- **Efficient Text Storage**: Line-based representation for O(1) line access
- **Minimal Allocations**: Careful memory management for responsiveness  
- **Non-blocking Input**: Immediate feedback on all operations
- **Composite Operations**: Complex operations (like block delete) execute atomically

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for detailed information on:

- 🚀 **Getting Started**: Development setup and workflow
- ✅ **Quality Checks**: Testing, coverage, formatting, and linting
- 📋 **Guidelines**: Code standards and best practices
- 🔄 **CI/CD Process**: Automated quality assurance
- 🎯 **Contribution Areas**: Where help is most needed

### Quick Start
1. Fork the repository and create a feature branch
2. Make your changes with comprehensive tests
3. Run quality checks: `cargo test && ./coverage.sh && cargo clippy`
4. Submit a pull request

**All PRs automatically receive quality checks and coverage analysis.**

## Roadmap

### Completed ✅
- [x] Modal editing (Normal, Insert, Command, Search)
- [x] Visual selection (character, line, block)
- [x] File operations and multi-file support
- [x] Undo/redo system with composite operations
- [x] Search and navigation
- [x] Comprehensive test coverage

### Planned 🚧
- [ ] Syntax highlighting
- [ ] Configuration system
- [ ] Plugin architecture
- [ ] Regular expression search
- [ ] Multiple windows/splits
- [ ] Improved performance optimizations

## Documentation

For detailed implementation and technical details, see:
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete system architecture and design overview
- **[docs/daily-summaries/](docs/daily-summaries/)** - Day-by-day implementation progress logs

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Vim and its philosophy of modal editing
- Built with Rust for memory safety and performance
- Terminal interface uses standard ANSI escape sequences for broad compatibility

## Resources and References

- [Kilo Editor Tutorial](https://viewsourcecode.org/snaptoken/kilo/) - Inspiration for raw terminal approach
- [Vim Documentation](https://vimhelp.org/) - Reference for Vim behavior
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code) - Terminal control sequences
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language reference

## Continuous Integration

The project includes comprehensive GitHub Actions workflows for automated quality assurance:

### 🔧 Build Workflow (`.github/workflows/build.yml`)
- **Cross-platform Builds**: Automatically builds for Linux, Windows, and macOS
- **Quality Checks**: Runs formatting (`cargo fmt`), linting (`cargo clippy`), and tests
- **Dependency Caching**: Optimized build times with smart caching
- **Automated Releases**: Creates GitHub releases with binaries when tags are pushed
- **Artifact Storage**: Build artifacts available for download

### 📊 Coverage Workflow (`.github/workflows/coverage.yml`)
- **Automated Coverage**: Runs on every push and pull request
- **Multiple Reports**: Generates HTML, LCOV, and summary formats
- **Codecov Integration**: Uploads to Codecov for detailed analysis and trending
- **PR Comments**: Posts coverage summaries directly on pull requests
- **GitHub Summaries**: Coverage results visible in Actions tab

### Workflow Triggers
- **Push to main/develop**: Full build and coverage analysis
- **Pull Requests**: Quality checks and coverage reporting
- **Version Tags**: Automated release creation with cross-platform binaries

## Development Setup

### Prerequisites
- Rust 1.85+ (install from [rustup.rs](https://rustup.rs/))
- Git for version control

### Setting Up Pre-commit Hooks

To ensure code quality and consistency, we provide pre-commit hooks that automatically check formatting and run lints:

```bash
# Quick setup - installs Git pre-commit hook
./scripts/install-pre-commit-hook.sh

# Alternative: Use pre-commit framework
pip install pre-commit
pre-commit install
```

The hooks will automatically run before each commit and check:
- **Code formatting** (`cargo fmt`)
- **Clippy lints** (`cargo clippy`)
- **File formatting** (YAML, TOML syntax, trailing whitespace, etc.)

See [scripts/README.md](scripts/README.md) for detailed setup instructions.

### Quality Checks

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-features

# Generate coverage report
./coverage.sh
```
