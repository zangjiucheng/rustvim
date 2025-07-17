//! Utility plugins
//!
//! General utility commands and demonstrations

use crate::editor::Editor;
use crate::plugin::PluginRegistry;

/// Hello world command function (:hello)
pub fn hello_command(editor: &mut Editor) -> Result<(), String> {
    editor.set_status_message("Hello from plugin system!".to_string());
    Ok(())
}

/// Show current time (:time)
pub fn time_command(editor: &mut Editor) -> Result<(), String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {e}"))?;

    let secs = now.as_secs();
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let seconds = secs % 60;

    editor.set_status_message(format!(
        "Current time: {hours:02}:{minutes:02}:{seconds:02} UTC"
    ));
    Ok(())
}

/// Show buffer status (:status)
pub fn status_command(editor: &mut Editor) -> Result<(), String> {
    let buffer = editor.buffer();
    let filename = editor.filename().unwrap_or("untitled");
    let modified = if editor.is_modified() { "[+]" } else { "" };
    let cursor = editor.cursor();

    editor.set_status_message(format!(
        "{}{} - Line {}/{}, Col {}, {} chars",
        filename,
        modified,
        cursor.row + 1,
        buffer.line_count(),
        cursor.col + 1,
        buffer.content().len()
    ));

    Ok(())
}

/// Register utility plugins
pub fn register_plugins(registry: &mut PluginRegistry) {
    registry.register_ex_command("hello".to_string(), hello_command);
    registry.register_ex_command("time".to_string(), time_command);
    registry.register_ex_command("status".to_string(), status_command);
}
