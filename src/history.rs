/// History management for undo/redo functionality
use crate::buffer::Position;

/// Represents different types of edit actions that can be undone/redone
#[derive(Debug, Clone)]
pub enum EditAction {
    /// Text was inserted at a position
    InsertText {
        pos: Position,
        text: String,
    },
    /// Text was deleted from a position
    DeleteText {
        pos: Position,
        text: String,
    },
}

impl EditAction {
    /// Create an insert text action
    pub fn insert_text(pos: Position, text: String) -> Self {
        EditAction::InsertText { pos, text }
    }
    
    /// Create a delete text action
    pub fn delete_text(pos: Position, text: String) -> Self {
        EditAction::DeleteText { pos, text }
    }
}

/// Manages the undo/redo history
pub struct History {
    /// Stack of actions that can be undone
    undo_stack: Vec<EditAction>,
    /// Stack of actions that can be redone (after undo)
    redo_stack: Vec<EditAction>,
    /// Maximum number of undo levels to keep
    max_undo_levels: usize,
}

impl History {
    /// Create a new history manager
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_undo_levels: 1000, // Reasonable default
        }
    }
    
    /// Add an action to the undo stack
    pub fn push(&mut self, action: EditAction) {
        // Clear redo stack when new action is performed
        self.redo_stack.clear();
        
        // Add to undo stack
        self.undo_stack.push(action);
        
        // Limit undo stack size
        if self.undo_stack.len() > self.max_undo_levels {
            self.undo_stack.remove(0);
        }
    }
    
    /// Push an action to the undo stack without clearing redo stack (for redo operations)
    pub fn push_undo(&mut self, action: EditAction) {
        // Add to undo stack without clearing redo stack
        self.undo_stack.push(action);
        
        // Limit undo stack size
        if self.undo_stack.len() > self.max_undo_levels {
            self.undo_stack.remove(0);
        }
    }
    
    /// Pop an action from the undo stack for undoing
    pub fn pop_undo(&mut self) -> Option<EditAction> {
        self.undo_stack.pop()
    }
    
    /// Push an action to the redo stack (when undoing)
    pub fn push_redo(&mut self, action: EditAction) {
        self.redo_stack.push(action);
    }
    
    /// Pop an action from the redo stack for redoing
    pub fn pop_redo(&mut self) -> Option<EditAction> {
        self.redo_stack.pop()
    }
    
    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    /// Get the number of undo levels available
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }
    
    /// Get the number of redo levels available
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }
    
    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
    
    /// Apply an undo operation to the buffer and return the action for redo stack
    pub fn apply_undo(&mut self, buffer: &mut crate::buffer::Buffer) -> Option<(EditAction, crate::buffer::Position)> {
        if let Some(action) = self.pop_undo() {
            let cursor_pos = match action.clone() {
                EditAction::InsertText { pos, text } => {
                    // Undo insertion by deleting the text that was inserted
                    // We need to delete from the end backwards to handle newlines correctly
                    Self::delete_inserted_text(buffer, pos, &text);
                    pos
                }
                EditAction::DeleteText { pos, text } => {
                    // Undo deletion by reinserting the deleted text
                    Self::reinsert_deleted_text(buffer, pos, &text);
                    pos
                }
            };
            
            // Push the action to redo stack
            self.push_redo(action.clone());
            Some((action, cursor_pos))
        } else {
            None
        }
    }
    
    /// Apply a redo operation to the buffer and return the action for undo stack
    pub fn apply_redo(&mut self, buffer: &mut crate::buffer::Buffer) -> Option<(EditAction, crate::buffer::Position)> {
        if let Some(action) = self.pop_redo() {
            let cursor_pos = match action.clone() {
                EditAction::InsertText { pos, text } => {
                    // Redo insertion - handle newlines properly
                    if text.contains('\n') {
                        Self::reinsert_deleted_text(buffer, pos, &text);
                        // Calculate final cursor position after multi-line insertion
                        let lines: Vec<&str> = text.split('\n').collect();
                        if lines.len() > 1 {
                            let last_line = lines[lines.len() - 1];
                            crate::buffer::Position::new(pos.row + lines.len() - 1, last_line.len())
                        } else {
                            crate::buffer::Position::new(pos.row, pos.col + text.chars().count())
                        }
                    } else {
                        // Simple character insertion
                        for (i, ch) in text.chars().enumerate() {
                            let insert_pos = crate::buffer::Position::new(pos.row, pos.col + i);
                            buffer.insert_char(insert_pos, ch);
                        }
                        crate::buffer::Position::new(pos.row, pos.col + text.chars().count())
                    }
                }
                EditAction::DeleteText { pos, text } => {
                    // Redo deletion
                    Self::delete_text_from_buffer(buffer, pos, &text);
                    pos
                }
            };
            
            // Push the action back to undo stack (without clearing redo stack)
            self.push_undo(action.clone());
            Some((action, cursor_pos))
        } else {
            None
        }
    }
    
    /// Helper method to reinsert deleted text, handling newlines properly
    fn reinsert_deleted_text(buffer: &mut crate::buffer::Buffer, pos: crate::buffer::Position, text: &str) {
        if text.contains('\n') {
            if text.starts_with('\n') {
                // This could be either "O" command (newline at beginning) or "o" command (newline at end)
                // Format: "\nSomeText" or just "\n"
                let text_after_newline = &text[1..]; // Remove the leading newline
                
                if pos.col == 0 {
                    // "O" command case: newline at beginning pushes content down
                    // First, insert the newline at the specified position
                    buffer.insert_newline(pos);
                    
                    // Then insert the text on the new line (which is now at pos.row)
                    if !text_after_newline.is_empty() {
                        for (i, ch) in text_after_newline.chars().enumerate() {
                            let insert_pos = crate::buffer::Position::new(pos.row, i);
                            buffer.insert_char(insert_pos, ch);
                        }
                    }
                } else {
                    // "o" command case: newline at end creates new line below
                    // First, insert the newline at the specified position (end of line)
                    buffer.insert_newline(pos);
                    
                    // Then insert the text on the new line below
                    if !text_after_newline.is_empty() {
                        for (i, ch) in text_after_newline.chars().enumerate() {
                            let insert_pos = crate::buffer::Position::new(pos.row + 1, i);
                            buffer.insert_char(insert_pos, ch);
                        }
                    }
                }
            } else {
                // Regular multi-line text insertion (contains newlines but doesn't start with one)
                let lines: Vec<&str> = text.split('\n').collect();
                let mut current_row = pos.row;
                let mut current_col = pos.col;
                
                for (line_idx, line) in lines.iter().enumerate() {
                    if line_idx == 0 {
                        // First line: insert at the specified position
                        for (char_idx, ch) in line.chars().enumerate() {
                            let insert_pos = crate::buffer::Position::new(current_row, current_col + char_idx);
                            buffer.insert_char(insert_pos, ch);
                        }
                        // If there are more lines, insert a newline
                        if lines.len() > 1 && line_idx < lines.len() - 1 {
                            let newline_pos = crate::buffer::Position::new(current_row, current_col + line.len());
                            buffer.insert_newline(newline_pos);
                            current_row += 1;
                            current_col = 0;
                        }
                    } else if line_idx == lines.len() - 1 && line.is_empty() {
                        // Last empty line (trailing newline) - already handled by previous newline insertion
                        break;
                    } else {
                        // Subsequent lines: insert at beginning of current row
                        for (char_idx, ch) in line.chars().enumerate() {
                            let insert_pos = crate::buffer::Position::new(current_row, char_idx);
                            buffer.insert_char(insert_pos, ch);
                        }
                        // Insert newline if not the last line
                        if line_idx < lines.len() - 1 {
                            let newline_pos = crate::buffer::Position::new(current_row, line.len());
                            buffer.insert_newline(newline_pos);
                            current_row += 1;
                        }
                    }
                }
            }
        } else {
            // Simple character deletion, insert characters sequentially
            for (i, ch) in text.chars().enumerate() {
                let insert_pos = crate::buffer::Position::new(pos.row, pos.col + i);
                buffer.insert_char(insert_pos, ch);
            }
        }
    }
    
    /// Helper method to delete text from buffer, handling newlines properly
    fn delete_text_from_buffer(buffer: &mut crate::buffer::Buffer, pos: crate::buffer::Position, text: &str) {
        if text.contains('\n') {
            // This is a line-based deletion, handle it specially
            let lines: Vec<&str> = text.split('\n').collect();
            let mut chars_to_delete = 0;
            
            // Count total characters including newlines
            for (line_idx, line) in lines.iter().enumerate() {
                chars_to_delete += line.len();
                // Add 1 for newline, except for the last line if it's empty (trailing newline)
                if line_idx < lines.len() - 1 || !line.is_empty() {
                    chars_to_delete += 1; // for the newline
                }
            }
            
            // Delete characters starting from the position
            for _ in 0..chars_to_delete {
                if buffer.delete_char(pos).is_none() {
                    break;
                }
            }
        } else {
            // Simple character deletion
            for _ in 0..text.chars().count() {
                if buffer.delete_char(pos).is_none() {
                    break;
                }
            }
        }
    }
    
    /// Helper method to delete inserted text, handling newlines properly
    fn delete_inserted_text(buffer: &mut crate::buffer::Buffer, start_pos: crate::buffer::Position, text: &str) {
        if text.contains('\n') {
            if text.starts_with('\n') {
                // This could be either "O" command (newline at beginning) or "o" command (newline at end)
                // Format: "\nSomeText" or just "\n"
                let text_after_newline = &text[1..]; // Remove the leading newline
                
                // For "O" command: newline inserted at beginning (start_pos = (row, 0))
                // For "o" command: newline inserted at end (start_pos = (row, line_end))
                
                if start_pos.col == 0 {
                    // "O" command case: newline at beginning pushes content down
                    // Delete the text that was typed on the new line (which is now at start_pos.row)
                    if !text_after_newline.is_empty() {
                        for _ in 0..text_after_newline.chars().count() {
                            if buffer.delete_char(start_pos).is_none() {
                                break;
                            }
                        }
                    }
                    
                    // Delete the newline that pushed content down
                    if buffer.line_count() > start_pos.row + 1 {
                        let current_line_len = buffer.line_length(start_pos.row);
                        let newline_pos = crate::buffer::Position::new(start_pos.row, current_line_len);
                        buffer.delete_char(newline_pos);
                    }
                } else {
                    // "o" command case: newline at end creates new line below
                    // Delete the text that was typed on the line below
                    if !text_after_newline.is_empty() && buffer.line_count() > start_pos.row + 1 {
                        for _ in 0..text_after_newline.chars().count() {
                            let del_pos = crate::buffer::Position::new(start_pos.row + 1, 0);
                            if buffer.delete_char(del_pos).is_none() {
                                break;
                            }
                        }
                    }
                    
                    // Delete the newline that was inserted at start_pos
                    // For this case, we need to delete the newline at the end of the current line
                    // This will join the current line with the next line (removing the empty line created)
                    if buffer.line_count() > start_pos.row + 1 {
                        let current_line_len = buffer.line_length(start_pos.row);
                        let newline_pos = crate::buffer::Position::new(start_pos.row, current_line_len);
                        buffer.delete_char(newline_pos);
                    }
                }
            } else {
                // Regular multi-line text insertion (contains newlines but doesn't start with one)
                let segments: Vec<&str> = text.split('\n').collect();
                let mut current_row = start_pos.row;
                
                // Calculate final position after all insertions
                for (segment_idx, _segment) in segments.iter().enumerate() {
                    if segment_idx > 0 {
                        current_row += 1;
                    }
                }
                
                // Delete backwards from the final position
                current_row = start_pos.row + segments.len() - 1;
                
                // Delete characters from each line, starting from the last line
                for (segment_idx, segment) in segments.iter().enumerate().rev() {
                    if segment_idx == 0 {
                        // First segment: delete characters from the original line
                        for _ in 0..segment.len() {
                            if buffer.delete_char(start_pos).is_none() {
                                break;
                            }
                        }
                    } else {
                        // Other segments: delete all characters from this line, then the newline
                        for _ in 0..segment.len() {
                            let pos = crate::buffer::Position::new(current_row, 0);
                            if buffer.delete_char(pos).is_none() {
                                break;
                            }
                        }
                        // After deleting all characters from this line, delete the newline
                        let prev_line_len = buffer.line_length(current_row - 1);
                        let newline_pos = crate::buffer::Position::new(current_row - 1, prev_line_len);
                        buffer.delete_char(newline_pos);
                        current_row -= 1;
                    }
                }
            }
        } else {
            // Simple character-only deletion
            for _ in 0..text.chars().count() {
                if buffer.delete_char(start_pos).is_none() {
                    break;
                }
            }
        }
    }
    
    /// Check if there are any operations available (for UI status display)
    pub fn has_operations(&self) -> bool {
        self.can_undo() || self.can_redo()
    }
    
    /// Get a status summary of available operations
    pub fn status_summary(&self) -> String {
        let undo_count = self.undo_count();
        let redo_count = self.redo_count();
        
        match (undo_count > 0, redo_count > 0) {
            (true, true) => format!("History: {} undo, {} redo", undo_count, redo_count),
            (true, false) => format!("History: {} undo", undo_count),
            (false, true) => format!("History: {} redo", redo_count),
            (false, false) => "History: empty".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_undo_redo() {
        let mut history = History::new();
        let action = EditAction::insert_text(Position::new(0, 0), "hello".to_string());
        
        // Push action
        history.push(action.clone());
        assert!(history.can_undo());
        assert!(!history.can_redo());
        
        // Undo
        let undone = history.pop_undo().unwrap();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        
        // Push to redo stack
        history.push_redo(undone);
        assert!(!history.can_undo());
        assert!(history.can_redo());
        
        // Redo
        let _redone = history.pop_redo().unwrap();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
    
    #[test]
    fn test_new_action_clears_redo() {
        let mut history = History::new();
        
        // Add and undo an action
        history.push(EditAction::insert_text(Position::new(0, 0), "hello".to_string()));
        let undone = history.pop_undo().unwrap();
        history.push_redo(undone);
        assert!(history.can_redo());
        
        // Add new action - should clear redo stack
        history.push(EditAction::insert_text(Position::new(0, 5), "world".to_string()));
        assert!(!history.can_redo());
        assert!(history.can_undo());
    }
    
    #[test]
    fn test_undo_insert_with_newlines() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Insert some initial text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');
        
        // Now simulate insert mode: cursor at end, insert "\nWorld"
        let start_pos = crate::buffer::Position::new(0, 5);
        let inserted_text = "\nWorld"; // newline + "World"
        
        // Do the actual insertion
        buffer.insert_newline(crate::buffer::Position::new(0, 5));
        buffer.insert_char(crate::buffer::Position::new(1, 0), 'W');
        buffer.insert_char(crate::buffer::Position::new(1, 1), 'o');
        buffer.insert_char(crate::buffer::Position::new(1, 2), 'r');
        buffer.insert_char(crate::buffer::Position::new(1, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(1, 4), 'd');
        
        // Buffer should now have:
        // Line 0: "Hello"
        // Line 1: "World" 
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
        
        // Record the insert action
        let action = EditAction::insert_text(start_pos, inserted_text.to_string());
        history.push(action);
        
        // Now undo - this should remove the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());
        
        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
    }
    
    #[test]
    fn test_complex_insert_mode_switches() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');
        
        // First insert mode session: add " World"
        let action1 = EditAction::insert_text(
            crate::buffer::Position::new(0, 5), 
            " World".to_string()
        );
        // Simulate the insertion
        for (i, ch) in " World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 5 + i), ch);
        }
        history.push(action1);
        
        // Buffer should now be: "Hello World"
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        
        // Second insert mode session: add newline and "Test"
        let action2 = EditAction::insert_text(
            crate::buffer::Position::new(0, 11), 
            "\nTest".to_string()
        );
        // Simulate the insertion
        buffer.insert_newline(crate::buffer::Position::new(0, 11));
        for (i, ch) in "Test".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        history.push(action2);
        
        // Buffer should now be:
        // Line 0: "Hello World"
        // Line 1: "Test"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
        
        // Third insert mode session: insert at beginning of first line
        let action3 = EditAction::insert_text(
            crate::buffer::Position::new(0, 0), 
            "Hi ".to_string()
        );
        // Simulate the insertion (this shifts existing text right)
        for (i, ch) in "Hi ".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        history.push(action3);
        
        // Buffer should now be:
        // Line 0: "Hi Hello World"
        // Line 1: "Test"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hi Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
        
        // Now test undo operations
        
        // First undo: should remove "Hi " from beginning
        let result1 = history.apply_undo(&mut buffer);
        assert!(result1.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
        
        // Second undo: should remove "\nTest"
        let result2 = history.apply_undo(&mut buffer);
        assert!(result2.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        
        // Third undo: should remove " World"
        let result3 = history.apply_undo(&mut buffer);
        assert!(result3.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Test redo operations
        
        // First redo: should add " World" back
        let redo1 = history.apply_redo(&mut buffer);
        assert!(redo1.is_some());
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        
        // Second redo: should add "\nTest" back
        let redo2 = history.apply_redo(&mut buffer);
        assert!(redo2.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
        
        // Third redo: should add "Hi " back at beginning
        let redo3 = history.apply_redo(&mut buffer);
        assert!(redo3.is_some());
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hi Hello World");
        assert_eq!(buffer.get_line(1).unwrap(), "Test");
    }
    
    #[test]
    fn test_real_world_mixed_operations() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Simulate a realistic editing session
        
        // 1. Start with empty buffer, type "fn main() {"
        let action1 = EditAction::insert_text(
            crate::buffer::Position::new(0, 0), 
            "fn main() {".to_string()
        );
        for (i, ch) in "fn main() {".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        history.push(action1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        
        // 2. Hit Enter and add indented content
        let action2 = EditAction::insert_text(
            crate::buffer::Position::new(0, 11), 
            "\n    println!(\"Hello, world!\");".to_string()
        );
        buffer.insert_newline(crate::buffer::Position::new(0, 11));
        for (i, ch) in "    println!(\"Hello, world!\");".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        history.push(action2);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
        
        // 3. Add closing brace on new line
        let action3 = EditAction::insert_text(
            crate::buffer::Position::new(1, 31), 
            "\n}".to_string()
        );
        buffer.insert_newline(crate::buffer::Position::new(1, 31));
        buffer.insert_char(crate::buffer::Position::new(2, 0), '}');
        history.push(action3);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(2).unwrap(), "}");
        
        // 4. Go back and add a comment on first line
        let action4 = EditAction::insert_text(
            crate::buffer::Position::new(0, 0), 
            "// Main function\n".to_string()
        );
        buffer.insert_char(crate::buffer::Position::new(0, 0), '/');
        buffer.insert_char(crate::buffer::Position::new(0, 1), '/');
        buffer.insert_char(crate::buffer::Position::new(0, 2), ' ');
        for (i, ch) in "Main function".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 3 + i), ch);
        }
        buffer.insert_newline(crate::buffer::Position::new(0, 16));
        history.push(action4);
        
        // Should now have 4 lines with the comment at top
        assert_eq!(buffer.line_count(), 4);
        assert_eq!(buffer.get_line(0).unwrap(), "// Main function");
        assert_eq!(buffer.get_line(1).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(2).unwrap(), "    println!(\"Hello, world!\");");
        assert_eq!(buffer.get_line(3).unwrap(), "}");
        
        // Now test undoing the entire session step by step
        
        // Undo 1: Remove comment
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
        assert_eq!(buffer.get_line(2).unwrap(), "}");
        
        // Undo 2: Remove closing brace
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
        
        // Undo 3: Remove println line
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        
        // Undo 4: Remove function declaration
        history.apply_undo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "");
        
        // Now test redoing everything back
        
        // Redo 1: Add function declaration
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        
        // Redo 2: Add println line
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
        
        // Redo 3: Add closing brace
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 3);
        assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
        assert_eq!(buffer.get_line(2).unwrap(), "}");
        
        // Redo 4: Add comment
        history.apply_redo(&mut buffer);
        assert_eq!(buffer.line_count(), 4);
        assert_eq!(buffer.get_line(0).unwrap(), "// Main function");
        assert_eq!(buffer.get_line(1).unwrap(), "fn main() {");
        assert_eq!(buffer.get_line(2).unwrap(), "    println!(\"Hello, world!\");");
        assert_eq!(buffer.get_line(3).unwrap(), "}");
    }
    
    #[test]
    fn test_open_line_commands_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');
        
        // Buffer should be: "Hello"
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Simulate "o" command: opens line below and enters insert mode
        // This should create a new line and then track any text inserted
        let open_line_pos = crate::buffer::Position::new(0, 5); // end of current line
        buffer.insert_newline(open_line_pos);
        
        // Now cursor should be at (1, 0) and in insert mode
        // Simulate typing "World" in insert mode
        let full_inserted_text = "\nWorld"; // This should include the newline from "o"
        
        // Insert the text
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        
        // Record the complete action (newline + text)
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);
        
        // Buffer should now be:
        // Line 0: "Hello"
        // Line 1: "World"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
        
        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());
        
        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());
        
        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
    }
    
    #[test]
    fn test_open_line_above_command_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Start with some text: "Hello"
        buffer.insert_char(crate::buffer::Position::new(0, 0), 'H');
        buffer.insert_char(crate::buffer::Position::new(0, 1), 'e');
        buffer.insert_char(crate::buffer::Position::new(0, 2), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 3), 'l');
        buffer.insert_char(crate::buffer::Position::new(0, 4), 'o');
        
        // Buffer should be: "Hello"
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Simulate "O" command: opens line above current line and enters insert mode
        // This should create a new line above and then track any text inserted
        let open_line_pos = crate::buffer::Position::new(0, 0); // beginning of current line
        buffer.insert_newline(open_line_pos);
        
        // After insert_newline(0,0), the buffer becomes:
        // Line 0: "" (empty line)
        // Line 1: "Hello" (original content pushed down)
        
        // Now cursor should be at (0, 0) and in insert mode
        // Simulate typing "World" in insert mode
        let full_inserted_text = "\nWorld"; // This should include the newline from "O"
        
        // Insert the text on the new empty line (line 0)
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        
        // Record the complete action (newline + text)
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);
        
        // Buffer should now be:
        // Line 0: "World"
        // Line 1: "Hello"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "World");
        assert_eq!(buffer.get_line(1).unwrap(), "Hello");
        
        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());
        
        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());
        
        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);        assert_eq!(buffer.get_line(0).unwrap(), "World");
        assert_eq!(buffer.get_line(1).unwrap(), "Hello");
    }

    #[test]
    fn test_open_line_below_command_undo() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Start with one line: "Hello"
        for (i, ch) in "Hello".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Simulate "o" command at the end of line 0
        // The "o" command should create a new line below and enter insert mode
        let open_line_pos = crate::buffer::Position::new(0, 5); // End of "Hello"
        
        // Insert a newline at the end of the current line
        buffer.insert_newline(open_line_pos);
        
        // Now buffer should be:
        // Line 0: "Hello"
        // Line 1: ""     (empty new line)
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "");
        
        // Insert the text on the new empty line (line 1)
        for (i, ch) in "World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(1, i), ch);
        }
        
        // Buffer should now be:
        // Line 0: "Hello"
        // Line 1: "World"
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
        
        // For "o" command, we record it differently - as regular insertion starting from where newline was inserted
        // The insertion includes newline + text, but positioned at end of original line
        let full_inserted_text = "\nWorld"; // newline + the text typed
        let action = EditAction::insert_text(open_line_pos, full_inserted_text.to_string());
        history.push(action);
        
        // Test undo - should remove both the newline and "World"
        let result = history.apply_undo(&mut buffer);
        assert!(result.is_some());
        
        // After undo, should be back to just "Hello" on one line
        assert_eq!(buffer.line_count(), 1);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Test redo - should restore the newline and "World"
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some());
        
        // After redo, should have both lines again
        assert_eq!(buffer.line_count(), 2);
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        assert_eq!(buffer.get_line(1).unwrap(), "World");
    }

    #[test]
    fn test_insert_mode_backspace_only_creates_undo_action() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Set up initial content: "Hello World"
        for (i, ch) in "Hello World".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        
        // Simulate the scenario: Enter insert mode and only use backspace to delete existing content
        // This tests that deletions during insert mode are properly tracked for undo
        
        // Record a delete action as if insert mode tracked the deletion of " World"
        let deleted_text = " World";
        let deletion_pos = crate::buffer::Position::new(0, 5); // Position where " World" starts
        
        // Manually delete the text (simulating what would happen during insert mode)
        for _ in 0..deleted_text.len() {
            let delete_pos = crate::buffer::Position::new(0, buffer.line_length(0) - 1);
            buffer.delete_char(delete_pos);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Record the deletion action (this is what the fixed insert mode should do)
        let action = EditAction::delete_text(deletion_pos, deleted_text.to_string());
        history.push(action);
        
        // Test undo - should restore the deleted text
        let undo_result = history.apply_undo(&mut buffer);
        assert!(undo_result.is_some(), "Undo should be available for insert mode deletions");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World", "Undo should restore deleted text");
        
        // Test redo - should delete the text again
        let redo_result = history.apply_redo(&mut buffer);
        assert!(redo_result.is_some(), "Redo should be available");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello", "Redo should delete text again");
    }

    #[test]
    fn test_mixed_insert_and_delete_operations() {
        let mut buffer = crate::buffer::Buffer::new();
        let mut history = History::new();
        
        // Set up initial content: "Hello"
        for (i, ch) in "Hello".chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Test Case 1: Insert some text, then delete some existing content
        
        // 1. Insert " World" at the end
        let insert_pos = crate::buffer::Position::new(0, 5);
        let inserted_text = " World";
        for (i, ch) in inserted_text.chars().enumerate() {
            buffer.insert_char(crate::buffer::Position::new(0, 5 + i), ch);
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
        
        // Record the insertion
        let insert_action = EditAction::insert_text(insert_pos, inserted_text.to_string());
        history.push(insert_action);
        
        // 2. Delete " World" (simulating backspace in insert mode or 'x' command)
        let delete_pos = crate::buffer::Position::new(0, 5);
        let deleted_text = " World";
        for _ in 0..deleted_text.len() {
            buffer.delete_char(crate::buffer::Position::new(0, 5));
        }
        assert_eq!(buffer.get_line(0).unwrap(), "Hello");
        
        // Record the deletion
        let delete_action = EditAction::delete_text(delete_pos, deleted_text.to_string());
        history.push(delete_action);
        
        // Test undo sequence
        
        // First undo: should restore " World"
        let undo1 = history.apply_undo(&mut buffer);
        assert!(undo1.is_some(), "First undo should work");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World", "First undo should restore deleted text");
        
        // Second undo: should remove " World" again (undoing the insert)
        let undo2 = history.apply_undo(&mut buffer);
        assert!(undo2.is_some(), "Second undo should work");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello", "Second undo should remove inserted text");
        
        // Test redo sequence
        
        // First redo: should add " World" back
        let redo1 = history.apply_redo(&mut buffer);
        assert!(redo1.is_some(), "First redo should work");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World", "First redo should restore inserted text");
        
        // Second redo: should delete " World" again
        let redo2 = history.apply_redo(&mut buffer);
        assert!(redo2.is_some(), "Second redo should work");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello", "Second redo should delete text again");
        
        // Verify we can undo again
        let undo3 = history.apply_undo(&mut buffer);
        assert!(undo3.is_some(), "Third undo should work");
        assert_eq!(buffer.get_line(0).unwrap(), "Hello World", "Third undo should work correctly");
    }
}
