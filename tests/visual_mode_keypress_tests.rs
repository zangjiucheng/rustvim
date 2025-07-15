use rustvim::buffer::Buffer;
use rustvim::editor::{Editor, Mode};

/// Helper function to create an editor with test content
fn create_test_editor() -> Editor {
    let mut editor = Editor::new();
    
    // Add some test content
    let test_content = "Hello World\nSecond Line\nThird Line";
    let buffer = Buffer::from_file(test_content);
    editor.buffers[0].buffer = buffer;
    editor.buffers[0].modified = false;
    
    editor
}

#[test]
fn test_visual_delete_integration() {
    let mut editor = create_test_editor();
    
    // Position cursor at start
    editor.move_cursor(0, 0);
    
    // Enter visual mode
    editor.enter_visual_mode();
    assert_eq!(editor.mode, Mode::Visual);
    
    // Move to select "Hello" (move to column 5)
    editor.move_cursor(0, 5);
    
    // Execute delete selection directly
    let d_result = editor.delete_visual_selection();
    assert!(d_result.is_ok());
    
    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);
    
    // Content should have "Hello " deleted (including space at position 5)
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "World");
    
    // Register should contain "Hello " (including space)
    assert!(!editor.register.content.is_empty());
    assert_eq!(editor.register.content, "Hello ");
}

#[test]
fn test_visual_yank_integration() {
    let mut editor = create_test_editor();
    
    // Position cursor at start
    editor.move_cursor(0, 0);
    
    // Enter visual mode
    editor.enter_visual_mode();
    assert_eq!(editor.mode, Mode::Visual);
    
    // Move to select "Hello" (move to column 5)
    editor.move_cursor(0, 5);
    
    // Execute yank selection directly
    let y_result = editor.yank_visual_selection();
    assert!(y_result.is_ok());
    
    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);
    
    // Content should be unchanged
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "Hello World");
    
    // Register should contain "Hello " (including space)
    assert!(!editor.register.content.is_empty());
    assert_eq!(editor.register.content, "Hello ");
}
