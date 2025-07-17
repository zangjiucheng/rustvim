#![allow(unused)]

use crate::editor::{Editor, Mode};
use crate::input::Key;

/// Represents an operator waiting for a motion
#[derive(Debug, Clone)]
pub enum Operator {
    Delete, // d
    Yank,   // y
    Change, // c (future)
}

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

impl Command for NormalCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        let count = editor.pending_count.unwrap_or(1);
        match self {
            NormalCommand::Movement(mov) => {
                MovementExecutor::execute_movement(editor, mov.clone(), count);
                Ok(())
            }
            NormalCommand::Edit(edit) => edit.execute(editor),
            NormalCommand::ModeSwitch(mode_switch) => mode_switch.execute(editor),
            NormalCommand::File(file) => file.execute(editor),
        }
    }
}

/// Trait for executing commands
pub trait Command {
    fn execute(&self, editor: &mut Editor) -> Result<(), String>;
}

/// Trait for motion calculations
pub trait Motion {
    fn calculate_end_position(
        &self,
        editor: &Editor,
        start: (usize, usize),
        count: usize,
    ) -> (usize, usize);
    fn is_line_motion(&self) -> bool {
        false
    }
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
    WordForward, // w
    WordBackward, // b
    WordEnd,      // e

    /// Line movements
    LineStart, // 0
    LineFirstChar, // ^
    LineEnd,       // $

    /// File movements
    FileStart, // gg
    FileEnd, // G

    /// Page movements
    PageUp, // Ctrl-U
    PageDown, // Ctrl-D
}

impl Motion for MovementCommand {
    fn calculate_end_position(
        &self,
        editor: &Editor,
        start: (usize, usize),
        count: usize,
    ) -> (usize, usize) {
        let mut temp_row = start.0;
        let mut temp_col = start.1;

        for _ in 0..count {
            match self {
                MovementCommand::Left => {
                    temp_col = temp_col.saturating_sub(1);
                }
                MovementCommand::Right => {
                    let line_len = editor.buffer().line_length(temp_row);
                    if temp_col < line_len.saturating_sub(1) {
                        temp_col += 1;
                    }
                }
                MovementCommand::Up => {
                    if temp_row > 0 {
                        temp_row -= 1;
                        let line_len = editor.buffer().line_length(temp_row);
                        temp_col = temp_col.min(line_len.saturating_sub(1));
                    }
                }
                MovementCommand::Down => {
                    if temp_row + 1 < editor.buffer().line_count() {
                        temp_row += 1;
                        let line_len = editor.buffer().line_length(temp_row);
                        temp_col = temp_col.min(line_len.saturating_sub(1));
                    }
                }
                MovementCommand::WordForward => {
                    MotionCalculator::word_forward(editor, &mut temp_row, &mut temp_col);
                }
                MovementCommand::WordBackward => {
                    MotionCalculator::word_backward(editor, &mut temp_row, &mut temp_col);
                }
                MovementCommand::WordEnd => {
                    MotionCalculator::word_end(editor, &mut temp_row, &mut temp_col);
                }
                MovementCommand::LineFirstChar => {
                    if let Some(line) = editor.buffer().get_line(temp_row) {
                        temp_col = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    }
                    break;
                }
                MovementCommand::LineStart => {
                    temp_col = 0;
                    break;
                }
                MovementCommand::LineEnd => {
                    let line_len = editor.buffer().line_length(temp_row);
                    temp_col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::FileStart => {
                    temp_row = 0;
                    temp_col = 0;
                    break;
                }
                MovementCommand::FileEnd => {
                    temp_row = editor.buffer().line_count().saturating_sub(1);
                    let line_len = editor.buffer().line_length(temp_row);
                    temp_col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::PageUp => {
                    let content_rows = editor.terminal.rows().saturating_sub(1);
                    let page_size = content_rows / 2; // Half page like Vim's Ctrl-U
                    temp_row = temp_row.saturating_sub(page_size);
                    let line_len = editor.buffer().line_length(temp_row);
                    temp_col = temp_col.min(line_len.saturating_sub(1));
                    break;
                }
                MovementCommand::PageDown => {
                    let content_rows = editor.terminal.rows().saturating_sub(1);
                    let page_size = content_rows / 2; // Half page like Vim's Ctrl-D
                    temp_row =
                        (temp_row + page_size).min(editor.buffer().line_count().saturating_sub(1));
                    let line_len = editor.buffer().line_length(temp_row);
                    temp_col = temp_col.min(line_len.saturating_sub(1));
                    break;
                }
            }
        }

        (temp_row, temp_col)
    }

    fn is_line_motion(&self) -> bool {
        match self {
            MovementCommand::FileStart |  // gg - go to start of file
            MovementCommand::FileEnd => true,  // G - go to end of file
            _ => false,
        }
    }
}

/// Edit commands for text modification
#[derive(Debug, Clone)]
pub enum EditCommand {
    /// Delete commands
    DeleteChar, // x
    DeleteLine,              // dd
    Delete(MovementCommand), // d{motion}
    DeleteSelection,         // d in visual mode

    /// Yank (copy) commands
    YankLine, // yy
    Yank(MovementCommand), // y{motion}
    YankSelection,         // y in visual mode

    /// Paste commands
    PasteAfter, // p
    PasteBefore, // P

    /// Undo/Redo
    Undo, // u
    Redo, // Ctrl-R

    /// Search commands
    SearchNext, // n
    SearchPrevious, // N
}

impl Command for EditCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        match self {
            EditCommand::DeleteChar => {
                TextOperations::delete_char_at_cursor(editor);
                Ok(())
            }
            EditCommand::DeleteLine => {
                let count = editor.pending_count.unwrap_or(1);
                OperatorExecutor::execute_delete_line(editor, count);
                Ok(())
            }
            EditCommand::Delete(motion) => {
                let count = editor.pending_count.unwrap_or(1);
                OperatorExecutor::execute_delete_motion(editor, motion.clone(), count);
                Ok(())
            }
            EditCommand::YankLine => {
                let count = editor.pending_count.unwrap_or(1);
                OperatorExecutor::execute_yank_line(editor, count);
                Ok(())
            }
            EditCommand::Yank(motion) => {
                let count = editor.pending_count.unwrap_or(1);
                OperatorExecutor::execute_yank_motion(editor, motion.clone(), count);
                Ok(())
            }
            EditCommand::PasteAfter => {
                OperatorExecutor::execute_paste_after(editor);
                Ok(())
            }
            EditCommand::PasteBefore => {
                OperatorExecutor::execute_paste_before(editor);
                Ok(())
            }
            EditCommand::Undo => {
                editor.undo();
                Ok(())
            }
            EditCommand::Redo => {
                editor.redo();
                Ok(())
            }
            EditCommand::SearchNext => {
                editor.search_next();
                Ok(())
            }
            EditCommand::SearchPrevious => {
                editor.search_previous();
                Ok(())
            }
            EditCommand::DeleteSelection => {
                editor
                    .delete_visual_selection()
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            EditCommand::YankSelection => {
                editor.yank_visual_selection().map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }
}

/// Mode switching commands
#[derive(Debug, Clone)]
pub enum ModeSwitchCommand {
    /// Insert mode variants
    InsertBefore, // i
    InsertAfter,     // a
    InsertLineEnd,   // A
    InsertLineStart, // I

    /// Open line commands
    OpenLineBelow, // o
    OpenLineAbove, // O

    /// Command mode
    CommandMode, // :

    /// Search mode
    SearchForward, // /
    SearchBackward, // ?

    /// Visual mode
    EnterVisual, // v
    EnterVisualLine, // V
    ExitVisual,      // Esc or mode change
}

impl Command for ModeSwitchCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        match self {
            ModeSwitchCommand::InsertBefore => {
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertAfter => {
                let line_len = editor.buffer().line_length(editor.cursor().row);
                if editor.cursor().col < line_len {
                    editor.cursor_mut().col += 1;
                }
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineEnd => {
                let line_len = editor.buffer().line_length(editor.cursor().row);
                editor.cursor_mut().col = line_len;
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineStart => {
                editor.cursor_mut().col = 0;
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineBelow => {
                let pos = crate::buffer::Position::new(
                    editor.cursor().row,
                    editor.buffer().line_length(editor.cursor().row),
                );
                editor.buffer_mut().insert_newline(pos);
                editor.cursor_mut().row += 1;
                editor.cursor_mut().col = 0;
                editor.set_modified(true);
                editor.update_scroll();
                // Start insert mode with the newline already included
                editor.start_insert_mode_with_initial_text(pos, "\n".to_string());
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineAbove => {
                let pos = crate::buffer::Position::new(editor.cursor().row, 0);
                editor.buffer_mut().insert_newline(pos);
                editor.cursor_mut().col = 0;
                editor.set_modified(true);
                editor.update_scroll();
                // Start insert mode with the newline already included
                editor.start_insert_mode_with_initial_text(pos, "\n".to_string());
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::CommandMode => {
                editor.start_command_mode();
            }
            ModeSwitchCommand::SearchForward => {
                editor.start_search();
            }
            ModeSwitchCommand::SearchBackward => {
                // For now, implement backward search the same as forward
                // In a more complete implementation, this could be a separate mode
                editor.start_search();
            }
            ModeSwitchCommand::EnterVisual => {
                editor.enter_visual_mode();
            }
            ModeSwitchCommand::EnterVisualLine => {
                editor.enter_visual_line_mode();
            }
            ModeSwitchCommand::ExitVisual => {
                editor.exit_visual_mode();
            }
        }
        Ok(())
    }
}

/// File operation commands
#[derive(Debug, Clone)]
pub enum FileCommand {
    Save,         // :w
    Quit,         // :q
    SaveQuit,     // :wq
    ForceQuit,    // :q!
    Open(String), // :e filename
}

impl Command for FileCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        // TODO: Implement file operations
        Ok(())
    }
}

/// Ex commands (colon commands) for editor operations
#[derive(Debug, Clone)]
pub enum ExCommand {
    /// Write file operations
    Write {
        filename: Option<String>,
    },
    /// Quit operations
    Quit {
        force: bool,
    },
    /// Quit all operations
    QuitAll {
        force: bool,
    },
    /// Write all operations
    WriteAll,
    /// Write and quit operations
    WriteQuit {
        force: bool,
    },
    /// Write all and quit operations
    WriteQuitAll,
    /// Edit file operations
    Edit {
        filename: String,
    },
    /// Buffer navigation
    BufferNext,
    BufferPrev,
    BufferSwitch {
        number: usize,
    },
    BufferList,
    /// Generic buffer number switch
    BufferNumber {
        number: usize,
    },
    /// Set commands for configuration
    Set {
        option: String,
        value: Option<String>,
    },
    /// Unknown command
    Unknown {
        command: String,
    },
}

impl Command for ExCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        match self {
            ExCommand::Write { filename } => {
                editor.write_file(filename.clone());
                Ok(())
            }
            ExCommand::Quit { force } => {
                editor.close_buffer(*force);
                Ok(())
            }
            ExCommand::QuitAll { force } => {
                editor.quit_all_editor(*force);
                Ok(())
            }
            ExCommand::WriteAll => {
                editor.write_all_buffers();
                Ok(())
            }
            ExCommand::WriteQuit { force: _ } => {
                if editor.write_file(None) {
                    editor.close_buffer(true); // Force close after successful write
                }
                Ok(())
            }
            ExCommand::WriteQuitAll => {
                if editor.write_all_buffers() {
                    editor.quit_all_editor(true); // Force quit after successful write
                }
                Ok(())
            }
            ExCommand::Edit { filename } => {
                ExCommandExecutor::execute_edit(editor, filename);
                Ok(())
            }
            ExCommand::BufferNext => {
                editor.next_buffer();
                editor.set_status_message(format!("Buffer {}", editor.current_buffer + 1));
                Ok(())
            }
            ExCommand::BufferPrev => {
                editor.prev_buffer();
                editor.set_status_message(format!("Buffer {}", editor.current_buffer + 1));
                Ok(())
            }
            ExCommand::BufferSwitch { number } => {
                if *number > 0 && editor.switch_to_buffer(*number - 1) {
                    editor.set_status_message(format!("Buffer {number}"));
                } else {
                    editor.set_status_message(format!("E86: Buffer {number} does not exist"));
                }
                Ok(())
            }
            ExCommand::BufferList => {
                let buffer_list = editor.list_buffers();
                let message = buffer_list.join(", ");
                editor.set_status_message(message);
                Ok(())
            }
            ExCommand::BufferNumber { number } => {
                if *number > 0 && editor.switch_to_buffer(*number - 1) {
                    editor.set_status_message(format!("Buffer {number}"));
                } else {
                    editor.set_status_message(format!("E86: Buffer {number} does not exist"));
                }
                Ok(())
            }
            ExCommand::Set { option, value } => {
                match editor.config.set_option(option, value.as_deref()) {
                    Ok(message) => {
                        // Update the deprecated show_line_numbers field for compatibility
                        editor.show_line_numbers = editor.config.show_line_numbers;
                        editor.set_status_message(message);
                    }
                    Err(error) => {
                        editor.set_status_message(error);
                    }
                }
                Ok(())
            }
            ExCommand::Unknown { command } => {
                editor.set_status_message(format!("E492: Not an editor command: {command}"));
                Ok(())
            }
        }
    }
}

/// Parser for Ex commands (colon commands)
pub struct ExCommandParser;

impl ExCommandParser {
    /// Parse an Ex command string into an ExCommand enum
    pub fn parse(command: &str) -> ExCommand {
        if command.is_empty() {
            return ExCommand::Unknown {
                command: command.to_string(),
            };
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return ExCommand::Unknown {
                command: command.to_string(),
            };
        }

        match parts[0] {
            "w" => {
                if parts.len() > 1 {
                    let filename = parts[1..].join(" ");
                    ExCommand::Write {
                        filename: Some(filename),
                    }
                } else {
                    ExCommand::Write { filename: None }
                }
            }
            "q" => ExCommand::Quit { force: false },
            "q!" => ExCommand::Quit { force: true },
            "qa" => ExCommand::QuitAll { force: false },
            "qa!" => ExCommand::QuitAll { force: true },
            "wa" => ExCommand::WriteAll,
            "wqa" | "xa" => ExCommand::WriteQuitAll,
            "wq" | "x" => ExCommand::WriteQuit { force: false },
            "e" => {
                if parts.len() > 1 {
                    let filename = parts[1..].join(" ");
                    ExCommand::Edit { filename }
                } else {
                    ExCommand::Unknown {
                        command: "E471: Argument required".to_string(),
                    }
                }
            }
            "bn" | "bnext" => ExCommand::BufferNext,
            "bp" | "bprev" => ExCommand::BufferPrev,
            "ls" | "buffers" => ExCommand::BufferList,
            "set" => {
                if parts.len() > 1 {
                    let option = parts[1];
                    let value = if parts.len() > 2 {
                        Some(parts[2..].join(" "))
                    } else {
                        None
                    };
                    ExCommand::Set {
                        option: option.to_string(),
                        value,
                    }
                } else {
                    ExCommand::Unknown {
                        command: "E471: Argument required".to_string(),
                    }
                }
            }
            "b" => {
                if parts.len() > 1 {
                    if let Ok(buffer_num) = parts[1].parse::<usize>() {
                        ExCommand::BufferSwitch { number: buffer_num }
                    } else {
                        ExCommand::Unknown {
                            command: "E86: Invalid buffer number".to_string(),
                        }
                    }
                } else {
                    ExCommand::Unknown {
                        command: "E471: Argument required".to_string(),
                    }
                }
            }
            _ => {
                // Check if it's a buffer number
                if let Ok(buffer_num) = parts[0].parse::<usize>() {
                    ExCommand::BufferNumber { number: buffer_num }
                } else {
                    ExCommand::Unknown {
                        command: command.to_string(),
                    }
                }
            }
        }
    }
}

/// Utility for executing complex Ex commands
pub struct ExCommandExecutor;

impl ExCommandExecutor {
    /// Execute edit command with file loading logic
    pub fn execute_edit(editor: &mut Editor, filename: &str) {
        use crate::buffer::Buffer;
        use crate::editor::{BufferInfo, Cursor};
        use crate::history::History;

        // Create a new buffer for the file
        match std::fs::read_to_string(filename) {
            Ok(content) => {
                // File exists, load its content
                let buffer_info = BufferInfo {
                    buffer: Buffer::from_file(&content),
                    filename: Some(filename.to_string()),
                    modified: false,
                    cursor: Cursor::new(),
                    scroll_offset: 0,
                    history: History::new(),
                };

                editor.add_buffer(buffer_info);
                let line_count = editor.buffer().line_count();
                editor.set_status_message(format!("\"{filename}\" {line_count}L read"));
            }
            Err(_) => {
                // File doesn't exist, create new empty buffer
                let buffer_info = BufferInfo {
                    buffer: Buffer::new(),
                    filename: Some(filename.to_string()),
                    modified: false,
                    cursor: Cursor::new(),
                    scroll_offset: 0,
                    history: History::new(),
                };

                editor.add_buffer(buffer_info);
                editor.set_status_message(format!("\"{filename}\" [New File]"));
            }
        }
    }
}

/// Character types for word motion logic
#[derive(Debug, Clone, Copy, PartialEq)]
enum CharType {
    Whitespace,
    Word,        // alphanumeric + underscore
    Punctuation, // symbols, punctuation
}

/// Motion calculation utilities
pub struct MotionCalculator;

impl MotionCalculator {
    /// Helper function to check if a character is a word character (alphanumeric + underscore)
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    /// Helper function to check if a character is punctuation/symbol
    fn is_punct_char(c: char) -> bool {
        !c.is_whitespace() && !Self::is_word_char(c)
    }

    /// Get the character type for word motion logic
    fn char_type(c: char) -> CharType {
        if c.is_whitespace() {
            CharType::Whitespace
        } else if Self::is_word_char(c) {
            CharType::Word
        } else {
            CharType::Punctuation
        }
    }

    /// Move cursor to the beginning of the next word (w command)
    #[allow(clippy::while_let_loop)]
    pub fn word_forward(editor: &Editor, row: &mut usize, col: &mut usize) {
        loop {
            if let Some(line) = editor.buffer().get_line(*row) {
                let chars: Vec<char> = line.chars().collect();

                if *col >= chars.len() {
                    // At end of line, move to next line
                    if *row + 1 < editor.buffer().line_count() {
                        *row += 1;
                        *col = 0;

                        // Find first non-whitespace character in new line
                        if let Some(next_line) = editor.buffer().get_line(*row) {
                            let next_chars: Vec<char> = next_line.chars().collect();

                            if next_chars.is_empty() {
                                break; // Empty line is a valid word boundary
                            }

                            // Skip leading whitespace
                            while *col < next_chars.len() && next_chars[*col].is_whitespace() {
                                *col += 1;
                            }

                            if *col < next_chars.len() {
                                break; // Found start of next word
                            }
                        }
                    } else {
                        break; // At end of file
                    }
                } else {
                    // Move at least one character forward
                    if *col < chars.len() {
                        let current_type = Self::char_type(chars[*col]);
                        *col += 1;

                        // Skip remaining characters of the same type
                        while *col < chars.len() && Self::char_type(chars[*col]) == current_type {
                            *col += 1;
                        }

                        // Skip whitespace to find next word/punctuation
                        while *col < chars.len() && chars[*col].is_whitespace() {
                            *col += 1;
                        }

                        if *col < chars.len() {
                            break; // Found start of next word/punctuation
                        }
                        // If we reached end of line, continue loop to handle line boundary
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Move cursor to the beginning of the previous word (b command)  
    #[allow(clippy::while_let_loop)]
    pub fn word_backward(editor: &Editor, row: &mut usize, col: &mut usize) {
        loop {
            if let Some(line) = editor.buffer().get_line(*row) {
                let chars: Vec<char> = line.chars().collect();

                if *col == 0 {
                    // At beginning of line, move to previous line
                    if *row > 0 {
                        *row -= 1;
                        if let Some(prev_line) = editor.buffer().get_line(*row) {
                            let prev_chars: Vec<char> = prev_line.chars().collect();

                            if prev_chars.is_empty() {
                                *col = 0;
                                break; // Empty line is a valid word boundary
                            }

                            *col = prev_chars.len();
                            // Continue to find word boundary in previous line
                        }
                    } else {
                        break; // At beginning of file
                    }
                } else {
                    // Move back at least one character
                    *col -= 1;

                    // Skip whitespace
                    while *col > 0 && chars[*col].is_whitespace() {
                        *col -= 1;
                    }

                    // If we're at position 0 and it's whitespace, handle line boundary
                    if *col == 0 && chars[*col].is_whitespace() {
                        if *row > 0 {
                            *row -= 1;
                            if let Some(prev_line) = editor.buffer().get_line(*row) {
                                let prev_chars: Vec<char> = prev_line.chars().collect();

                                if prev_chars.is_empty() {
                                    *col = 0;
                                    break;
                                }

                                *col = prev_chars.len();
                                continue; // Continue processing in previous line
                            }
                        }
                        break;
                    }

                    // Find the beginning of the current word/punctuation group
                    if *col < chars.len() {
                        let current_type = Self::char_type(chars[*col]);

                        // Move back while characters are of the same type
                        while *col > 0 && Self::char_type(chars[*col - 1]) == current_type {
                            *col -= 1;
                        }
                    }

                    break;
                }
            } else {
                break;
            }
        }
    }

    /// Move cursor to the end of the current/next word (e command)
    #[allow(clippy::while_let_loop)]
    pub fn word_end(editor: &Editor, row: &mut usize, col: &mut usize) {
        loop {
            if let Some(line) = editor.buffer().get_line(*row) {
                let chars: Vec<char> = line.chars().collect();

                if chars.is_empty() {
                    // Empty line, move to next line
                    if *row + 1 < editor.buffer().line_count() {
                        *row += 1;
                        *col = 0;
                        continue;
                    } else {
                        break; // End of file
                    }
                }

                if *col >= chars.len() {
                    // At end of line, move to next line
                    if *row + 1 < editor.buffer().line_count() {
                        *row += 1;
                        *col = 0;
                        continue;
                    } else {
                        break; // End of file
                    }
                }

                // Special case: if we're at the end of a word/punctuation, move forward first
                if *col < chars.len() {
                    let current_type = Self::char_type(chars[*col]);

                    // If we're at the end of a word/punctuation group, or on whitespace, move to next
                    if current_type == CharType::Whitespace
                        || (*col + 1 < chars.len()
                            && Self::char_type(chars[*col + 1]) != current_type)
                        || (*col + 1 >= chars.len())
                    {
                        // Move forward past current character and whitespace
                        *col += 1;

                        // Skip whitespace
                        while *col < chars.len() && chars[*col].is_whitespace() {
                            *col += 1;
                        }

                        if *col >= chars.len() {
                            continue; // Continue to next line
                        }
                    }
                }

                // Now find the end of the current word/punctuation group
                if *col < chars.len() {
                    let current_type = Self::char_type(chars[*col]);

                    // Move to the end of the current word/punctuation group
                    while *col + 1 < chars.len() && Self::char_type(chars[*col + 1]) == current_type
                    {
                        *col += 1;
                    }
                }

                break;
            } else {
                break;
            }
        }
    }
}

/// Text manipulation operations
pub struct TextOperations;

impl TextOperations {
    /// Delete character at cursor position
    pub fn delete_char_at_cursor(editor: &mut Editor) {
        let cursor_pos = editor.cursor_position();
        if let Some(deleted_char) = editor.buffer_mut().delete_char(cursor_pos) {
            // Record the deletion for undo
            let action =
                crate::history::EditAction::delete_text(cursor_pos, deleted_char.to_string());
            editor.history_mut().push(action);

            // Store the deleted character in the register
            editor.register.store_text(deleted_char.to_string());

            editor.set_modified(true);
            let line_len = editor.buffer().line_length(editor.cursor().row);
            if editor.cursor().col >= line_len && line_len > 0 {
                editor.cursor_mut().col = line_len - 1;
            } else if line_len == 0 && editor.cursor().col > 0 {
                editor.cursor_mut().col = 0;
            }
            editor.update_scroll();
        }
    }

    /// Delete text in a range from start to end position
    pub fn delete_range(editor: &mut Editor, start: (usize, usize), end: (usize, usize)) {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        let (from_row, from_col, to_row, to_col) =
            if start_row < end_row || (start_row == end_row && start_col <= end_col) {
                (start_row, start_col, end_row, end_col)
            } else {
                (end_row, end_col, start_row, start_col)
            };

        #[allow(clippy::comparison_chain)]
        if from_row == to_row {
            if from_col < to_col {
                for _ in from_col..to_col {
                    if from_col < editor.buffer().line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, from_col);
                        editor.buffer_mut().delete_char(pos);
                    }
                }
            } else if from_col > to_col {
                for _ in to_col..from_col {
                    if to_col < editor.buffer().line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, to_col);
                        editor.buffer_mut().delete_char(pos);
                    }
                }
            }
        } else if from_row < to_row {
            let start_line_len = editor.buffer().line_length(from_row);
            for _ in from_col..start_line_len {
                if from_col < editor.buffer().line_length(from_row) {
                    let pos = crate::buffer::Position::new(from_row, from_col);
                    editor.buffer_mut().delete_char(pos);
                }
            }

            for row in (from_row + 1..to_row).rev() {
                Self::delete_line_at(editor, row);
            }

            let adjusted_end_row = from_row + 1;
            if adjusted_end_row < editor.buffer().line_count() {
                for _ in 0..to_col {
                    if 0 < editor.buffer().line_length(adjusted_end_row) {
                        let pos = crate::buffer::Position::new(adjusted_end_row, 0);
                        editor.buffer_mut().delete_char(pos);
                    }
                }

                if let Some(remaining_text) = editor.buffer().get_line(adjusted_end_row) {
                    let remaining = remaining_text.clone();
                    Self::delete_line_at(editor, adjusted_end_row);

                    for ch in remaining.chars() {
                        let pos = crate::buffer::Position::new(
                            from_row,
                            editor.buffer().line_length(from_row),
                        );
                        editor.buffer_mut().insert_char(pos, ch);
                    }
                }
            }
        } else {
            for _ in 0..from_col {
                if 0 < editor.buffer().line_length(from_row) {
                    let pos = crate::buffer::Position::new(from_row, 0);
                    editor.buffer_mut().delete_char(pos);
                }
            }

            for row in (to_row..from_row).rev() {
                Self::delete_line_at(editor, row);
            }
        }
    }

    /// Delete a line at specific row
    pub fn delete_line_at(editor: &mut Editor, row: usize) {
        let line_len = editor.buffer().line_length(row);
        for _ in 0..line_len {
            let pos = crate::buffer::Position::new(row, 0);
            editor.buffer_mut().delete_char(pos);
        }

        if row < editor.buffer().line_count() - 1 {
            let pos = crate::buffer::Position::new(row, 0);
            if editor.buffer().line_length(row) == 0 {
                editor.buffer_mut().delete_char(pos);
            }
        }
    }

    /// Clear a line at specific row
    pub fn clear_line_at(editor: &mut Editor, row: usize) {
        let line_len = editor.buffer().line_length(row);
        for _ in 0..line_len {
            let pos = crate::buffer::Position::new(row, 0);
            editor.buffer_mut().delete_char(pos);
        }
    }

    /// Extract text from a range without modifying the buffer (for yank operations)
    pub fn extract_range(editor: &Editor, start: (usize, usize), end: (usize, usize)) -> String {
        editor.buffer().extract_range(start, end)
    }

    /// Ensure cursor is within valid buffer bounds
    pub fn clamp_cursor_to_buffer(editor: &mut Editor) {
        editor.cursor_mut().row = editor
            .cursor()
            .row
            .min(editor.buffer().line_count().saturating_sub(1));

        let line_len = editor.buffer().line_length(editor.cursor().row);
        if line_len > 0 {
            editor.cursor_mut().col = editor.cursor().col.min(line_len - 1);
        } else {
            editor.cursor_mut().col = 0;
        }
    }
}

/// Movement execution logic
pub struct MovementExecutor;

impl MovementExecutor {
    /// Execute movement command
    pub fn execute_movement(editor: &mut Editor, command: MovementCommand, count: usize) {
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
                    editor.cursor_mut().col = 0;
                    break;
                }
                MovementCommand::LineFirstChar => {
                    let line = if let Some(line) = editor.buffer().get_line(editor.cursor().row) {
                        line
                    } else {
                        break;
                    };

                    let first_non_blank =
                        line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    editor.cursor_mut().col = first_non_blank;
                    break;
                }
                MovementCommand::LineEnd => {
                    let line_len = editor.buffer().line_length(editor.cursor().row);
                    editor.cursor_mut().col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::FileStart => {
                    editor.cursor_mut().row = 0;
                    editor.cursor_mut().col = 0;
                    break;
                }
                MovementCommand::FileEnd => {
                    editor.cursor_mut().row = editor.buffer().line_count().saturating_sub(1);
                    let line_len = editor.buffer().line_length(editor.cursor().row);
                    editor.cursor_mut().col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::WordForward => {
                    let mut row = editor.cursor().row;
                    let mut col = editor.cursor().col;
                    MotionCalculator::word_forward(editor, &mut row, &mut col);
                    editor.cursor_mut().row = row;
                    editor.cursor_mut().col = col;
                }
                MovementCommand::WordBackward => {
                    let mut row = editor.cursor().row;
                    let mut col = editor.cursor().col;
                    MotionCalculator::word_backward(editor, &mut row, &mut col);
                    editor.cursor_mut().row = row;
                    editor.cursor_mut().col = col;
                }
                MovementCommand::WordEnd => {
                    let mut row = editor.cursor().row;
                    let mut col = editor.cursor().col;
                    MotionCalculator::word_end(editor, &mut row, &mut col);
                    editor.cursor_mut().row = row;
                    editor.cursor_mut().col = col;
                }
                _ => {
                    break;
                }
            }
        }
        editor.update_scroll();
    }
}

/// Operator execution logic
pub struct OperatorExecutor;

impl OperatorExecutor {
    /// Execute delete line command (dd)
    pub fn execute_delete_line(editor: &mut Editor, count: usize) {
        let start_row = editor.cursor().row;
        let mut deleted_lines = Vec::new();

        // First, collect the lines to be deleted for the register
        for i in 0..count {
            let row = start_row + i;
            if row < editor.buffer().line_count() {
                if let Some(line) = editor.buffer().get_line(row) {
                    deleted_lines.push(line.clone());
                }
            }
        }

        // Store in register
        if !deleted_lines.is_empty() {
            let deleted_text = deleted_lines.join("\n") + "\n";
            editor.register.store_lines(deleted_text.clone());

            // Record the line deletion for undo
            let delete_pos = crate::buffer::Position::new(start_row, 0);
            let action = crate::history::EditAction::delete_text(delete_pos, deleted_text);
            editor.history_mut().push(action);
        }

        // Now delete the lines
        for _ in 0..count {
            if editor.buffer().line_count() > 1 {
                if let Some(_deleted_line) = editor.buffer().get_line(editor.cursor().row) {
                    TextOperations::delete_line_at(editor, editor.cursor().row);

                    if editor.cursor().row >= editor.buffer().line_count() {
                        editor.cursor_mut().row = editor.buffer().line_count().saturating_sub(1);
                    }

                    let line_len = editor.buffer().line_length(editor.cursor().row);
                    if editor.cursor().col >= line_len && line_len > 0 {
                        editor.cursor_mut().col = line_len - 1;
                    } else if line_len == 0 {
                        editor.cursor_mut().col = 0;
                    }
                }
            } else {
                if let Some(_line) = editor.buffer().get_line(0) {
                    TextOperations::clear_line_at(editor, 0);
                    editor.cursor_mut().col = 0;
                }
                break;
            }
        }

        editor.set_modified(true);
        editor.update_scroll();
    }

    /// Execute delete with motion command (dw, d$, etc.)
    pub fn execute_delete_motion(editor: &mut Editor, motion: MovementCommand, count: usize) {
        let start_pos = (editor.cursor().row, editor.cursor().col);
        let end_pos = motion.calculate_end_position(editor, start_pos, count);

        // Extract text for register before deleting
        let deleted_text = TextOperations::extract_range(editor, start_pos, end_pos);

        // Store in register
        if !deleted_text.is_empty() {
            if motion.is_line_motion() {
                editor.register.store_lines(deleted_text.clone());
            } else {
                editor.register.store_text(deleted_text.clone());
            }

            // Record the deletion for undo - use the correct position for reinsertion
            let (delete_row, delete_col) = if start_pos.0 < end_pos.0
                || (start_pos.0 == end_pos.0 && start_pos.1 <= end_pos.1)
            {
                // Forward motion (like dw, d$) - reinsert at start position
                start_pos
            } else {
                // Backward motion (like dgg) - reinsert at end position
                end_pos
            };
            let delete_pos = crate::buffer::Position::new(delete_row, delete_col);
            let action = crate::history::EditAction::delete_text(delete_pos, deleted_text);
            editor.history_mut().push(action);
        }

        let final_cursor_pos = match motion {
            MovementCommand::WordBackward
            | MovementCommand::LineStart
            | MovementCommand::LineFirstChar
            | MovementCommand::FileStart
            | MovementCommand::Left => end_pos,
            _ => start_pos,
        };

        TextOperations::delete_range(editor, start_pos, end_pos);

        editor.cursor_mut().row = final_cursor_pos.0;
        editor.cursor_mut().col = final_cursor_pos.1;

        TextOperations::clamp_cursor_to_buffer(editor);

        editor.set_modified(true);
        editor.update_scroll();
    }

    /// Execute yank line command (yy)
    pub fn execute_yank_line(editor: &mut Editor, count: usize) {
        let start_row = editor.cursor().row;
        let mut yanked_lines = Vec::new();

        for i in 0..count {
            let row = start_row + i;
            if row < editor.buffer().line_count() {
                if let Some(line) = editor.buffer().get_line(row) {
                    yanked_lines.push(line.clone());
                }
            }
        }

        if !yanked_lines.is_empty() {
            let yanked_text = yanked_lines.join("\n") + "\n";
            editor.register.store_lines(yanked_text);

            // Set status message
            let message = if count == 1 {
                "1 line yanked".to_string()
            } else {
                format!("{count} lines yanked")
            };
            editor.status_msg = Some(message);
        }
    }

    /// Execute yank with motion command (yw, y$, etc.)
    pub fn execute_yank_motion(editor: &mut Editor, motion: MovementCommand, count: usize) {
        let start_pos = (editor.cursor().row, editor.cursor().col);
        let end_pos = motion.calculate_end_position(editor, start_pos, count);

        // Extract text from the range
        let yanked_text = TextOperations::extract_range(editor, start_pos, end_pos);

        if !yanked_text.is_empty() {
            // Determine if this is a line-based motion
            let is_line_motion = motion.is_line_motion();

            if is_line_motion {
                editor.register.store_lines(yanked_text);
            } else {
                editor.register.store_text(yanked_text);
            }

            // Set status message
            let message = if is_line_motion {
                let line_count = editor.register.content.matches('\n').count();
                if line_count <= 1 {
                    "1 line yanked".to_string()
                } else {
                    format!("{line_count} lines yanked")
                }
            } else {
                let char_count = editor.register.content.len();
                if char_count == 1 {
                    "1 character yanked".to_string()
                } else {
                    format!("{char_count} characters yanked")
                }
            };
            editor.status_msg = Some(message);
        }
    }

    /// Execute paste after cursor (p)
    pub fn execute_paste_after(editor: &mut Editor) {
        if editor.register.is_empty() {
            editor.status_msg = Some("Nothing to paste".to_string());
            return;
        }

        let is_line_based = editor.register.is_line_based;
        let content = editor.register.content.clone();

        let paste_pos = if is_line_based {
            crate::buffer::Position::new(editor.cursor().row + 1, 0)
        } else {
            crate::buffer::Position::new(editor.cursor().row, editor.cursor().col + 1)
        };

        if is_line_based {
            // Insert lines after current line
            let lines: Vec<&str> = content.trim_end_matches('\n').split('\n').collect();

            // Record the line-based paste for undo
            let action = crate::history::EditAction::insert_text(paste_pos, content.clone());
            editor.history_mut().push(action);

            for (i, line) in lines.iter().enumerate() {
                let insert_row = editor.cursor().row + 1 + i;
                editor
                    .buffer_mut()
                    .insert_line(insert_row, line.to_string());
            }

            // Move cursor to the start of the first inserted line
            if !lines.is_empty() {
                editor.cursor_mut().row += 1;
                editor.cursor_mut().col = 0;

                // Move to first non-blank character
                let line_len = editor.buffer().line_length(editor.cursor().row);
                for col in 0..line_len {
                    if let Some(ch) = editor.buffer().get_char((editor.cursor().row, col)) {
                        if !ch.is_whitespace() {
                            editor.cursor_mut().col = col;
                            break;
                        }
                    }
                }
            }
        } else {
            // Insert text after cursor position
            let chars: Vec<char> = content.chars().collect();
            let mut insert_col = editor.cursor().col + 1;

            // Clamp insert position to line length
            let line_len = editor.buffer().line_length(editor.cursor().row);
            if insert_col > line_len {
                insert_col = line_len;
            }

            // Record the text paste for undo
            let final_pos = crate::buffer::Position::new(editor.cursor().row, insert_col);
            let action = crate::history::EditAction::insert_text(final_pos, content);
            editor.history_mut().push(action);

            for (i, ch) in chars.iter().enumerate() {
                let pos = crate::buffer::Position::new(editor.cursor().row, insert_col + i);
                editor.buffer_mut().insert_char(pos, *ch);
            }

            // Move cursor to end of pasted text
            editor.cursor_mut().col = insert_col + chars.len().saturating_sub(1);
        }

        editor.set_modified(true);
        TextOperations::clamp_cursor_to_buffer(editor);
        editor.update_scroll();
    }

    /// Execute paste before cursor (P)
    pub fn execute_paste_before(editor: &mut Editor) {
        if editor.register.is_empty() {
            editor.status_msg = Some("Nothing to paste".to_string());
            return;
        }

        let is_line_based = editor.register.is_line_based;
        let content = editor.register.content.clone();

        let paste_pos = if is_line_based {
            crate::buffer::Position::new(editor.cursor().row, 0)
        } else {
            crate::buffer::Position::new(editor.cursor().row, editor.cursor().col)
        };

        if is_line_based {
            // Insert lines before current line
            let lines: Vec<&str> = content.trim_end_matches('\n').split('\n').collect();

            // Record the line-based paste for undo
            let action = crate::history::EditAction::insert_text(paste_pos, content.clone());
            editor.history_mut().push(action);

            for (i, line) in lines.iter().enumerate() {
                let insert_row = editor.cursor().row + i;
                editor
                    .buffer_mut()
                    .insert_line(insert_row, line.to_string());
            }

            // Move cursor to the start of the first inserted line
            editor.cursor_mut().col = 0;

            // Move to first non-blank character
            let line_len = editor.buffer().line_length(editor.cursor().row);
            for col in 0..line_len {
                if let Some(ch) = editor.buffer().get_char((editor.cursor().row, col)) {
                    if !ch.is_whitespace() {
                        editor.cursor_mut().col = col;
                        break;
                    }
                }
            }
        } else {
            // Insert text at cursor position
            let chars: Vec<char> = content.chars().collect();

            // Record the text paste for undo
            let final_pos = crate::buffer::Position::new(editor.cursor().row, editor.cursor().col);
            let action = crate::history::EditAction::insert_text(final_pos, content);
            editor.history_mut().push(action);

            for (i, ch) in chars.iter().enumerate() {
                let pos =
                    crate::buffer::Position::new(editor.cursor().row, editor.cursor().col + i);
                editor.buffer_mut().insert_char(pos, *ch);
            }

            // Move cursor to end of pasted text
            editor.cursor_mut().col += chars.len().saturating_sub(1);
        }

        editor.set_modified(true);
        TextOperations::clamp_cursor_to_buffer(editor);
        editor.update_scroll();
    }
}
