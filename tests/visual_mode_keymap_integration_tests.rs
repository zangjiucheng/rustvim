use rustvim::buffer::Buffer;
use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;

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
fn test_visual_mode_delete_key_integration() {
    let mut editor = create_test_editor();

    // Position cursor at start
    editor.move_cursor(0, 0);

    // Enter visual mode with 'v' key through keymap
    let v_result = editor.handle_keymap_input(&Key::Char('v'));
    assert!(v_result.is_ok());
    assert_eq!(editor.mode, Mode::Visual);

    // Move to select "Hello" (move to column 5)
    editor.move_cursor(0, 5);

    // Delete selection with 'd' key through keymap
    let d_result = editor.handle_keymap_input(&Key::Char('d'));
    assert!(d_result.is_ok());

    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Content should have "Hello " deleted
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "World");

    // Register should contain "Hello "
    assert!(!editor.register.content.is_empty());
    assert_eq!(editor.register.content, "Hello ");
}

#[test]
fn test_visual_mode_yank_key_integration() {
    let mut editor = create_test_editor();

    // Position cursor at start
    editor.move_cursor(0, 0);

    // Enter visual mode with 'v' key through keymap
    let v_result = editor.handle_keymap_input(&Key::Char('v'));
    assert!(v_result.is_ok());
    assert_eq!(editor.mode, Mode::Visual);

    // Move to select "Hello" (move to column 5)
    editor.move_cursor(0, 5);

    // Yank selection with 'y' key through keymap
    let y_result = editor.handle_keymap_input(&Key::Char('y'));
    assert!(y_result.is_ok());

    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Content should be unchanged
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

    // Register should contain "Hello "
    assert!(!editor.register.content.is_empty());
    assert_eq!(editor.register.content, "Hello ");
}

#[test]
fn test_visual_line_mode_delete_key_integration() {
    let mut editor = create_test_editor();

    // Position cursor at start of second line
    editor.move_cursor(1, 5);

    // Enter visual line mode with 'V' key through keymap
    let v_result = editor.handle_keymap_input(&Key::Char('V'));
    assert!(v_result.is_ok());
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_line_mode);

    // Delete line with 'd' key through keymap
    let d_result = editor.handle_keymap_input(&Key::Char('d'));
    assert!(d_result.is_ok());

    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Second line should be deleted (line removed entirely)
    assert_eq!(editor.buffer().line_count(), 2); // Line count should decrease
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hello World");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "Third Line"); // Third line moved up

    // Register should contain the deleted line
    assert!(!editor.register.content.is_empty());
    assert!(editor.register.is_line_based);
    assert!(editor.register.content.contains("Second Line"));
}

#[test]
fn test_normal_mode_d_still_waits_for_motion() {
    let mut editor = create_test_editor();

    // Position cursor at start
    editor.move_cursor(0, 0);

    // Ensure we're in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Press 'd' key - should enter operator-pending mode
    let d_result = editor.handle_keymap_input(&Key::Char('d'));
    assert!(d_result.is_ok());

    // Should still be in normal mode but waiting for motion
    assert_eq!(editor.mode, Mode::Normal);

    // Content should be unchanged (no deletion yet)
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "Hello World");

    // Now press 'd' again for 'dd' (delete line)
    let d2_result = editor.handle_keymap_input(&Key::Char('d'));
    assert!(d2_result.is_ok());

    // Now the line should be deleted
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "Second Line");
}

#[test]
fn test_normal_mode_y_still_waits_for_motion() {
    let mut editor = create_test_editor();

    // Position cursor at start
    editor.move_cursor(0, 0);

    // Ensure we're in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Press 'y' key - should enter operator-pending mode
    let y_result = editor.handle_keymap_input(&Key::Char('y'));
    assert!(y_result.is_ok());

    // Should still be in normal mode but waiting for motion
    assert_eq!(editor.mode, Mode::Normal);

    // Register should be empty (no yank yet)
    assert!(editor.register.content.is_empty());

    // Now press 'y' again for 'yy' (yank line)
    let y2_result = editor.handle_keymap_input(&Key::Char('y'));
    assert!(y2_result.is_ok());

    // Now the line should be yanked
    assert!(!editor.register.content.is_empty());
    assert!(editor.register.content.contains("Hello World"));
    assert!(editor.register.is_line_based);
}

#[test]
fn test_visual_mode_movement_extends_selection() {
    let mut editor = create_test_editor();

    // Position cursor at start
    editor.move_cursor(0, 2); // Position at 'l' in "Hello"

    // Enter visual mode
    let v_result = editor.handle_keymap_input(&Key::Char('v'));
    assert!(v_result.is_ok());
    assert_eq!(editor.mode, Mode::Visual);

    // Move right with 'l' key through keymap
    let l_result = editor.handle_keymap_input(&Key::Char('l'));
    assert!(l_result.is_ok());

    // Should still be in visual mode
    assert_eq!(editor.mode, Mode::Visual);

    // Cursor should have moved
    assert_eq!(editor.cursor().col, 3);

    // Delete the selection
    let d_result = editor.handle_keymap_input(&Key::Char('d'));
    assert!(d_result.is_ok());

    // Should delete "ll" (from position 2 to 3)
    let buffer = editor.buffer();
    assert_eq!(buffer.get_line(0).unwrap(), "Heo World");

    // Register should contain "ll"
    assert_eq!(editor.register.content, "ll");
}
