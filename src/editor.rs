use crate::buffer::Buffer;
use crate::terminal::Terminal;
use crate::commands::{CommandProcessor, InsertModeProcessor};
use crate::history::History;

/// Represents the current mode of the editor
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
}

/// Represents a cursor position in the buffer
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
    
    pub fn at(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    
    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
}

/// Main editor state and controller
pub struct Editor {
    /// Current editing mode
    pub mode: Mode,
    
    /// Current cursor position
    pub cursor: Cursor,
    
    /// Reference to the active text buffer
    pub buffer: Buffer,
    
    /// Terminal interface
    pub terminal: Terminal,
    
    /// Whether the editor is running
    pub running: bool,
    
    /// Whether the buffer has been modified
    pub modified: bool,
    
    /// Current filename (if any)
    pub filename: Option<String>,
    
    /// Current status message
    pub status_msg: Option<String>,
    
    /// Scroll offset for viewport
    pub scroll_offset: usize,
    
    /// Pending count for commands (e.g., 5j to move down 5 lines)
    pub pending_count: Option<usize>,
    
    /// Pending operator for commands (e.g., d waiting for motion)
    pub pending_operator: Option<crate::commands::Operator>,
    
    /// Register to hold yanked/deleted text
    pub register: Register,
    
    /// History manager for undo/redo functionality
    pub history: History,
    
    /// Tracks changes during insert mode for grouping
    pub insert_mode_changes: Option<InsertModeGroup>,
}

/// Represents the content and type of yanked/deleted text
#[derive(Debug, Clone)]
pub struct Register {
    /// The stored text content
    pub content: String,
    /// Whether the content is line-based (whole lines) or character-based
    pub is_line_based: bool,
}

impl Register {
    /// Create a new empty register
    pub fn new() -> Self {
        Self {
            content: String::new(),
            is_line_based: false,
        }
    }
    
    /// Store text as character-based content
    pub fn store_text(&mut self, text: String) {
        self.content = text;
        self.is_line_based = false;
    }
    
    /// Store text as line-based content (whole lines)
    pub fn store_lines(&mut self, text: String) {
        self.content = text;
        self.is_line_based = true;
    }
    
    /// Check if the register is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl Editor {
    /// Create a new editor instance
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            cursor: Cursor::new(),
            buffer: Buffer::new(),
            terminal: Terminal::new(),
            running: true,
            modified: false,
            filename: None,
            status_msg: None,
            scroll_offset: 0,
            pending_count: None,
            pending_operator: None,
            register: Register::new(),
            history: History::new(),
            insert_mode_changes: None,
        }
    }
    
    /// Main editor event loop
    pub fn run(&mut self) -> std::io::Result<()> {
        // Enter raw mode
        let _raw_guard = self.terminal.enter_raw_mode()?;
        
        // Create input handler
        let mut input_handler = crate::input::InputHandler::new();
        
        // Initial screen refresh
        self.refresh_screen()?;
        
        // Main event loop
        while self.running {
            // Read key input
            let key = input_handler.read_key()?;
            
            // Handle global commands first
            match &key {
                // Quit commands
                crate::input::Key::Ctrl('q') => {
                    self.running = false;
                    continue;
                }
                crate::input::Key::Esc => {
                    match self.mode {
                        Mode::Insert => {
                            // Exit insert mode, move cursor left if possible
                            self.end_insert_mode();
                            self.mode = Mode::Normal;
                            if self.cursor.col > 0 {
                                self.cursor.col -= 1;
                            }
                        }
                        _ => {
                            // For now, ESC in normal mode quits (temporary)
                            self.running = false;
                        }
                    }
                    self.refresh_screen()?;
                    continue;
                }
                _ => {}
            }
            
            // Handle mode-specific commands
            match self.mode {
                Mode::Normal => {
                    CommandProcessor::handle_normal_mode_input(self, &key, &mut input_handler)?;
                }
                Mode::Insert => {
                    InsertModeProcessor::handle_input(self, &key);
                }
                Mode::Command => {
                    // TODO: Implement command mode handling
                }
                Mode::Visual => {
                    // TODO: Implement visual mode handling
                }
            }
            
            // Refresh screen after each key press
            self.refresh_screen()?;
        }
        
        Ok(())
    }
    
    /// Refresh the screen display with current buffer content
    pub fn refresh_screen(&self) -> std::io::Result<()> {
        // Hide cursor during redraw
        self.terminal.hide_cursor()?;
        
        // Move to home position
        self.terminal.move_cursor_home()?;
        
        // Draw buffer content
        self.draw_buffer()?;
        
        // Draw status line
        self.draw_status_line()?;
        
        // Position cursor at editor cursor position
        let screen_row = self.cursor.row.saturating_sub(self.scroll_offset) + 1;
        let screen_col = self.cursor.col + 1;
        self.terminal.move_cursor(screen_row, screen_col)?;
        
        // Show cursor
        self.terminal.show_cursor()?;
        
        Ok(())
    }
    
    /// Draw buffer content to screen
    fn draw_buffer(&self) -> std::io::Result<()> {
        let rows = self.terminal.rows();
        let cols = self.terminal.cols();
        
        // Reserve last row for status line
        let content_rows = rows.saturating_sub(1);
        
        for screen_row in 0..content_rows {
            let buffer_row = screen_row + self.scroll_offset;
            
            if buffer_row < self.buffer.line_count() {
                // Draw actual buffer line
                if let Some(line) = self.buffer.get_line(buffer_row) {
                    self.terminal.write_truncated(line, cols)?;
                }
            } else {
                // Draw tilde for empty lines (like Vim)
                self.terminal.write("~")?;
            }
            
            // Clear rest of line and move to next
            self.terminal.clear_line()?;
            if screen_row < content_rows - 1 {
                self.terminal.write("\r\n")?;
            }
        }
        
        Ok(())
    }
    
    /// Draw status line at bottom of screen
    fn draw_status_line(&self) -> std::io::Result<()> {
        let rows = self.terminal.rows();
        let cols = self.terminal.cols();
        
        // Move to status line (last row)
        self.terminal.move_cursor(rows, 1)?;
        
        // Create status line content
        let filename = self.filename.as_deref().unwrap_or("[No Name]");
        let modified = if self.modified { " [Modified]" } else { "" };
        let mode = format!("{:?}", self.mode);
        let position = format!("{}:{}", self.cursor.row + 1, self.cursor.col + 1);
        let lines = format!("{} lines", self.buffer.line_count());
        
        let left = format!("{}{} - {}", filename, modified, mode);
        let right = format!("{} - {}", position, lines);
        
        // Calculate spacing
        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let spacing = if left_len + right_len < cols {
            cols - left_len - right_len
        } else {
            0
        };
        
        // Write status line with background
        self.terminal.write_highlighted(&format!("{}{}{}", 
            left,
            " ".repeat(spacing),
            right
        ))?;
        
        Ok(())
    }
    
    /// Update scroll offset to keep cursor visible
    pub fn update_scroll(&mut self) {
        let rows = self.terminal.rows();
        let content_rows = rows.saturating_sub(1); // Reserve space for status line
        
        // Scroll up if cursor is above visible area
        if self.cursor.row < self.scroll_offset {
            self.scroll_offset = self.cursor.row;
        }
        
        // Scroll down if cursor is below visible area
        if self.cursor.row >= self.scroll_offset + content_rows {
            self.scroll_offset = self.cursor.row - content_rows + 1;
        }
    }
    
    /// Move cursor safely within buffer bounds
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        // Clamp row to buffer bounds
        self.cursor.row = row.min(self.buffer.line_count().saturating_sub(1));
        
        // Clamp column to line length
        let line_len = self.buffer.line_length(self.cursor.row);
        self.cursor.col = col.min(line_len);
    }
    
    /// Move cursor up one line
    pub fn cursor_up(&mut self) {
        if self.cursor.row > 0 {
            let new_row = self.cursor.row - 1;
            let line_len = self.buffer.line_length(new_row);
            self.cursor.row = new_row;
            // In normal mode, cursor should not go past the last character
            self.cursor.col = self.cursor.col.min(line_len.saturating_sub(1));
        }
    }
    
    /// Move cursor down one line
    pub fn cursor_down(&mut self) {
        if self.cursor.row + 1 < self.buffer.line_count() {
            let new_row = self.cursor.row + 1;
            let line_len = self.buffer.line_length(new_row);
            self.cursor.row = new_row;
            // In normal mode, cursor should not go past the last character
            self.cursor.col = self.cursor.col.min(line_len.saturating_sub(1));
        }
    }
    
    /// Move cursor left one position
    pub fn cursor_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        }
        // For now, don't wrap to previous line (keep it simple for Day 6)
    }
    
    /// Move cursor right one position
    pub fn cursor_right(&mut self) {
        let line_len = self.buffer.line_length(self.cursor.row);
        // In normal mode, don't go past the last character
        let max_col = line_len.saturating_sub(1);
        if self.cursor.col < max_col {
            self.cursor.col += 1;
        }
        // For now, don't wrap to next line (keep it simple for Day 6)
    }
    
    /// Get current cursor position as Position
    pub fn cursor_position(&self) -> crate::buffer::Position {
        crate::buffer::Position::new(self.cursor.row, self.cursor.col)
    }
    
    /// Load file into buffer
    pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        let content = std::fs::read_to_string(filename)?;
        self.buffer = Buffer::from_file(&content);
        self.filename = Some(filename.to_string());
        self.modified = false;
        // Reset cursor to top of file
        self.cursor = Cursor::new();
        // Clear history when loading a new file
        self.history.clear();
        Ok(())
    }
    
    /// Save buffer to file
    pub fn save_file(&mut self) -> std::io::Result<()> {
        if let Some(filename) = &self.filename {
            std::fs::write(filename, self.buffer.to_string())?;
            self.modified = false;
            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "No filename set"))
        }
    }
    
    /// Save buffer to specific file
    pub fn save_file_as(&mut self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.buffer.to_string())?;
        self.filename = Some(filename.to_string());
        self.modified = false;
        Ok(())
    }
    
    /// Perform undo operation
    pub fn undo(&mut self) {
        if let Some((_action, cursor_pos)) = self.history.apply_undo(&mut self.buffer) {
            self.cursor.row = cursor_pos.row;
            self.cursor.col = cursor_pos.col;
            self.modified = true;
            
            // Enhanced status message with undo count
            let remaining = self.history.undo_count();
            if remaining > 0 {
                self.status_msg = Some(format!("Undone ({} more available)", remaining));
            } else {
                self.status_msg = Some("Undone (oldest change)".to_string());
            }
        } else {
            self.status_msg = Some("Already at oldest change".to_string());
        }
        self.update_scroll();
    }
    
    /// Perform redo operation
    pub fn redo(&mut self) {
        if let Some((_action, cursor_pos)) = self.history.apply_redo(&mut self.buffer) {
            self.cursor.row = cursor_pos.row;
            self.cursor.col = cursor_pos.col;
            self.modified = true;
            
            // Enhanced status message with redo count
            let remaining = self.history.redo_count();
            if remaining > 0 {
                self.status_msg = Some(format!("Redone ({} more available)", remaining));
            } else {
                self.status_msg = Some("Redone (newest change)".to_string());
            }
        } else {
            self.status_msg = Some("Already at newest change".to_string());
        }
        self.update_scroll();
    }
    
    /// Start tracking insert mode changes
    pub fn start_insert_mode(&mut self) {
        let start_pos = crate::buffer::Position::new(self.cursor.row, self.cursor.col);
        self.insert_mode_changes = Some(InsertModeGroup::new(start_pos));
    }
    
    /// End tracking insert mode changes and record them as a single undo action
    pub fn end_insert_mode(&mut self) {
        if let Some(changes) = self.insert_mode_changes.take() {
            // Record insertions if any
            if !changes.inserted_text.is_empty() {
                let action = crate::history::EditAction::insert_text(
                    changes.start_pos,
                    changes.inserted_text
                );
                self.history.push(action);
            }
            
            // Record deletions if any (these happened during insert mode via backspace)
            if !changes.deleted_text.is_empty() {
                if let Some(deletion_pos) = changes.deletion_start_pos {
                    let action = crate::history::EditAction::delete_text(
                        deletion_pos,
                        changes.deleted_text
                    );
                    self.history.push(action);
                }
            }
        }
    }
    
    /// Record a character insertion during insert mode
    pub fn insert_mode_char(&mut self, ch: char) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            changes.add_char(ch);
        }
    }
    
    /// Record a newline insertion during insert mode
    pub fn insert_mode_newline(&mut self) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            changes.add_newline();
        }
    }
    
    /// Record a character deletion during insert mode (backspace)
    pub fn insert_mode_backspace(&mut self, deleted_char: Option<char>, deletion_pos: Option<crate::buffer::Position>) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            // Try to remove a character from recently inserted text first
            if !changes.inserted_text.is_empty() {
                changes.remove_char();
            } else if let (Some(ch), Some(pos)) = (deleted_char, deletion_pos) {
                // If we're not removing recent insertions, this is deleting existing buffer content
                changes.add_deleted_char(ch, pos);
            }
        }
    }
    
    /// Start tracking insert mode changes with initial text already inserted
    pub fn start_insert_mode_with_initial_text(&mut self, start_pos: crate::buffer::Position, initial_text: String) {
        let mut changes = InsertModeGroup::new(start_pos);
        changes.inserted_text = initial_text;
        self.insert_mode_changes = Some(changes);
    }
}

/// Tracks changes made during a single insert mode session
#[derive(Debug, Clone)]
pub struct InsertModeGroup {
    /// The starting position where insert mode began
    pub start_pos: crate::buffer::Position,
    /// The accumulated text that was inserted
    pub inserted_text: String,
    /// The text that was deleted during this insert session (for backspace on existing content)
    pub deleted_text: String,
    /// The position where deletions started (for backspace on existing content)
    pub deletion_start_pos: Option<crate::buffer::Position>,
}

impl InsertModeGroup {
    pub fn new(start_pos: crate::buffer::Position) -> Self {
        Self {
            start_pos,
            inserted_text: String::new(),
            deleted_text: String::new(),
            deletion_start_pos: None,
        }
    }
    
    pub fn add_char(&mut self, ch: char) {
        self.inserted_text.push(ch);
    }
    
    pub fn add_newline(&mut self) {
        self.inserted_text.push('\n');
    }
    
    pub fn remove_char(&mut self) {
        self.inserted_text.pop();
    }
    
    /// Record a character that was deleted from existing buffer content (not just undoing recent insertions)
    pub fn add_deleted_char(&mut self, ch: char, pos: crate::buffer::Position) {
        if self.deletion_start_pos.is_none() {
            self.deletion_start_pos = Some(pos);
        }
        self.deleted_text.insert(0, ch); // Insert at beginning to maintain order
    }
    
    /// Check if this insert session has any changes (insertions or deletions)
    pub fn has_changes(&self) -> bool {
        !self.inserted_text.is_empty() || !self.deleted_text.is_empty()
    }
}
