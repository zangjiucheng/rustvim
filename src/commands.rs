#![allow(unused)]

use crate::editor::{Editor, Mode};
use crate::input::Key;

/// Represents different types of commands in Normal mode
#[derive(Debug, Clone)]
#[allow(unused)]
pub enum NormalCommand {
    /// Movement commands
    Movement(MovementCommand),
    
    /// Edit commands
    Edit(EditCommand),
    
    /// Mode switch commands
    ModeSwitch(ModeSwitchCommand),
    
    /// File operations
    File(FileCommand),
}

/// Movement commands for cursor navigation
#[derive(Debug, Clone)]
pub enum MovementCommand {
    /// Basic movements
    Left,
    Right,
    Up,
    Down,
    
    /// Word movements
    WordForward,      // w
    WordBackward,     // b
    WordEnd,          // e
    
    /// Line movements
    LineStart,        // 0
    LineFirstChar,    // ^
    LineEnd,          // $
    
    /// File movements
    FileStart,        // gg
    FileEnd,          // G
    
    /// Page movements
    PageUp,           // Ctrl-U
    PageDown,         // Ctrl-D
}

/// Edit commands for text modification
#[derive(Debug, Clone)]
pub enum EditCommand {
    /// Delete commands
    DeleteChar,       // x
    DeleteLine,       // dd
    Delete(MovementCommand), // d{motion}
    
    /// Yank (copy) commands
    YankLine,         // yy
    Yank(MovementCommand), // y{motion}
    
    /// Paste commands
    PasteAfter,       // p
    PasteBefore,      // P
    
    /// Undo/Redo
    Undo,             // u
    Redo,             // Ctrl-R
}

/// Mode switching commands
#[derive(Debug, Clone)]
pub enum ModeSwitchCommand {
    /// Insert mode variants
    InsertBefore,     // i
    InsertAfter,      // a
    InsertLineEnd,    // A
    InsertLineStart,  // I
    
    /// Open line commands
    OpenLineBelow,    // o
    OpenLineAbove,    // O
    
    /// Command mode
    CommandMode,      // :
    
    /// Search mode
    SearchForward,    // /
    SearchBackward,   // ?
}

/// File operation commands
#[derive(Debug, Clone)]
pub enum FileCommand {
    Save,             // :w
    Quit,             // :q
    SaveQuit,         // :wq
    ForceQuit,        // :q!
    Open(String),     // :e filename
}

/// Command parser and executor
pub struct CommandProcessor;

impl CommandProcessor {
    /// Parse a key into a command for Normal mode
    pub fn parse_normal_command(key: &Key, count: Option<usize>) -> Option<NormalCommand> {
        match key {
            // Movement commands
            Key::Char('h') | Key::Left => Some(NormalCommand::Movement(MovementCommand::Left)),
            Key::Char('j') | Key::Down => Some(NormalCommand::Movement(MovementCommand::Down)),
            Key::Char('k') | Key::Up => Some(NormalCommand::Movement(MovementCommand::Up)),
            Key::Char('l') | Key::Right => Some(NormalCommand::Movement(MovementCommand::Right)),
            
            Key::Char('w') => Some(NormalCommand::Movement(MovementCommand::WordForward)),
            Key::Char('b') => Some(NormalCommand::Movement(MovementCommand::WordBackward)),
            Key::Char('e') => Some(NormalCommand::Movement(MovementCommand::WordEnd)),
            
            Key::Char('0') => Some(NormalCommand::Movement(MovementCommand::LineStart)),
            Key::Char('^') => Some(NormalCommand::Movement(MovementCommand::LineFirstChar)),
            Key::Char('$') => Some(NormalCommand::Movement(MovementCommand::LineEnd)),
            
            Key::Char('G') => Some(NormalCommand::Movement(MovementCommand::FileEnd)),
            
            // Edit commands
            Key::Char('x') => Some(NormalCommand::Edit(EditCommand::DeleteChar)),
            Key::Char('u') => Some(NormalCommand::Edit(EditCommand::Undo)),
            Key::Ctrl('r') => Some(NormalCommand::Edit(EditCommand::Redo)),
            
            Key::Char('p') => Some(NormalCommand::Edit(EditCommand::PasteAfter)),
            Key::Char('P') => Some(NormalCommand::Edit(EditCommand::PasteBefore)),
            
            // Mode switch commands
            Key::Char('i') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertBefore)),
            Key::Char('a') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertAfter)),
            Key::Char('A') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertLineEnd)),
            Key::Char('I') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertLineStart)),
            
            Key::Char('o') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::OpenLineBelow)),
            Key::Char('O') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::OpenLineAbove)),
            
            Key::Char(':') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::CommandMode)),
            Key::Char('/') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::SearchForward)),
            Key::Char('?') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::SearchBackward)),
            
            _ => None,
        }
    }
    
    /// Parse special multi-key commands (like 'gg')
    pub fn parse_multi_key_command(first_key: &Key, input_handler: &mut crate::input::InputHandler) -> Option<NormalCommand> {
        match first_key {
            Key::Char('g') => {
                // Wait for second 'g' to go to top of file
                if let Ok(Key::Char('g')) = input_handler.read_key() {
                    Some(NormalCommand::Movement(MovementCommand::FileStart))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    
    /// Execute a command on the editor
    pub fn execute_command(editor: &mut Editor, command: NormalCommand) {
        let count = editor.pending_count.unwrap_or(1);
        match command {
            NormalCommand::Movement(mov) => {
                Self::execute_movement(editor, mov, count);
            }
            NormalCommand::Edit(edit) => {
                Self::execute_edit(editor, edit);
            }
            NormalCommand::ModeSwitch(mode_switch) => {
                Self::execute_mode_switch(editor, mode_switch);
            }
            NormalCommand::File(file) => {
                Self::execute_file_command(editor, file);
            }
        }
    }
    
    /// Execute movement command
    fn execute_movement(editor: &mut Editor, command: MovementCommand, count: usize) {
        for _ in 0..count {
            match command {
                MovementCommand::Left => {
                    editor.cursor_left();
                }
                MovementCommand::Right => {
                    editor.cursor_right();
                }
                MovementCommand::Up => {
                    editor.cursor_up();
                }
                MovementCommand::Down => {
                    editor.cursor_down();
                }
                MovementCommand::LineStart => {
                    editor.cursor.col = 0;
                    break; // Don't repeat line start
                }
                MovementCommand::LineFirstChar => {
                    // Move to first non-blank character of the line
                    let line = if let Some(line) = editor.buffer.get_line(editor.cursor.row) {
                        line
                    } else {
                        break;
                    };
                    
                    let first_non_blank = line.chars()
                        .position(|c| !c.is_whitespace())
                        .unwrap_or(0);
                    editor.cursor.col = first_non_blank;
                    break; // Don't repeat line first char
                }
                MovementCommand::LineEnd => {
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                    break; // Don't repeat line end
                }
                MovementCommand::FileStart => {
                    editor.cursor.row = 0;
                    editor.cursor.col = 0;
                    break; // Don't repeat file start
                }
                MovementCommand::FileEnd => {
                    editor.cursor.row = editor.buffer.line_count().saturating_sub(1);
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                    break; // Don't repeat file end
                }
                MovementCommand::WordForward => {
                    Self::move_word_forward(editor);
                }
                MovementCommand::WordBackward => {
                    Self::move_word_backward(editor);
                }
                MovementCommand::WordEnd => {
                    Self::move_word_end(editor);
                }
                _ => {
                    // TODO: Implement page movements
                    break;
                }
            }
        }
        editor.update_scroll();
    }
    
    /// Execute edit command
    fn execute_edit(editor: &mut Editor, command: EditCommand) {
        match command {
            EditCommand::DeleteChar => {
                // Delete character under cursor (x command)
                let cursor_pos = editor.cursor_position();
                if let Some(_deleted_char) = editor.buffer.delete_char(cursor_pos) {
                    editor.modified = true;
                    // Ensure cursor position is still valid after deletion
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    if editor.cursor.col >= line_len && line_len > 0 {
                        editor.cursor.col = line_len - 1;
                    } else if line_len == 0 && editor.cursor.col > 0 {
                        editor.cursor.col = 0;
                    }
                    // Update scroll position if needed
                    editor.update_scroll();
                }
            }
            EditCommand::Undo => {
                // TODO: Implement undo functionality
            }
            EditCommand::Redo => {
                // TODO: Implement redo functionality
            }
            _ => {
                // TODO: Implement other edit commands
            }
        }
    }
    
    /// Execute mode switch command
    fn execute_mode_switch(editor: &mut Editor, command: ModeSwitchCommand) {
        match command {
            ModeSwitchCommand::InsertBefore => {
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertAfter => {
                // Move cursor right one position for append, then enter insert mode
                let line_len = editor.buffer.line_length(editor.cursor.row);
                if editor.cursor.col < line_len {
                    editor.cursor.col += 1;
                }
                // If already at end of line, cursor.col should equal line_len for appending
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineEnd => {
                // Move to end of line, then enter insert mode
                let line_len = editor.buffer.line_length(editor.cursor.row);
                editor.cursor.col = line_len; // After last character for append
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineStart => {
                // Move to first non-blank character, then enter insert mode
                editor.cursor.col = 0; // Simplified - move to start of line
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineBelow => {
                // Create new line below current line and enter insert mode
                let pos = crate::buffer::Position::new(editor.cursor.row, editor.buffer.line_length(editor.cursor.row));
                editor.buffer.insert_newline(pos);
                editor.cursor.row += 1;
                editor.cursor.col = 0;
                editor.modified = true;
                editor.update_scroll();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineAbove => {
                // Create new line above current line and enter insert mode
                let pos = crate::buffer::Position::new(editor.cursor.row, 0);
                editor.buffer.insert_newline(pos);
                editor.cursor.col = 0;
                editor.modified = true;
                editor.update_scroll();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::CommandMode => {
                editor.mode = Mode::Command;
            }
            _ => {
                // TODO: Implement search modes
            }
        }
    }
    
    /// Execute file command (to be implemented)
    fn execute_file_command(editor: &mut Editor, command: FileCommand) {
        // TODO: Implement file operations
    }
    
    /// Helper function to check if a character is a word character
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
    
    /// Move cursor to the beginning of the next word (w command)
    fn move_word_forward(editor: &mut Editor) {
        let mut row = editor.cursor.row;
        let mut col = editor.cursor.col;
        
        // Move forward until we find the start of the next word
        loop {
            if let Some(line) = editor.buffer.get_line(row) {
                let chars: Vec<char> = line.chars().collect();
                
                // If we're at the end of the line, move to next line
                if col >= chars.len() {
                    if row + 1 < editor.buffer.line_count() {
                        row += 1;
                        col = 0;
                        
                        // Check if the new line is empty
                        if let Some(next_line) = editor.buffer.get_line(row) {
                            let next_chars: Vec<char> = next_line.chars().collect();
                            
                            // If the line is empty (or only whitespace), this is a word boundary
                            if next_chars.is_empty() {
                                break; // Empty line is considered a word boundary
                            }
                            
                            // Skip leading whitespace on the new line
                            while col < next_chars.len() && next_chars[col].is_whitespace() {
                                col += 1;
                            }
                            
                            // If we found non-whitespace, we're at start of word
                            if col < next_chars.len() {
                                break;
                            }
                            // If line was all whitespace, continue to next line
                        }
                    } else {
                        // At end of file
                        break;
                    }
                } else {
                    // Move at least one character forward
                    col += 1;
                    
                    // Skip current word if we're on a word character
                    while col < chars.len() && Self::is_word_char(chars[col]) {
                        col += 1;
                    }
                    
                    // Skip non-word characters (separators)
                    while col < chars.len() && !Self::is_word_char(chars[col]) && !chars[col].is_whitespace() {
                        col += 1;
                    }
                    
                    // Skip whitespace
                    while col < chars.len() && chars[col].is_whitespace() {
                        col += 1;
                    }
                    
                    // If we found a word character, we're at the start of next word
                    if col < chars.len() && Self::is_word_char(chars[col]) {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        
        editor.cursor.row = row;
        editor.cursor.col = col;
    }
    
    /// Move cursor to the beginning of the previous word (b command)
    fn move_word_backward(editor: &mut Editor) {
        let mut row = editor.cursor.row;
        let mut col = editor.cursor.col;
        
        // Move backward to find the start of the previous word
        loop {
            if let Some(line) = editor.buffer.get_line(row) {
                let chars: Vec<char> = line.chars().collect();
                
                // If we're at the start of the line, move to previous line
                if col == 0 {
                    if row > 0 {
                        row -= 1;
                        if let Some(prev_line) = editor.buffer.get_line(row) {
                            let prev_chars: Vec<char> = prev_line.chars().collect();
                            
                            // If previous line is empty, this is a word boundary
                            if prev_chars.is_empty() {
                                col = 0;
                                break;
                            }
                            
                            col = prev_chars.len();
                        }
                    } else {
                        // At start of file
                        break;
                    }
                } else {
                    // Move back at least one character
                    col -= 1;
                    
                    // Skip whitespace and separators
                    while col > 0 && (chars[col].is_whitespace() || (!Self::is_word_char(chars[col]) && !chars[col].is_whitespace())) {
                        col -= 1;
                    }
                    
                    // If we're on whitespace at col 0, skip it
                    if col == 0 && (chars[col].is_whitespace() || (!Self::is_word_char(chars[col]) && !chars[col].is_whitespace())) {
                        // Continue to previous line or stay here
                        if row > 0 {
                            row -= 1;
                            if let Some(prev_line) = editor.buffer.get_line(row) {
                                let prev_chars: Vec<char> = prev_line.chars().collect();
                                
                                // If previous line is empty, this is a word boundary
                                if prev_chars.is_empty() {
                                    col = 0;
                                    break;
                                }
                                
                                col = prev_chars.len();
                                continue;
                            }
                        }
                        break;
                    }
                    
                    // Now skip to the beginning of the current word
                    while col > 0 && Self::is_word_char(chars[col - 1]) {
                        col -= 1;
                    }
                    
                    break;
                }
            } else {
                break;
            }
        }
        
        editor.cursor.row = row;
        editor.cursor.col = col;
    }
    
    /// Move cursor to the end of the current/next word (e command)
    fn move_word_end(editor: &mut Editor) {
        let mut row = editor.cursor.row;
        let mut col = editor.cursor.col;
        
        if let Some(line) = editor.buffer.get_line(row) {
            let chars: Vec<char> = line.chars().collect();
            
            // Check if current line is empty (treat as word boundary)
            if chars.is_empty() {
                // Move to next non-empty line
                while row + 1 < editor.buffer.line_count() {
                    row += 1;
                    if let Some(next_line) = editor.buffer.get_line(row) {
                        let next_chars: Vec<char> = next_line.chars().collect();
                        if !next_chars.is_empty() {
                            col = 0;
                            // Find first word and move to its end
                            while col < next_chars.len() && !Self::is_word_char(next_chars[col]) {
                                col += 1;
                            }
                            if col < next_chars.len() {
                                while col < next_chars.len() && Self::is_word_char(next_chars[col]) {
                                    col += 1;
                                }
                                col = col.saturating_sub(1);
                            }
                            break;
                        }
                    }
                }
                editor.cursor.row = row;
                editor.cursor.col = col;
                return;
            }
            
            // If we're at the end of line, move to next line
            if col >= chars.len() {
                if row + 1 < editor.buffer.line_count() {
                    row += 1;
                    col = 0;
                }
            } else {
                // If we're on a word character, skip to end of current word
                if Self::is_word_char(chars[col]) {
                    while col < chars.len() && Self::is_word_char(chars[col]) {
                        col += 1;
                    }
                    col = col.saturating_sub(1); // Move back to last word character
                } else {
                    // Skip non-word characters and whitespace to find next word
                    while col < chars.len() && !Self::is_word_char(chars[col]) {
                        col += 1;
                    }
                    
                    // If we found a word, move to its end
                    if col < chars.len() {
                        while col < chars.len() && Self::is_word_char(chars[col]) {
                            col += 1;
                        }
                        col = col.saturating_sub(1); // Move back to last word character
                    } else if row + 1 < editor.buffer.line_count() {
                        // No word found on current line, try next line
                        row += 1;
                        col = 0;
                        if let Some(next_line) = editor.buffer.get_line(row) {
                            let next_chars: Vec<char> = next_line.chars().collect();
                            
                            // If next line is empty, stop there
                            if next_chars.is_empty() {
                                editor.cursor.row = row;
                                editor.cursor.col = 0;
                                return;
                            }
                            
                            // Skip leading whitespace
                            while col < next_chars.len() && next_chars[col].is_whitespace() {
                                col += 1;
                            }
                            // Move to end of first word on next line
                            while col < next_chars.len() && Self::is_word_char(next_chars[col]) {
                                col += 1;
                            }
                            col = col.saturating_sub(1);
                        }
                    }
                }
            }
        }
        
        editor.cursor.row = row;
        editor.cursor.col = col;
    }
}

/// Insert mode command processor
pub struct InsertModeProcessor;

impl InsertModeProcessor {
    /// Handle insert mode input
    pub fn handle_input(editor: &mut Editor, key: &Key) {
        match key {
            // Regular character insertion
            Key::Char(c) => {
                editor.buffer.insert_char(
                    crate::buffer::Position::new(editor.cursor.row, editor.cursor.col), 
                    *c
                );
                editor.cursor.col += 1;
                editor.modified = true;
            }
            
            // Enter key - split line
            Key::Enter => {
                editor.buffer.insert_newline(
                    crate::buffer::Position::new(editor.cursor.row, editor.cursor.col)
                );
                editor.cursor.row += 1;
                editor.cursor.col = 0;
                editor.modified = true;
                editor.update_scroll();
            }
            
            // Backspace - delete character to the left
            Key::Backspace => {
                if editor.cursor.col > 0 {
                    // Delete character to the left in current line
                    editor.cursor.col -= 1;
                    editor.buffer.delete_char(
                        crate::buffer::Position::new(editor.cursor.row, editor.cursor.col)
                    );
                    editor.modified = true;
                } else if editor.cursor.row > 0 {
                    // At beginning of line - join with previous line
                    editor.cursor.row -= 1;
                    editor.cursor.col = editor.buffer.line_length(editor.cursor.row);
                    
                    // Delete the newline (which will merge the lines)
                    editor.buffer.delete_char(
                        crate::buffer::Position::new(editor.cursor.row, editor.cursor.col)
                    );
                    
                    editor.modified = true;
                    editor.update_scroll();
                }
            }
            
            // Arrow keys in insert mode (for navigation without leaving insert)
            Key::Left => {
                if editor.cursor.col > 0 {
                    editor.cursor.col -= 1;
                }
            }
            Key::Right => {
                let line_len = editor.buffer.line_length(editor.cursor.row);
                // In insert mode, allow cursor to go to line_len (after last character)
                if editor.cursor.col < line_len {
                    editor.cursor.col += 1;
                }
            }
            Key::Up => {
                if editor.cursor.row > 0 {
                    editor.cursor.row -= 1;
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    if editor.cursor.col > line_len {
                        editor.cursor.col = line_len;
                    }
                }
                editor.update_scroll();
            }
            Key::Down => {
                if editor.cursor.row + 1 < editor.buffer.line_count() {
                    editor.cursor.row += 1;
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    if editor.cursor.col > line_len {
                        editor.cursor.col = line_len;
                    }
                }
                editor.update_scroll();
            }
            
            _ => {
                // Unhandled key in insert mode - ignore
            }
        }
    }
}
