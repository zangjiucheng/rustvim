use std::fs;
use tempfile::tempdir;
use vimlike_editor::editor::Editor;

#[test]
fn test_load_multiple_files() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    
    // Create test files
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    let file3 = temp_dir.path().join("file3.txt");
    
    fs::write(&file1, "Content of file 1\n").expect("Failed to write file1");
    fs::write(&file2, "Content of file 2").expect("Failed to write file2");
    fs::write(&file3, "Content of file 3\nLine 2\n").expect("Failed to write file3");
    
    let mut editor = Editor::new();
    let results = editor.load_files(&[
        file1.to_str().unwrap().to_string(),
        file2.to_str().unwrap().to_string(),
        file3.to_str().unwrap().to_string(),
    ]);
    
    // All files should load successfully
    assert!(results[0].is_ok());
    assert!(results[1].is_ok());
    assert!(results[2].is_ok());
    
    // Should have 3 buffers
    assert_eq!(editor.buffers.len(), 3);
    
    // Current buffer should be the first one (file1)
    assert_eq!(editor.current_buffer, 0);
    assert_eq!(*editor.filename(), Some(file1.to_str().unwrap().to_string()));
    assert_eq!(editor.buffer().line_count(), 1);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Content of file 1");
    assert!(editor.buffer().ends_with_newline);
    
    // Cleanup: temp_dir automatically cleans up when dropped
    drop(temp_dir);
}

#[test]
fn test_load_single_file_with_load_files() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("single.txt");
    
    fs::write(&test_file, "Single file content").expect("Failed to write test file");
    
    let mut editor = Editor::new();
    let results = editor.load_files(&[test_file.to_str().unwrap().to_string()]);
    
    assert!(results[0].is_ok());
    assert_eq!(editor.buffers.len(), 1);
    assert_eq!(editor.current_buffer, 0);
    assert_eq!(*editor.filename(), Some(test_file.to_str().unwrap().to_string()));
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Single file content");
    assert!(!editor.buffer().ends_with_newline);
    
    // Cleanup: temp_dir automatically cleans up when dropped
    drop(temp_dir);
}

#[test]
fn test_load_files_with_nonexistent_file() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let existing_file = temp_dir.path().join("exists.txt");
    let nonexistent_file = temp_dir.path().join("does_not_exist.txt");
    
    fs::write(&existing_file, "I exist!").expect("Failed to write test file");
    
    let mut editor = Editor::new();
    let results = editor.load_files(&[
        existing_file.to_str().unwrap().to_string(),
        nonexistent_file.to_str().unwrap().to_string(),
    ]);
    
    // First file should load successfully, second should fail
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    
    // Should have 2 buffers (including the empty one for the nonexistent file)
    assert_eq!(editor.buffers.len(), 2);
    
    // Current buffer should be the first one (existing file)
    assert_eq!(editor.current_buffer, 0);
    assert_eq!(*editor.filename(), Some(existing_file.to_str().unwrap().to_string()));
    assert_eq!(editor.buffer().get_line(0).unwrap(), "I exist!");
    
    // Cleanup: temp_dir automatically cleans up when dropped
    drop(temp_dir);
}
