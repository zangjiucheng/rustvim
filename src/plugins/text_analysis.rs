//! Text analysis plugins
//!
//! Plugins for analyzing buffer content like word count, line count, etc.

use crate::editor::Editor;
use crate::plugin::PluginRegistry;

/// Word count command function (:wc)
pub fn word_count_command(editor: &mut Editor) -> Result<(), String> {
    let buffer = editor.buffer();
    let content = buffer.content();

    let word_count = content.split_whitespace().count();
    let line_count = buffer.line_count();
    let char_count = content.len();

    editor.set_status_message(format!(
        "Words: {word_count}, Lines: {line_count}, Characters: {char_count}"
    ));

    Ok(())
}

/// Character frequency analysis (:charfreq)
pub fn character_frequency_command(editor: &mut Editor) -> Result<(), String> {
    let buffer = editor.buffer();
    let content = buffer.content();

    let mut char_counts = std::collections::HashMap::new();
    for ch in content.chars() {
        if !ch.is_whitespace() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }
    }

    let total_chars = char_counts.values().sum::<usize>();
    let unique_chars = char_counts.len();

    editor.set_status_message(format!(
        "Total chars: {}, Unique chars: {}, Most frequent: {}",
        total_chars,
        unique_chars,
        char_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(ch, count)| format!("'{ch}' ({count})"))
            .unwrap_or_else(|| "none".to_string())
    ));

    Ok(())
}

/// Register text analysis plugins
pub fn register_plugins(registry: &mut PluginRegistry) {
    registry.register_ex_command("wc".to_string(), word_count_command);
    registry.register_ex_command("charfreq".to_string(), character_frequency_command);
}
