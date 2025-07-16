use crate::gap_buffer::GapBufferLine;

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
/// Now uses GapBufferLine for efficient intra-line editing
pub struct Buffer {
    /// Lines of text (each GapBufferLine for efficient editing)
    lines: Vec<GapBufferLine>,
    /// Whether the file ended with a newline when loaded
    pub ends_with_newline: bool,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            lines: vec![GapBufferLine::new()], // Start with one empty line
            ends_with_newline: true,           // New files typically end with newline
        }
    }

    /// Create a buffer from file content
    pub fn from_file(content: &str) -> Self {
        // Check if the original content ended with a newline
        let ends_with_newline = content.ends_with('\n');

        let lines: Vec<GapBufferLine> = content.lines().map(GapBufferLine::from_string).collect();

        // Ensure at least one line exists
        let lines = if lines.is_empty() {
            vec![GapBufferLine::new()]
        } else {
            lines
        };

        Self {
            lines,
            ends_with_newline,
        }
    }

    /// Get the number of lines in the buffer
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a line at the specified index
    pub fn get_line(&self, index: usize) -> Option<String> {
        self.lines.get(index).map(|line| line.to_string())
    }

    /// Get the length of a specific line (in characters)
    pub fn line_length(&self, index: usize) -> usize {
        self.lines.get(index).map_or(0, |line| line.len())
    }

    /// Insert a character at the specified position
    pub fn insert_char(&mut self, pos: Position, ch: char) {
        if pos.row < self.lines.len() {
            let line = &mut self.lines[pos.row];
            line.insert(pos.col, ch);
        }
    }

    /// Delete a character at the specified position
    pub fn delete_char(&mut self, pos: Position) -> Option<char> {
        if pos.row >= self.lines.len() {
            return None;
        }

        // Check if we're deleting within the line
        if pos.col < self.lines[pos.row].len() {
            return self.lines[pos.row].delete(pos.col);
        }

        // Check if we're deleting newline (merge with next line)
        if pos.col == self.lines[pos.row].len() && pos.row + 1 < self.lines.len() {
            let line_len = self.lines[pos.row].len(); // Store length before borrow
            let next_line = self.lines.remove(pos.row + 1);
            let next_line_str = next_line.to_string();
            self.lines[pos.row].insert_str(line_len, &next_line_str);
            return Some('\n');
        }

        None
    }

    /// Insert a new line at the specified position
    pub fn insert_newline(&mut self, pos: Position) {
        if pos.row < self.lines.len() {
            let current_line = &self.lines[pos.row];
            let line_len = current_line.len();
            let split_pos = pos.col.min(line_len);

            // Get the part after the split position
            let after_split = if split_pos < line_len {
                current_line.substring(split_pos, line_len)
            } else {
                String::new()
            };

            // Truncate current line at split position
            let before_split = current_line.substring(0, split_pos);
            self.lines[pos.row] = GapBufferLine::from_string(&before_split);

            // Insert new line with the after part
            self.lines
                .insert(pos.row + 1, GapBufferLine::from_string(&after_split));
        }
    }

    /// Convert buffer to string (for saving to file)  
    pub fn content(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Check if the buffer is empty (only contains one empty line)
    pub fn is_empty(&self) -> bool {
        self.lines.len() == 1 && self.lines[0].is_empty()
    }

    /// Insert a new line at the specified row index
    pub fn insert_line(&mut self, row: usize, content: String) {
        if row <= self.lines.len() {
            self.lines.insert(row, GapBufferLine::from_string(&content));
        }
    }

    /// Remove a line at the specified row index
    /// Returns the removed line content if successful
    pub fn remove_line(&mut self, row: usize) -> Option<String> {
        if row < self.lines.len() && self.lines.len() > 1 {
            Some(self.lines.remove(row).to_string())
        } else {
            None
        }
    }

    /// Clear the content of a line but keep the line in the buffer
    pub fn clear_line(&mut self, row: usize) -> Option<String> {
        if row < self.lines.len() {
            let old_content = self.lines[row].to_string();
            self.lines[row].clear();
            Some(old_content)
        } else {
            None
        }
    }

    /// Get a character at the specified position
    pub fn get_char(&self, pos: (usize, usize)) -> Option<char> {
        let (row, col) = pos;
        if let Some(line) = self.lines.get(row) {
            line.get_char(col)
        } else {
            None
        }
    }

    /// Extract text from a range (for yank operations)
    pub fn extract_range(&self, start: (usize, usize), end: (usize, usize)) -> String {
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;

        let (from_row, from_col, to_row, to_col) =
            if start_row < end_row || (start_row == end_row && start_col <= end_col) {
                (start_row, start_col, end_row, end_col)
            } else {
                (end_row, end_col, start_row, start_col)
            };

        if from_row == to_row {
            // Single line extraction
            if let Some(line) = self.lines.get(from_row) {
                let line_len = line.len();
                let start_idx = from_col.min(line_len);
                let end_idx = to_col.min(line_len);
                if start_idx < end_idx {
                    line.substring(start_idx, end_idx)
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
                let line_len = line.len();
                let start_idx = from_col.min(line_len);
                if start_idx < line_len {
                    result.push_str(&line.substring(start_idx, line_len));
                }
                result.push('\n');
            }

            // Middle lines: entire lines
            for row in (from_row + 1)..to_row {
                if let Some(line) = self.lines.get(row) {
                    result.push_str(&line.to_string());
                    result.push('\n');
                }
            }

            // Last line: from start to end_col
            if to_row < self.lines.len() {
                if let Some(line) = self.lines.get(to_row) {
                    let line_len = line.len();
                    let end_idx = to_col.min(line_len);
                    if end_idx > 0 {
                        result.push_str(&line.substring(0, end_idx));
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
    fn get_line(&self, index: usize) -> Option<String>;
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

    fn get_line(&self, index: usize) -> Option<String> {
        self.get_line(index)
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}
