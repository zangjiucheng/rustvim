use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::KeymapProcessor;

/// Test the real backspace/undo issue
#[test]
fn test_real_backspace_undo_issue() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start with empty buffer
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 0;
    
    // Enter insert mode
    editor.mode = Mode::Insert;
    editor.start_insert_mode();
    
    // Insert "Hello"
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('H'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('e'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('l'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('l'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('o'));
    
    // Current should be "Hello"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello");
    assert_eq!(editor.cursor().col, 5);
    
    // Backspace twice  
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    
    // Should now be "Hel"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hel");
    assert_eq!(editor.cursor().col, 3);
    
    // Exit insert mode 
    editor.mode = Mode::Normal;
    editor.end_insert_mode();
    
    // Now undo - this should revert the entire insert operation
    editor.undo();
    
    // Should be back to empty
    assert_eq!(editor.buffer().get_line(0).unwrap(), "");
    assert_eq!(editor.cursor().col, 0);
}

/// Test undo after inserting and backspacing
#[test]
fn test_insert_backspace_undo_complete() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start with "Test" 
    editor.buffer_mut().insert_line(0, "Test".to_string());
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 4; // At end
    
    // Enter insert mode
    editor.mode = Mode::Insert;
    editor.start_insert_mode();
    
    // Insert " Hello"
    let _ = keymap_processor.process_key(&mut editor, &Key::Char(' '));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('H'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('e'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('l'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('l'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('o'));
    
    // Should be "Test Hello"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Test Hello");
    
    // Backspace 3 times (remove "llo")
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    
    // Should be "Test He"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Test He");
    
    // Exit insert mode
    editor.mode = Mode::Normal;
    editor.end_insert_mode();
    
    // Undo should revert the entire insert session
    editor.undo();
    
    // Should be back to "Test"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Test");
}
