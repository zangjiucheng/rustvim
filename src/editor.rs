use crate::buffer::Buffer;
use crate::commands::Command;
use crate::history::History;
use crate::keymap::KeymapProcessor;
use crate::terminal::Terminal;

/// Represents the current mode of the editor
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
    Search,
}

/// Represents a cursor position in the buffer
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn at(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a single buffer
pub struct BufferInfo {
    /// The buffer contents
    pub buffer: Buffer,
    /// The filename (if any)
    pub filename: Option<String>,
    /// Whether the buffer has been modified
    pub modified: bool,
    /// Cursor position for this buffer
    pub cursor: Cursor,
    /// Scroll offset for this buffer
    pub scroll_offset: usize,
    /// Undo/redo history for this buffer
    pub history: History,
}

/// Main editor state and controller
pub struct Editor {
    /// Current editing mode
    pub mode: Mode,

    /// List of open buffers
    pub buffers: Vec<BufferInfo>,

    /// Index of the currently active buffer
    pub current_buffer: usize,

    /// Terminal interface
    pub terminal: Terminal,

    /// Whether the editor is running
    pub running: bool,

    /// Current status message
    pub status_msg: Option<String>,

    /// Pending count for commands (e.g., 5j to move down 5 lines)
    pub pending_count: Option<usize>,

    /// Pending operator for commands (e.g., d waiting for motion)
    pub pending_operator: Option<crate::commands::Operator>,

    /// Register to hold yanked/deleted text
    pub register: Register,

    /// Tracks changes during insert mode for grouping
    pub insert_mode_changes: Option<InsertModeGroup>,

    /// Current search query (for search mode and repeat search)
    pub search_query: Option<String>,

    /// Current search input buffer (while typing search query)
    pub search_input: String,

    /// Position of current search match (for highlighting)
    pub search_match: Option<(usize, usize, usize)>, // (row, col, length)

    /// Current command input buffer (while typing command)
    pub command_input: String,

    /// Keymap processor for handling key→action mappings
    pub keymap_processor: KeymapProcessor,
    /// Visual mode selection start position (when in Visual mode)
    pub visual_start: Option<Cursor>,

    /// Visual mode type: character-based (v), line-based (V), or block-based (Ctrl+V)
    pub visual_line_mode: bool,
    pub visual_block_mode: bool,
}

/// Represents the content and type of yanked/deleted text
#[derive(Debug, Clone)]
pub struct Register {
    /// The stored text content
    pub content: String,
    /// Whether the content is line-based (whole lines) or character-based
    pub is_line_based: bool,
}

impl Register {
    /// Create a new empty register
    pub fn new() -> Self {
        Self {
            content: String::new(),
            is_line_based: false,
        }
    }

    /// Store text as character-based content
    pub fn store_text(&mut self, text: String) {
        self.content = text;
        self.is_line_based = false;
    }

    /// Store text as line-based content (whole lines)
    pub fn store_lines(&mut self, text: String) {
        self.content = text;
        self.is_line_based = true;
    }

    /// Check if the register is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    /// Create a new editor instance
    pub fn new() -> Self {
        let buffers = vec![BufferInfo {
            buffer: Buffer::new(),
            filename: None,
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        }];

        Self {
            mode: Mode::Normal,
            buffers,
            current_buffer: 0,
            terminal: Terminal::new(),
            running: true,
            status_msg: None,
            pending_count: None,
            pending_operator: None,
            register: Register::new(),
            insert_mode_changes: None,
            search_query: None,
            search_input: String::new(),
            search_match: None,
            command_input: String::new(),
            keymap_processor: KeymapProcessor::new(),
            visual_start: None,
            visual_line_mode: false,
            visual_block_mode: false,
        }
    }

    /// Main editor event loop
    pub fn run(&mut self) -> std::io::Result<()> {
        // Enter raw mode
        let _raw_guard = self.terminal.enter_raw_mode()?;

        // Create input handler
        let mut input_handler = crate::input::InputHandler::new();

        // Initial screen refresh
        self.refresh_screen()?;

        // Main event loop
        while self.running {
            // Read key input
            let key = input_handler.read_key()?;

            // Clear status message on any key press (except in search/command mode)
            if self.mode != Mode::Search && self.mode != Mode::Command {
                self.clear_status_message();
            }

            // Handle global commands first
            if key == crate::input::Key::Esc {
                match self.mode {
                    Mode::Insert => {
                        // Exit insert mode, move cursor left if possible
                        self.end_insert_mode();
                        self.mode = Mode::Normal;
                        if self.cursor().col > 0 {
                            self.cursor_mut().col -= 1;
                        }
                    }
                    Mode::Search => {
                        // Cancel search and return to normal mode
                        self.mode = Mode::Normal;
                        self.search_input.clear();
                        self.search_match = None;
                    }
                    Mode::Command => {
                        // Cancel command and return to normal mode
                        self.mode = Mode::Normal;
                        self.command_input.clear();
                    }
                    Mode::Visual => {
                        // Exit visual mode when escape is pressed
                        self.exit_visual_mode();
                    }
                    Mode::Normal => {
                        // In normal mode, ESC does nothing (Vim behavior)
                        // Could clear status messages or pending operations
                        self.clear_status_message();
                        self.pending_count = None;
                        self.pending_operator = None;
                    }
                }
                self.refresh_screen()?;
                continue;
            }

            // Handle mode-specific commands using unified keymap processor
            self.handle_keymap_input(&key)?;

            // Refresh screen after each key press
            self.refresh_screen()?;
        }

        Ok(())
    }

    /// Refresh the screen display with current buffer content
    pub fn refresh_screen(&self) -> std::io::Result<()> {
        // Hide cursor during redraw
        self.terminal.hide_cursor()?;

        // Move to home position
        self.terminal.move_cursor_home()?;

        // Draw buffer content
        self.draw_buffer()?;

        // Draw status line
        self.draw_status_line()?;

        // Position cursor at editor cursor position
        let screen_row = self.cursor().row.saturating_sub(self.scroll_offset()) + 1;
        let screen_col = self.cursor().col + 1;
        self.terminal.move_cursor(screen_row, screen_col)?;

        // Show cursor
        self.terminal.show_cursor()?;

        Ok(())
    }

    /// Draw buffer content to screen
    fn draw_buffer(&self) -> std::io::Result<()> {
        let rows = self.terminal.rows();
        let cols = self.terminal.cols();

        // Reserve last row for status line
        let content_rows = rows.saturating_sub(1);

        for screen_row in 0..content_rows {
            let buffer_row = screen_row + self.scroll_offset();

            if buffer_row < self.buffer().line_count() {
                // Draw actual buffer line
                if let Some(line) = self.buffer().get_line(buffer_row) {
                    // Check if this line has a search match to highlight
                    if let Some((match_row, match_col, match_len)) = self.search_match {
                        if buffer_row == match_row {
                            // Split line into parts: before match, match, after match
                            let before = &line[..match_col.min(line.len())];
                            let match_end = (match_col + match_len).min(line.len());
                            let matched = &line[match_col.min(line.len())..match_end];
                            let after = &line[match_end..];

                            // Write before match
                            if !before.is_empty() {
                                self.terminal.write(before)?;
                            }

                            // Write highlighted match
                            if !matched.is_empty() {
                                self.terminal.write_highlighted(matched)?;
                            }

                            // Write after match
                            if !after.is_empty() {
                                self.terminal.write(after)?;
                            }
                        } else {
                            self.draw_line_with_visual_highlight(&line, buffer_row, cols)?;
                        }
                    } else {
                        self.draw_line_with_visual_highlight(&line, buffer_row, cols)?;
                    }
                }
            } else {
                // Draw tilde for empty lines (like Vim)
                self.terminal.write("~")?;
            }

            // Clear rest of line and move to next
            self.terminal.clear_line()?;
            if screen_row < content_rows - 1 {
                self.terminal.write("\r\n")?;
            }
        }

        Ok(())
    }

    /// Draw a line with visual mode highlighting if applicable
    fn draw_line_with_visual_highlight(
        &self,
        line: &str,
        row: usize,
        max_width: usize,
    ) -> std::io::Result<()> {
        if self.mode == Mode::Visual {
            if let Some((start, end)) = self.get_visual_selection() {
                if self.visual_block_mode {
                    // Block-wise selection: highlight rectangular block
                    if row >= start.row && row <= end.row {
                        let chars: Vec<char> = line.chars().collect();
                        let min_col = std::cmp::min(start.col, end.col);
                        let max_col = std::cmp::max(start.col, end.col);

                        let before = chars.iter().take(min_col).collect::<String>();
                        let selected = chars
                            .iter()
                            .skip(min_col)
                            .take(max_col + 1 - min_col)
                            .collect::<String>();
                        let after = chars.iter().skip(max_col + 1).collect::<String>();

                        self.terminal.write(&before)?;
                        if !selected.is_empty() {
                            self.terminal.write_highlighted(&selected)?;
                        }
                        self.terminal.write(&after)?;
                    } else {
                        // Not in block selection rows
                        self.terminal.write_truncated(line, max_width)?;
                    }
                } else if self.visual_line_mode && row >= start.row && row <= end.row {
                    // Line-wise selection: highlight entire line
                    self.terminal
                        .write_highlighted(&line.chars().take(max_width).collect::<String>())?;
                } else if !self.visual_line_mode {
                    // Character-wise selection
                    if start.row == end.row && row == start.row {
                        // Single line selection
                        let chars: Vec<char> = line.chars().collect();
                        let before = chars.iter().take(start.col).collect::<String>();
                        let selected = chars
                            .iter()
                            .skip(start.col)
                            .take(end.col + 1 - start.col)
                            .collect::<String>();
                        let after = chars.iter().skip(end.col + 1).collect::<String>();

                        self.terminal.write(&before)?;
                        if !selected.is_empty() {
                            self.terminal.write_highlighted(&selected)?;
                        }
                        self.terminal.write(&after)?;
                    } else if row == start.row {
                        // First line of multi-line selection
                        let chars: Vec<char> = line.chars().collect();
                        let before = chars.iter().take(start.col).collect::<String>();
                        let selected = chars.iter().skip(start.col).collect::<String>();

                        self.terminal.write(&before)?;
                        if !selected.is_empty() {
                            self.terminal.write_highlighted(&selected)?;
                        }
                    } else if row == end.row {
                        // Last line of multi-line selection
                        let chars: Vec<char> = line.chars().collect();
                        let selected = chars.iter().take(end.col + 1).collect::<String>();
                        let after = chars.iter().skip(end.col + 1).collect::<String>();

                        if !selected.is_empty() {
                            self.terminal.write_highlighted(&selected)?;
                        }
                        self.terminal.write(&after)?;
                    } else if row > start.row && row < end.row {
                        // Middle line of multi-line selection
                        self.terminal
                            .write_highlighted(&line.chars().take(max_width).collect::<String>())?;
                    } else {
                        // Not in selection
                        self.terminal.write_truncated(line, max_width)?;
                    }
                } else {
                    // Not in selection
                    self.terminal.write_truncated(line, max_width)?;
                }
            } else {
                // No visual selection
                self.terminal.write_truncated(line, max_width)?;
            }
        } else {
            // Not in visual mode
            self.terminal.write_truncated(line, max_width)?;
        }

        Ok(())
    }

    /// Draw status line at bottom of screen
    fn draw_status_line(&self) -> std::io::Result<()> {
        let rows = self.terminal.rows();
        let cols = self.terminal.cols();

        // Move to status line (last row)
        self.terminal.move_cursor(rows, 1)?;

        // Handle search mode specially
        if self.mode == Mode::Search {
            let search_prompt = format!("/{}", self.search_input);
            let padded_prompt = format!(
                "{}{}",
                search_prompt,
                " ".repeat(cols.saturating_sub(search_prompt.len()))
            );
            self.terminal.write_highlighted(&padded_prompt)?;
            return Ok(());
        }

        // Handle command mode specially
        if self.mode == Mode::Command {
            let command_prompt = format!(":{}", self.command_input);
            let padded_prompt = format!(
                "{}{}",
                command_prompt,
                " ".repeat(cols.saturating_sub(command_prompt.len()))
            );
            self.terminal.write_highlighted(&padded_prompt)?;
            return Ok(());
        }

        // Handle status messages
        if let Some(ref msg) = self.status_msg {
            let padded_msg = format!("{}{}", msg, " ".repeat(cols.saturating_sub(msg.len())));
            self.terminal.write_highlighted(&padded_msg)?;
            return Ok(());
        }

        // Create regular status line content
        let filename = self.filename().as_deref().unwrap_or("[No Name]");
        let modified = if self.is_modified() {
            " [Modified]"
        } else {
            ""
        };
        let buffer_info = format!("({}/{})", self.current_buffer + 1, self.buffers.len());
        let mode = match self.mode {
            Mode::Insert => "-- INSERT --".to_string(),
            Mode::Visual => {
                if self.visual_block_mode {
                    "-- VISUAL BLOCK --".to_string()
                } else if self.visual_line_mode {
                    "-- VISUAL LINE --".to_string()
                } else {
                    "-- VISUAL --".to_string()
                }
            }
            Mode::Command => "-- COMMAND --".to_string(),
            Mode::Search => "-- SEARCH --".to_string(),
            Mode::Normal => "".to_string(), // Normal mode shows no mode indicator
        };
        let position = format!("{}:{}", self.cursor().row + 1, self.cursor().col + 1);
        let lines = format!("{} lines", self.buffer().line_count());

        let left = format!("{}{} {} {}", filename, modified, buffer_info, mode);
        let right = format!("{} - {}", position, lines);

        // Calculate spacing
        let left_len = left.chars().count();
        let right_len = right.chars().count();
        let spacing = if left_len + right_len < cols {
            cols - left_len - right_len
        } else {
            0
        };

        // Write status line with background
        self.terminal
            .write_highlighted(&format!("{}{}{}", left, " ".repeat(spacing), right))?;

        Ok(())
    }

    /// Update scroll offset to keep cursor visible
    pub fn update_scroll(&mut self) {
        let rows = self.terminal.rows();
        let content_rows = rows.saturating_sub(1); // Reserve space for status line

        // Scroll up if cursor is above visible area
        if self.cursor().row < self.scroll_offset() {
            self.set_scroll_offset(self.cursor().row);
        }

        // Scroll down if cursor is below visible area
        if self.cursor().row >= self.scroll_offset() + content_rows {
            self.set_scroll_offset(self.cursor().row - content_rows + 1);
        }
    }

    /// Move cursor safely within buffer bounds
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        // Clamp row to buffer bounds
        self.cursor_mut().row = row.min(self.buffer().line_count().saturating_sub(1));

        // Clamp column to line length
        let line_len = self.buffer().line_length(self.cursor().row);
        self.cursor_mut().col = col.min(line_len);
    }

    /// Move cursor up one line
    pub fn cursor_up(&mut self) {
        if self.cursor().row > 0 {
            let new_row = self.cursor().row - 1;
            let line_len = self.buffer().line_length(new_row);
            self.cursor_mut().row = new_row;
            // In normal mode, cursor should not go past the last character
            self.cursor_mut().col = self.cursor().col.min(line_len.saturating_sub(1));
        }
    }

    /// Move cursor down one line
    pub fn cursor_down(&mut self) {
        if self.cursor().row + 1 < self.buffer().line_count() {
            let new_row = self.cursor().row + 1;
            let line_len = self.buffer().line_length(new_row);
            self.cursor_mut().row = new_row;
            // In normal mode, cursor should not go past the last character
            self.cursor_mut().col = self.cursor().col.min(line_len.saturating_sub(1));
        }
    }

    /// Move cursor left one position
    pub fn cursor_left(&mut self) {
        if self.cursor().col > 0 {
            self.cursor_mut().col -= 1;
        }
        // For now, don't wrap to previous line (keep it simple for Day 6)
    }

    /// Move cursor right one position
    pub fn cursor_right(&mut self) {
        let line_len = self.buffer().line_length(self.cursor().row);
        // In normal mode, don't go past the last character
        let max_col = line_len.saturating_sub(1);
        if self.cursor().col < max_col {
            self.cursor_mut().col += 1;
        }
        // For now, don't wrap to next line (keep it simple for Day 6)
    }

    /// Get current cursor position as Position
    pub fn cursor_position(&self) -> crate::buffer::Position {
        crate::buffer::Position::new(self.cursor().row, self.cursor().col)
    }

    /// Perform undo operation
    pub fn undo(&mut self) {
        let current_buffer = &mut self.buffers[self.current_buffer];
        if let Some((_action, cursor_pos)) = current_buffer
            .history
            .apply_undo(&mut current_buffer.buffer)
        {
            current_buffer.cursor.row = cursor_pos.row;
            current_buffer.cursor.col = cursor_pos.col;
            current_buffer.modified = true;

            // Enhanced status message with undo count
            let remaining = current_buffer.history.undo_count();
            if remaining > 0 {
                self.status_msg = Some(format!("Undone ({} more available)", remaining));
            } else {
                self.status_msg = Some("Undone (oldest change)".to_string());
            }
        } else {
            self.status_msg = Some("Already at oldest change".to_string());
        }
        self.update_scroll();
    }

    /// Perform redo operation
    pub fn redo(&mut self) {
        let current_buffer = &mut self.buffers[self.current_buffer];
        if let Some((_action, cursor_pos)) = current_buffer
            .history
            .apply_redo(&mut current_buffer.buffer)
        {
            current_buffer.cursor.row = cursor_pos.row;
            current_buffer.cursor.col = cursor_pos.col;
            current_buffer.modified = true;

            // Enhanced status message with redo count
            let remaining = current_buffer.history.redo_count();
            if remaining > 0 {
                self.status_msg = Some(format!("Redone ({} more available)", remaining));
            } else {
                self.status_msg = Some("Redone (newest change)".to_string());
            }
        } else {
            self.status_msg = Some("Already at newest change".to_string());
        }
        self.update_scroll();
    }

    /// Start tracking insert mode changes
    pub fn start_insert_mode(&mut self) {
        let start_pos = crate::buffer::Position::new(self.cursor().row, self.cursor().col);
        self.insert_mode_changes = Some(InsertModeGroup::new(start_pos));
    }

    /// End tracking insert mode changes and record them as a single undo action
    pub fn end_insert_mode(&mut self) {
        if let Some(changes) = self.insert_mode_changes.take() {
            if changes.has_changes() {
                // Create a single composite undo action for the entire insert session
                let action = crate::history::EditAction::insert_mode_session(
                    changes.start_pos,
                    changes.inserted_text,
                    changes.deleted_text,
                    changes.deletion_start_pos,
                );
                self.history_mut().push(action);
            }
        }
    }

    /// Record a character insertion during insert mode
    pub fn insert_mode_char(&mut self, ch: char) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            changes.add_char(ch);
        }
    }

    /// Record a newline insertion during insert mode
    pub fn insert_mode_newline(&mut self) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            changes.add_newline();
        }
    }

    /// Record a character deletion during insert mode (backspace)
    pub fn insert_mode_backspace(
        &mut self,
        deleted_char: Option<char>,
        deletion_pos: Option<crate::buffer::Position>,
    ) {
        if let Some(ref mut changes) = self.insert_mode_changes {
            // Always try to remove from recently inserted text first
            if !changes.inserted_text.is_empty() {
                changes.remove_char();
            } else if let (Some(ch), Some(pos)) = (deleted_char, deletion_pos) {
                // If we're not removing recent insertions, this is deleting existing buffer content
                changes.add_deleted_char(ch, pos);
            }
        }
    }

    /// Start tracking insert mode changes with initial text already inserted
    pub fn start_insert_mode_with_initial_text(
        &mut self,
        start_pos: crate::buffer::Position,
        initial_text: String,
    ) {
        let mut changes = InsertModeGroup::new(start_pos);
        changes.inserted_text = initial_text;
        self.insert_mode_changes = Some(changes);
    }

    /// Start search mode with / command
    pub fn start_search(&mut self) {
        self.mode = Mode::Search;
        self.search_input.clear();
        self.search_match = None; // Clear previous highlighting
    }

    /// Search forward from current cursor position
    pub fn search_forward(&mut self, query: &str) {
        if query.is_empty() {
            return;
        }

        // Save the query for repeat searches
        self.search_query = Some(query.to_string());

        let start_row = self.cursor().row;
        let start_col = self.cursor().col + 1; // Start after current position

        self.search_from_position(query, start_row, start_col);
    }

    /// Search from a specific position
    fn search_from_position(&mut self, query: &str, start_row: usize, start_col: usize) {
        // Search from given position to end of buffer
        if let Some((row, col)) = self.find_text_from_position(query, start_row, start_col) {
            self.move_cursor_to_match(row, col, query.len());
            return;
        }

        // If not found after start position, wrap around and search from beginning
        if let Some((row, col)) = self.find_text_from_position(query, 0, 0) {
            if row < start_row || (row == start_row && col < start_col) {
                self.move_cursor_to_match(row, col, query.len());
                self.set_status_message("Search wrapped around".to_string());
                return;
            }
        }

        // Pattern not found
        self.set_status_message("Pattern not found".to_string());
        self.search_match = None;
    }

    /// Search for next occurrence of last query
    pub fn search_next(&mut self) {
        if let Some(ref query) = self.search_query.clone() {
            // If we have a current match, start searching after it
            let (start_row, start_col) = if let Some((row, col, length)) = self.search_match {
                (row, col + length)
            } else {
                (self.cursor().row, self.cursor().col + 1)
            };

            self.search_from_position(query, start_row, start_col);
        }
    }

    /// Search for previous occurrence of last query
    pub fn search_previous(&mut self) {
        if let Some(ref query) = self.search_query.clone() {
            // If we have a current match, start searching before it
            let (start_row, start_col) = if let Some((row, col, _length)) = self.search_match {
                if col > 0 {
                    (row, col - 1)
                } else if row > 0 {
                    // Move to end of previous line
                    let prev_row = row - 1;
                    let prev_line_len = self
                        .buffer()
                        .get_line(prev_row)
                        .map(|line| line.len())
                        .unwrap_or(0);
                    (prev_row, prev_line_len)
                } else {
                    // At beginning of buffer, search from end
                    let last_row = self.buffer().line_count().saturating_sub(1);
                    let last_col = self
                        .buffer()
                        .get_line(last_row)
                        .map(|line| line.len())
                        .unwrap_or(0);
                    (last_row, last_col)
                }
            } else {
                (self.cursor().row, self.cursor().col)
            };

            self.search_backward_from_position(query, start_row, start_col);
        }
    }

    /// Search backward from current cursor position
    pub fn search_backward(&mut self, query: &str) {
        if query.is_empty() {
            return;
        }

        // Save the query for repeat searches
        self.search_query = Some(query.to_string());

        self.search_backward_from_position(query, self.cursor().row, self.cursor().col);
    }

    /// Search backward from a specific position
    fn search_backward_from_position(&mut self, query: &str, start_row: usize, start_col: usize) {
        // Search from beginning to current position
        let mut last_match: Option<(usize, usize)> = None;

        for row in 0..=start_row {
            if let Some(line) = self.buffer().get_line(row) {
                let search_limit = if row == start_row {
                    start_col.min(line.len())
                } else {
                    line.len()
                };

                let search_text = &line[..search_limit];
                let mut start = 0;

                while let Some(pos) = search_text[start..].find(query) {
                    let actual_pos = start + pos;

                    // Skip the current match if we're sitting on it
                    if let Some((current_row, current_col, _)) = self.search_match {
                        if row == current_row && actual_pos == current_col {
                            start = actual_pos + 1;
                            continue;
                        }
                    }

                    last_match = Some((row, actual_pos));
                    start = actual_pos + 1;

                    if start >= search_limit {
                        break;
                    }
                }
            }
        }

        if let Some((row, col)) = last_match {
            self.move_cursor_to_match(row, col, query.len());
        } else {
            // If not found before cursor, wrap around and search from end
            self.search_backward_wrap_around(query);
        }
    }

    /// Wrap around search for backward direction
    fn search_backward_wrap_around(&mut self, query: &str) {
        let mut last_match_wrapped: Option<(usize, usize)> = None;

        // Search the entire buffer from the end to find the last occurrence
        for row in 0..self.buffer().line_count() {
            if let Some(line) = self.buffer().get_line(row) {
                let mut start = 0;

                // Find all occurrences in this line and keep the last one
                while let Some(pos) = line[start..].find(query) {
                    let actual_pos = start + pos;

                    // Only consider matches that are after our current position (for wrap-around)
                    // or if we're not on the same row
                    if row > self.cursor().row
                        || (row == self.cursor().row && actual_pos > self.cursor().col)
                    {
                        last_match_wrapped = Some((row, actual_pos));
                    }

                    start = actual_pos + 1;
                }
            }
        }

        // If no match found after cursor, find the very last match in the entire buffer
        if last_match_wrapped.is_none() {
            for row in 0..self.buffer().line_count() {
                if let Some(line) = self.buffer().get_line(row) {
                    let mut start = 0;

                    while let Some(pos) = line[start..].find(query) {
                        let actual_pos = start + pos;
                        last_match_wrapped = Some((row, actual_pos));
                        start = actual_pos + 1;
                    }
                }
            }
        }

        if let Some((row, col)) = last_match_wrapped {
            self.move_cursor_to_match(row, col, query.len());
            self.set_status_message("Search wrapped around".to_string());
        } else {
            self.set_status_message("Pattern not found".to_string());
            self.search_match = None;
        }
    }

    /// Find text starting from a specific position
    fn find_text_from_position(
        &self,
        query: &str,
        start_row: usize,
        start_col: usize,
    ) -> Option<(usize, usize)> {
        for row in start_row..self.buffer().line_count() {
            if let Some(line) = self.buffer().get_line(row) {
                let search_start = if row == start_row { start_col } else { 0 };

                if search_start < line.len() {
                    let search_text = &line[search_start..];
                    if let Some(pos) = search_text.find(query) {
                        return Some((row, search_start + pos));
                    }
                }
            }
        }
        None
    }

    /// Move cursor to search match and set up highlighting
    fn move_cursor_to_match(&mut self, row: usize, col: usize, length: usize) {
        self.cursor_mut().row = row;
        self.cursor_mut().col = col;
        self.search_match = Some((row, col, length));
        self.update_scroll();
    }

    /// Set a temporary status message
    pub fn set_status_message(&mut self, message: String) {
        self.status_msg = Some(message);
    }

    /// Clear the status message
    pub fn clear_status_message(&mut self) {
        self.status_msg = None;
    }

    /// Start command mode with : command
    pub fn start_command_mode(&mut self) {
        self.mode = Mode::Command;
        self.command_input.clear();
    }

    /// Execute Ex command (like :w, :q, :wq, etc.)
    pub fn execute_ex_command(&mut self, command: &str) {
        let ex_command = crate::commands::ExCommandParser::parse(command);
        let _ = ex_command.execute(self);
    }

    /// Delete text in a range (for keymap system)
    pub fn delete_range(
        &mut self,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
    ) -> Result<(), String> {
        crate::commands::TextOperations::delete_range(self, start_pos, end_pos);
        Ok(())
    }

    /// Yank text in a range (for keymap system)
    pub fn yank_range(
        &mut self,
        start_pos: (usize, usize),
        end_pos: (usize, usize),
    ) -> Result<(), String> {
        let yanked_text = crate::commands::TextOperations::extract_range(self, start_pos, end_pos);
        if !yanked_text.is_empty() {
            self.register.store_text(yanked_text);
            self.status_msg = Some("Text yanked".to_string());
        }
        Ok(())
    }

    /// Configure global keymap settings (example usage)
    pub fn configure_keymap(&mut self, config: crate::keymap::KeymapConfig) {
        self.keymap_processor.update_config(config);
    }

    /// Add custom key binding (convenience method)
    pub fn bind_key(&mut self, mode: Mode, key: crate::input::Key, action: crate::keymap::Action) {
        self.keymap_processor.keymap_mut().bind(mode, key, action);
    }

    /// Get current keymap configuration (for saving to file)
    pub fn get_keymap_config(&self) -> crate::keymap::KeymapConfig {
        self.keymap_processor.get_config()
    }

    /// Reset keymap to defaults
    pub fn reset_keymap_to_defaults(&mut self) {
        self.keymap_processor = KeymapProcessor::new();
    }

    // ==== Accessor methods for buffer, cursor, and state ====

    /// Get reference to current buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer].buffer
    }

    /// Get mutable reference to current buffer
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer].buffer
    }

    /// Get reference to current cursor
    pub fn cursor(&self) -> &Cursor {
        &self.buffers[self.current_buffer].cursor
    }

    /// Get mutable reference to current cursor
    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.buffers[self.current_buffer].cursor
    }

    /// Get current filename
    pub fn filename(&self) -> &Option<String> {
        &self.buffers[self.current_buffer].filename
    }

    /// Set filename for current buffer
    pub fn set_filename(&mut self, filename: Option<String>) {
        self.buffers[self.current_buffer].filename = filename;
    }

    /// Check if current buffer is modified
    pub fn is_modified(&self) -> bool {
        self.buffers[self.current_buffer].modified
    }

    /// Set modified flag for current buffer
    pub fn set_modified(&mut self, modified: bool) {
        self.buffers[self.current_buffer].modified = modified;
    }

    /// Get current scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.buffers[self.current_buffer].scroll_offset
    }

    /// Set scroll offset for current buffer
    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.buffers[self.current_buffer].scroll_offset = offset;
    }

    /// Get mutable reference to current buffer's history
    pub fn history_mut(&mut self) -> &mut History {
        &mut self.buffers[self.current_buffer].history
    }

    /// Get reference to current buffer's history
    pub fn history(&self) -> &History {
        &self.buffers[self.current_buffer].history
    }

    /// Get buffer information by filename (returns buffer index and line count)
    pub fn get_buffer_info(&self, filename: &str) -> Option<(usize, usize)> {
        for (index, buffer) in self.buffers.iter().enumerate() {
            if let Some(ref buf_filename) = buffer.filename {
                if buf_filename == filename {
                    return Some((index, buffer.buffer.line_count()));
                }
            }
        }
        None
    }

    // ==== File I/O methods (implemented in io.rs) ====

    /// Add a new buffer and switch to it
    pub fn add_buffer(&mut self, buffer_info: BufferInfo) {
        self.buffers.push(buffer_info);
        self.current_buffer = self.buffers.len() - 1;
    }

    /// Switch to next buffer
    pub fn next_buffer(&mut self) {
        if self.buffers.len() > 1 {
            self.current_buffer = (self.current_buffer + 1) % self.buffers.len();
        }
    }

    /// Switch to previous buffer
    pub fn prev_buffer(&mut self) {
        if self.buffers.len() > 1 {
            self.current_buffer = if self.current_buffer == 0 {
                self.buffers.len() - 1
            } else {
                self.current_buffer - 1
            };
        }
    }

    /// Switch to specific buffer by index
    pub fn switch_to_buffer(&mut self, index: usize) -> bool {
        if index < self.buffers.len() {
            self.current_buffer = index;
            true
        } else {
            false
        }
    }

    /// List all buffers for display
    pub fn list_buffers(&self) -> Vec<String> {
        self.buffers
            .iter()
            .enumerate()
            .map(|(i, buf)| {
                let name = buf.filename.as_deref().unwrap_or("[No Name]");
                let modified = if buf.modified { "+" } else { "" };
                let current = if i == self.current_buffer { "%" } else { " " };
                format!("{}{} {} {}", i + 1, current, name, modified)
            })
            .collect()
    }

    // ==== Keymap handling methods ====

    /// Handle keymap input for any mode
    pub fn handle_keymap_input(&mut self, key: &crate::input::Key) -> std::io::Result<()> {
        // We need to extract the keymap processor temporarily to avoid borrowing issues
        let mut keymap_processor = std::mem::take(&mut self.keymap_processor);
        let _ = keymap_processor.process_key(self, key);
        self.keymap_processor = keymap_processor;
        Ok(())
    }
}

/// Tracks changes made during a single insert mode session
#[derive(Debug, Clone)]
pub struct InsertModeGroup {
    /// The starting position where insert mode began
    pub start_pos: crate::buffer::Position,
    /// The accumulated text that was inserted
    pub inserted_text: String,
    /// The text that was deleted during this insert session (for backspace on existing content)
    pub deleted_text: String,
    /// The position where deletions started (for backspace on existing content)
    pub deletion_start_pos: Option<crate::buffer::Position>,
}

impl InsertModeGroup {
    pub fn new(start_pos: crate::buffer::Position) -> Self {
        Self {
            start_pos,
            inserted_text: String::new(),
            deleted_text: String::new(),
            deletion_start_pos: None,
        }
    }

    pub fn add_char(&mut self, ch: char) {
        self.inserted_text.push(ch);
    }

    pub fn add_newline(&mut self) {
        self.inserted_text.push('\n');
    }

    pub fn remove_char(&mut self) {
        self.inserted_text.pop();
    }

    /// Record a character that was deleted from existing buffer content (not just undoing recent insertions)
    pub fn add_deleted_char(&mut self, ch: char, pos: crate::buffer::Position) {
        if self.deletion_start_pos.is_none() {
            // For backspace operations, the restoration position should be the leftmost deleted position
            self.deletion_start_pos = Some(pos);
        } else {
            // Update deletion_start_pos to the leftmost position
            if let Some(current_pos) = self.deletion_start_pos {
                if pos.row < current_pos.row
                    || (pos.row == current_pos.row && pos.col < current_pos.col)
                {
                    self.deletion_start_pos = Some(pos);
                }
            }
        }
        self.deleted_text.insert(0, ch); // Insert at beginning to maintain order
    }

    /// Check if this insert session has any changes (insertions or deletions)
    pub fn has_changes(&self) -> bool {
        !self.inserted_text.is_empty() || !self.deleted_text.is_empty()
    }
}

impl Editor {
    // === Visual Mode Methods ===

    /// Enter character-wise Visual mode (v)
    pub fn enter_visual_mode(&mut self) {
        self.mode = Mode::Visual;
        self.visual_start = Some(*self.cursor());
        self.visual_line_mode = false;
        self.visual_block_mode = false;
    }

    /// Enter line-wise Visual mode (V)
    pub fn enter_visual_line_mode(&mut self) {
        self.mode = Mode::Visual;
        self.visual_start = Some(*self.cursor());
        self.visual_line_mode = true;
        self.visual_block_mode = false;
    }

    /// Enter block-wise Visual mode (Ctrl+V)
    pub fn enter_visual_block_mode(&mut self) {
        self.mode = Mode::Visual;
        self.visual_start = Some(*self.cursor());
        self.visual_line_mode = false;
        self.visual_block_mode = true;
    }

    /// Exit Visual mode and return to Normal mode
    pub fn exit_visual_mode(&mut self) {
        self.mode = Mode::Normal;
        self.visual_start = None;
        self.visual_line_mode = false;
        self.visual_block_mode = false;
    }

    /// Get the current visual selection range (start, end)
    /// Returns None if not in Visual mode
    pub fn get_visual_selection(&self) -> Option<(Cursor, Cursor)> {
        if self.mode != Mode::Visual {
            return None;
        }

        let start = self.visual_start?;
        let end = *self.cursor();

        // Ensure start is before end
        let (start, end) = if start.row < end.row || (start.row == end.row && start.col <= end.col)
        {
            (start, end)
        } else {
            (end, start)
        };

        Some((start, end))
    }

    /// Check if a position is within the visual selection
    pub fn is_in_visual_selection(&self, row: usize, col: usize) -> bool {
        if let Some((start, end)) = self.get_visual_selection() {
            if self.visual_block_mode {
                // Block-wise selection: rectangular block
                let min_col = std::cmp::min(start.col, end.col);
                let max_col = std::cmp::max(start.col, end.col);
                row >= start.row && row <= end.row && col >= min_col && col <= max_col
            } else if self.visual_line_mode {
                // Line-wise selection: entire lines
                row >= start.row && row <= end.row
            } else {
                // Character-wise selection
                if start.row == end.row {
                    // Single line selection
                    row == start.row && col >= start.col && col <= end.col
                } else {
                    // Multi-line selection
                    if row == start.row {
                        col >= start.col
                    } else if row == end.row {
                        col <= end.col
                    } else {
                        row > start.row && row < end.row
                    }
                }
            }
        } else {
            false
        }
    }

    /// Delete the current visual selection
    pub fn delete_visual_selection(&mut self) -> Result<(), String> {
        let selection = self.get_visual_selection().ok_or("No visual selection")?;
        let (start, end) = selection;

        if self.visual_block_mode {
            // Delete rectangular block
            let min_col = std::cmp::min(start.col, end.col);
            let max_col = std::cmp::max(start.col, end.col);

            // Collect all the text that will be deleted for both register and undo
            let mut deleted_block = Vec::new();

            // First pass: collect all the text that will be deleted
            for row in start.row..=end.row {
                if row < self.buffer().line_count() {
                    if let Some(line) = self.buffer().get_line(row) {
                        let chars: Vec<char> = line.chars().collect();
                        let actual_max_col = std::cmp::min(max_col, chars.len().saturating_sub(1));
                        let actual_min_col = std::cmp::min(min_col, chars.len());

                        if actual_min_col <= actual_max_col && actual_min_col < chars.len() {
                            let deleted_part: String =
                                chars[actual_min_col..=actual_max_col].iter().collect();
                            deleted_block.push(deleted_part);
                        } else {
                            deleted_block.push(String::new());
                        }
                    } else {
                        deleted_block.push(String::new());
                    }
                } else {
                    deleted_block.push(String::new());
                }
            }

            // Store in register for pasting
            if !deleted_block.is_empty() {
                let deleted_text = deleted_block.join("\n");
                self.register.store_text(deleted_text);
            }

            // Create a custom block undo action that will restore the entire block
            let block_undo = crate::history::EditAction::BlockDelete {
                start_row: start.row,
                start_col: min_col,
                end_row: end.row,
                end_col: max_col,
                deleted_text: deleted_block.clone(),
            };
            self.history_mut().push(block_undo);

            // Second pass: actually delete the characters from bottom to top
            for row in (start.row..=end.row).rev() {
                if row < self.buffer().line_count() {
                    if let Some(line) = self.buffer().get_line(row) {
                        let chars: Vec<char> = line.chars().collect();
                        let actual_max_col = std::cmp::min(max_col, chars.len().saturating_sub(1));
                        let actual_min_col = std::cmp::min(min_col, chars.len());

                        if actual_min_col <= actual_max_col && actual_min_col < chars.len() {
                            // Delete characters one by one from right to left to maintain indices
                            for col in (actual_min_col..=actual_max_col).rev() {
                                let pos = crate::buffer::Position::new(row, col);
                                self.buffer_mut().delete_char(pos);
                            }
                        }
                    }
                }
            }

            // Position cursor at top-left of the deleted block
            self.move_cursor(start.row, min_col);
        } else if self.visual_line_mode {
            // Delete entire lines using proper line deletion
            let line_count = end.row - start.row + 1;

            // First collect the lines for register and undo
            let mut deleted_lines = Vec::new();
            for i in 0..line_count {
                let row = start.row + i;
                if row < self.buffer().line_count() {
                    if let Some(line) = self.buffer().get_line(row) {
                        deleted_lines.push(line.clone());
                    }
                }
            }

            // Store in register
            if !deleted_lines.is_empty() {
                let deleted_text = deleted_lines.join("\n") + "\n";
                self.register.store_lines(deleted_text.clone());

                // Record the line deletion for undo
                let delete_pos = crate::buffer::Position::new(start.row, 0);
                let action = crate::history::EditAction::delete_text(delete_pos, deleted_text);
                self.history_mut().push(action);
            }

            // Delete the lines properly by removing them from the buffer
            for _ in 0..line_count {
                if self.buffer().line_count() > 1 && start.row < self.buffer().line_count() {
                    // Remove the line completely
                    self.buffer_mut().remove_line(start.row);
                } else if self.buffer().line_count() == 1 {
                    // If it's the last line in the buffer, just clear it
                    self.buffer_mut().clear_line(start.row);
                    break;
                }
            }

            // Ensure cursor is within bounds
            if start.row >= self.buffer().line_count() {
                self.cursor_mut().row = self.buffer().line_count().saturating_sub(1);
            } else {
                self.cursor_mut().row = start.row;
            }
            self.cursor_mut().col = 0;
        } else {
            // Delete character range with proper undo tracking
            let start_pos = (start.row, start.col);
            let end_pos = if end.col < self.buffer().line_length(end.row) {
                (end.row, end.col + 1) // Make inclusive by adding 1 if not at line end
            } else {
                (end.row, end.col) // At line end, don't go beyond
            };

            // Extract text for register and undo
            let deleted_text =
                crate::commands::TextOperations::extract_range(self, start_pos, end_pos);
            if !deleted_text.is_empty() {
                self.register.store_text(deleted_text.clone());

                // Record the deletion for undo
                let delete_pos = crate::buffer::Position::new(start.row, start.col);
                let action = crate::history::EditAction::delete_text(delete_pos, deleted_text);
                self.history_mut().push(action);
            }

            // Delete the range
            crate::commands::TextOperations::delete_range(self, start_pos, end_pos);

            // Position cursor at start of deleted range
            self.move_cursor(start.row, start.col);
        }

        self.set_modified(true);
        self.update_scroll();

        // Exit visual mode
        self.exit_visual_mode();

        Ok(())
    }

    /// Yank (copy) the current visual selection
    pub fn yank_visual_selection(&mut self) -> Result<(), String> {
        let selection = self.get_visual_selection().ok_or("No visual selection")?;
        let (start, end) = selection;

        if self.visual_block_mode {
            // Yank rectangular block
            let min_col = std::cmp::min(start.col, end.col);
            let max_col = std::cmp::max(start.col, end.col);
            let mut yanked_lines = Vec::new();

            for row in start.row..=end.row {
                if row < self.buffer().line_count() {
                    if let Some(line) = self.buffer().get_line(row) {
                        let chars: Vec<char> = line.chars().collect();
                        let actual_max_col = std::cmp::min(max_col, chars.len().saturating_sub(1));
                        let actual_min_col = std::cmp::min(min_col, chars.len());

                        if actual_min_col <= actual_max_col && actual_min_col < chars.len() {
                            let yanked_part: String =
                                chars[actual_min_col..=actual_max_col].iter().collect();
                            yanked_lines.push(yanked_part);
                        } else {
                            yanked_lines.push(String::new());
                        }
                    }
                }
            }

            let yanked_text = yanked_lines.join("\n");
            self.register.store_text(yanked_text);
        } else if self.visual_line_mode {
            // Yank entire lines
            let yanked_text = self.extract_line_range(start.row, end.row);
            self.register.store_lines(yanked_text);
        } else {
            // Yank character range using existing method
            // For character-wise visual selection, we need to include the end character
            let start_pos = (start.row, start.col);
            let end_pos = if end.col < self.buffer().line_length(end.row) {
                (end.row, end.col + 1) // Make inclusive by adding 1 if not at line end
            } else {
                (end.row, end.col) // At line end, don't go beyond
            };
            self.yank_range(start_pos, end_pos)?;
        }

        // Exit visual mode
        self.exit_visual_mode();

        Ok(())
    }

    /// Extract text from a range of lines (inclusive)
    fn extract_line_range(&self, start_row: usize, end_row: usize) -> String {
        let buffer = self.buffer();
        let mut text = String::new();

        for row in start_row..=end_row {
            if let Some(line) = buffer.get_line(row) {
                text.push_str(&line);
                text.push('\n');
            }
        }

        text
    }
}
