//! Text manipulation plugins
//!
//! Plugins for modifying buffer content like sorting, reversing, etc.

use crate::editor::Editor;
use crate::plugin::PluginRegistry;

/// Line sort command function (:sort)
pub fn sort_lines_command(editor: &mut Editor) -> Result<(), String> {
    let line_count = editor.buffer().line_count();

    if line_count == 0 {
        editor.set_status_message("No lines to sort".to_string());
        return Ok(());
    }

    // Collect all lines
    let mut lines: Vec<String> = Vec::new();
    for i in 0..line_count {
        if let Some(line) = editor.buffer().get_line(i) {
            lines.push(line);
        }
    }

    // Sort the lines
    lines.sort();

    // Replace current buffer with a new one containing sorted content
    let sorted_content = lines.join("\n");
    let filename = editor.filename().map(|s| s.to_string());

    // Create new buffer info with sorted content
    let new_buffer_info = crate::editor::BufferInfo {
        buffer: crate::buffer::Buffer::from_file(&sorted_content),
        filename,
        modified: true,
        cursor: crate::editor::Cursor::new(),
        scroll_offset: 0,
        history: crate::history::History::new(),
    };

    // Replace current buffer
    editor.buffers[editor.current_buffer] = new_buffer_info;

    editor.set_status_message(format!("Sorted {} lines", lines.len()));
    editor.update_scroll();

    Ok(())
}

/// Reverse lines command function (:reverse)
pub fn reverse_lines_command(editor: &mut Editor) -> Result<(), String> {
    let line_count = editor.buffer().line_count();

    if line_count == 0 {
        editor.set_status_message("No lines to reverse".to_string());
        return Ok(());
    }

    // Collect all lines
    let mut lines: Vec<String> = Vec::new();
    for i in 0..line_count {
        if let Some(line) = editor.buffer().get_line(i) {
            lines.push(line);
        }
    }

    // Reverse the lines
    lines.reverse();

    // Replace current buffer with reversed content
    let reversed_content = lines.join("\n");
    let filename = editor.filename().map(|s| s.to_string());

    // Create new buffer info with reversed content
    let new_buffer_info = crate::editor::BufferInfo {
        buffer: crate::buffer::Buffer::from_file(&reversed_content),
        filename,
        modified: true,
        cursor: crate::editor::Cursor::new(),
        scroll_offset: 0,
        history: crate::history::History::new(),
    };

    // Replace current buffer
    editor.buffers[editor.current_buffer] = new_buffer_info;

    editor.set_status_message(format!("Reversed {} lines", lines.len()));
    editor.update_scroll();

    Ok(())
}

/// Remove duplicate lines (:uniq)
pub fn unique_lines_command(editor: &mut Editor) -> Result<(), String> {
    let line_count = editor.buffer().line_count();

    if line_count == 0 {
        editor.set_status_message("No lines to process".to_string());
        return Ok(());
    }

    // Collect all lines and remove duplicates while preserving order
    let mut lines: Vec<String> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for i in 0..line_count {
        if let Some(line) = editor.buffer().get_line(i) {
            if seen.insert(line.clone()) {
                lines.push(line);
            }
        }
    }

    let original_count = line_count;
    let unique_count = lines.len();

    // Replace current buffer with unique content
    let unique_content = lines.join("\n");
    let filename = editor.filename().map(|s| s.to_string());

    // Create new buffer info with unique content
    let new_buffer_info = crate::editor::BufferInfo {
        buffer: crate::buffer::Buffer::from_file(&unique_content),
        filename,
        modified: true,
        cursor: crate::editor::Cursor::new(),
        scroll_offset: 0,
        history: crate::history::History::new(),
    };

    // Replace current buffer
    editor.buffers[editor.current_buffer] = new_buffer_info;

    editor.set_status_message(format!(
        "Removed {} duplicate lines, {} unique lines remain",
        original_count - unique_count,
        unique_count
    ));
    editor.update_scroll();

    Ok(())
}

/// Register text manipulation plugins
pub fn register_plugins(registry: &mut PluginRegistry) {
    registry.register_ex_command("sort".to_string(), sort_lines_command);
    registry.register_ex_command("reverse".to_string(), reverse_lines_command);
    registry.register_ex_command("uniq".to_string(), unique_lines_command);
}
