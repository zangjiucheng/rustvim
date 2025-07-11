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
        match command {
            NormalCommand::Movement(mov) => {
                Self::execute_movement(editor, mov);
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
    fn execute_movement(editor: &mut Editor, command: MovementCommand) {
        match command {
            MovementCommand::Left => {
                editor.cursor_left();
                editor.update_scroll();
            }
            MovementCommand::Right => {
                editor.cursor_right();
                editor.update_scroll();
            }
            MovementCommand::Up => {
                editor.cursor_up();
                editor.update_scroll();
            }
            MovementCommand::Down => {
                editor.cursor_down();
                editor.update_scroll();
            }
            MovementCommand::LineStart => {
                editor.cursor.col = 0;
            }
            MovementCommand::LineEnd => {
                let line_len = editor.buffer.line_length(editor.cursor.row);
                editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
            }
            MovementCommand::FileStart => {
                editor.cursor.row = 0;
                editor.cursor.col = 0;
                editor.update_scroll();
            }
            MovementCommand::FileEnd => {
                editor.cursor.row = editor.buffer.line_count().saturating_sub(1);
                let line_len = editor.buffer.line_length(editor.cursor.row);
                editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                editor.update_scroll();
            }
            _ => {
                // TODO: Implement word movements and page movements
            }
        }
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
