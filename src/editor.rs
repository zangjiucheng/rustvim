use crate::buffer::Buffer;
use crate::terminal::Terminal;

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
            match input_handler.read_key()? {
                // Quit commands
                crate::input::Key::Ctrl('q') => {
                    self.running = false;
                }
                crate::input::Key::Esc => {
                    match self.mode {
                        Mode::Insert => {
                            // Exit insert mode, move cursor left if possible
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
                }
                
                // Navigation keys (hjkl and arrows) - only in Normal mode
                key if self.mode == Mode::Normal => {
                    match key {
                        // Insert mode entry
                        crate::input::Key::Char('i') => {
                            self.mode = Mode::Insert;
                        }
                        
                        // Vim-style navigation with arrow key support
                        crate::input::Key::Char('h') | crate::input::Key::Left => {
                            self.cursor_left();
                            self.update_scroll();
                        }
                        crate::input::Key::Char('j') | crate::input::Key::Down => {
                            self.cursor_down();
                            self.update_scroll();
                        }
                        crate::input::Key::Char('k') | crate::input::Key::Up => {
                            self.cursor_up();
                            self.update_scroll();
                        }
                        crate::input::Key::Char('l') | crate::input::Key::Right => {
                            self.cursor_right();
                            self.update_scroll();
                        }
                        
                        // Line navigation
                        crate::input::Key::Char('0') => {
                            self.cursor.col = 0;
                        }
                        crate::input::Key::Char('$') => {
                            let line_len = self.buffer.line_length(self.cursor.row);
                            // Move to last character, or stay at 0 if line is empty
                            self.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                        }
                        
                        // File navigation
                        crate::input::Key::Char('g') => {
                            // Wait for second 'g' to go to top of file
                            if let Ok(crate::input::Key::Char('g')) = input_handler.read_key() {
                                self.cursor.row = 0;
                                self.cursor.col = 0;
                                self.update_scroll();
                            }
                        }
                        crate::input::Key::Char('G') => {
                            // Go to end of file
                            self.cursor.row = self.buffer.line_count().saturating_sub(1);
                            let line_len = self.buffer.line_length(self.cursor.row);
                            // Move to last character, or stay at 0 if line is empty
                            self.cursor.col = if line_len > 0 { line_len - 1 } else { 0 };
                            self.update_scroll();
                        }
                        
                        _ => {
                            // Unhandled key in normal mode - ignore for now
                        }
                    }
                }
                
                // Insert mode input handling
                key if self.mode == Mode::Insert => {
                    match key {
                        // Regular character insertion
                        crate::input::Key::Char(c) => {
                            // Insert character at cursor position
                            self.buffer.insert_char(crate::buffer::Position::new(self.cursor.row, self.cursor.col), c);
                            // Move cursor forward
                            self.cursor.col += 1;
                            // Mark as modified
                            self.modified = true;
                        }
                        
                        // Enter key - split line
                        crate::input::Key::Enter => {
                            // Insert newline (split current line at cursor)
                            self.buffer.insert_newline(crate::buffer::Position::new(self.cursor.row, self.cursor.col));
                            // Move cursor to beginning of new line
                            self.cursor.row += 1;
                            self.cursor.col = 0;
                            // Mark as modified
                            self.modified = true;
                            // Update scroll if needed
                            self.update_scroll();
                        }
                        
                        // Backspace - delete character to the left
                        crate::input::Key::Backspace => {
                            if self.cursor.col > 0 {
                                // Delete character to the left in current line
                                self.cursor.col -= 1;
                                self.buffer.delete_char(crate::buffer::Position::new(self.cursor.row, self.cursor.col));
                                self.modified = true;
                            } else if self.cursor.row > 0 {
                                // At beginning of line - join with previous line
                                // Move cursor to end of previous line
                                self.cursor.row -= 1;
                                self.cursor.col = self.buffer.line_length(self.cursor.row);
                                
                                // Delete the newline (which will merge the lines)
                                self.buffer.delete_char(crate::buffer::Position::new(self.cursor.row, self.cursor.col));
                                
                                self.modified = true;
                                self.update_scroll();
                            }
                        }
                        
                        // Arrow keys in insert mode (for navigation without leaving insert)
                        crate::input::Key::Left => {
                            self.cursor_left();
                        }
                        crate::input::Key::Right => {
                            self.cursor_right();
                        }
                        crate::input::Key::Up => {
                            self.cursor_up();
                            self.update_scroll();
                        }
                        crate::input::Key::Down => {
                            self.cursor_down();
                            self.update_scroll();
                        }
                        
                        _ => {
                            // Unhandled key in insert mode - ignore
                        }
                    }
                }
                
                _ => {
                    // Unhandled key - ignore for now
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
}
