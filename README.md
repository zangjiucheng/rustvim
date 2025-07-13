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

- **`editor.rs`** - Main editor state, modal editing controller, undo/redo integration
- **`buffer.rs`** - Unicode-safe text storage with efficient editing operations  
- **`terminal.rs`** - Raw terminal I/O and screen control using ANSI escape codes
- **`input.rs`** - Comprehensive keystroke reading and escape sequence parsing
- **`commands.rs`** - Complete command system for Normal/Insert modes
- **`history.rs`** - Vim-like undo/redo system with change grouping
- **`lib.rs`** - Library interface for modular architecture and testing

### Design Principles

1. **Modular Architecture**: Each component has clear responsibilities
2. **Raw Terminal Control**: Direct ANSI escape sequence usage for learning
3. **Safety**: Rust's ownership system ensures memory safety
4. **Extensibility**: Trait-based design allows for future improvements

## Current Status (Day 12 Complete)

✅ **Completed Features:**
- **Project Foundation**: Cargo setup, Git repository, modular architecture
- **Terminal Control**: Raw mode implementation with RAII guard pattern
- **Input Handling**: Comprehensive keystroke reading, escape sequence parsing
- **Text Buffer**: Unicode-safe editing with efficient line operations
- **Screen Rendering**: Full file display with viewport scrolling and status line
- **Modal Editing**: Complete Normal/Insert mode switching (i, a, A, o, O, ESC)
- **Cursor Navigation**: Vim-style movement (hjkl, arrows, 0, $, gg, G)
- **Text Editing**: Character insertion, deletion (x), line operations (Enter, Backspace)
- **File Operations**: Load and display files from command line
- **Delete Operations**: Single character (x) and line deletion (dd) with motions
- **Word Motion**: Word navigation (w, b, e) and text object movements
- **Yank/Paste System**: Copy (yy, yw) and paste (p, P) with register management
- **Undo/Redo System**: Complete vim-like undo (u) and redo (Ctrl-R) functionality
- **Insert Mode Grouping**: Intelligent change grouping for undo operations
- **Integration Testing**: Comprehensive test suite for all modules

🎯 **Current Capabilities:**
- Load files: `cargo run filename.txt`
- Modal editing with all basic Vim commands
- Text insertion, deletion, and navigation
- Copy/paste operations with proper register handling
- Full undo/redo support with vim-like behavior
- Professional status line and interface

📅 **Next Steps (Days 13-30):**
- Search functionality (find in buffer)
- Command-line mode (Ex commands for file I/O)
- Visual mode and advanced text selection
- Multiple buffers and buffer switching

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

# Run the editor (empty buffer)
cargo run

# Load and edit a specific file
cargo run filename.txt

# Run comprehensive test suite
cargo test

# Run integration tests
cargo test --test integration_tests
```

## Usage

### Basic Operation
- **File Loading**: `cargo run README.md` to open any text file
- **Modal Editing**: Switch between Normal mode (navigation) and Insert mode (editing)
- **Navigation**: Use `hjkl` or arrow keys, `0`/`$` for line start/end, `gg`/`G` for file start/end
- **Editing**: Press `i` to enter Insert mode, type text, press `ESC` to return to Normal mode
- **Advanced**: Word navigation (`w`, `b`, `e`), deletion (`x`, `dd`), copy/paste (`yy`, `p`)
- **Undo/Redo**: Use `u` to undo changes, `Ctrl-R` to redo
- **Exit**: Press `Ctrl+Q` to quit the editor

### Vim Commands Supported
```
Navigation:     h j k l (arrows), 0 $ gg G, w b e
Insert modes:   i a A o O
Edit:           x (delete char), dd (delete line) 
Copy/Paste:     yy yw (yank), p P (paste)
Undo/Redo:      u (undo), Ctrl-R (redo)
Exit:           Ctrl+Q
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

Implements the complete command system:
- **Normal mode commands**: Navigation (hjkl, 0$, ggG), deletion (x, dd), yank (yy, yw)
- **Insert mode commands**: Text insertion, line operations, cursor movement
- **Operator-pending mode**: Delete/yank with motion commands (dw, d$, y0)
- **Mode switching**: Seamless transitions between Normal and Insert modes
- **Command execution**: Integrated with editor state and undo system

### History Module (`history.rs`)

Provides vim-like undo/redo functionality:
- **Action tracking**: Records all text modifications with position information
- **Change grouping**: Groups insert mode sessions as single undo actions
- **Efficient storage**: Memory-bounded history with configurable limits
- **Cursor restoration**: Maintains cursor position across undo/redo operations
- **Integration**: Seamlessly works with all editing operations

## Learning Objectives

This project demonstrates practical implementation of:

1. **Systems Programming**: Raw terminal control, POSIX APIs, and low-level I/O
2. **Rust Language Features**: Ownership, borrowing, pattern matching, traits, and RAII
3. **Software Architecture**: Modular design, separation of concerns, and clean interfaces
4. **Text Editor Internals**: Buffer management, modal editing, command parsing, and undo systems
5. **UNIX Programming**: Terminal control, escape sequences, and signal handling
6. **Testing Strategies**: Unit testing, integration testing, and interactive validation

## Project Highlights

### Technical Achievements
- **Zero-dependency core**: Raw terminal control without external crates
- **Memory safe**: 100% safe Rust with comprehensive bounds checking
- **Vim-compatible**: Behavior closely matches Vim for supported commands
- **Robust architecture**: Clean module separation with extensive testing
- **Educational value**: Well-documented code perfect for learning systems programming

### Real-world Applicability
- **Production patterns**: RAII guards, error handling, and resource management
- **Performance considerations**: Efficient data structures and minimal allocations
- **User experience**: Responsive interface with immediate visual feedback
- **Maintainability**: Modular design supports easy feature additions

## Testing Strategy

The project includes comprehensive testing for all functionality:

```bash
# Run all tests (unit + integration)
cargo test

# Run specific module tests
cargo test buffer
cargo test editor
cargo test history

# Run integration tests only
cargo test --test integration_tests

# Test interactive functionality
cargo run README.md  # Load this file and try editing
```

### Test Coverage
- **Unit Tests**: Core functionality in buffer, terminal, input, and history modules
- **Integration Tests**: Real module interactions and workflow testing
- **Manual Testing**: Interactive editing scenarios and edge cases
- **Regression Testing**: Ensures new features don't break existing functionality

## Contributing

This is an educational project following a structured 30-day plan. Each day builds upon the previous implementation, adding specific features and improvements.

## License

This project is created for educational purposes. Feel free to use and modify as needed for learning.

## Resources and References

- [Kilo Editor Tutorial](https://viewsourcecode.org/snaptoken/kilo/) - Inspiration for raw terminal approach
- [Vim Documentation](https://vimhelp.org/) - Reference for Vim behavior
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code) - Terminal control sequences
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language reference
