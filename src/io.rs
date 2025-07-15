use crate::buffer::Buffer;
use crate::editor::Editor;

/// File I/O operations for the editor
impl Editor {
    /// Write buffer to file with optional filename
    pub fn write_file(&mut self, filename: Option<String>) -> bool {
        let target_filename = if let Some(name) = filename {
            // Update the current filename if saving as
            self.set_filename(Some(name.clone()));
            name
        } else if let Some(ref name) = self.filename().clone() {
            name.clone()
        } else {
            self.set_status_message("E32: No file name".to_string());
            return false;
        };
        
        // Gather all buffer content
        let mut content = String::new();
        for i in 0..self.buffer().line_count() {
            if let Some(line) = self.buffer().get_line(i) {
                content.push_str(line);
                if i < self.buffer().line_count() - 1 {
                    content.push('\n');
                }
            }
        }
        
        // Add final newline if the buffer originally ended with one or if it's a new file
        if self.buffer().ends_with_newline {
            content.push('\n');
        }
        
        // Write to file
        let char_count = content.len();
        match std::fs::write(&target_filename, content) {
            Ok(_) => {
                self.set_modified(false);
                let line_count = self.buffer().line_count();
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
    pub fn edit_file(&mut self, filename: &str) -> std::io::Result<()> {
        if !self.is_modified() || self.confirm_discard_changes() {
            match std::fs::read_to_string(filename) {
                Ok(content) => {
                    // Load new file content using Buffer::from_file to preserve newline info
                    *self.buffer_mut() = Buffer::from_file(&content);
                    
                    self.set_filename(Some(filename.to_string()));
                    self.set_modified(false);
                    self.cursor_mut().row = 0;
                    self.cursor_mut().col = 0;
                    self.set_scroll_offset(0);
                    self.history_mut().clear(); // Clear undo history
                    
                    let line_count = self.buffer().line_count();
                    self.set_status_message(format!("\"{}\" {}L read", filename, line_count));
                }
                Err(_) => {
                    // File doesn't exist, create new buffer
                    *self.buffer_mut() = Buffer::new();
                    // Buffer::new() already creates one empty line, so we don't need to insert another
                    self.set_filename(Some(filename.to_string()));
                    self.set_modified(false);
                    self.cursor_mut().row = 0;
                    self.cursor_mut().col = 0;
                    self.set_scroll_offset(0);
                    self.history_mut().clear();
                    
                    self.set_status_message(format!("\"{}\" [New File]", filename));
                }
            }
        }
        Ok(())
    }

    /// Confirm with user before discarding changes (simplified version for now)
    fn confirm_discard_changes(&self) -> bool {
        // For now, just return false to prevent accidental data loss
        // In a full implementation, this would show a prompt to the user
        false
    }

    /// Load multiple files into separate buffers
    pub fn load_files(&mut self, filenames: &[String]) -> Vec<Result<(), std::io::Error>> {
        let mut results = Vec::new();
        
        // Clear the default empty buffer if we're loading files
        if self.buffers.len() == 1 && self.filename().is_none() && !self.is_modified() {
            self.buffers.clear();
            self.current_buffer = 0;
        }
        
        for (i, filename) in filenames.iter().enumerate() {
            match std::fs::read_to_string(filename) {
                Ok(content) => {
                    let buffer_info = crate::editor::BufferInfo {
                        buffer: Buffer::from_file(&content),
                        filename: Some(filename.clone()),
                        modified: false,
                        cursor: crate::editor::Cursor::new(),
                        scroll_offset: 0,
                        history: crate::history::History::new(),
                    };
                    
                    if i == 0 && self.buffers.is_empty() {
                        // First buffer becomes the current buffer
                        self.buffers.push(buffer_info);
                        self.current_buffer = 0;
                    } else {
                        // Additional buffers are added but don't become current
                        self.buffers.push(buffer_info);
                    }
                    
                    results.push(Ok(()));
                }
                Err(e) => {
                    // For files that couldn't be loaded, create an empty buffer with the filename
                    let buffer_info = crate::editor::BufferInfo {
                        buffer: Buffer::new(),
                        filename: Some(filename.clone()),
                        modified: false,
                        cursor: crate::editor::Cursor::new(),
                        scroll_offset: 0,
                        history: crate::history::History::new(),
                    };
                    
                    if i == 0 && self.buffers.is_empty() {
                        self.buffers.push(buffer_info);
                        self.current_buffer = 0;
                    } else {
                        self.buffers.push(buffer_info);
                    }
                    
                    results.push(Err(e));
                }
            }
        }
        
        results
    }

    /// Write all modified buffers
    pub fn write_all_buffers(&mut self) -> bool {
        let mut all_saved = true;
        let mut saved_count = 0;
        let mut error_count = 0;
        
        // Save current buffer index to restore later
        let original_buffer = self.current_buffer;
        
        for i in 0..self.buffers.len() {
            // Switch to each buffer to write it
            self.current_buffer = i;
            
            if self.is_modified() {
                if self.filename().is_some() {
                    if self.write_file(None) {
                        saved_count += 1;
                    } else {
                        all_saved = false;
                        error_count += 1;
                    }
                } else {
                    // Buffer has no filename, can't save
                    all_saved = false;
                    error_count += 1;
                    self.set_status_message(format!("E32: No file name for buffer {}", i + 1));
                }
            }
        }
        
        // Restore original buffer
        self.current_buffer = original_buffer;
        
        if error_count > 0 {
            self.set_status_message(format!("{} files written, {} errors", saved_count, error_count));
        } else if saved_count > 0 {
            self.set_status_message(format!("{} files written", saved_count));
        } else {
            self.set_status_message("No files needed saving".to_string());
        }
        
        all_saved
    }
    
    
    /// Close current buffer (remove it from the buffers list)
    pub fn close_buffer(&mut self, force: bool) {
        // Check if buffer has unsaved changes
        if !force && self.is_modified() {
            self.set_status_message("E37: No write since last change (add ! to override)".to_string());
            return;
        }

        // If this is the only buffer, exit the editor
        if self.buffers.len() == 1 {
            self.running = false;
            return;
        }

        // Remove current buffer
        self.buffers.remove(self.current_buffer);

        // Adjust current buffer index
        if self.current_buffer >= self.buffers.len() {
            self.current_buffer = self.buffers.len() - 1;
        }

        // Show status
        self.set_status_message(format!("Buffer closed. Now showing buffer {}", self.current_buffer + 1));
    }

    /// Quit all buffers
    pub fn quit_all_editor(&mut self, force: bool) {
        if !force {
            // Check if any buffer has modifications
            for (i, buffer) in self.buffers.iter().enumerate() {
                if buffer.modified {
                    self.set_status_message(format!("E37: No write since last change in buffer {} (add ! to override)", i + 1));
                    return;
                }
            }
        }
        
        self.running = false;
    }
}
