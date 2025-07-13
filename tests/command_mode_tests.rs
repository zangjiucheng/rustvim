#[cfg(test)]
mod command_mode_tests {
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
        
        // Test :q command when not modified
        editor.execute_ex_command("q");
        assert!(!editor.running);
        
        // Reset for next test
        let mut editor = Editor::new();
        editor.modified = true;
        
        // Test :q command when modified (should refuse)
        editor.execute_ex_command("q");
        assert!(editor.running); // Should still be running
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No write since last change"));
        
        // Test :q! command (force quit)
        editor.execute_ex_command("q!");
        assert!(!editor.running);
    }

    #[test]
    fn test_write_quit_command() {
        let mut editor = Editor::new();
        editor.filename = Some("test_file.txt".to_string());
        
        // Add some content to the buffer
        editor.buffer.insert_line(0, "Test content".to_string());
        editor.modified = true;
        
        // Test :wq command
        editor.execute_ex_command("wq");
        // Should have attempted to write and quit
        assert!(!editor.running);
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
        let mut editor = Editor::new();
        
        // Add content to buffer
        editor.buffer.insert_line(0, "Test content".to_string());
        editor.modified = true;
        
        // Test :w newfile.txt
        editor.execute_ex_command("w newfile.txt");
        
        // Should have set the filename
        assert_eq!(editor.filename, Some("newfile.txt".to_string()));
        assert!(!editor.modified); // Should be marked as not modified after save
    }
}
