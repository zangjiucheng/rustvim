use std::fs;
use tempfile::tempdir;
use vimlike_editor::editor::Editor;

#[test]
fn test_newline_preservation_in_new_files() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let new_file = temp_dir.path().join("new_file.txt");
    
    let mut editor = Editor::new();
    
    // Edit new file (file doesn't exist yet)
    editor.edit_file(new_file.to_str().unwrap());
    
    // Verify initial state: single empty line, ends with newline
    assert_eq!(editor.buffer.line_count(), 1);
    assert_eq!(editor.buffer.get_line(0).unwrap(), "");
    assert!(editor.buffer.ends_with_newline);
    
    // Add content
    editor.buffer.insert_char(vimlike_editor::buffer::Position::new(0, 0), 'h');
    editor.buffer.insert_char(vimlike_editor::buffer::Position::new(0, 1), 'i');
    editor.modified = true;
    
    // Verify content after editing
    assert_eq!(editor.buffer.line_count(), 1);
    assert_eq!(editor.buffer.get_line(0).unwrap(), "hi");
    assert_eq!(editor.buffer.to_string(), "hi");
    
    // Write file
    assert!(editor.write_file(None));
    
    // Verify saved content has trailing newline
    let saved_content = fs::read_to_string(&new_file).expect("Failed to read saved file");
    assert_eq!(saved_content, "hi\n");
    assert!(saved_content.ends_with('\n'));
}

#[test]
fn test_newline_preservation_detailed_workflow() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Test 1: Create file with content ending in newline
    let file1 = temp_dir.path().join("file_with_newline.txt");
    fs::write(&file1, "line1\nline2\n").expect("Failed to create test file");
    
    let mut editor = Editor::new();
    editor.load_file(file1.to_str().unwrap()).expect("Failed to load file");
    
    // Verify initial state
    assert_eq!(editor.buffer.line_count(), 2);
    assert_eq!(editor.buffer.get_line(0).unwrap(), "line1");
    assert_eq!(editor.buffer.get_line(1).unwrap(), "line2");
    assert!(editor.buffer.ends_with_newline);
    
    // Modify content
    editor.buffer.insert_char(vimlike_editor::buffer::Position::new(1, 5), '!');
    editor.modified = true;
    
    // Save and verify newline is preserved
    assert!(editor.write_file(None));
    let saved_content = fs::read_to_string(&file1).expect("Failed to read saved file");
    assert_eq!(saved_content, "line1\nline2!\n");
    assert!(saved_content.ends_with('\n'));
    
    // Test 2: Create file with content NOT ending in newline
    let file2 = temp_dir.path().join("file_without_newline.txt");
    fs::write(&file2, "no newline").expect("Failed to create test file");
    
    editor.load_file(file2.to_str().unwrap()).expect("Failed to load file");
    
    // Verify initial state
    assert_eq!(editor.buffer.line_count(), 1);
    assert_eq!(editor.buffer.get_line(0).unwrap(), "no newline");
    assert!(!editor.buffer.ends_with_newline);
    
    // Modify content
    editor.buffer.insert_char(vimlike_editor::buffer::Position::new(0, 10), '!');
    editor.modified = true;
    
    // Save and verify no newline is added
    assert!(editor.write_file(None));
    let saved_content = fs::read_to_string(&file2).expect("Failed to read saved file");
    assert_eq!(saved_content, "no newline!");
    assert!(!saved_content.ends_with('\n'));
}
