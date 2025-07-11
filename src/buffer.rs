/// Represents a position in the buffer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

/// Text buffer for storing and editing text content
pub struct Buffer {
    /// Lines of text (each String represents one line)
    lines: Vec<String>,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()], // Start with one empty line
        }
    }
    
    /// Create a buffer from file content
    pub fn from_file(content: &str) -> Self {
        let lines: Vec<String> = content
            .lines()
            .map(|line| line.to_string())
            .collect();
            
        // Ensure at least one line exists
        let lines = if lines.is_empty() {
            vec![String::new()]
        } else {
            lines
        };
        
        Self { lines }
    }
    
    /// Get the number of lines in the buffer
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// Get a line at the specified index
    pub fn get_line(&self, index: usize) -> Option<&String> {
        self.lines.get(index)
    }
    
    /// Get the length of a specific line (in characters)
    pub fn line_length(&self, index: usize) -> usize {
        self.lines.get(index).map_or(0, |line| line.chars().count())
    }
    
    /// Insert a character at the specified position
    pub fn insert_char(&mut self, pos: Position, ch: char) {
        if pos.row < self.lines.len() {
            let line = &mut self.lines[pos.row];
            // Convert character position to byte position
            let byte_pos = line.char_indices().nth(pos.col).map(|(i, _)| i).unwrap_or(line.len());
            if byte_pos <= line.len() {
                line.insert(byte_pos, ch);
            }
        }
    }
    
    /// Delete a character at the specified position
    pub fn delete_char(&mut self, pos: Position) -> Option<char> {
        if pos.row < self.lines.len() {
            let line = &mut self.lines[pos.row];
            // Convert character position to byte position
            if let Some((byte_pos, ch)) = line.char_indices().nth(pos.col) {
                line.remove(byte_pos);
                return Some(ch);
            } else if pos.col == line.chars().count() && pos.row + 1 < self.lines.len() {
                // Delete newline - merge with next line
                let next_line = self.lines.remove(pos.row + 1);
                self.lines[pos.row].push_str(&next_line);
                return Some('\n');
            }
        }
        None
    }
    
    /// Insert a new line at the specified position
    pub fn insert_newline(&mut self, pos: Position) {
        if pos.row < self.lines.len() {
            let current_line = self.lines[pos.row].clone();
            let char_count = current_line.chars().count();
            let split_pos = pos.col.min(char_count);
            
            // Find the byte position for the character position
            let byte_pos = if split_pos == 0 {
                0
            } else if split_pos >= char_count {
                current_line.len()
            } else {
                current_line.char_indices().nth(split_pos).map(|(i, _)| i).unwrap_or(current_line.len())
            };
            
            let (before, after) = current_line.split_at(byte_pos);
            
            self.lines[pos.row] = before.to_string();
            self.lines.insert(pos.row + 1, after.to_string());
        }
    }
    
    /// Convert buffer to string (for saving to file)
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
    
    /// Check if the buffer is empty (only contains one empty line)
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }
}

/// Trait for text buffer operations (for future extensibility)
pub trait TextBuffer {
    fn insert_char(&mut self, pos: Position, ch: char);
    fn delete_char(&mut self, pos: Position) -> Option<char>;
    fn insert_newline(&mut self, pos: Position);
    fn line_count(&self) -> usize;
    fn get_line(&self, index: usize) -> Option<&String>;
}

impl TextBuffer for Buffer {
    fn insert_char(&mut self, pos: Position, ch: char) {
        self.insert_char(pos, ch);
    }
    
    fn delete_char(&mut self, pos: Position) -> Option<char> {
        self.delete_char(pos)
    }
    
    fn insert_newline(&mut self, pos: Position) {
        self.insert_newline(pos);
    }
    
    fn line_count(&self) -> usize {
        self.line_count()
    }
    
    fn get_line(&self, index: usize) -> Option<&String> {
        self.get_line(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new();
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0), Some(&String::new()));
    }
    
    #[test]
    fn test_insert_char() {
        let mut buffer = Buffer::new();
        buffer.insert_char(Position::new(0, 0), 'h');
        buffer.insert_char(Position::new(0, 1), 'i');
        
        assert_eq!(buffer.get_line(0), Some(&"hi".to_string()));
    }
    
    #[test]
    fn test_delete_char() {
        let mut buffer = Buffer::new();
        buffer.insert_char(Position::new(0, 0), 'h');
        buffer.insert_char(Position::new(0, 1), 'i');
        
        let deleted = buffer.delete_char(Position::new(0, 1));
        assert_eq!(deleted, Some('i'));
        assert_eq!(buffer.get_line(0), Some(&"h".to_string()));
    }
    
    #[test]
    fn test_insert_newline() {
        let mut buffer = Buffer::new();
        buffer.insert_char(Position::new(0, 0), 'h');
        buffer.insert_char(Position::new(0, 1), 'i');
        buffer.insert_newline(Position::new(0, 1));
        
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0), Some(&"h".to_string()));
        assert_eq!(buffer.get_line(1), Some(&"i".to_string()));
    }
}
