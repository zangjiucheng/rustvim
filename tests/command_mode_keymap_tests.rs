use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::KeymapProcessor;

/// Test command mode keymap integration
#[test]
fn test_command_mode_keymap_integration() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start command mode
    editor.mode = Mode::Command;
    editor.command_input.clear();
    
    // Type characters using keymap
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('w'));
    assert_eq!(editor.command_input, "w");
    
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('q'));
    assert_eq!(editor.command_input, "wq");
    
    // Test backspace using keymap
    let _ = keymap_processor.process_key(&mut editor, &Key::Backspace);
    assert_eq!(editor.command_input, "w");
    
    // Test Enter to execute command
    let _ = keymap_processor.process_key(&mut editor, &Key::Enter);
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.command_input.is_empty());
}

/// Test command mode keymap cancel
#[test] 
fn test_command_mode_keymap_cancel() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start command mode
    editor.mode = Mode::Command;
    editor.command_input.clear();
    
    // Type some characters
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('e'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('s'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    assert_eq!(editor.command_input, "test");
    
    // Cancel with Escape key
    let _ = keymap_processor.process_key(&mut editor, &Key::Esc);
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.command_input.is_empty());
}

/// Test command mode special characters
#[test]
fn test_command_mode_special_chars() {
    let mut editor = Editor::new();
    let mut keymap_processor = KeymapProcessor::new();
    
    // Start command mode
    editor.mode = Mode::Command;
    editor.command_input.clear();
    
    // Test various special characters
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('e'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char(' '));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('~'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('/'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('e'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('s'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('.'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('x'));
    let _ = keymap_processor.process_key(&mut editor, &Key::Char('t'));
    
    assert_eq!(editor.command_input, "e ~/test.txt");
}

/// Test command mode through editor integration
#[test]
fn test_command_mode_editor_integration() {
    let mut editor = Editor::new();
    
    // Start command mode
    editor.start_command_mode();
    assert_eq!(editor.mode, Mode::Command);
    assert!(editor.command_input.is_empty());
    
    // Use the editor's handle_keymap_input method
    editor.handle_keymap_input(&Key::Char('q')).unwrap();
    assert_eq!(editor.command_input, "q");
    
    editor.handle_keymap_input(&Key::Char('!')).unwrap();
    assert_eq!(editor.command_input, "q!");
    
    // Execute the command - should quit forcefully
    editor.handle_keymap_input(&Key::Enter).unwrap();
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.command_input.is_empty());
    assert!(!editor.running); // Force quit should have exited
}
