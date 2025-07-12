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
    
    /// Insert a new line at the specified row index
    pub fn insert_line(&mut self, row: usize, content: String) {
        if row <= self.lines.len() {
            self.lines.insert(row, content);
        }
    }
    
    /// Get a character at the specified position
    pub fn get_char(&self, pos: (usize, usize)) -> Option<char> {
        let (row, col) = pos;
        if let Some(line) = self.lines.get(row) {
            line.chars().nth(col)
        } else {
            None
        }
    }
    
    /// Extract text from a range (for yank operations)
    pub fn extract_range(&self, start: (usize, usize), end: (usize, usize)) -> String {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
        
        let (from_row, from_col, to_row, to_col) = if start_row < end_row || 
            (start_row == end_row && start_col <= end_col) {
            (start_row, start_col, end_row, end_col)
        } else {
            (end_row, end_col, start_row, start_col)
        };
        
        if from_row == to_row {
            // Single line extraction
            if let Some(line) = self.lines.get(from_row) {
                let chars: Vec<char> = line.chars().collect();
                let start_idx = from_col.min(chars.len());
                let end_idx = to_col.min(chars.len());
                if start_idx < end_idx {
                    chars[start_idx..end_idx].iter().collect()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            // Multi-line extraction
            let mut result = String::new();
            
            // First line: from start_col to end of line
            if let Some(line) = self.lines.get(from_row) {
                let chars: Vec<char> = line.chars().collect();
                let start_idx = from_col.min(chars.len());
                if start_idx < chars.len() {
                    result.push_str(&chars[start_idx..].iter().collect::<String>());
                }
                result.push('\n');
            }
            
            // Middle lines: entire lines
            for row in (from_row + 1)..to_row {
                if let Some(line) = self.lines.get(row) {
                    result.push_str(line);
                    result.push('\n');
                }
            }
            
            // Last line: from start to end_col
            if to_row < self.lines.len() {
                if let Some(line) = self.lines.get(to_row) {
                    let chars: Vec<char> = line.chars().collect();
                    let end_idx = to_col.min(chars.len());
                    if end_idx > 0 {
                        result.push_str(&chars[0..end_idx].iter().collect::<String>());
                    }
                }
            }
            
            result
        }
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
    
    #[test]
    fn test_yank_paste_operations() {
        let mut buffer = Buffer::new();
        buffer.insert_char(Position::new(0, 0), 'h');
        buffer.insert_char(Position::new(0, 1), 'e');
        buffer.insert_char(Position::new(0, 2), 'l');
        buffer.insert_char(Position::new(0, 3), 'l');
        buffer.insert_char(Position::new(0, 4), 'o');
        buffer.insert_newline(Position::new(0, 5));
        buffer.insert_char(Position::new(1, 0), 'w');
        buffer.insert_char(Position::new(1, 1), 'o');
        buffer.insert_char(Position::new(1, 2), 'r');
        buffer.insert_char(Position::new(1, 3), 'l');
        buffer.insert_char(Position::new(1, 4), 'd');
        
        // Extract "lo\nwor" - from position (0,3) which is 'l' to position (1,3) which is 'l' (exclusive)
        let yanked = buffer.extract_range((0, 3), (1, 3));
        assert_eq!(yanked, "lo\nwor");
        
        // Add a new line for testing
        buffer.insert_newline(Position::new(1, 5));
        
        // Paste at beginning of new line
        buffer.insert_char(Position::new(2, 0), 'P');
        buffer.insert_char(Position::new(2, 1), 'A');
        buffer.insert_char(Position::new(2, 2), 'S');
        buffer.insert_char(Position::new(2, 3), 'T');
        buffer.insert_char(Position::new(2, 4), 'E');
        
        assert_eq!(buffer.get_line(2), Some(&"PASTE".to_string()));
    }
}
