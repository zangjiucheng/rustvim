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
    
    /// Read a single key from input with full escape sequence parsing
    pub fn read_key(&mut self) -> io::Result<Key> {
        let mut buffer = [0; 1];
        
        // Read exactly one byte (this will block until available)
        match self.stdin.read_exact(&mut buffer) {
            Ok(()) => {
                match buffer[0] {
                    // ESC key or start of escape sequence
                    b'\x1b' => {
                        // Try to parse escape sequence, but handle timeout gracefully
                        self.parse_escape_sequence()
                    }
                    // Enter key (both CR and LF)
                    b'\r' | b'\n' => Ok(Key::Enter),
                    // Backspace (both DEL and BS)
                    b'\x7f' | b'\x08' => Ok(Key::Backspace),
                    // Tab
                    b'\t' => Ok(Key::Tab),
                    // Control characters (Ctrl+A = 1, Ctrl+B = 2, etc.)
                    1..=26 => {
                        // Convert to the corresponding letter (1 = Ctrl+A, 2 = Ctrl+B, etc.)
                        let control_char = (b'a' + buffer[0] - 1) as char;
                        Ok(Key::Ctrl(control_char))
                    }
                    // Null character
                    0 => Ok(Key::Unknown),
                    // Printable ASCII characters (space to tilde)
                    32..=126 => Ok(Key::Char(buffer[0] as char)),
                    // Everything else is unknown
                    _ => Ok(Key::Unknown),
                }
            }
            Err(e) => Err(e),
        }
    }
    
    /// Parse escape sequences for special keys (arrows, home, end, etc.)
    fn parse_escape_sequence(&mut self) -> io::Result<Key> {
        // For Day 3, we'll assume escape sequences arrive together
        // In production, we'd want timeout handling to distinguish lone ESC
        let mut buffer = [0; 1];
        
        // Try to read the next byte after ESC
        match self.stdin.read_exact(&mut buffer) {
            Ok(()) => {
                match buffer[0] {
                    b'[' => {
                        // CSI (Control Sequence Introducer) - read the command byte
                        match self.stdin.read_exact(&mut buffer) {
                            Ok(()) => {
                                match buffer[0] {
                                    // Arrow keys - single letter after ESC[
                                    b'A' => Ok(Key::Up),
                                    b'B' => Ok(Key::Down),
                                    b'C' => Ok(Key::Right),
                                    b'D' => Ok(Key::Left),
                                    // Home and End (some terminals)
                                    b'H' => Ok(Key::Home),
                                    b'F' => Ok(Key::End),
                                    // Extended sequences (number followed by ~)
                                    b'1'..=b'6' => {
                                        let digit = buffer[0];
                                        // Try to read the terminating ~ character
                                        match self.stdin.read_exact(&mut buffer) {
                                            Ok(()) if buffer[0] == b'~' => {
                                                match digit {
                                                    b'1' => Ok(Key::Home),       // ESC[1~
                                                    b'3' => Ok(Key::Delete),     // ESC[3~
                                                    b'4' => Ok(Key::End),        // ESC[4~
                                                    b'5' => Ok(Key::PageUp),     // ESC[5~
                                                    b'6' => Ok(Key::PageDown),   // ESC[6~
                                                    _ => Ok(Key::Unknown),
                                                }
                                            }
                                            _ => Ok(Key::Unknown),
                                        }
                                    }
                                    _ => Ok(Key::Unknown),
                                }
                            }
                            Err(_) => Ok(Key::Esc), // Couldn't read more - lone ESC
                        }
                    }
                    b'O' => {
                        // SS3 (Single Shift 3) sequences
                        match self.stdin.read_exact(&mut buffer) {
                            Ok(()) => {
                                match buffer[0] {
                                    b'H' => Ok(Key::Home),       // ESC O H
                                    b'F' => Ok(Key::End),        // ESC O F
                                    b'P' => Ok(Key::Function(1)), // F1
                                    b'Q' => Ok(Key::Function(2)), // F2
                                    b'R' => Ok(Key::Function(3)), // F3
                                    b'S' => Ok(Key::Function(4)), // F4
                                    _ => Ok(Key::Unknown),
                                }
                            }
                            Err(_) => Ok(Key::Esc), // Couldn't read more - lone ESC
                        }
                    }
                    // Any other byte after ESC
                    _ => Ok(Key::Unknown),
                }
            }
            Err(_) => {
                // Couldn't read another byte after ESC - this was a lone ESC key
                Ok(Key::Esc)
            }
        }
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
