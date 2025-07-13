use crate::buffer::Buffer;
use crate::editor::{Editor, Cursor};
use crate::history::History;

/// File I/O operations for the editor
impl Editor {
    /// Load file into buffer
    pub fn load_file(&mut self, filename: &str) -> std::io::Result<()> {
        let content = std::fs::read_to_string(filename)?;
        self.buffer = Buffer::from_file(&content);
        self.filename = Some(filename.to_string());
        self.modified = false;
        // Reset cursor to top of file
        self.cursor = Cursor::new();
        // Clear history when loading a new file
        self.history.clear();
        Ok(())
    }

    /// Write buffer to file with optional filename
    pub fn write_file(&mut self, filename: Option<String>) -> bool {
        let target_filename = if let Some(name) = filename {
            // Update the current filename if saving as
            self.filename = Some(name.clone());
            name
        } else if let Some(ref name) = self.filename.clone() {
            name.clone()
        } else {
            self.set_status_message("E32: No file name".to_string());
            return false;
        };
        
        // Gather all buffer content
        let mut content = String::new();
        for i in 0..self.buffer.line_count() {
            if let Some(line) = self.buffer.get_line(i) {
                content.push_str(line);
                if i < self.buffer.line_count() - 1 {
                    content.push('\n');
                }
            }
        }
        
        // Add final newline if the buffer originally ended with one or if it's a new file
        if self.buffer.ends_with_newline {
            content.push('\n');
        }
        
        // Write to file
        let char_count = content.len();
        match std::fs::write(&target_filename, content) {
            Ok(_) => {
                self.modified = false;
                let line_count = self.buffer.line_count();
                self.set_status_message(format!("\"{}\" {}L, {}C written", target_filename, line_count, char_count));
                true
            }
            Err(e) => {
                self.set_status_message(format!("E212: Can't open file for writing: {}", e));
                false
            }
        }
    }

    /// Edit a new file (load or create new buffer)
    pub fn edit_file(&mut self, filename: &str) {
        if !self.modified || self.confirm_discard_changes() {
            match std::fs::read_to_string(filename) {
                Ok(content) => {
                    // Load new file content using Buffer::from_file to preserve newline info
                    self.buffer = Buffer::from_file(&content);
                    
                    self.filename = Some(filename.to_string());
                    self.modified = false;
                    self.cursor = Cursor::new();
                    self.scroll_offset = 0;
                    self.history = History::new(); // Clear undo history
                    
                    let line_count = self.buffer.line_count();
                    self.set_status_message(format!("\"{}\" {}L read", filename, line_count));
                }
                Err(_) => {
                    // File doesn't exist, create new buffer
                    self.buffer = Buffer::new();
                    // Buffer::new() already creates one empty line, so we don't need to insert another
                    self.filename = Some(filename.to_string());
                    self.modified = false;
                    self.cursor = Cursor::new();
                    self.scroll_offset = 0;
                    self.history = History::new();
                    
                    self.set_status_message(format!("\"{}\" [New File]", filename));
                }
            }
        }
    }

    /// Confirm with user before discarding changes (simplified version for now)
    fn confirm_discard_changes(&self) -> bool {
        // For now, just return false to prevent accidental data loss
        // In a full implementation, this would show a prompt to the user
        false
    }

    /// Quit the editor
    pub fn quit_editor(&mut self, force: bool) {
        if !force && self.modified {
            self.set_status_message("E37: No write since last change (add ! to override)".to_string());
            return;
        }
        
        self.running = false;
    }
}
