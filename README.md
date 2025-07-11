# VimLike Editor

A minimal Vim-inspired text editor built in Rust, following a 30-day implementation plan.

## Project Overview

This project implements a text editor that mimics core Vim functionality while being built from scratch in Rust. The editor focuses on:

- Modal editing (Normal, Insert, Command, Visual modes)
- Raw terminal control without external libraries
- Efficient text buffer management
- Vim-like keybindings and commands
- Educational code structure for learning purposes

## Architecture

The editor is structured into logical modules:

### Core Modules

- **`editor.rs`** - Main editor state and event loop controller
- **`buffer.rs`** - Text storage and manipulation with efficient editing operations
- **`terminal.rs`** - Raw terminal I/O and screen control using ANSI escape codes
- **`input.rs`** - Keystroke reading and parsing, including special key sequences
- **`commands.rs`** - Command definitions and execution for different modes

### Design Principles

1. **Modular Architecture**: Each component has clear responsibilities
2. **Raw Terminal Control**: Direct ANSI escape sequence usage for learning
3. **Safety**: Rust's ownership system ensures memory safety
4. **Extensibility**: Trait-based design allows for future improvements

## Current Status (Day 2)

✅ **Completed:**
- Project initialization with Cargo
- Git repository setup  
- Core module structure definition
- Basic data structures and enums
- Architectural planning and documentation
- **Terminal raw mode implementation**
- **RAII guard for safe terminal restoration**
- **ANSI escape sequence control**
- **Interactive raw mode testing**

🚧 **In Progress:**
- Keystroke reading and parsing (Day 3 ready)
- Input event loop foundation
- Basic screen rendering framework

📅 **Next Steps (Day 3):**
- Single-byte input reading and classification
- Escape sequence parsing for special keys
- Key enum mapping and event generation

## Documentation

For detailed implementation progress and technical details, see the **[docs/](docs/)** directory:

- **[Daily Progress Summaries](docs/daily-summaries/)** - Day-by-day implementation logs
- **[Project Architecture](docs/README.md)** - Comprehensive technical documentation
- **[Learning Resources](docs/README.md#resources-and-references)** - Reference materials and guides

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd vimlike_editor

# Build the project
cargo build

# Run the editor
cargo run

# Run tests
cargo test
```

## Module Documentation

### Buffer Module (`buffer.rs`)

The text buffer uses a simple `Vec<String>` approach where each string represents a line. This provides:

- **Simple line operations**: Easy to insert/delete entire lines
- **Bounds safety**: All operations include bounds checking
- **Future extensibility**: Trait-based design allows for buffer implementation swapping

Key operations:
- `insert_char(pos, ch)` - Insert character at position
- `delete_char(pos)` - Delete character at position
- `insert_newline(pos)` - Split line at position
- Line merging when deleting newlines

### Editor Module (`editor.rs`)

Central coordinator that manages:
- Current editing mode (Normal/Insert/Command/Visual)
- Cursor position and viewport scrolling
- Integration between buffer, terminal, and input
- Overall application state

### Terminal Module (`terminal.rs`)

Handles low-level terminal operations using POSIX termios:
- **Raw mode implementation** with complete termios configuration
- **RAII guard pattern** for automatic terminal restoration
- **ANSI escape sequence generation** for screen control
- **Screen clearing and cursor positioning** 
- **Safe cleanup** even on program panic or crash

### Input Module (`input.rs`)

Processes raw keyboard input:
- Single byte reading from stdin
- Escape sequence parsing for special keys
- Key classification and enumeration
- Multi-byte sequence handling

### Commands Module (`commands.rs`)

Defines the command system:
- Normal mode command parsing
- Movement, edit, and mode-switch operations
- Operator-pending state management
- Command execution framework

## Learning Objectives

This project serves as a comprehensive example of:

1. **Systems Programming**: Low-level terminal control and I/O
2. **Rust Language Features**: Ownership, borrowing, pattern matching, traits
3. **Software Architecture**: Modular design and separation of concerns
4. **Text Editor Internals**: Buffer management, modal editing, command parsing
5. **UNIX Programming**: Terminal APIs and escape sequences

## Testing Strategy

The project includes unit tests for core functionality:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test buffer
cargo test editor
```

## Contributing

This is an educational project following a structured 30-day plan. Each day builds upon the previous implementation, adding specific features and improvements.

## License

This project is created for educational purposes. Feel free to use and modify as needed for learning.

## Resources and References

- [Kilo Editor Tutorial](https://viewsourcecode.org/snaptoken/kilo/) - Inspiration for raw terminal approach
- [Vim Documentation](https://vimhelp.org/) - Reference for Vim behavior
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code) - Terminal control sequences
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language reference
