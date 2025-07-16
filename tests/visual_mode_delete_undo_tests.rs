use rustvim::buffer::Buffer;
use rustvim::editor::{Editor, Mode};

// Helper function to create an editor with test content
fn create_test_editor_with_lines(content: &str) -> Editor {
    let mut editor = Editor::new();
    let buffer = Buffer::from_file(content);
    editor.buffers[0].buffer = buffer;
    editor.buffers[0].modified = false;
    editor
}

#[test]
fn test_visual_line_delete_no_extra_newline() {
    let mut editor = create_test_editor_with_lines("line 1\nline 2\nline 3\nline 4");

    // Move cursor to line 1 (row 1)
    editor.move_cursor(1, 0);

    // Enter visual line mode
    editor.enter_visual_line_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_line_mode);

    // Move down to select 2 lines (line 2 and line 3)
    editor.cursor_mut().row += 1;

    // Delete the selection
    editor.delete_visual_selection().unwrap();

    // Verify we're back in normal mode
    assert_eq!(editor.mode, Mode::Normal);
    assert!(!editor.visual_line_mode);

    // Verify the correct lines were deleted
    assert_eq!(editor.buffer().line_count(), 2); // Should have 2 lines left

    // Verify the remaining content
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 4");

    // Verify cursor position
    assert_eq!(editor.cursor().row, 1); // Should be on the line that moved up
    assert_eq!(editor.cursor().col, 0);
}

#[test]
fn test_visual_line_delete_undo() {
    let mut editor = create_test_editor_with_lines("line 1\nline 2\nline 3");

    // Move cursor to line 1 (row 1)
    editor.move_cursor(1, 0);

    // Enter visual line mode and delete line 2
    editor.enter_visual_line_mode();
    editor.delete_visual_selection().unwrap();

    // Verify line 2 was deleted
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 3");

    // Undo the deletion
    editor.undo();

    // Verify the line was restored
    assert_eq!(editor.buffer().line_count(), 3);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 2");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "line 3");
}

#[test]
fn test_visual_char_delete_undo() {
    let mut editor = create_test_editor_with_lines("hello world");

    // Position cursor at 'w' in "world"
    editor.move_cursor(0, 6);

    // Enter visual mode and select "wor"
    editor.enter_visual_mode();
    editor.cursor_mut().col += 1; // now on 'o'
    editor.cursor_mut().col += 1; // now on 'r'

    // Delete the selection
    editor.delete_visual_selection().unwrap();

    // Verify the text was deleted
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello ld");
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 6); // Cursor should be at deletion start

    // Undo the deletion
    editor.undo();

    // Verify the text was restored
    assert_eq!(editor.buffer().get_line(0).unwrap(), "hello world");
}

#[test]
fn test_visual_line_delete_single_line() {
    let mut editor = create_test_editor_with_lines("line 1\nline 2\nline 3");

    // Move cursor to line 1 (middle line)
    editor.move_cursor(1, 3);

    // Enter visual line mode (no movement, so just one line selected)
    editor.enter_visual_line_mode();

    // Delete the single line
    editor.delete_visual_selection().unwrap();

    // Verify only the middle line was deleted
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 3");

    // Verify cursor position (should be on line that moved up)
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 0); // Should be at start of line
}

#[test]
fn test_visual_line_delete_last_line() {
    let mut editor = create_test_editor_with_lines("line 1\nline 2\nline 3");

    // Move cursor to last line
    editor.move_cursor(2, 0);

    // Enter visual line mode and delete last line
    editor.enter_visual_line_mode();
    editor.delete_visual_selection().unwrap();

    // Verify last line was deleted
    assert_eq!(editor.buffer().line_count(), 2);
    assert_eq!(editor.buffer().get_line(0).unwrap(), "line 1");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "line 2");

    // Verify cursor moved up
    assert_eq!(editor.cursor().row, 1); // Should be on the last available line
    assert_eq!(editor.cursor().col, 0);
}
