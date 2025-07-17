# VimLike Editor - Architecture Overview

## Current Status: Full-Featured Vim-like Text Editor

This is a comprehensive vim-like text editor implemented in Rust, featuring multiple editing modes, file I/O operations, search functionality, command-line interface, robust undo/redo system, and a flexible configuration system.

## Core Features

### Editing Modes
✅ **Normal Mode**: Navigation and command execution
✅ **Insert Mode**: Text insertion with full editing capabilities
✅ **Command Mode**: Ex-command execution (`:w`, `:q`, `:wq`, `:e`, etc.)
✅ **Search Mode**: Forward and backward text search with highlighting
✅ **Visual Mode**: Character-wise and line-wise text selection
✅ **Visual Block Mode**: Rectangular block selection with Ctrl+V

### File Operations
✅ **File Loading**: Load any text file with proper newline preservation
✅ **File Saving**: Save with `:w`, save as with `:w filename`
✅ **New File Creation**: Create new files with `:e filename`
✅ **Multiple File Support**: Switch between files with proper change detection
✅ **Newline Preservation**: Maintains original file formatting

### Navigation & Editing
✅ **Cursor Movement**: h/j/k/l, arrow keys, word navigation (w/b)
✅ **Line Operations**: Insert lines (o/O), delete lines (dd), go to line (G)
✅ **Character Operations**: Insert/delete characters, backspace support
✅ **Search & Replace**: Forward (/) and backward (?) search with n/N repeat
✅ **Scroll Management**: Automatic viewport scrolling for large files

### Advanced Features
✅ **Undo/Redo System**: Full history tracking with u/Ctrl+R
✅ **Yank/Put Operations**: Copy (y) and paste (p) with register support
✅ **Visual Selection**: Character-wise (v), line-wise (V), and block-wise (Ctrl+V) selection
✅ **Block Operations**: Rectangular text selection, yank, and delete with composite undo
✅ **Count Prefixes**: Numeric prefixes for commands (5j, 3dd, etc.)
✅ **Operator-Motion Commands**: Combine operators with motions (d5j, y3w)
✅ **Status Messages**: Vim-style error and informational messages
✅ **Bell/Flash Feedback**: Immediate feedback for invalid keys
✅ **Configurable Keymap**: Foundation for custom keybindings
✅ **Configuration System**: TOML-based `~/.rustvimrc` config, runtime `:set` commands, automatic loading

### User Interface
✅ **Status Line**: File info, mode display, cursor position, line count
✅ **Command Prompt**: Real-time command input with proper highlighting
✅ **Search Highlighting**: Visual feedback for search matches
✅ **Error Handling**: Comprehensive error messages and recovery

## Configuration System (NEW)

- **TOML-based config file**: `~/.rustvimrc` loaded automatically at startup
- **Runtime settings**: Change options instantly with `:set` commands
- **Comprehensive options**: Tab size, line numbers, search behavior, auto-save, and more
- **Extensible**: Foundation for future keymap and plugin configuration
- **Error handling**: Graceful fallback to defaults and robust validation

## Usage

### Starting the Editor
```bash
cargo run [filename]
# Or run the built binary
./target/release/rustvim [filename]
```

### Configuration Setup
```bash
cp .rustvimrc.example ~/.rustvimrc
# Edit ~/.rustvimrc to customize settings
```
- Change options at runtime with `:set` commands (e.g. `:set number`, `:set tabsize=8`)

### Basic Operations
- **ESC**: Return to normal mode from any mode
- **i**: Enter insert mode
- **v**: Enter visual mode (character-wise selection)
- **V**: Enter visual line mode (line-wise selection)
- **Ctrl+V**: Enter visual block mode (rectangular selection)
- **:**: Enter command mode
- **/**: Enter search mode
- **h/j/k/l**: Navigate (left/down/up/right)
- **w/b**: Word navigation (forward/backward)
- **G**: Go to end of file, **gg**: Go to beginning
- **o/O**: Insert new line below/above
- **dd**: Delete current line
- **yy**: Yank (copy) current line
- **p**: Put (paste) after cursor
- **u**: Undo, **Ctrl+R**: Redo

### Visual Mode Operations
- **v**: Start character-wise visual selection
- **V**: Start line-wise visual selection
- **Ctrl+V**: Start block-wise (rectangular) visual selection
- **d**: Delete selected text
- **y**: Yank (copy) selected text
- **ESC**: Exit visual mode

### File Commands
- **:w**: Save current file
- **:w filename**: Save as filename
- **:q**: Quit (with change detection)
- **:q!**: Force quit (discard changes)
- **:wq** or **:x**: Save and quit
- **:e filename**: Edit new file

### Search Operations
- **/pattern**: Search forward for pattern
- **?pattern**: Search backward for pattern
- **n**: Next match, **N**: Previous match
- **ESC**: Exit search mode

## Architecture

### Module Organization

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Module exports
├── editor.rs            # Core editor state and main event loop
├── io.rs                # File I/O operations (load, save, edit)
├── buffer.rs            # Text buffer with newline preservation
├── terminal.rs          # Terminal control and rendering
├── input.rs             # Key input handling and parsing
├── commands.rs          # Command processing and execution
├── keymap.rs            # Key mapping and action system
├── config.rs            # Configuration system (Day 20)
└── history.rs           # Undo/redo system

tests/
├── buffer_tests.rs         # Buffer functionality tests
├── command_mode_tests.rs   # Ex-command system tests
├── file_io_tests.rs       # File I/O and newline preservation
├── history_tests.rs       # Undo/redo system tests
├── integration_tests.rs   # Core functionality integration
├── keymap_tests.rs        # Key mapping and action system tests
├── newline_preservation_tests.rs # Newline handling edge cases
├── search_tests.rs        # Search functionality tests
├── visual_block_mode_tests.rs # Visual block mode functionality
├── visual_mode_tests.rs   # Visual mode selection tests
└── additional_command_tests.rs # Extended command tests
```

### Core Components

#### Editor (`editor.rs`)
- **Main event loop**: Handles all user input and mode transitions
- **Mode management**: Coordinates between Normal, Insert, Command, Search, and Visual modes
- **Visual selection**: Character-wise, line-wise, and block-wise text selection
- **Screen rendering**: Manages display updates, cursor positioning, and selection highlighting
- **State management**: Tracks cursor, scroll, search matches, status messages, visual selection

#### Buffer (`buffer.rs`)
- **Text storage**: Efficient line-based text representation
- **Edit operations**: Character/line insertion, deletion, modification
- **Position tracking**: Cursor and range position management
- **Newline preservation**: Maintains original file formatting integrity

#### File I/O (`io.rs`)
- **File loading**: Read files with proper content parsing
- **File saving**: Write files with newline preservation
- **Error handling**: Comprehensive file operation error management
- **New file creation**: Handle non-existent files gracefully

#### Command System (`commands.rs`)
- **Normal mode commands**: Movement, editing, operators
- **Insert mode processing**: Character insertion and navigation
- **Operator-motion combinations**: Complex command sequences
- **Count prefix handling**: Numeric command multipliers

#### History System (`history.rs`)
- **Action tracking**: Record all buffer modifications
- **Undo operations**: Reverse changes with cursor restoration
- **Redo operations**: Reapply undone changes
- **Insert mode grouping**: Batch related changes for better UX
- **Composite operations**: Single undo for complex operations like visual block delete

#### Keymap System (`keymap.rs`)
- **Action definitions**: Comprehensive set of editor actions
- **Key bindings**: Mode-specific key-to-action mappings
- **Visual mode support**: Actions for character, line, and block selection
- **Extensible framework**: Easy addition of new commands and key bindings

#### Configuration (`config.rs`)
- **EditorConfig struct**: Holds all configurable options
- **Automatic loading**: Loads `~/.rustvimrc` at startup, falls back to defaults
- **Runtime updates**: `:set` command parser for instant changes
- **Persistence**: Save config to file
- **Extensible**: Ready for future keymap/plugin options

#### Terminal Interface (`terminal.rs`)
- **Raw mode control**: Direct terminal input/output
- **ANSI escape sequences**: Cursor movement and text formatting
- **Screen management**: Clear, scroll, and refresh operations
- **Selection highlighting**: Visual feedback for text selection
- **Cross-platform support**: Works on Unix/Linux/macOS

## Testing

Comprehensive test suite with **148+ tests** covering:
- Core buffer operations and edge cases
- File I/O with various content types
- Command mode functionality
- Search operations and highlighting
- Undo/redo system integrity
- Visual mode selection (character, line, and block)
- Visual block operations and composite undo
- Key mapping and action system
- Newline preservation across platforms
- Integration scenarios

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test buffer_tests
cargo test --test command_mode_tests
cargo test --test visual_block_mode_tests
```

## Technical Highlights

### Performance
- **Efficient text representation**: Line-based storage for O(1) line access
- **Minimal allocations**: Reuse buffers and avoid unnecessary clones
- **Responsive UI**: Non-blocking input handling with immediate feedback

### Reliability
- **Comprehensive error handling**: Graceful recovery from all error conditions
- **Data integrity**: Robust undo/redo with consistent state management
- **Memory safety**: Rust's ownership system prevents common editor bugs

### Vim Compatibility
- **Authentic behavior**: Faithful reproduction of vim's core functionality
- **Standard key bindings**: Familiar navigation and editing commands
- **Error messages**: Vim-style error codes and descriptions
- **Mode transitions**: Proper ESC handling and mode switching

### Extensibility
- **Modular design**: Easy to add new config options, keybindings, or plugins
- **Configuration foundation**: Centralized config enables runtime flexibility
- **Keymap system**: Table-driven, supports future custom keybindings
- **Plugin-ready**: Clean interfaces for future extension

## Future Extensions

The architecture supports easy extension for:
- **Visual block paste**: Paste rectangular content as blocks
- **Block insert mode**: Insert text across multiple lines simultaneously
- **Multiple buffers**: Tab/window management
- **Syntax highlighting**: Language-aware text coloring
- **Plugin system**: External command and feature integration
- **Configuration**: User preferences and key mapping
- **Advanced search**: Regular expressions and replace operations

This editor provides a solid foundation for any text editing needs while maintaining the familiar vim workflow that developers love.
