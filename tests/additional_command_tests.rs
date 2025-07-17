#[cfg(test)]
mod additional_command_tests {
    use rustvim::editor::Editor;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_edit_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("nonexistent.txt");

        let mut editor = Editor::new();

        // Test :e command without filename
        editor.execute_ex_command("e");
        assert!(editor.status_msg.is_some());
        assert!(editor
            .status_msg
            .as_ref()
            .unwrap()
            .contains("Argument required"));

        // Test :e with filename
        let initial_buffer_count = editor.buffers.len();
        editor.execute_ex_command(&format!("e {}", test_file.to_str().unwrap()));

        // Should have created a new buffer
        assert_eq!(editor.buffers.len(), initial_buffer_count + 1);

        // Should be on the new buffer
        assert_eq!(editor.current_buffer, initial_buffer_count);
        assert_eq!(editor.filename(), Some(test_file.to_str().unwrap()));
        assert!(!editor.is_modified());
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("[New File]"));

        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_write_command_variations() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("test.txt");

        let mut editor = Editor::new();

        // Test writing without filename set
        editor.execute_ex_command("w");
        assert!(editor.status_msg.is_some());
        assert!(editor.status_msg.as_ref().unwrap().contains("No file name"));

        // Set filename and test write
        editor.set_filename(Some(test_file.to_str().unwrap().to_string()));
        editor.buffer_mut().insert_line(0, "content".to_string());
        editor.set_modified(true);

        editor.execute_ex_command("w");
        assert!(!editor.is_modified()); // Should be marked as saved

        // Verify file was written
        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).expect("Failed to read test file");
        assert!(content.contains("content"));

        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_x_command_alias() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("test.txt");

        let mut editor = Editor::new();
        editor.set_filename(Some(test_file.to_str().unwrap().to_string()));
        editor.buffer_mut().insert_line(0, "content".to_string());
        editor.set_modified(true);

        // Test :x command (should write and quit like :wq)
        editor.execute_ex_command("x");
        assert!(!editor.is_modified());
        assert!(!editor.running);

        // Verify file was written
        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).expect("Failed to read test file");
        assert!(content.contains("content"));

        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }

    #[test]
    fn test_command_with_multiple_arguments() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let test_file = temp_dir.path().join("my file with spaces.txt");

        let mut editor = Editor::new();

        // Test :w with filename containing spaces
        editor.buffer_mut().insert_line(0, "content".to_string());
        editor.set_modified(true);

        editor.execute_ex_command(&format!("w {}", test_file.to_str().unwrap()));
        assert_eq!(editor.filename(), Some(test_file.to_str().unwrap()));
        assert!(!editor.is_modified());

        // Verify file was written
        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).expect("Failed to read test file");
        assert!(content.contains("content"));

        // Cleanup: temp_dir automatically cleans up when dropped
        drop(temp_dir);
    }
}
