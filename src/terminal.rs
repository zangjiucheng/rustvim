use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

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
    
    /// Enter raw mode and return a guard for safe restoration
    pub fn enter_raw_mode(&self) -> io::Result<RawModeGuard> {
        RawModeGuard::new()
    }
    
    /// Get terminal size
    pub fn size(&self) -> (usize, usize) {
        self.size
    }
    
    /// Clear the entire screen
    pub fn clear_screen(&self) -> io::Result<()> {
        // Clear entire screen and move cursor to home
        print!("\x1b[2J\x1b[H");
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
    
    /// Write a line of text with proper line ending
    pub fn write_line(&self, text: &str) -> io::Result<()> {
        // In raw mode, we need to explicitly use \r\n for line endings
        print!("{}\r\n", text);
        io::stdout().flush()
    }
}

/// RAII guard for raw mode - ensures terminal is restored on drop
pub struct RawModeGuard {
    /// Original terminal attributes for restoration
    original_termios: libc::termios,
}

impl RawModeGuard {
    /// Create a new raw mode guard and enable raw mode
    pub fn new() -> io::Result<Self> {
        let stdin_fd = io::stdin().as_raw_fd();
        
        // Get current terminal attributes
        let mut original_termios = std::mem::MaybeUninit::<libc::termios>::uninit();
        let result = unsafe { libc::tcgetattr(stdin_fd, original_termios.as_mut_ptr()) };
        
        if result != 0 {
            return Err(io::Error::last_os_error());
        }
        
        let original_termios = unsafe { original_termios.assume_init() };
        
        // Create a copy to modify for raw mode
        let mut raw_termios = original_termios;
        
        // Disable canonical mode and echo (enter raw mode)
        // ECHO: Don't echo characters
        // ICANON: Disable canonical mode (process input char by char)
        // IEXTEN: Disable extended input processing (like Ctrl-V)
        // ISIG: Disable signal generation (Ctrl-C, Ctrl-Z)
        raw_termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        
        // Disable input processing
        // IXON: Disable Ctrl-S and Ctrl-Q flow control
        // ICRNL: Disable translation of carriage return to newline
        // BRKINT: Disable break interrupt
        // INPCK: Disable parity checking
        // ISTRIP: Disable stripping of 8th bit
        raw_termios.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
        
        // Disable output processing
        // OPOST: Disable output processing (like \n to \r\n translation)
        raw_termios.c_oflag &= !libc::OPOST;
        
        // Set character size to 8 bits per character
        raw_termios.c_cflag |= libc::CS8;
        
        // Set read timeout: return after one character or 100ms
        raw_termios.c_cc[libc::VMIN] = 0;  // Minimum number of characters
        raw_termios.c_cc[libc::VTIME] = 1; // Timeout in tenths of a second
        
        // Apply the new settings
        let result = unsafe { libc::tcsetattr(stdin_fd, libc::TCSAFLUSH, &raw_termios) };
        
        if result != 0 {
            return Err(io::Error::last_os_error());
        }
        
        Ok(Self { original_termios })
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Restore original terminal settings
        let stdin_fd = io::stdin().as_raw_fd();
        unsafe {
            libc::tcsetattr(stdin_fd, libc::TCSAFLUSH, &self.original_termios);
        }
        // Note: We ignore errors during restoration since we're in Drop
        // and can't return a Result. In a real implementation, we might
        // want to log errors or use a different error handling strategy.
    }
}
