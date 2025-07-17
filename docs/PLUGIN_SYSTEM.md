# RustVim Plugin System (Day 21)

The RustVim editor now includes a native Rust-based plugin system that allows contributors to easily extend the editor with new commands and functionality.

## Overview

The plugin system is designed around:
- **Native Rust functions** instead of scripting languages for maximum performance and safety
- **Function pointers** to avoid complex trait object borrowing issues
- **Compile-time safety** ensuring plugins can't crash the editor
- **Simple registration** making it easy for contributors to add new commands

## Architecture

### Core Components

1. **PluginRegistry**: Central registry that stores function pointers for custom commands
2. **ExCommand integration**: Unknown commands automatically check the plugin registry
3. **Event system**: Foundation for plugins to react to editor events
4. **Example plugins**: Demonstrative implementations showing the pattern

### Plugin Function Signature

All plugin commands must follow this signature:
```rust
fn my_plugin_command(editor: &mut Editor) -> Result<(), String>
```

## Adding New Plugin Commands

### Step 1: Write the Command Function

```rust
pub fn my_custom_command(editor: &mut Editor) -> Result<(), String> {
    // Your command logic here
    editor.set_status_message("Custom command executed!".to_string());
    Ok(())
}
```

### Step 2: Register the Command

Add your command to the plugin registry:

```rust
// In src/plugins/ directory, create a new module file
// For example, src/plugins/my_plugin.rs
pub fn register_my_plugins(registry: &mut PluginRegistry) {
    registry.register_ex_command("mycmd".to_string(), my_custom_command);
}
```

### Step 3: Call Registration

The registration is automatically called during editor initialization in `Editor::new()`.

## Project Structure

The plugin system is organized in a dedicated plugins directory:

```
src/
├── plugin.rs           # Core plugin system (PluginRegistry, EditorEvent)
├── plugins/            # Plugin implementations
│   ├── mod.rs         # Plugin module exports
│   └── utils.rs       # Built-in utility plugins (wc, hello, sort)
├── editor.rs          # Editor with plugin registry integration
├── commands.rs        # Ex command system with plugin checking
└── lib.rs            # Module exports including plugins
```

### Adding New Plugins

1. **Create a new plugin file**: Add your plugin in `src/plugins/my_plugin.rs`
2. **Export the module**: Add `pub mod my_plugin;` to `src/plugins/mod.rs`
3. **Register commands**: Call your registration function in `register_builtin_plugins()`

## Built-in Example Commands

The plugin system ships with three example commands:

### `:wc` - Word Count
Displays word, line, and character count for the current buffer.

```
Words: 42, Lines: 10, Characters: 234
```

### `:hello` - Hello World
Simple demonstration command that shows a greeting message.

```
Hello from plugin system!
```

### `:sort` - Sort Lines
Sorts all lines in the current buffer alphabetically.

## Usage

Once registered, plugin commands work exactly like built-in commands:

1. Enter command mode with `:`
2. Type the command name (e.g., `:wc`, `:hello`, `:sort`)
3. Press Enter to execute

If a command isn't found in the built-in commands, the system automatically checks the plugin registry before showing an error.

## API Reference

### PluginRegistry Methods

```rust
// Register an Ex command (colon command)
pub fn register_ex_command(&mut self, name: String, command: fn(&mut Editor) -> Result<(), String>)

// Register a key command for a specific mode
pub fn register_key_command(&mut self, mode: Mode, key: Key, command: fn(&mut Editor) -> Result<(), String>)

// Register an event handler
pub fn register_event_handler(&mut self, event: EditorEvent, handler: fn(&mut Editor))

// Check if a command exists
pub fn has_ex_command(&self, name: &str) -> bool

// Get a command function
pub fn get_ex_command(&self, name: &str) -> Option<fn(&mut Editor) -> Result<(), String>>
```

### Editor Methods Available to Plugins

```rust
// Buffer operations
editor.buffer()         // Get buffer reference
editor.buffer_mut()     // Get mutable buffer reference
editor.cursor()         // Get cursor position
editor.cursor_mut()     // Get mutable cursor

// UI operations
editor.set_status_message(message)  // Show status message
editor.update_scroll()              // Update display scroll
editor.set_modified(true)           // Mark buffer as modified

// File operations
editor.filename()       // Get current filename
editor.is_modified()    // Check if buffer is modified
```

## Event System

The plugin system includes an event framework for reacting to editor actions:

### Available Events

```rust
pub enum EditorEvent {
    FileOpened(String),
    FileSaved(String),
    ModeChanged { from: Mode, to: Mode },
    BufferModified,
    SearchPerformed(String),
    CommandExecuted(String),
}
```

### Registering Event Handlers

```rust
pub fn my_file_save_handler(editor: &mut Editor) {
    editor.set_status_message("File saved by plugin monitor!".to_string());
}

// Register the handler
registry.register_event_handler(
    EditorEvent::FileSaved("".to_string()),
    my_file_save_handler
);
```

## Example Plugin Implementation

Here's a complete example of creating a custom line numbering plugin:

### Step 1: Create the plugin file `src/plugins/line_numbers.rs`

```rust
use crate::editor::Editor;

/// Toggle line numbers command
pub fn toggle_line_numbers(editor: &mut Editor) -> Result<(), String> {
    editor.config_mut().show_line_numbers = !editor.config().show_line_numbers;
    editor.show_line_numbers = editor.config().show_line_numbers;
    
    let status = if editor.config().show_line_numbers {
        "Line numbers enabled"
    } else {
        "Line numbers disabled"
    };
    
    editor.set_status_message(status.to_string());
    Ok(())
}

/// Register the plugin
pub fn register_line_number_plugin(registry: &mut crate::plugin::PluginRegistry) {
    registry.register_ex_command("nu".to_string(), toggle_line_numbers);
    registry.register_ex_command("number".to_string(), toggle_line_numbers);
}
```

### Step 2: Export the module in `src/plugins/mod.rs`

```rust
pub mod utils;
pub mod line_numbers;  // Add this line

pub use utils::*;
pub use line_numbers::*;  // Add this line
```

### Step 3: Register in the main registration function

```rust
// In src/plugins/mod.rs, update register_builtin_plugins()
pub fn register_builtin_plugins(registry: &mut crate::plugin::PluginRegistry) {
    register_utility_plugins(registry);
    register_line_number_plugin(registry);  // Add this line
}
```

## Benefits of This Approach

1. **Performance**: Native Rust execution, no interpreter overhead
2. **Safety**: Compile-time checking prevents runtime errors
3. **Simplicity**: Contributors use familiar Rust tools and patterns
4. **Integration**: Plugins work seamlessly with existing undo/redo, configuration, etc.
5. **Debugging**: Standard Rust debugging tools work out of the box

## Testing Plugins

The plugin system includes comprehensive tests demonstrating:
- Command registration and lookup
- Command execution with proper error handling
- Integration with the Ex command system
- Buffer manipulation and status message handling

Run plugin tests with:
```bash
cargo test --test plugin_system_tests
```

## Future Extensions

The plugin system is designed to be easily extensible:
- Key-based commands for normal/visual/insert modes
- Event-driven plugins that react to editor state changes
- Configuration hooks for plugin-specific settings
- Dynamic loading of plugin modules

This foundation provides a robust, safe, and performant way to extend RustVim while maintaining the editor's reliability and speed.
