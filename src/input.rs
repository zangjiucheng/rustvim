use std::io::{self, Read};

/// Represents different types of key inputs
#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    /// Regular character input
    Char(char),
    
    /// Control keys
    Ctrl(char),
    
    /// Special keys
    Esc,
    Enter,
    Backspace,
    Delete,
    Tab,
    
    /// Arrow keys
    Up,
    Down,
    Left,
    Right,
    
    /// Navigation keys
    Home,
    End,
    PageUp,
    PageDown,
    
    /// Function keys (F1-F12)
    Function(u8),
    
    /// Unknown/unsupported key sequence
    Unknown,
}

/// Input handler for reading and parsing keystrokes
pub struct InputHandler {
    stdin: std::io::Stdin,
}

impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            stdin: std::io::stdin(),
        }
    }
    
    /// Read a single key from input (to be implemented in Day 3)
    pub fn read_key(&mut self) -> io::Result<Key> {
        // TODO: Implement raw byte reading and key parsing
        // For now, return a placeholder
        let mut buffer = [0; 1];
        self.stdin.read_exact(&mut buffer)?;
        
        match buffer[0] {
            b'\x1b' => {
                // Escape sequence - need to read more bytes
                // TODO: Implement escape sequence parsing
                Ok(Key::Esc)
            }
            b'\r' | b'\n' => Ok(Key::Enter),
            b'\x7f' | b'\x08' => Ok(Key::Backspace),
            b'\t' => Ok(Key::Tab),
            0..=26 => {
                // Control characters (Ctrl+A = 1, Ctrl+B = 2, etc.)
                if buffer[0] == 0 {
                    Ok(Key::Unknown)
                } else {
                    Ok(Key::Ctrl((b'a' + buffer[0] - 1) as char))
                }
            }
            32..=126 => {
                // Printable ASCII
                Ok(Key::Char(buffer[0] as char))
            }
            _ => Ok(Key::Unknown),
        }
    }
    
    /// Parse escape sequences for special keys (to be implemented)
    fn parse_escape_sequence(&mut self) -> io::Result<Key> {
        // TODO: Read following bytes after ESC and parse sequences like:
        // ESC [ A = Up arrow
        // ESC [ B = Down arrow
        // ESC [ C = Right arrow
        // ESC [ D = Left arrow
        // etc.
        Ok(Key::Unknown)
    }
}

/// Helper functions for key classification
impl Key {
    /// Check if this is a printable character
    pub fn is_char(&self) -> bool {
        matches!(self, Key::Char(_))
    }
    
    /// Check if this is a control key
    pub fn is_ctrl(&self) -> bool {
        matches!(self, Key::Ctrl(_))
    }
    
    /// Check if this is an arrow key
    pub fn is_arrow(&self) -> bool {
        matches!(self, Key::Up | Key::Down | Key::Left | Key::Right)
    }
    
    /// Get the character if this is a Char key
    pub fn as_char(&self) -> Option<char> {
        if let Key::Char(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}
