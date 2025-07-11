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
    
    /// Main editor event loop (to be implemented)
    pub fn run(&mut self) {
        // TODO: Implement the main event loop
        // This will handle:
        // 1. Reading input
        // 2. Processing commands based on mode
        // 3. Updating the screen
        // 4. Managing state transitions
        println!("Editor event loop - to be implemented");
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
            self.cursor.col = self.cursor.col.min(line_len);
        }
    }
    
    /// Move cursor down one line
    pub fn cursor_down(&mut self) {
        if self.cursor.row + 1 < self.buffer.line_count() {
            let new_row = self.cursor.row + 1;
            let line_len = self.buffer.line_length(new_row);
            self.cursor.row = new_row;
            self.cursor.col = self.cursor.col.min(line_len);
        }
    }
    
    /// Move cursor left one position
    pub fn cursor_left(&mut self) {
        if self.cursor.col > 0 {
            self.cursor.col -= 1;
        } else if self.cursor.row > 0 {
            // Move to end of previous line
            self.cursor.row -= 1;
            self.cursor.col = self.buffer.line_length(self.cursor.row);
        }
    }
    
    /// Move cursor right one position
    pub fn cursor_right(&mut self) {
        let line_len = self.buffer.line_length(self.cursor.row);
        if self.cursor.col < line_len {
            self.cursor.col += 1;
        } else if self.cursor.row + 1 < self.buffer.line_count() {
            // Move to start of next line
            self.cursor.row += 1;
            self.cursor.col = 0;
        }
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
