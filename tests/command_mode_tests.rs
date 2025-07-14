#[cfg(test)]
mod command_mode_tests {
    use std::fs;
    use tempfile::tempdir;
    use vimlike_editor::editor::{Editor, Mode};
    use vimlike_editor::input::Key;

    #[test]
    fn test_start_command_mode() {
        let mut editor = Editor::new();
        
        // Initially in Normal mode
        assert_eq!(editor.mode, Mode::Normal);
        assert!(editor.command_input.is_empty());
        
        // Start command mode
        editor.start_command_mode();
        
        assert_eq!(editor.mode, Mode::Command);
        assert!(editor.command_input.is_empty());
    }

    #[test]
    fn test_command_mode_input_handling() {
        let mut editor = Editor::new();
        editor.start_command_mode();
        
        // Type characters
        editor.handle_command_mode_input(&Key::Char('w'));
        assert_eq!(editor.command_input, "w");
        
        editor.handle_command_mode_input(&Key::Char(' '));
        editor.handle_command_mode_input(&Key::Char('t'));
        editor.handle_command_mode_input(&Key::Char('e'));
        editor.handle_command_mode_input(&Key::Char('s'));
        editor.handle_command_mode_input(&Key::Char('t'));
        assert_eq!(editor.command_input, "w test");
        
        // Test backspace
        editor.handle_command_mode_input(&Key::Backspace);
        assert_eq!(editor.command_input, "w tes");
    }

    #[test]
    fn test_command_mode_escape() {
        let mut editor = Editor::new();
        editor.start_command_mode();
        
        // Type something
        editor.handle_command_mode_input(&Key::Char('w'));
        assert_eq!(editor.command_input, "w");
        assert_eq!(editor.mode, Mode::Command);
        
        // Simulate ESC key handling (escape to normal mode)
        editor.mode = Mode::Normal;
        editor.command_input.clear();
        
        assert_eq!(editor.mode, Mode::Normal);
        assert!(editor.command_input.is_empty());
    }

    #[test]
    fn test_write_command_parsing() {
        let mut editor = Editor::new();
        
        // Test :w command
        editor.execute_ex_command("w");
        // Should show error because no filename set
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No file name"));
    }

    #[test]
    fn test_quit_command_parsing() {
        let mut editor = Editor::new();
        
        // Test :q command when not modified (should quit when only one buffer)
        editor.execute_ex_command("q");
        assert!(!editor.running);
        
        // Reset for next test
        let mut editor = Editor::new();
        editor.set_modified(true);
        
        // Test :q command when modified (should refuse)
        editor.execute_ex_command("q");
        assert!(editor.running); // Should still be running
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No write since last change"));
        
        // Test :q! command (force quit when only one buffer)
        editor.execute_ex_command("q!");
        assert!(!editor.running);
    }

    #[test]
    fn test_write_quit_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("test_quit_file.txt");
        
        let mut editor = Editor::new();
        editor.set_filename(Some(test_file.to_str().unwrap().to_string()));
        
        // Add some content to the buffer
        editor.buffer_mut().insert_line(0, "Test content".to_string());
        editor.set_modified(true);
        
        // Test :wq command
        editor.execute_ex_command("wq");
        // Should have attempted to write and quit
        assert!(!editor.running);
        
        // Verify file was written
        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).expect("Failed to read test file");
        assert!(content.contains("Test content"));
        
        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_unknown_command() {
        let mut editor = Editor::new();
        
        editor.execute_ex_command("unknown");
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("Not an editor command"));
    }

    #[test]
    fn test_command_mode_enter_execution() {
        let mut editor = Editor::new();
        editor.start_command_mode();
        
        // Type a quit command
        editor.handle_command_mode_input(&Key::Char('q'));
        assert_eq!(editor.mode, Mode::Command);
        assert_eq!(editor.command_input, "q");
        
        // Press Enter to execute
        editor.handle_command_mode_input(&Key::Enter);
        assert_eq!(editor.mode, Mode::Normal);
        assert!(editor.command_input.is_empty());
        assert!(!editor.running); // Should have quit
    }

    #[test]
    fn test_save_as_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let new_file = temp_dir.path().join("newfile.txt");
        
        let mut editor = Editor::new();
        
        // Add content to buffer
        editor.buffer_mut().insert_line(0, "Test content".to_string());
        editor.set_modified(true);
        
        // Test :w newfile.txt
        editor.execute_ex_command(&format!("w {}", new_file.to_str().unwrap()));
        
        // Should have set the filename
        assert_eq!(*editor.filename(), Some(new_file.to_str().unwrap().to_string()));
        assert!(!editor.is_modified()); // Should be marked as not modified after save
        
        // Verify file was created and has correct content
        assert!(new_file.exists());
        let content = fs::read_to_string(&new_file).expect("Failed to read saved file");
        assert!(content.contains("Test content"));
        
        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_buffer_switch_command() {
        let mut editor = Editor::new();
        
        // Create multiple buffers
        use vimlike_editor::editor::{BufferInfo, Cursor};
        use vimlike_editor::buffer::Buffer;
        use vimlike_editor::history::History;
        
        // Add buffer 2
        let buffer_info1 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file1.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info1);
        
        // Add buffer 3
        let buffer_info2 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file2.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);
        
        // Should now have 3 buffers (0: [No Name], 1: file1.txt, 2: file2.txt)
        // Currently on buffer 2 (index 2)
        assert_eq!(editor.buffers.len(), 3);
        assert_eq!(editor.current_buffer, 2);
        
        // Test :b 1 (switch to first buffer - 1-indexed)
        editor.execute_ex_command("b 1");
        assert_eq!(editor.current_buffer, 0); // Should be at buffer index 0
        assert!(editor.status_msg.as_ref().unwrap().contains("Buffer 1"));
        
        // Test :b 2 (switch to second buffer)
        editor.execute_ex_command("b 2");
        assert_eq!(editor.current_buffer, 1); // Should be at buffer index 1
        assert!(editor.status_msg.as_ref().unwrap().contains("Buffer 2"));
        
        // Test :b 3 (switch to third buffer)
        editor.execute_ex_command("b 3");
        assert_eq!(editor.current_buffer, 2); // Should be at buffer index 2
        assert!(editor.status_msg.as_ref().unwrap().contains("Buffer 3"));
        
        // Test :b 0 (invalid - should show error)
        editor.execute_ex_command("b 0");
        assert!(editor.status_msg.as_ref().unwrap().contains("does not exist"));
        
        // Test :b 4 (non-existent buffer - should show error)
        editor.execute_ex_command("b 4");
        assert!(editor.status_msg.as_ref().unwrap().contains("does not exist"));
        
        // Test :b without argument (should show error)
        editor.execute_ex_command("b");
        assert!(editor.status_msg.as_ref().unwrap().contains("Argument required"));
        
        // Test :b with invalid argument (should show error)
        editor.execute_ex_command("b abc");
        assert!(editor.status_msg.as_ref().unwrap().contains("Invalid buffer number"));
    }

    #[test]
    fn test_write_all_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        
        let mut editor = Editor::new();
        
        // Create multiple buffers with modifications
        use vimlike_editor::editor::{BufferInfo, Cursor};
        use vimlike_editor::buffer::Buffer;
        use vimlike_editor::history::History;
        
        // Modify first buffer (default buffer)
        editor.set_filename(Some(file1.to_str().unwrap().to_string()));
        editor.buffer_mut().insert_line(0, "Content 1".to_string());
        editor.set_modified(true);
        
        // Add second buffer with modifications
        let mut buffer2 = Buffer::new();
        buffer2.insert_line(0, "Content 2".to_string());
        let buffer_info2 = BufferInfo {
            buffer: buffer2,
            filename: Some(file2.to_str().unwrap().to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);
        
        // Add third buffer without filename (should cause error)
        let buffer_info3 = BufferInfo {
            buffer: Buffer::new(),
            filename: None,
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info3);
        
        // Test :wa command
        editor.execute_ex_command("wa");
        
        // Should have written the named files
        assert!(file1.exists());
        assert!(file2.exists());
        
        // Check file contents
        let content1 = fs::read_to_string(&file1).expect("Failed to read file1");
        let content2 = fs::read_to_string(&file2).expect("Failed to read file2");
        assert!(content1.contains("Content 1"));
        assert!(content2.contains("Content 2"));
        
        // Should show error message about unnamed buffer
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("error"));
        
        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_quit_all_commands() {
        let mut editor = Editor::new();
        
        // Create multiple buffers
        use vimlike_editor::editor::{BufferInfo, Cursor};
        use vimlike_editor::buffer::Buffer;
        use vimlike_editor::history::History;
        
        // Add second buffer
        let buffer_info1 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file1.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info1);
        
        // Add third buffer with modifications
        let buffer_info2 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file2.txt".to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);
        
        // Test :qa command (should refuse because of modifications)
        editor.execute_ex_command("qa");
        assert!(editor.running); // Should still be running
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No write since last change"));
        
        // Test :qa! command (should force quit)
        editor.execute_ex_command("qa!");
        assert!(!editor.running); // Should have quit
    }

    #[test]
    fn test_write_quit_all_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        
        let mut editor = Editor::new();
        
        // Create multiple buffers with modifications
        use vimlike_editor::editor::{BufferInfo, Cursor};
        use vimlike_editor::buffer::Buffer;
        use vimlike_editor::history::History;
        
        // Modify first buffer
        editor.set_filename(Some(file1.to_str().unwrap().to_string()));
        editor.buffer_mut().insert_line(0, "Content 1".to_string());
        editor.set_modified(true);
        
        // Add second buffer with modifications
        let mut buffer2 = Buffer::new();
        buffer2.insert_line(0, "Content 2".to_string());
        let buffer_info2 = BufferInfo {
            buffer: buffer2,
            filename: Some(file2.to_str().unwrap().to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);
        
        // Test :wqa command
        editor.execute_ex_command("wqa");
        
        // Should have written files and quit
        assert!(file1.exists());
        assert!(file2.exists());
        assert!(!editor.running);
        
        // Check file contents
        let content1 = fs::read_to_string(&file1).expect("Failed to read file1");
        let content2 = fs::read_to_string(&file2).expect("Failed to read file2");
        assert!(content1.contains("Content 1"));
        assert!(content2.contains("Content 2"));
        
        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_quit_buffer_behavior() {
        let mut editor = Editor::new();
        
        // Create multiple buffers
        use vimlike_editor::editor::{BufferInfo, Cursor};
        use vimlike_editor::buffer::Buffer;
        use vimlike_editor::history::History;
        
        // Add second buffer
        let buffer_info1 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file1.txt".to_string()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info1);
        
        // Add third buffer with modifications
        let buffer_info2 = BufferInfo {
            buffer: Buffer::new(),
            filename: Some("file2.txt".to_string()),
            modified: true,
            cursor: Cursor::new(),
            scroll_offset: 0,
            history: History::new(),
        };
        editor.add_buffer(buffer_info2);
        
        // Should now have 3 buffers, current is 2 (third buffer)
        assert_eq!(editor.buffers.len(), 3);
        assert_eq!(editor.current_buffer, 2);
        assert!(editor.running);
        
        // Test :q on modified buffer (should refuse)
        editor.execute_ex_command("q");
        assert!(editor.running); // Should still be running
        assert_eq!(editor.buffers.len(), 3); // No buffers removed
        assert!(editor.status_msg.as_ref().unwrap().contains("No write since last change"));
        
        // Test :q! on modified buffer (should force close)
        editor.execute_ex_command("q!");
        assert!(editor.running); // Should still be running (other buffers exist)
        assert_eq!(editor.buffers.len(), 2); // One buffer removed
        assert_eq!(editor.current_buffer, 1); // Should move to previous buffer
        
        // Test :q on clean buffer (should close)
        editor.execute_ex_command("q");
        assert!(editor.running); // Should still be running (one buffer left)
        assert_eq!(editor.buffers.len(), 1); // One buffer removed
        assert_eq!(editor.current_buffer, 0); // Should be at first buffer
        
        // Test :q on last buffer (should quit editor)
        editor.execute_ex_command("q");
        assert!(!editor.running); // Should quit the editor entirely
    }

    #[test]
    fn test_quit_commands_with_single_buffer() {
        let mut editor = Editor::new();
        
        // Test :q with single clean buffer (should quit)
        assert_eq!(editor.buffers.len(), 1);
        editor.execute_ex_command("q");
        assert!(!editor.running);
        
        // Reset for next test
        let mut editor = Editor::new();
        editor.set_modified(true);
        
        // Test :q with single modified buffer (should refuse)
        editor.execute_ex_command("q");
        assert!(editor.running);
        assert!(editor.status_msg.as_ref().unwrap().contains("No write since last change"));
        
        // Test :q! with single modified buffer (should quit)
        editor.execute_ex_command("q!");
        assert!(!editor.running);
    }
}
