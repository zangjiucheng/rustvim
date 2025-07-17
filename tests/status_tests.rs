#[cfg(test)]
mod status_tests {
    use rustvim::buffer::Buffer;
    use rustvim::editor::{BufferInfo, Cursor, Editor, Mode};
    use rustvim::history::History;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_status_line_single_buffer() {
        let editor = Editor::new();

        // Test single buffer with no name
        assert_eq!(editor.buffers.len(), 1);
        assert_eq!(editor.current_buffer, 0);

        // Check that filename returns None for new buffer
        assert_eq!(*editor.filename(), None);
        assert!(!editor.is_modified());

        // Verify buffer count and current index
        assert_eq!(editor.current_buffer + 1, 1);
        assert_eq!(editor.buffers.len(), 1);
    }

    #[test]
    fn test_status_line_multiple_buffers() {
        let mut editor = Editor::new();

        // Add a second buffer
        let buffer_info = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file1.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info);

        // Should now have 2 buffers, current should be 1 (second buffer)
        assert_eq!(editor.buffers.len(), 2);
        assert_eq!(editor.current_buffer, 1);
        assert_eq!(*editor.filename(), Some("file1.txt".to_string()));

        // Add a third buffer
        let buffer_info2 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file2.txt".to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);

        // Should now have 3 buffers, current should be 2 (third buffer)
        assert_eq!(editor.buffers.len(), 3);
        assert_eq!(editor.current_buffer, 2);
        assert_eq!(*editor.filename(), Some("file2.txt".to_string()));
        assert!(editor.is_modified());
    }

    #[test]
    fn test_status_line_buffer_navigation() {
        let mut editor = Editor::new();

        // Add two more buffers for a total of 3
        let buffer_info1 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file1.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info1);

        let buffer_info2 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file2.txt".to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);

        // Currently at buffer 2 (3rd buffer)
        assert_eq!(editor.current_buffer, 2);
        assert_eq!(editor.buffers.len(), 3);

        // Test next buffer navigation
        editor.next_buffer();
        assert_eq!(editor.current_buffer, 0); // Should wrap to first buffer

        editor.next_buffer();
        assert_eq!(editor.current_buffer, 1); // Second buffer

        editor.next_buffer();
        assert_eq!(editor.current_buffer, 2); // Third buffer

        // Test previous buffer navigation
        editor.prev_buffer();
        assert_eq!(editor.current_buffer, 1); // Second buffer

        editor.prev_buffer();
        assert_eq!(editor.current_buffer, 0); // First buffer

        editor.prev_buffer();
        assert_eq!(editor.current_buffer, 2); // Should wrap to last buffer
    }

    #[test]
    fn test_status_line_modified_flag() {
        let mut editor = Editor::new();

        // Initially not modified
        assert!(!editor.is_modified());

        // Mark as modified
        editor.set_modified(true);
        assert!(editor.is_modified());

        // Mark as not modified
        editor.set_modified(false);
        assert!(!editor.is_modified());
    }

    #[test]
    fn test_status_line_with_filename() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("test_status.txt");

        // Create a test file
        fs::write(&test_file, "Test content\nLine 2\n").expect("Failed to write test file");

        let mut editor = Editor::new();
        editor.set_filename(Some(test_file.to_str().unwrap().to_string()));

        // Load content into buffer
        let content = fs::read_to_string(&test_file).expect("Failed to read test file");
        *editor.buffer_mut() = Buffer::from_file(&content);

        assert_eq!(
            *editor.filename(),
            Some(test_file.to_str().unwrap().to_string())
        );
        assert_eq!(editor.buffer().line_count(), 2);

        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_status_line_buffer_info_format() {
        let mut editor = Editor::new();

        // Test with single buffer - should be (1/1)
        assert_eq!(editor.current_buffer + 1, 1);
        assert_eq!(editor.buffers.len(), 1);

        // Add more buffers
        for i in 1..4 {
            let buffer_info = BufferInfo {
                buffer: Buffer::new(),
                filename: Some(format!("file{i}.txt")),
                modified: i % 2 == 0, // Alternate modified state
                cursor: Cursor::new(),
                scroll_offset: 0,
                history: History::new(),
            };
            editor.add_buffer(buffer_info);
        }

        // Should now have 4 buffers, current is 3 (4th buffer)
        assert_eq!(editor.current_buffer + 1, 4);
        assert_eq!(editor.buffers.len(), 4);

        // Navigate to different buffers and verify indexing
        editor.switch_to_buffer(0);
        assert_eq!(editor.current_buffer + 1, 1);
        assert_eq!(editor.buffers.len(), 4);

        editor.switch_to_buffer(1);
        assert_eq!(editor.current_buffer + 1, 2);
        assert_eq!(editor.buffers.len(), 4);

        editor.switch_to_buffer(2);
        assert_eq!(editor.current_buffer + 1, 3);
        assert_eq!(editor.buffers.len(), 4);
    }

    #[test]
    fn test_status_line_different_modes() {
        let mut editor = Editor::new();

        // Test Normal mode
        assert_eq!(editor.mode, Mode::Normal);

        // Test Insert mode
        editor.mode = Mode::Insert;
        assert_eq!(editor.mode, Mode::Insert);

        // Test Command mode
        editor.mode = Mode::Command;
        assert_eq!(editor.mode, Mode::Command);

        // Test Search mode
        editor.mode = Mode::Search;
        assert_eq!(editor.mode, Mode::Search);

        // Test Visual mode
        editor.mode = Mode::Visual;
        assert_eq!(editor.mode, Mode::Visual);
    }

    #[test]
    fn test_status_line_cursor_position() {
        let mut editor = Editor::new();

        // Add some content to test cursor positioning
        editor.buffer_mut().insert_line(0, "First line".to_string());
        editor
            .buffer_mut()
            .insert_line(1, "Second line".to_string());
        editor.buffer_mut().insert_line(2, "Third line".to_string());

        // Test initial cursor position (should be 0,0 internally, displayed as 1,1)
        assert_eq!(editor.cursor().row, 0);
        assert_eq!(editor.cursor().col, 0);

        // Move cursor and verify
        editor.cursor_mut().row = 1;
        editor.cursor_mut().col = 5;
        assert_eq!(editor.cursor().row, 1);
        assert_eq!(editor.cursor().col, 5);

        // Verify line count (buffer starts with 1 empty line, we insert 3, so total is 4)
        assert_eq!(editor.buffer().line_count(), 4);
    }

    #[test]
    fn test_status_message_handling() {
        let mut editor = Editor::new();

        // Initially no status message
        assert!(editor.status_msg.is_none());

        // Set a status message
        editor.set_status_message("Test message".to_string());
        assert_eq!(editor.status_msg, Some("Test message".to_string()));

        // Clear status message
        editor.clear_status_message();
        assert!(editor.status_msg.is_none());
    }

    #[test]
    fn test_list_buffers_format() {
        let mut editor = Editor::new();

        // Start with one buffer (no name)
        let buffer_list = editor.list_buffers();
        assert_eq!(buffer_list.len(), 1);
        assert!(buffer_list[0].contains("1"));
        assert!(buffer_list[0].contains("%")); // Current buffer marker
        assert!(buffer_list[0].contains("[No Name]"));

        // Add a named buffer
        let buffer_info = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("test.txt".to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info);

        let buffer_list = editor.list_buffers();
        assert_eq!(buffer_list.len(), 2);

        // Check first buffer (not current, so should have space instead of %)
        assert!(buffer_list[0].contains("1"));
        assert!(buffer_list[0].contains(" ")); // Not current
        assert!(buffer_list[0].contains("[No Name]"));

        // Check second buffer (current, modified)
        assert!(buffer_list[1].contains("2"));
        assert!(buffer_list[1].contains("%")); // Current buffer marker
        assert!(buffer_list[1].contains("+")); // Modified marker
        assert!(buffer_list[1].contains("test.txt"));
    }
}
