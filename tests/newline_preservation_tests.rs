use std::fs;
use tempfile::tempdir;
use vimlike_editor::editor::Editor;

#[test]
fn test_file_newline_preservation() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Test 1: File that ends with newline
    let file_with_newline = temp_dir.path().join("with_newline.txt");
    let content_with_newline = "line1\nline2\n";
    fs::write(&file_with_newline, content_with_newline).expect("Failed to write test file");
    
    let mut editor = Editor::new();
    editor.load_file(file_with_newline.to_str().unwrap()).expect("Failed to load file");
    
    // Verify the buffer correctly detected the trailing newline
    assert!(editor.buffer.ends_with_newline);
    assert_eq!(editor.buffer.line_count(), 2);
    assert_eq!(editor.buffer.get_line(0), Some(&"line1".to_string()));
    assert_eq!(editor.buffer.get_line(1), Some(&"line2".to_string()));
    
    // Save the file and check that the newline is preserved
    let output_file = temp_dir.path().join("output_with_newline.txt");
    assert!(editor.write_file(Some(output_file.to_str().unwrap().to_string())));
    
    let saved_content = fs::read_to_string(&output_file).expect("Failed to read saved file");
    assert_eq!(saved_content, content_with_newline);
    assert!(saved_content.ends_with('\n'));
    
    // Test 2: File that does NOT end with newline
    let file_without_newline = temp_dir.path().join("without_newline.txt");
    let content_without_newline = "line1\nline2";
    fs::write(&file_without_newline, content_without_newline).expect("Failed to write test file");
    
    let mut editor2 = Editor::new();
    editor2.load_file(file_without_newline.to_str().unwrap()).expect("Failed to load file");
    
    // Verify the buffer correctly detected NO trailing newline
    assert!(!editor2.buffer.ends_with_newline);
    assert_eq!(editor2.buffer.line_count(), 2);
    assert_eq!(editor2.buffer.get_line(0), Some(&"line1".to_string()));
    assert_eq!(editor2.buffer.get_line(1), Some(&"line2".to_string()));
    
    // Save the file and check that no newline is added
    let output_file2 = temp_dir.path().join("output_without_newline.txt");
    assert!(editor2.write_file(Some(output_file2.to_str().unwrap().to_string())));
    
    let saved_content2 = fs::read_to_string(&output_file2).expect("Failed to read saved file");
    assert_eq!(saved_content2, content_without_newline);
    assert!(!saved_content2.ends_with('\n'));
    
    // Test 3: Create new file and verify it gets a trailing newline by default
    let new_file = temp_dir.path().join("new_file.txt");
    let mut editor3 = Editor::new();
    editor3.edit_file(new_file.to_str().unwrap()); // This creates a new file
    
    // New buffers should end with newline by default
    assert!(editor3.buffer.ends_with_newline);
    
    // Write some content
    editor3.buffer.insert_char(vimlike_editor::buffer::Position::new(0, 0), 'h');
    editor3.buffer.insert_char(vimlike_editor::buffer::Position::new(0, 1), 'i');
    editor3.modified = true;
    
    assert!(editor3.write_file(None));
    
    let new_file_content = fs::read_to_string(&new_file).expect("Failed to read new file");
    assert_eq!(new_file_content, "hi\n");
    assert!(new_file_content.ends_with('\n'));
}

#[test]
fn test_empty_file_handling() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Test empty file
    let empty_file = temp_dir.path().join("empty.txt");
    fs::write(&empty_file, "").expect("Failed to write empty file");
    
    let mut editor = Editor::new();
    editor.load_file(empty_file.to_str().unwrap()).expect("Failed to load empty file");
    
    // Empty files should not end with newline
    assert!(!editor.buffer.ends_with_newline);
    assert_eq!(editor.buffer.line_count(), 1);
    assert_eq!(editor.buffer.get_line(0), Some(&String::new()));
    
    // Save and verify it stays empty
    let output_empty = temp_dir.path().join("output_empty.txt");
    assert!(editor.write_file(Some(output_empty.to_str().unwrap().to_string())));
    
    let saved_empty = fs::read_to_string(&output_empty).expect("Failed to read saved empty file");
    assert_eq!(saved_empty, "");
    assert!(!saved_empty.ends_with('\n'));
}

#[test]
fn test_single_newline_file() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Test file with just a newline
    let newline_file = temp_dir.path().join("newline_only.txt");
    fs::write(&newline_file, "\n").expect("Failed to write newline file");
    
    let mut editor = Editor::new();
    editor.load_file(newline_file.to_str().unwrap()).expect("Failed to load newline file");
    
    // Should end with newline and have one empty line
    assert!(editor.buffer.ends_with_newline);
    assert_eq!(editor.buffer.line_count(), 1);
    assert_eq!(editor.buffer.get_line(0), Some(&String::new()));
    
    // Save and verify newline is preserved
    let output_newline = temp_dir.path().join("output_newline.txt");
    assert!(editor.write_file(Some(output_newline.to_str().unwrap().to_string())));
    
    let saved_newline = fs::read_to_string(&output_newline).expect("Failed to read saved newline file");
    assert_eq!(saved_newline, "\n");
    assert!(saved_newline.ends_with('\n'));
}
