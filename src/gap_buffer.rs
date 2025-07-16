/// Gap Buffer implementation for efficient text editing
///
/// A gap buffer maintains a contiguous array of characters with a "gap" region
/// that represents empty space. This allows O(1) insertions at the gap position
/// and efficient editing for contiguous operations.
use std::fmt;

/// A gap buffer for a single line of text
///
/// The buffer is structured as: [content][gap][content]
/// where gap_start and gap_end define the empty region
#[derive(Clone)]
pub struct GapBufferLine {
    /// The underlying character buffer including the gap
    buffer: Vec<char>,
    /// Start index of the gap (inclusive)
    gap_start: usize,
    /// End index of the gap (exclusive)
    gap_end: usize,
}

impl GapBufferLine {
    /// Create a new empty gap buffer with initial capacity
    pub fn new() -> Self {
        Self::with_capacity(16)
    }

    /// Create a new gap buffer with specified initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, '\0'); // Fill with null chars for the gap

        Self {
            buffer,
            gap_start: 0,
            gap_end: capacity,
        }
    }

    /// Create a gap buffer from an existing string
    pub fn from_string(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        let capacity = (len * 2).max(16); // Double capacity for growth

        let mut buffer = Vec::with_capacity(capacity);

        // Add the string content
        buffer.extend_from_slice(&chars);

        // Fill the rest with gap space
        buffer.resize(capacity, '\0');

        Self {
            buffer,
            gap_start: len,
            gap_end: capacity,
        }
    }

    /// Get the logical length of the text (excluding gap)
    pub fn len(&self) -> usize {
        self.buffer.len() - (self.gap_end - self.gap_start)
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the size of the gap
    fn gap_size(&self) -> usize {
        self.gap_end - self.gap_start
    }

    /// Move the gap to the specified logical position
    /// This is O(n) but happens less frequently than insertions
    fn move_gap_to(&mut self, pos: usize) {
        let logical_len = self.len();
        let pos = pos.min(logical_len);

        use std::cmp::Ordering;
        match pos.cmp(&self.gap_start) {
            Ordering::Less => {
                // Move gap left: move characters from before gap to after gap
                let move_count = self.gap_start - pos;
                let src_start = pos;
                let dst_start = self.gap_end - move_count;

                // Copy characters that need to move in reverse order to avoid overwriting
                for i in (0..move_count).rev() {
                    self.buffer[dst_start + i] = self.buffer[src_start + i];
                }

                // Update gap boundaries
                self.gap_start = pos;
                self.gap_end -= move_count;
            }
            Ordering::Greater => {
                // Move gap right: move characters from after gap to before gap
                let move_count = pos - self.gap_start;
                let src_start = self.gap_end;
                let dst_start = self.gap_start;

                // Copy characters that need to move
                for i in 0..move_count {
                    self.buffer[dst_start + i] = self.buffer[src_start + i];
                }

                // Update gap boundaries
                self.gap_start = pos;
                self.gap_end += move_count;
            }
            Ordering::Equal => {
                // If pos == gap_start, no movement needed
            }
        }
    }

    /// Ensure the gap has at least the specified size
    /// If not, grow the buffer and expand the gap
    fn ensure_gap_size(&mut self, required_size: usize) {
        if self.gap_size() < required_size {
            let current_len = self.buffer.len();
            let new_capacity = (current_len * 2).max(current_len + required_size);
            let additional_space = new_capacity - current_len;

            // Save content after gap
            let after_gap: Vec<char> = self.buffer[self.gap_end..].to_vec();

            // Extend buffer with gap space
            self.buffer.resize(new_capacity, '\0');

            // Move content after gap to the new end
            let new_gap_end = self.gap_end + additional_space;
            for (i, &ch) in after_gap.iter().enumerate() {
                self.buffer[new_gap_end + i] = ch;
            }

            // Update gap end to include new space
            self.gap_end = new_gap_end;
        }
    }

    /// Insert a character at the specified logical position
    /// O(1) if position is at gap, O(n) if gap needs to be moved
    pub fn insert(&mut self, pos: usize, ch: char) {
        let logical_len = self.len();
        let pos = pos.min(logical_len);

        // Move gap to insertion position if needed
        self.move_gap_to(pos);

        // Ensure we have space in the gap
        self.ensure_gap_size(1);

        // Insert character at gap start
        self.buffer[self.gap_start] = ch;
        self.gap_start += 1;
    }

    /// Delete character at the specified logical position
    /// Returns the deleted character if successful
    pub fn delete(&mut self, pos: usize) -> Option<char> {
        let logical_len = self.len();
        if pos >= logical_len {
            return None;
        }

        // Move gap to just after the position to delete
        self.move_gap_to(pos + 1);

        // The character to delete is now just before the gap
        if self.gap_start > 0 {
            self.gap_start -= 1;
            let ch = self.buffer[self.gap_start];
            Some(ch)
        } else {
            None
        }
    }

    /// Get character at logical position
    pub fn get_char(&self, pos: usize) -> Option<char> {
        let logical_len = self.len();
        if pos >= logical_len {
            return None;
        }

        // Convert logical to physical position
        let physical_pos = if pos < self.gap_start {
            pos
        } else {
            pos + self.gap_size()
        };

        Some(self.buffer[physical_pos])
    }

    /// Convert the gap buffer to a string
    pub fn content(&self) -> String {
        let mut result = String::with_capacity(self.len());

        // Add characters before gap
        for i in 0..self.gap_start {
            if i < self.buffer.len() {
                result.push(self.buffer[i]);
            }
        }

        // Add characters after gap
        for i in self.gap_end..self.buffer.len() {
            result.push(self.buffer[i]);
        }

        result
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.gap_start = 0;
        self.gap_end = self.buffer.len();
    }

    /// Insert a string at the specified position
    pub fn insert_str(&mut self, pos: usize, s: &str) {
        let chars: Vec<char> = s.chars().collect();
        let logical_len = self.len();
        let pos = pos.min(logical_len);

        // Move gap to insertion position
        self.move_gap_to(pos);

        // Ensure we have enough space
        self.ensure_gap_size(chars.len());

        // Insert all characters
        for ch in chars {
            self.buffer[self.gap_start] = ch;
            self.gap_start += 1;
        }
    }

    /// Delete a range of characters [start, end)
    /// Returns the deleted text
    pub fn delete_range(&mut self, start: usize, end: usize) -> String {
        let logical_len = self.len();
        let start = start.min(logical_len);
        let end = end.min(logical_len);

        if start >= end {
            return String::new();
        }

        // Extract the text to be deleted
        let mut deleted = String::new();
        for _ in start..end {
            if let Some(ch) = self.get_char(start) {
                deleted.push(ch);
                self.delete(start);
            }
        }

        deleted
    }

    /// Get substring from start to end (exclusive)
    pub fn substring(&self, start: usize, end: usize) -> String {
        let logical_len = self.len();
        let start = start.min(logical_len);
        let end = end.min(logical_len);

        if start >= end {
            return String::new();
        }

        let mut result = String::new();
        for i in start..end {
            if let Some(ch) = self.get_char(i) {
                result.push(ch);
            }
        }
        result
    }
}

impl Default for GapBufferLine {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GapBufferLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl fmt::Debug for GapBufferLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GapBufferLine")
            .field("content", &self.content())
            .field("gap_start", &self.gap_start)
            .field("gap_end", &self.gap_end)
            .field("logical_length", &self.len())
            .field("buffer_capacity", &self.buffer.len())
            .finish()
    }
}
