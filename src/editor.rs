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
    
    /// Refresh the screen display (to be implemented)
    pub fn refresh_screen(&self) {
        // TODO: Clear screen, draw buffer content, position cursor
    }
    
    /// Process a key input based on current mode (to be implemented)
    pub fn process_key(&mut self, key: crate::input::Key) {
        // TODO: Handle key input based on current mode
    }
}
