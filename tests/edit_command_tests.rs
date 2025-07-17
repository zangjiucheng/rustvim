use rustvim::editor::Editor;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_edit_command_creates_multiple_buffers() {
    let temp_dir = tempdir().expect("Failed to create temp dir");

    // Create test files
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join("new_file.txt");

    fs::write(&file1, "Content of file 1").expect("Failed to write file1");
    fs::write(&file2, "Content of file 2").expect("Failed to write file2");
    // file3 doesn't exist yet

    let mut editor = Editor::new();

    // Initially should have 1 buffer (the default empty buffer)
    assert_eq!(editor.buffers.len(), 1);
    assert_eq!(editor.current_buffer, 0);

    // Edit first file
    editor.execute_ex_command(&format!("e {}", file1.to_str().unwrap()));
    assert_eq!(editor.buffers.len(), 2);
    assert_eq!(editor.current_buffer, 1);
    assert_eq!(editor.filename(), Some(file1.to_str().unwrap()));
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Content of file 1");

    // Edit second file
    editor.execute_ex_command(&format!("e {}", file2.to_str().unwrap()));
    assert_eq!(editor.buffers.len(), 3);
    assert_eq!(editor.current_buffer, 2);
    assert_eq!(editor.filename(), Some(file2.to_str().unwrap()));
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Content of file 2");

    // Edit a new file (doesn't exist)
    editor.execute_ex_command(&format!("e {}", file3.to_str().unwrap()));
    assert_eq!(editor.buffers.len(), 4);
    assert_eq!(editor.current_buffer, 3);
    assert_eq!(editor.filename(), Some(file3.to_str().unwrap()));
    assert_eq!(editor.buffer().get_line(0).unwrap(), ""); // Empty buffer
    assert!(editor.status_msg.as_ref().unwrap().contains("[New File]"));

    // Test buffer navigation
    editor.execute_ex_command("bp"); // Previous buffer
    assert_eq!(editor.current_buffer, 2);
    assert_eq!(editor.filename(), Some(file2.to_str().unwrap()));

    editor.execute_ex_command("bn"); // Next buffer
    assert_eq!(editor.current_buffer, 3);
    assert_eq!(editor.filename(), Some(file3.to_str().unwrap()));

    // Test switching to buffer by number
    editor.execute_ex_command("2"); // Switch to buffer 2 (file1)
    assert_eq!(editor.current_buffer, 1); // 0-indexed, so buffer 2 is index 1
    assert_eq!(editor.filename(), Some(file1.to_str().unwrap()));

    // Cleanup: temp_dir automatically cleans up when dropped
    drop(temp_dir);
}

#[test]
fn test_edit_command_preserves_existing_buffer_modifications() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");

    fs::write(&file1, "Original content").expect("Failed to write file1");
    fs::write(&file2, "File 2 content").expect("Failed to write file2");

    let mut editor = Editor::new();

    // Edit first file and modify it
    editor.execute_ex_command(&format!("e {}", file1.to_str().unwrap()));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(0, 16), '!');
    editor.set_modified(true);

    assert_eq!(editor.buffer().get_line(0).unwrap(), "Original content!");
    assert!(editor.is_modified());

    // Edit second file (this should create a new buffer, not replace the modified one)
    editor.execute_ex_command(&format!("e {}", file2.to_str().unwrap()));

    // Should be on buffer 2 now
    assert_eq!(editor.buffers.len(), 3); // default + file1 + file2
    assert_eq!(editor.current_buffer, 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "File 2 content");
    assert!(!editor.is_modified()); // New buffer should not be modified

    // Go back to first buffer and verify modifications are preserved
    editor.execute_ex_command("2"); // Switch to buffer 2 (file1)
    assert_eq!(editor.current_buffer, 1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Original content!");
    assert!(editor.is_modified()); // Should still be modified

    // Cleanup: temp_dir automatically cleans up when dropped
    drop(temp_dir);
}
