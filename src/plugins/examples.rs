//! Extended plugin examples
//!
//! Advanced examples demonstrating key commands and event handlers

use crate::editor::{Editor, Mode};
use crate::input::Key;
use crate::plugin::{EditorEvent, PluginRegistry};

/// Example key command: Quick save on Ctrl+S in normal mode
pub fn quick_save_command(editor: &mut Editor) -> Result<(), String> {
    if editor.write_file(None) {
        Ok(())
    } else {
        Err("Quick save failed".to_string())
    }
}

/// Example key command: Toggle line numbers on F2 in normal mode
pub fn toggle_line_numbers_command(editor: &mut Editor) -> Result<(), String> {
    editor.show_line_numbers = !editor.show_line_numbers;
    editor.config_mut().show_line_numbers = editor.show_line_numbers;

    let status = if editor.show_line_numbers {
        "Line numbers enabled"
    } else {
        "Line numbers disabled"
    };

    editor.set_status_message(status.to_string());
    Ok(())
}

/// Example key command: Insert timestamp on F3 in insert mode
pub fn insert_timestamp_command(editor: &mut Editor) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let timestamp = format!("[{}]", now.as_secs());

    // Insert timestamp character by character
    for ch in timestamp.chars() {
        editor.insert_mode_char(ch);
    }

    editor.set_status_message("Timestamp inserted".to_string());
    Ok(())
}

/// Example key command: Delete word backwards on Ctrl+W in insert mode
pub fn delete_word_backward_command(editor: &mut Editor) -> Result<(), String> {
    let cursor_pos = editor.cursor_position();
    let line_content = editor.buffer().get_line(cursor_pos.row).unwrap_or_default();

    if cursor_pos.col == 0 {
        editor.set_status_message("Already at beginning of line".to_string());
        return Ok(());
    }

    // Find word boundary
    let chars: Vec<char> = line_content.chars().collect();
    let mut delete_count = 0;
    let mut pos = cursor_pos.col.saturating_sub(1);

    // Skip whitespace backwards
    while pos > 0 && chars.get(pos).is_some_and(|c| c.is_whitespace()) {
        delete_count += 1;
        if pos == 0 {
            break;
        }
        pos = pos.saturating_sub(1);
    }

    // Delete word characters backwards
    while pos > 0 && chars.get(pos).is_some_and(|c| !c.is_whitespace()) {
        delete_count += 1;
        if pos == 0 {
            break;
        }
        pos = pos.saturating_sub(1);
    }

    // Include the first character if it's part of the word
    if pos == 0 && chars.get(pos).is_some_and(|c| !c.is_whitespace()) {
        delete_count += 1;
    }

    // Perform the deletion using the editor's backspace function
    for _ in 0..delete_count {
        // We need to provide parameters for insert_mode_backspace
        let cursor_pos = editor.cursor_position();
        if cursor_pos.col > 0 {
            let line = editor.buffer().get_line(cursor_pos.row).unwrap_or_default();
            let chars: Vec<char> = line.chars().collect();
            let deleted_char = chars.get(cursor_pos.col.saturating_sub(1)).copied();
            let deletion_pos = crate::buffer::Position::new(cursor_pos.row, cursor_pos.col - 1);
            editor.insert_mode_backspace(deleted_char, Some(deletion_pos));
        }
    }

    editor.set_status_message(format!("Deleted {delete_count} characters"));
    Ok(())
}

/// Event handler: Auto-save on buffer modification
pub fn auto_save_handler(editor: &mut Editor) {
    if !editor.config().auto_save {
        return;
    }

    if editor.filename().is_some() {
        editor.write_file(None);
        editor.set_status_message("Auto-saved".to_string());
    }
}

/// Event handler: Log file operations
pub fn file_operation_logger(editor: &mut Editor) {
    // In a real implementation, this might write to a log file
    // For demo purposes, we'll just show a status message
    editor.set_status_message("File operation logged".to_string());
}

/// Event handler: Welcome message on file open
pub fn welcome_message_handler(editor: &mut Editor) {
    if let Some(filename) = editor.filename() {
        let line_count = editor.buffer().line_count();
        editor.set_status_message(format!(
            "Welcome! Opened {filename} with {line_count} lines"
        ));
    }
}

/// Event handler: Mode transition feedback
pub fn mode_transition_handler(editor: &mut Editor) {
    // This would typically receive the from/to modes, but for simplicity
    // we'll just show a generic message
    editor.set_status_message("Mode changed - plugin notified".to_string());
}

/// Event handler: Search statistics
pub fn search_stats_handler(editor: &mut Editor) {
    // This could track search patterns, frequency, etc.
    editor.set_status_message("Search tracked by plugin".to_string());
}

/// Event handler: Command execution tracker
pub fn command_tracker_handler(editor: &mut Editor) {
    // This could maintain usage statistics for commands
    editor.set_status_message("Command execution tracked".to_string());
}

/// Register all example key commands and event handlers
pub fn register_example_plugins(registry: &mut PluginRegistry) {
    // Register key commands
    registry.register_key_command(Mode::Normal, Key::Ctrl('s'), quick_save_command);

    registry.register_key_command(Mode::Normal, Key::Function(2), toggle_line_numbers_command);

    registry.register_key_command(Mode::Insert, Key::Function(3), insert_timestamp_command);

    registry.register_key_command(Mode::Insert, Key::Ctrl('w'), delete_word_backward_command);

    // Register event handlers
    registry.register_event_handler(EditorEvent::BufferModified, auto_save_handler);

    registry.register_event_handler(
        EditorEvent::FileOpened("".to_string()),
        file_operation_logger,
    );

    registry.register_event_handler(
        EditorEvent::FileOpened("".to_string()),
        welcome_message_handler,
    );

    registry.register_event_handler(
        EditorEvent::FileSaved("".to_string()),
        file_operation_logger,
    );

    registry.register_event_handler(
        EditorEvent::ModeChanged {
            from: Mode::Normal,
            to: Mode::Insert,
        },
        mode_transition_handler,
    );

    registry.register_event_handler(
        EditorEvent::SearchPerformed("".to_string()),
        search_stats_handler,
    );

    registry.register_event_handler(
        EditorEvent::CommandExecuted("".to_string()),
        command_tracker_handler,
    );
}

/// Example Ex command that demonstrates using key commands
pub fn bind_key_command(editor: &mut Editor) -> Result<(), String> {
    // This is a meta-command that could dynamically register key bindings
    editor.set_status_message(
        "Key binding examples registered! Try Ctrl+S, F2, F3, Ctrl+W".to_string(),
    );
    Ok(())
}

/// Example Ex command that demonstrates event handling
pub fn enable_events_command(editor: &mut Editor) -> Result<(), String> {
    editor.set_status_message(
        "Event handlers enabled! File ops, mode changes, and searches will be tracked".to_string(),
    );
    Ok(())
}

/// Register the meta commands
pub fn register_meta_commands(registry: &mut PluginRegistry) {
    registry.register_ex_command("bindkeys".to_string(), bind_key_command);
    registry.register_ex_command("events".to_string(), enable_events_command);
}
