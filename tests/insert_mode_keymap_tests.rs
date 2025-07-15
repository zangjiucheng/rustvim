use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::KeymapProcessor;

/// Test that insert mode uses the keymap system correctly
#[test]
fn test_insert_mode_keymap_integration() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start with some content
    editor.buffer_mut().insert_line(0, "Hello World".to_string());
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 6; // After "Hello "
    
    // Switch to insert mode
    editor.mode = Mode::Insert;
    
    // Test character insertion using keymap
    let result = keymap_processor.process_key(&mut editor, &Key::Char('B'));
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Verify character was inserted
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello BWorld");
    assert_eq!(editor.cursor().col, 7);
    
    // Test another character
    let result = keymap_processor.process_key(&mut editor, &Key::Char('i'));
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello BiWorld");
    assert_eq!(editor.cursor().col, 8);
    
    // Test character 'g'
    let result = keymap_processor.process_key(&mut editor, &Key::Char('g'));
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello BigWorld");
    assert_eq!(editor.cursor().col, 9);
}

/// Test insert mode backspace using keymap
#[test]
fn test_insert_mode_backspace_keymap() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Set up content
    editor.buffer_mut().insert_line(0, "Hello World".to_string());
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 5; // After "Hello"
    editor.mode = Mode::Insert;
    
    // Test backspace
    let result = keymap_processor.process_key(&mut editor, &Key::Backspace);
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Should have deleted 'o' and cursor moved back
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hell World");
    assert_eq!(editor.cursor().col, 4);
}

/// Test insert mode newline using keymap
#[test]
fn test_insert_mode_newline_keymap() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Set up content
    editor.buffer_mut().insert_line(0, "Hello World".to_string());
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 6; // After "Hello "
    editor.mode = Mode::Insert;
    
    // Test newline insertion
    let result = keymap_processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Should have split the line
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello ");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "World");
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 0);
}

/// Test insert mode navigation using keymap
#[test]
fn test_insert_mode_navigation_keymap() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Set up content
    editor.buffer_mut().insert_line(0, "Hello".to_string());
    editor.buffer_mut().insert_line(1, "World".to_string());
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 2; // At 'l' in "Hello"
    editor.mode = Mode::Insert;
    
    // Test left navigation
    let result = keymap_processor.process_key(&mut editor, &Key::Left);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.cursor().col, 1);
    
    // Test right navigation
    let result = keymap_processor.process_key(&mut editor, &Key::Right);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.cursor().col, 2);
    
    // Test down navigation
    let result = keymap_processor.process_key(&mut editor, &Key::Down);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 2);
    
    // Test up navigation
    let result = keymap_processor.process_key(&mut editor, &Key::Up);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 2);
}

/// Test that insert mode keymap handles unknown keys gracefully
#[test]
fn test_insert_mode_unknown_keys() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    editor.mode = Mode::Insert;
    
    // Test with an unmapped special key (should return false)
    let result = keymap_processor.process_key(&mut editor, &Key::Function(1));
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Should return false for unknown keys
}

/// Test insert mode character edge cases
#[test]
fn test_insert_mode_special_characters() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start with empty buffer
    editor.mode = Mode::Insert;
    
    // Test space character
    let result = keymap_processor.process_key(&mut editor, &Key::Char(' '));
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.buffer().get_line(0).unwrap(), " ");
    
    // Test tab character
    let result = keymap_processor.process_key(&mut editor, &Key::Char('\t'));
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.buffer().get_line(0).unwrap(), " \t");
    
    // Test unicode character
    let result = keymap_processor.process_key(&mut editor, &Key::Char('α'));
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(editor.buffer().get_line(0).unwrap(), " \tα");
}

/// Test that insert mode works end-to-end through editor
#[test]
fn test_insert_mode_integration_through_editor() {
    let mut editor = Editor::new();
    
    // Start insert mode
    editor.mode = Mode::Insert;
    
    // Use the editor's handle_keymap_input method
    let _ = editor.handle_keymap_input(&Key::Char('H'));
    let _ = editor.handle_keymap_input(&Key::Char('i'));
    let _ = editor.handle_keymap_input(&Key::Char('!'));
    
    // Verify the text was inserted
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hi!");
    assert_eq!(editor.cursor().col, 3);
}
