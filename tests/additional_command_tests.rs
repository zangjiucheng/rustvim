#[cfg(test)]
mod additional_command_tests {
    use vimlike_editor::editor::Editor;

    #[test]
    fn test_edit_command() {
        let mut editor = Editor::new();
        
        // Test :e command without filename
        editor.execute_ex_command("e");
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("Argument required"));
        
        // Test :e with filename
        editor.execute_ex_command("e nonexistent.txt");
        assert_eq!(editor.filename, Some("nonexistent.txt".to_string()));
        assert!(!editor.modified);
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("[New File]"));
    }

    #[test]
    fn test_write_command_variations() {
        let mut editor = Editor::new();
        
        // Test writing without filename set
        editor.execute_ex_command("w");
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No file name"));
        
        // Set filename and test write
        editor.filename = Some("test.txt".to_string());
        editor.buffer.insert_line(0, "content".to_string());
        editor.modified = true;
        
        editor.execute_ex_command("w");
        assert!(!editor.modified); // Should be marked as saved
    }

    #[test]
    fn test_x_command_alias() {
        let mut editor = Editor::new();
        editor.filename = Some("test.txt".to_string());
        editor.buffer.insert_line(0, "content".to_string());
        editor.modified = true;
        
        // Test :x command (should write and quit like :wq)
        editor.execute_ex_command("x");
        assert!(!editor.modified);
        assert!(!editor.running);
    }

    #[test]
    fn test_command_with_multiple_arguments() {
        let mut editor = Editor::new();
        
        // Test :w with filename containing spaces
        editor.buffer.insert_line(0, "content".to_string());
        editor.modified = true;
        
        editor.execute_ex_command("w my file with spaces.txt");
        assert_eq!(editor.filename, Some("my file with spaces.txt".to_string()));
        assert!(!editor.modified);
    }
}
