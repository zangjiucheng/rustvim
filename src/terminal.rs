use std::io::{self, Write};

/// Terminal interface for raw mode and screen control
pub struct Terminal {
    /// Terminal size (rows, cols)
    size: (usize, usize),
}

impl Terminal {
    /// Create a new terminal interface
    pub fn new() -> Self {
        Self {
            size: (24, 80), // Default size, will be updated when we implement size detection
        }
    }
    
    /// Enter raw mode (to be implemented in Day 2)
    pub fn enter_raw_mode(&self) -> io::Result<RawModeGuard> {
        // TODO: Implement raw mode using termios
        // For now, return a placeholder guard
        Ok(RawModeGuard::new())
    }
    
    /// Get terminal size
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
    
    /// Clear the entire screen
    pub fn clear_screen(&self) -> io::Result<()> {
        print!("\x1b[2J");
        io::stdout().flush()
    }
    
    /// Move cursor to specific position (1-based)
    pub fn move_cursor(&self, row: usize, col: usize) -> io::Result<()> {
        print!("\x1b[{};{}H", row, col);
        io::stdout().flush()
    }
    
    /// Move cursor to home position (top-left)
    pub fn move_cursor_home(&self) -> io::Result<()> {
        print!("\x1b[H");
        io::stdout().flush()
    }
    
    /// Hide the cursor
    pub fn hide_cursor(&self) -> io::Result<()> {
        print!("\x1b[?25l");
        io::stdout().flush()
    }
    
    /// Show the cursor
    pub fn show_cursor(&self) -> io::Result<()> {
        print!("\x1b[?25h");
        io::stdout().flush()
    }
    
    /// Write text to the terminal
    pub fn write(&self, text: &str) -> io::Result<()> {
        print!("{}", text);
        io::stdout().flush()
    }
    
    /// Write a line of text
    pub fn write_line(&self, text: &str) -> io::Result<()> {
        println!("{}", text);
        io::stdout().flush()
    }
}

/// RAII guard for raw mode - ensures terminal is restored on drop
pub struct RawModeGuard {
    // TODO: Store original terminal settings
}

impl RawModeGuard {
    fn new() -> Self {
        // TODO: Save current terminal settings and enable raw mode
        Self {}
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // TODO: Restore original terminal settings
        // This ensures the terminal is properly restored even if the program panics
    }
}
