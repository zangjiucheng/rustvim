/// History management for undo/redo functionality
use crate::buffer::Position;

/// Tracks changes during an insert mode session
#[derive(Debug, Clone)]
pub struct InsertModeGroup {
    pub start_pos: Position,
    pub inserted_text: String,
    pub deleted_text: String,
    pub deletion_start_pos: Option<Position>,
}

impl InsertModeGroup {
    /// Create a new insert mode group starting at the given position
    pub fn new(start_pos: Position) -> Self {
        Self {
            start_pos,
            inserted_text: String::new(),
            deleted_text: String::new(),
            deletion_start_pos: None,
        }
    }

    /// Add inserted text to this group
    pub fn add_insertion(&mut self, text: String) {
        self.inserted_text.push_str(&text);
    }

    /// Add deleted text to this group
    pub fn add_deletion(&mut self, pos: Position, text: String) {
        if self.deletion_start_pos.is_none() {
            self.deletion_start_pos = Some(pos);
        }
        self.deleted_text.push_str(&text);
    }

    /// Convert this group to an EditAction
    pub fn to_edit_action(self) -> EditAction {
        EditAction::insert_mode_session(
            self.start_pos,
            self.inserted_text,
            self.deleted_text,
            self.deletion_start_pos,
        )
    }

    /// Check if this group has any changes
    pub fn has_changes(&self) -> bool {
        !self.inserted_text.is_empty() || !self.deleted_text.is_empty()
    }

    /// Add a single character to inserted text
    pub fn add_char(&mut self, ch: char) {
        self.inserted_text.push(ch);
    }

    /// Add a newline to inserted text
    pub fn add_newline(&mut self) {
        self.inserted_text.push('\n');
    }

    /// Remove the last character from inserted text
    pub fn remove_char(&mut self) {
        self.inserted_text.pop();
    }

    /// Add a deleted character at the given position
    pub fn add_deleted_char(&mut self, ch: char, pos: Position) {
        if self.deletion_start_pos.is_none() {
            self.deletion_start_pos = Some(pos);
        }
        self.deleted_text.push(ch);
    }
}

/// Represents different types of edit actions that can be undone/redone
#[derive(Debug, Clone)]
pub enum EditAction {
    /// Text was inserted at a position
    InsertText { pos: Position, text: String },
    /// Text was deleted from a position
    DeleteText { pos: Position, text: String },
    /// A complete insert mode session (may include both insertions and deletions)
    InsertModeSession {
        start_pos: Position,
        inserted_text: String,
        deleted_text: String,
        deletion_start_pos: Option<Position>,
    },
    /// Visual block delete operation (rectangular block of text)
    BlockDelete {
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
        deleted_text: Vec<String>,
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

    /// Create an insert mode session action
    pub fn insert_mode_session(
        start_pos: Position,
        inserted_text: String,
        deleted_text: String,
        deletion_start_pos: Option<Position>,
    ) -> Self {
        EditAction::InsertModeSession {
            start_pos,
            inserted_text,
            deleted_text,
            deletion_start_pos,
        }
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

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn apply_undo(
        &mut self,
        buffer: &mut crate::buffer::Buffer,
    ) -> Option<(EditAction, crate::buffer::Position)> {
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
                EditAction::InsertModeSession {
                    start_pos,
                    inserted_text,
                    deleted_text,
                    deletion_start_pos,
                } => {
                    // Undo the entire insert mode session
                    // First, delete any text that was inserted during the session
                    if !inserted_text.is_empty() {
                        Self::delete_inserted_text(buffer, start_pos, &inserted_text);
                    }

                    // Then, re-insert any text that was deleted during the session
                    if !deleted_text.is_empty() {
                        if let Some(del_pos) = deletion_start_pos {
                            Self::reinsert_deleted_text(buffer, del_pos, &deleted_text);
                        }
                    }

                    start_pos
                }
                EditAction::BlockDelete {
                    start_row,
                    start_col,
                    end_row: _,
                    end_col: _,
                    deleted_text,
                } => {
                    // Undo block delete by reinserting all the deleted text at original positions
                    for (i, line_text) in deleted_text.iter().enumerate() {
                        let row = start_row + i;
                        if !line_text.is_empty() {
                            for (j, ch) in line_text.chars().enumerate() {
                                let pos = crate::buffer::Position::new(row, start_col + j);
                                buffer.insert_char(pos, ch);
                            }
                        }
                    }
                    crate::buffer::Position::new(start_row, start_col)
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
    pub fn apply_redo(
        &mut self,
        buffer: &mut crate::buffer::Buffer,
    ) -> Option<(EditAction, crate::buffer::Position)> {
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
                EditAction::InsertModeSession {
                    start_pos,
                    inserted_text,
                    deleted_text,
                    deletion_start_pos,
                } => {
                    // First, delete any text that was inserted
                    if !deleted_text.is_empty() {
                        if let Some(del_pos) = deletion_start_pos {
                            Self::delete_text_from_buffer(buffer, del_pos, &deleted_text);
                        }
                    }

                    // Then, re-insert any text that was deleted
                    if !inserted_text.is_empty() {
                        Self::reinsert_deleted_text(buffer, start_pos, &inserted_text);
                    }

                    start_pos
                }
                EditAction::BlockDelete {
                    start_row,
                    start_col,
                    end_row: _,
                    end_col: _,
                    deleted_text,
                } => {
                    // Redo block delete by deleting the same rectangular area again
                    for (i, line_text) in deleted_text.iter().enumerate() {
                        let row = start_row + i;
                        if !line_text.is_empty() {
                            for j in (0..line_text.len()).rev() {
                                let pos = crate::buffer::Position::new(row, start_col + j);
                                buffer.delete_char(pos);
                            }
                        }
                    }
                    crate::buffer::Position::new(start_row, start_col)
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
    fn reinsert_deleted_text(
        buffer: &mut crate::buffer::Buffer,
        pos: crate::buffer::Position,
        text: &str,
    ) {
        if text.contains('\n') {
            if let Some(text_after_newline) = text.strip_prefix('\n') {
                // This could be either "O" command (newline at beginning) or "o" command (newline at end)
                // Format: "\nSomeText" or just "\n"

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
                            let insert_pos =
                                crate::buffer::Position::new(current_row, current_col + char_idx);
                            buffer.insert_char(insert_pos, ch);
                        }
                        // If there are more lines, insert a newline
                        if lines.len() > 1 && line_idx < lines.len() - 1 {
                            let newline_pos =
                                crate::buffer::Position::new(current_row, current_col + line.len());
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
    fn delete_text_from_buffer(
        buffer: &mut crate::buffer::Buffer,
        pos: crate::buffer::Position,
        text: &str,
    ) {
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
    fn delete_inserted_text(
        buffer: &mut crate::buffer::Buffer,
        start_pos: crate::buffer::Position,
        text: &str,
    ) {
        if text.contains('\n') {
            if let Some(text_after_newline) = text.strip_prefix('\n') {
                // This could be either "O" command (newline at beginning) or "o" command (newline at end)
                // Format: "\nSomeText" or just "\n"

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
                        let newline_pos =
                            crate::buffer::Position::new(start_pos.row, current_line_len);
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
                        let newline_pos =
                            crate::buffer::Position::new(start_pos.row, current_line_len);
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
                        let newline_pos =
                            crate::buffer::Position::new(current_row - 1, prev_line_len);
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
            (true, true) => format!("History: {undo_count} undo, {redo_count} redo"),
            (true, false) => format!("History: {undo_count} undo"),
            (false, true) => format!("History: {redo_count} redo"),
            (false, false) => "History: empty".to_string(),
        }
    }
}
