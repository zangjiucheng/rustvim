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

- **`editor.rs`** - Main editor state, modal editing controller, and event loop
- **`buffer.rs`** - Unicode-safe text storage with efficient editing operations and newline preservation
- **`terminal.rs`** - Raw terminal I/O and screen control using ANSI escape codes
- **`input.rs`** - Comprehensive keystroke reading and escape sequence parsing
- **`commands.rs`** - Complete command system for Normal/Insert modes with operator-motion support
- **`history.rs`** - Vim-like undo/redo system with intelligent change grouping
- **`io.rs`** - File I/O operations with proper newline handling and error management
- **`lib.rs`** - Library interface for modular architecture and comprehensive testing

### Design Principles

1. **Modular Architecture**: Each component has clear responsibilities
2. **Raw Terminal Control**: Direct ANSI escape sequence usage for learning
3. **Safety**: Rust's ownership system ensures memory safety
4. **Extensibility**: Trait-based design allows for future improvements

## Current Status: Production-Ready Vim-like Editor

✅ **Completed Features:**
- **Project Foundation**: Cargo setup, Git repository, comprehensive modular architecture
- **Terminal Control**: Raw mode implementation with RAII guard pattern and full escape sequence support
- **Input Handling**: Complete keystroke reading, escape sequence parsing, and special key support
- **Text Buffer**: Unicode-safe editing with efficient line operations and newline preservation
- **Screen Rendering**: Full file display with viewport scrolling, status line, and search highlighting
- **Modal Editing**: Complete Normal/Insert/Command/Search mode implementation with proper transitions
- **Cursor Navigation**: Full Vim-style movement (hjkl, arrows, 0$, gg/G, w/b/e, f/F/t/T)
- **Text Editing**: Character insertion, deletion (x), line operations (o/O, dd), advanced editing commands
- **File Operations**: Complete file I/O with load, save (:w), save-as, new file creation (:e), and change detection
- **Command System**: Full Ex-command implementation (:w, :q, :wq, :q!, :e) with proper error handling
- **Delete Operations**: Character (x), line (dd), and motion-based deletion with register support
- **Word Motion**: Complete word navigation (w, b, e) and text object movements
- **Yank/Paste System**: Copy (yy, yw) and paste (p, P) with proper register management
- **Undo/Redo System**: Complete vim-like undo (u) and redo (Ctrl-R) with cursor position restoration
- **Insert Mode Grouping**: Intelligent change grouping for natural undo behavior
- **Search Functionality**: Pattern search (/) and reverse search (?) with next/previous (n/N), highlighting, and wrap-around
- **Error Handling**: Comprehensive error management with Vim-style error messages and recovery
- **Testing Suite**: 45+ comprehensive tests covering all functionality and edge cases

🎯 **Current Capabilities:**
- Full vim-like editing experience with all essential commands
- Load files: `cargo run filename.txt` with proper error handling
- Modal editing with seamless mode transitions (Normal ↔ Insert ↔ Command ↔ Search)
- Complete file operations with change detection and newline preservation  
- Advanced text manipulation with operator-motion combinations
- Copy/paste operations with proper register handling and line/character modes
- Full undo/redo support with vim-like behavior and cursor restoration
- Search through buffer with pattern highlighting and wrap-around navigation
- Professional status line with file info, mode display, and cursor position
- Comprehensive error handling and user feedback

� **Future Extensions:**
- Visual mode and advanced text selection
- Multiple buffers and buffer switching  
- Configuration and settings system
- Syntax highlighting and language-aware features
- Plugin system and extensibility
- Advanced search with regular expressions

## Documentation

For detailed implementation and technical details, see:

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Complete system architecture and design overview
- **[docs/daily-summaries/](docs/daily-summaries/)** - Day-by-day implementation progress logs

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd rustvim

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
- **File Operations**: `:w` to save, `:q` to quit, `:wq` to save and quit, `:e filename` to edit new file
- **Advanced**: Word navigation (`w`, `b`, `e`), deletion (`x`, `dd`), copy/paste (`yy`, `p`)
- **Search**: Press `/` to search forward, `?` to search backward, `n` for next match, `N` for previous
- **Undo/Redo**: Use `u` to undo changes, `Ctrl-R` to redo

### Vim Commands Supported
```
Navigation:     h j k l (arrows), 0 $ gg G, w b e
Insert modes:   i a A o O
Edit:           x (delete char), dd (delete line), o O (new lines)
Copy/Paste:     yy yw (yank), p P (paste)
Search:         / (forward), ? (backward), n (next), N (previous)
File Ops:       :w (save), :q (quit), :wq (save+quit), :e (edit file)
Undo/Redo:      u (undo), Ctrl-R (redo)
Mode Switch:    ESC (to normal), i a A o O (to insert), : (to command), / ? (to search)
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
- **Unit Tests**: Core functionality in buffer, terminal, input, history, and I/O modules
- **Integration Tests**: Real module interactions and complete workflow testing
- **File I/O Tests**: Newline preservation, file operations, and edge cases
- **Command Tests**: Ex-command system with error handling and validation
- **Search Tests**: Pattern matching, highlighting, and navigation
- **History Tests**: Undo/redo system integrity and cursor restoration
- **Manual Testing**: Interactive editing scenarios and real-world usage
- **Regression Testing**: Ensures new features don't break existing functionality

Total: **45+ comprehensive tests** covering all functionality and edge cases.

## Contributing

This is a production-ready vim-like text editor that demonstrates modern Rust development practices. The codebase is well-structured and thoroughly tested, making it an excellent reference for:

- Systems programming in Rust
- Terminal application development
- Text editor implementation
- Modal editing systems
- Clean architecture design

Feel free to use, study, and extend this editor for your own projects or learning purposes.

## License

This project is created for educational purposes. Feel free to use and modify as needed for learning.

## Resources and References

- [Kilo Editor Tutorial](https://viewsourcecode.org/snaptoken/kilo/) - Inspiration for raw terminal approach
- [Vim Documentation](https://vimhelp.org/) - Reference for Vim behavior
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code) - Terminal control sequences
- [Rust Book](https://doc.rust-lang.org/book/) - Rust language reference
