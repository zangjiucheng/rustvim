#![allow(unused)]

use crate::editor::{Editor, Mode};
use crate::input::Key;

/// Represents an operator waiting for a motion
#[derive(Debug, Clone)]
pub enum Operator {
    Delete,    // d
    Yank,      // y
    Change,    // c (future)
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
            NormalCommand::Edit(edit) => {
                edit.execute(editor)
            }
            NormalCommand::ModeSwitch(mode_switch) => {
                mode_switch.execute(editor)
            }
            NormalCommand::File(file) => {
                file.execute(editor)
            }
        }
    }
}

/// Trait for executing commands
pub trait Command {
    fn execute(&self, editor: &mut Editor) -> Result<(), String>;
}

/// Trait for motion calculations
pub trait Motion {
    fn calculate_end_position(&self, editor: &Editor, start: (usize, usize), count: usize) -> (usize, usize);
    fn is_line_motion(&self) -> bool { false }
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

impl Motion for MovementCommand {
    fn calculate_end_position(&self, editor: &Editor, start: (usize, usize), count: usize) -> (usize, usize) {
        let mut temp_row = start.0;
        let mut temp_col = start.1;
        
        for _ in 0..count {
            match self {
                MovementCommand::Left => {
                    if temp_col > 0 {
                        temp_col -= 1;
                    }
                }
                MovementCommand::Right => {
                    let line_len = editor.buffer.line_length(temp_row);
                    if temp_col < line_len.saturating_sub(1) {
                        temp_col += 1;
                    }
                }
                MovementCommand::Up => {
                    if temp_row > 0 {
                        temp_row -= 1;
                        let line_len = editor.buffer.line_length(temp_row);
                        temp_col = temp_col.min(line_len.saturating_sub(1));
                    }
                }
                MovementCommand::Down => {
                    if temp_row + 1 < editor.buffer.line_count() {
                        temp_row += 1;
                        let line_len = editor.buffer.line_length(temp_row);
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
                    if let Some(line) = editor.buffer.get_line(temp_row) {
                        temp_col = line.chars()
                            .position(|c| !c.is_whitespace())
                            .unwrap_or(0);
                    }
                    break;
                }
                MovementCommand::LineStart => {
                    temp_col = 0;
                    break;
                }
                MovementCommand::LineEnd => {
                    let line_len = editor.buffer.line_length(temp_row);
                    temp_col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::FileStart => {
                    temp_row = 0;
                    temp_col = 0;
                    break;
                }
                MovementCommand::FileEnd => {
                    temp_row = editor.buffer.line_count().saturating_sub(1);
                    let line_len = editor.buffer.line_length(temp_row);
                    temp_col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                _ => break,
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
    
    /// Search commands
    SearchNext,       // n
    SearchPrevious,   // N
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
        }
    }
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

impl Command for ModeSwitchCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        match self {
            ModeSwitchCommand::InsertBefore => {
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertAfter => {
                let line_len = editor.buffer.line_length(editor.cursor.row);
                if editor.cursor.col < line_len {
                    editor.cursor.col += 1;
                }
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineEnd => {
                let line_len = editor.buffer.line_length(editor.cursor.row);
                editor.cursor.col = line_len;
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertLineStart => {
                editor.cursor.col = 0;
                editor.start_insert_mode();
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineBelow => {
                let pos = crate::buffer::Position::new(editor.cursor.row, editor.buffer.line_length(editor.cursor.row));
                editor.buffer.insert_newline(pos);
                editor.cursor.row += 1;
                editor.cursor.col = 0;
                editor.modified = true;
                editor.update_scroll();
                // Start insert mode with the newline already included
                editor.start_insert_mode_with_initial_text(pos, "\n".to_string());
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::OpenLineAbove => {
                let pos = crate::buffer::Position::new(editor.cursor.row, 0);
                editor.buffer.insert_newline(pos);
                editor.cursor.col = 0;
                editor.modified = true;
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
        }
        Ok(())
    }
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

impl Command for FileCommand {
    fn execute(&self, editor: &mut Editor) -> Result<(), String> {
        // TODO: Implement file operations
        Ok(())
    }
}

/// Motion calculation utilities
pub struct MotionCalculator;

impl MotionCalculator {
    /// Helper function to check if a character is a word character
    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }
    
    /// Move cursor to the beginning of the next word (w command)
    pub fn word_forward(editor: &Editor, row: &mut usize, col: &mut usize) {
        loop {
            if let Some(line) = editor.buffer.get_line(*row) {
                let chars: Vec<char> = line.chars().collect();
                
                if *col >= chars.len() {
                    if *row + 1 < editor.buffer.line_count() {
                        *row += 1;
                        *col = 0;
                        
                        if let Some(next_line) = editor.buffer.get_line(*row) {
                            let next_chars: Vec<char> = next_line.chars().collect();
                            
                            if next_chars.is_empty() {
                                break;
                            }
                            
                            while *col < next_chars.len() && next_chars[*col].is_whitespace() {
                                *col += 1;
                            }
                            
                            if *col < next_chars.len() {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    *col += 1;
                    
                    while *col < chars.len() && Self::is_word_char(chars[*col]) {
                        *col += 1;
                    }
                    
                    while *col < chars.len() && !Self::is_word_char(chars[*col]) && !chars[*col].is_whitespace() {
                        *col += 1;
                    }
                    
                    while *col < chars.len() && chars[*col].is_whitespace() {
                        *col += 1;
                    }
                    
                    if *col < chars.len() && Self::is_word_char(chars[*col]) {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
    
    /// Move cursor to the beginning of the previous word (b command)
    pub fn word_backward(editor: &Editor, row: &mut usize, col: &mut usize) {
        loop {
            if let Some(line) = editor.buffer.get_line(*row) {
                let chars: Vec<char> = line.chars().collect();
                
                if *col == 0 {
                    if *row > 0 {
                        *row -= 1;
                        if let Some(prev_line) = editor.buffer.get_line(*row) {
                            let prev_chars: Vec<char> = prev_line.chars().collect();
                            
                            if prev_chars.is_empty() {
                                *col = 0;
                                break;
                            }
                            
                            *col = prev_chars.len();
                        }
                    } else {
                        break;
                    }
                } else {
                    *col -= 1;
                    
                    while *col > 0 && (chars[*col].is_whitespace() || (!Self::is_word_char(chars[*col]) && !chars[*col].is_whitespace())) {
                        *col -= 1;
                    }
                    
                    if *col == 0 && (chars[*col].is_whitespace() || (!Self::is_word_char(chars[*col]) && !chars[*col].is_whitespace())) {
                        if *row > 0 {
                            *row -= 1;
                            if let Some(prev_line) = editor.buffer.get_line(*row) {
                                let prev_chars: Vec<char> = prev_line.chars().collect();
                                
                                if prev_chars.is_empty() {
                                    *col = 0;
                                    break;
                                }
                                
                                *col = prev_chars.len();
                                continue;
                            }
                        }
                        break;
                    }
                    
                    while *col > 0 && Self::is_word_char(chars[*col - 1]) {
                        *col -= 1;
                    }
                    
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    /// Move cursor to the end of the current/next word (e command)
    pub fn word_end(editor: &Editor, row: &mut usize, col: &mut usize) {
        if let Some(line) = editor.buffer.get_line(*row) {
            let chars: Vec<char> = line.chars().collect();
            
            if chars.is_empty() {
                while *row + 1 < editor.buffer.line_count() {
                    *row += 1;
                    if let Some(next_line) = editor.buffer.get_line(*row) {
                        let next_chars: Vec<char> = next_line.chars().collect();
                        if !next_chars.is_empty() {
                            *col = 0;
                            while *col < next_chars.len() && !Self::is_word_char(next_chars[*col]) {
                                *col += 1;
                            }
                            if *col < next_chars.len() {
                                while *col < next_chars.len() && Self::is_word_char(next_chars[*col]) {
                                    *col += 1;
                                }
                                *col = col.saturating_sub(1);
                            }
                            break;
                        }
                    }
                }
                return;
            }
            
            if *col >= chars.len() {
                if *row + 1 < editor.buffer.line_count() {
                    *row += 1;
                    *col = 0;
                }
            } else {
                if Self::is_word_char(chars[*col]) {
                    while *col < chars.len() && Self::is_word_char(chars[*col]) {
                        *col += 1;
                    }
                    *col = col.saturating_sub(1);
                } else {
                    while *col < chars.len() && !Self::is_word_char(chars[*col]) {
                        *col += 1;
                    }
                    
                    if *col < chars.len() {
                        while *col < chars.len() && Self::is_word_char(chars[*col]) {
                            *col += 1;
                        }
                        *col = col.saturating_sub(1);
                    } else if *row + 1 < editor.buffer.line_count() {
                        *row += 1;
                        *col = 0;
                        if let Some(next_line) = editor.buffer.get_line(*row) {
                            let next_chars: Vec<char> = next_line.chars().collect();
                            
                            if next_chars.is_empty() {
                                return;
                            }
                            
                            while *col < next_chars.len() && next_chars[*col].is_whitespace() {
                                *col += 1;
                            }
                            while *col < next_chars.len() && Self::is_word_char(next_chars[*col]) {
                                *col += 1;
                            }
                            *col = col.saturating_sub(1);
                        }
                    }
                }
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
        if let Some(deleted_char) = editor.buffer.delete_char(cursor_pos) {
            // Record the deletion for undo
            let action = crate::history::EditAction::delete_text(cursor_pos, deleted_char.to_string());
            editor.history.push(action);
            
            // Store the deleted character in the register
            editor.register.store_text(deleted_char.to_string());
            
            editor.modified = true;
            let line_len = editor.buffer.line_length(editor.cursor.row);
            if editor.cursor.col >= line_len && line_len > 0 {
                editor.cursor.col = line_len - 1;
            } else if line_len == 0 && editor.cursor.col > 0 {
                editor.cursor.col = 0;
            }
            editor.update_scroll();
        }
    }
    
    /// Delete text in a range from start to end position
    pub fn delete_range(editor: &mut Editor, start: (usize, usize), end: (usize, usize)) {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
        
        let (from_row, from_col, to_row, to_col) = if start_row < end_row || 
            (start_row == end_row && start_col <= end_col) {
            (start_row, start_col, end_row, end_col)
        } else {
            (end_row, end_col, start_row, start_col)
        };
        
        if from_row == to_row {
            if from_col < to_col {
                for _ in from_col..to_col {
                    if from_col < editor.buffer.line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, from_col);
                        editor.buffer.delete_char(pos);
                    }
                }
            } else if from_col > to_col {
                for _ in to_col..from_col {
                    if to_col < editor.buffer.line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, to_col);
                        editor.buffer.delete_char(pos);
                    }
                }
            }
        } else {
            if from_row < to_row {
                let start_line_len = editor.buffer.line_length(from_row);
                for _ in from_col..start_line_len {
                    if from_col < editor.buffer.line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, from_col);
                        editor.buffer.delete_char(pos);
                    }
                }
                
                for row in (from_row + 1..to_row).rev() {
                    Self::delete_line_at(editor, row);
                }
                
                let adjusted_end_row = from_row + 1;
                if adjusted_end_row < editor.buffer.line_count() {
                    for _ in 0..to_col {
                        if 0 < editor.buffer.line_length(adjusted_end_row) {
                            let pos = crate::buffer::Position::new(adjusted_end_row, 0);
                            editor.buffer.delete_char(pos);
                        }
                    }
                    
                    if let Some(remaining_text) = editor.buffer.get_line(adjusted_end_row) {
                        let remaining = remaining_text.clone();
                        Self::delete_line_at(editor, adjusted_end_row);
                        
                        for ch in remaining.chars() {
                            let pos = crate::buffer::Position::new(from_row, editor.buffer.line_length(from_row));
                            editor.buffer.insert_char(pos, ch);
                        }
                    }
                }
            } else {
                for _ in 0..from_col {
                    if 0 < editor.buffer.line_length(from_row) {
                        let pos = crate::buffer::Position::new(from_row, 0);
                        editor.buffer.delete_char(pos);
                    }
                }
                
                for row in (to_row..from_row).rev() {
                    Self::delete_line_at(editor, row);
                }
            }
        }
    }
    
    /// Delete a line at specific row
    pub fn delete_line_at(editor: &mut Editor, row: usize) {
        let line_len = editor.buffer.line_length(row);
        for _ in 0..line_len {
            let pos = crate::buffer::Position::new(row, 0);
            editor.buffer.delete_char(pos);
        }
        
        if row < editor.buffer.line_count() - 1 {
            let pos = crate::buffer::Position::new(row, 0);
            if editor.buffer.line_length(row) == 0 {
                editor.buffer.delete_char(pos);
            }
        }
    }
    
    /// Clear a line at specific row
    pub fn clear_line_at(editor: &mut Editor, row: usize) {
        let line_len = editor.buffer.line_length(row);
        for _ in 0..line_len {
            let pos = crate::buffer::Position::new(row, 0);
            editor.buffer.delete_char(pos);
        }
    }
    
    /// Extract text from a range without modifying the buffer (for yank operations)
    pub fn extract_range(editor: &Editor, start: (usize, usize), end: (usize, usize)) -> String {
        editor.buffer.extract_range(start, end)
    }
    
    /// Ensure cursor is within valid buffer bounds
    pub fn clamp_cursor_to_buffer(editor: &mut Editor) {
        editor.cursor.row = editor.cursor.row.min(editor.buffer.line_count().saturating_sub(1));
        
        let line_len = editor.buffer.line_length(editor.cursor.row);
        if line_len > 0 {
            editor.cursor.col = editor.cursor.col.min(line_len - 1);
        } else {
            editor.cursor.col = 0;
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
                    editor.cursor.col = 0;
                    break;
                }
                MovementCommand::LineFirstChar => {
                    let line = if let Some(line) = editor.buffer.get_line(editor.cursor.row) {
                        line
                    } else {
                        break;
                    };
                    
                    let first_non_blank = line.chars()
                        .position(|c| !c.is_whitespace())
                        .unwrap_or(0);
                    editor.cursor.col = first_non_blank;
                    break;
                }
                MovementCommand::LineEnd => {
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::FileStart => {
                    editor.cursor.row = 0;
                    editor.cursor.col = 0;
                    break;
                }
                MovementCommand::FileEnd => {
                    editor.cursor.row = editor.buffer.line_count().saturating_sub(1);
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    editor.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                    break;
                }
                MovementCommand::WordForward => {
                    let mut row = editor.cursor.row;
                    let mut col = editor.cursor.col;
                    MotionCalculator::word_forward(editor, &mut row, &mut col);
                    editor.cursor.row = row;
                    editor.cursor.col = col;
                }
                MovementCommand::WordBackward => {
                    let mut row = editor.cursor.row;
                    let mut col = editor.cursor.col;
                    MotionCalculator::word_backward(editor, &mut row, &mut col);
                    editor.cursor.row = row;
                    editor.cursor.col = col;
                }
                MovementCommand::WordEnd => {
                    let mut row = editor.cursor.row;
                    let mut col = editor.cursor.col;
                    MotionCalculator::word_end(editor, &mut row, &mut col);
                    editor.cursor.row = row;
                    editor.cursor.col = col;
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
        let start_row = editor.cursor.row;
        let mut deleted_lines = Vec::new();
        
        // First, collect the lines to be deleted for the register
        for i in 0..count {
            let row = start_row + i;
            if row < editor.buffer.line_count() {
                if let Some(line) = editor.buffer.get_line(row) {
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
            editor.history.push(action);
        }
        
        // Now delete the lines
        for _ in 0..count {
            if editor.buffer.line_count() > 1 {
                if let Some(_deleted_line) = editor.buffer.get_line(editor.cursor.row) {
                    TextOperations::delete_line_at(editor, editor.cursor.row);
                    
                    if editor.cursor.row >= editor.buffer.line_count() {
                        editor.cursor.row = editor.buffer.line_count().saturating_sub(1);
                    }
                    
                    let line_len = editor.buffer.line_length(editor.cursor.row);
                    if editor.cursor.col >= line_len && line_len > 0 {
                        editor.cursor.col = line_len - 1;
                    } else if line_len == 0 {
                        editor.cursor.col = 0;
                    }
                }
            } else {
                if let Some(_line) = editor.buffer.get_line(0) {
                    TextOperations::clear_line_at(editor, 0);
                    editor.cursor.col = 0;
                }
                break;
            }
        }
        
        editor.modified = true;
        editor.update_scroll();
    }
    
    /// Execute delete with motion command (dw, d$, etc.)
    pub fn execute_delete_motion(editor: &mut Editor, motion: MovementCommand, count: usize) {
        let start_pos = (editor.cursor.row, editor.cursor.col);
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
            let (delete_row, delete_col) = if start_pos.0 < end_pos.0 || 
                (start_pos.0 == end_pos.0 && start_pos.1 <= end_pos.1) {
                // Forward motion (like dw, d$) - reinsert at start position
                start_pos
            } else {
                // Backward motion (like dgg) - reinsert at end position  
                end_pos
            };
            let delete_pos = crate::buffer::Position::new(delete_row, delete_col);
            let action = crate::history::EditAction::delete_text(delete_pos, deleted_text);
            editor.history.push(action);
        }
        
        let final_cursor_pos = match motion {
            MovementCommand::WordBackward | 
            MovementCommand::LineStart | 
            MovementCommand::LineFirstChar |
            MovementCommand::FileStart |
            MovementCommand::Left => end_pos,
            _ => start_pos,
        };
        
        TextOperations::delete_range(editor, start_pos, end_pos);
        
        editor.cursor.row = final_cursor_pos.0;
        editor.cursor.col = final_cursor_pos.1;
        
        TextOperations::clamp_cursor_to_buffer(editor);
        
        editor.modified = true;
        editor.update_scroll();
    }
    
    /// Execute yank line command (yy)
    pub fn execute_yank_line(editor: &mut Editor, count: usize) {
        let start_row = editor.cursor.row;
        let mut yanked_lines = Vec::new();
        
        for i in 0..count {
            let row = start_row + i;
            if row < editor.buffer.line_count() {
                if let Some(line) = editor.buffer.get_line(row) {
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
                format!("{} lines yanked", count)
            };
            editor.status_msg = Some(message);
        }
    }
    
    /// Execute yank with motion command (yw, y$, etc.)
    pub fn execute_yank_motion(editor: &mut Editor, motion: MovementCommand, count: usize) {
        let start_pos = (editor.cursor.row, editor.cursor.col);
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
                    format!("{} lines yanked", line_count)
                }
            } else {
                let char_count = editor.register.content.len();
                if char_count == 1 {
                    "1 character yanked".to_string()
                } else {
                    format!("{} characters yanked", char_count)
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
        
        let paste_pos = if editor.register.is_line_based {
            crate::buffer::Position::new(editor.cursor.row + 1, 0)
        } else {
            crate::buffer::Position::new(editor.cursor.row, editor.cursor.col + 1)
        };
        
        if editor.register.is_line_based {
            // Insert lines after current line
            let lines: Vec<&str> = editor.register.content.trim_end_matches('\n').split('\n').collect();
            
            // Record the line-based paste for undo
            let action = crate::history::EditAction::insert_text(paste_pos, editor.register.content.clone());
            editor.history.push(action);
            
            for (i, line) in lines.iter().enumerate() {
                let insert_row = editor.cursor.row + 1 + i;
                editor.buffer.insert_line(insert_row, line.to_string());
            }
            
            // Move cursor to the start of the first inserted line
            if lines.len() > 0 {
                editor.cursor.row += 1;
                editor.cursor.col = 0;
                
                // Move to first non-blank character
                let line_len = editor.buffer.line_length(editor.cursor.row);
                for col in 0..line_len {
                    if let Some(ch) = editor.buffer.get_char((editor.cursor.row, col)) {
                        if !ch.is_whitespace() {
                            editor.cursor.col = col;
                            break;
                        }
                    }
                }
            }
        } else {
            // Insert text after cursor position
            let chars: Vec<char> = editor.register.content.chars().collect();
            let mut insert_col = editor.cursor.col + 1;
            
            // Clamp insert position to line length
            let line_len = editor.buffer.line_length(editor.cursor.row);
            if insert_col > line_len {
                insert_col = line_len;
            }
            
            // Record the text paste for undo
            let final_pos = crate::buffer::Position::new(editor.cursor.row, insert_col);
            let action = crate::history::EditAction::insert_text(final_pos, editor.register.content.clone());
            editor.history.push(action);
            
            for (i, ch) in chars.iter().enumerate() {
                let pos = crate::buffer::Position::new(editor.cursor.row, insert_col + i);
                editor.buffer.insert_char(pos, *ch);
            }
            
            // Move cursor to end of pasted text
            editor.cursor.col = insert_col + chars.len().saturating_sub(1);
        }
        
        editor.modified = true;
        TextOperations::clamp_cursor_to_buffer(editor);
        editor.update_scroll();
    }
    
    /// Execute paste before cursor (P)
    pub fn execute_paste_before(editor: &mut Editor) {
        if editor.register.is_empty() {
            editor.status_msg = Some("Nothing to paste".to_string());
            return;
        }
        
        let paste_pos = if editor.register.is_line_based {
            crate::buffer::Position::new(editor.cursor.row, 0)
        } else {
            crate::buffer::Position::new(editor.cursor.row, editor.cursor.col)
        };
        
        if editor.register.is_line_based {
            // Insert lines before current line
            let lines: Vec<&str> = editor.register.content.trim_end_matches('\n').split('\n').collect();
            
            // Record the line-based paste for undo
            let action = crate::history::EditAction::insert_text(paste_pos, editor.register.content.clone());
            editor.history.push(action);
            
            for (i, line) in lines.iter().enumerate() {
                let insert_row = editor.cursor.row + i;
                editor.buffer.insert_line(insert_row, line.to_string());
            }
            
            // Move cursor to the start of the first inserted line
            editor.cursor.col = 0;
            
            // Move to first non-blank character
            let line_len = editor.buffer.line_length(editor.cursor.row);
            for col in 0..line_len {
                if let Some(ch) = editor.buffer.get_char((editor.cursor.row, col)) {
                    if !ch.is_whitespace() {
                        editor.cursor.col = col;
                        break;
                    }
                }
            }
        } else {
            // Insert text at cursor position
            let chars: Vec<char> = editor.register.content.chars().collect();
            
            // Record the text paste for undo
            let action = crate::history::EditAction::insert_text(paste_pos, editor.register.content.clone());
            editor.history.push(action);
            
            for (i, ch) in chars.iter().enumerate() {
                let pos = crate::buffer::Position::new(editor.cursor.row, editor.cursor.col + i);
                editor.buffer.insert_char(pos, *ch);
            }
            
            // Move cursor to end of pasted text
            editor.cursor.col += chars.len().saturating_sub(1);
        }
        
        editor.modified = true;
        TextOperations::clamp_cursor_to_buffer(editor);
        editor.update_scroll();
    }
}

/// Command parser and executor
pub struct CommandProcessor;

impl CommandProcessor {
    /// Handle Normal mode key input - moved from editor.rs for better separation of concerns
    pub fn handle_normal_mode_input(
        editor: &mut Editor, 
        key: &crate::input::Key, 
        input_handler: &mut crate::input::InputHandler
    ) -> std::io::Result<()> {
        // Handle digit inputs for count accumulation
        if let crate::input::Key::Char(c) = key {
            if c.is_ascii_digit() && (*c != '0' || editor.pending_count.is_some()) {
                let digit = c.to_digit(10).unwrap() as usize;
                editor.pending_count = Some(editor.pending_count.unwrap_or(0) * 10 + digit);
                return Ok(());
            }
        }
        
        // Check if we have a pending operator (operator-pending mode)
        if let Some(operator) = &editor.pending_operator {
            let operator = operator.clone();
            editor.pending_operator = None;
            
            match key {
                crate::input::Key::Char('d') if matches!(operator, Operator::Delete) => {
                    let count = editor.pending_count.unwrap_or(1);
                    OperatorExecutor::execute_delete_line(editor, count);
                }
                crate::input::Key::Char('y') if matches!(operator, Operator::Yank) => {
                    let count = editor.pending_count.unwrap_or(1);
                    OperatorExecutor::execute_yank_line(editor, count);
                }
                crate::input::Key::Char('g') => {
                    if let Ok(crate::input::Key::Char('g')) = input_handler.read_key() {
                        match operator {
                            Operator::Delete => {
                                let count = editor.pending_count.unwrap_or(1);
                                OperatorExecutor::execute_delete_motion(editor, MovementCommand::FileStart, count);
                            }
                            Operator::Yank => {
                                let count = editor.pending_count.unwrap_or(1);
                                OperatorExecutor::execute_yank_motion(editor, MovementCommand::FileStart, count);
                            }
                            _ => {} // TODO: Implement other operators
                        }
                    }
                }
                _ => {
                    if let Some(NormalCommand::Movement(motion)) = 
                        Self::parse_normal_command(key, editor.pending_count) {
                        match operator {
                            Operator::Delete => {
                                let count = editor.pending_count.unwrap_or(1);
                                OperatorExecutor::execute_delete_motion(editor, motion, count);
                            }
                            Operator::Yank => {
                                let count = editor.pending_count.unwrap_or(1);
                                OperatorExecutor::execute_yank_motion(editor, motion, count);
                            }
                            _ => {} // TODO: Implement other operators
                        }
                    }
                }
            }
            editor.pending_count = None;
            return Ok(());
        }
        
        // Handle operator keys
        match key {
            crate::input::Key::Char('d') => {
                editor.pending_operator = Some(Operator::Delete);
                return Ok(());
            }
            crate::input::Key::Char('y') => {
                editor.pending_operator = Some(Operator::Yank);
                return Ok(());
            }
            _ => {}
        }
        
        // Handle multi-key commands
        if let Some(command) = Self::parse_multi_key_command(key, input_handler) {
            if let Err(e) = command.execute(editor) {
                eprintln!("Command execution failed: {}", e);
            }
            editor.pending_count = None;
            return Ok(());
        }
        
        // Handle single-key commands
        if let Some(command) = Self::parse_normal_command(key, editor.pending_count) {
            if let Err(e) = command.execute(editor) {
                eprintln!("Command execution failed: {}", e);
            }
            editor.pending_count = None;
        }
        
        Ok(())
    }

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
            
            // Search repeat commands
            Key::Char('n') => Some(NormalCommand::Edit(EditCommand::SearchNext)),
            Key::Char('N') => Some(NormalCommand::Edit(EditCommand::SearchPrevious)),
            
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
                editor.insert_mode_char(*c);
                editor.cursor.col += 1;
                editor.modified = true;
            }
            
            // Enter key - split line
            Key::Enter => {
                editor.buffer.insert_newline(
                    crate::buffer::Position::new(editor.cursor.row, editor.cursor.col)
                );
                editor.insert_mode_newline();
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
                    let pos = crate::buffer::Position::new(editor.cursor.row, editor.cursor.col);
                    let deleted_char = editor.buffer.delete_char(pos);
                    editor.insert_mode_backspace(deleted_char, Some(pos));
                    editor.modified = true;
                } else if editor.cursor.row > 0 {
                    // At beginning of line - join with previous line
                    editor.cursor.row -= 1;
                    editor.cursor.col = editor.buffer.line_length(editor.cursor.row);
                    
                    // Delete the newline (which will merge the lines)
                    let pos = crate::buffer::Position::new(editor.cursor.row, editor.cursor.col);
                    let deleted_char = editor.buffer.delete_char(pos);
                    
                    editor.insert_mode_backspace(deleted_char, Some(pos));
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
