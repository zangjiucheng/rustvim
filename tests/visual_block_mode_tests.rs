use rustvim::buffer::Buffer;
use rustvim::editor::{Editor, Mode};

#[test]
fn test_visual_block_mode_basic() {
    let mut editor = Editor::new();

    // Create buffer with content suitable for block selection
    let buffer = Buffer::from_file("line1_abcd\nline2_efgh\nline3_ijkl\nline4_mnop");
    editor.buffers[0].buffer = buffer;

    // Start at position (0, 6) - the 'a' in "abcd"
    editor.move_cursor(0, 6);

    // Enter visual block mode
    editor.enter_visual_block_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_block_mode);
    assert!(!editor.visual_line_mode);

    // Move cursor to create a 2x3 block (2 chars wide, 3 rows tall)
    editor.move_cursor(2, 7); // Now at row 2, col 7

    // Test visual selection detection
    assert!(editor.is_in_visual_selection(0, 6)); // 'a'
    assert!(editor.is_in_visual_selection(0, 7)); // 'b'
    assert!(!editor.is_in_visual_selection(0, 8)); // 'c' - outside block
    assert!(editor.is_in_visual_selection(1, 6)); // 'e'
    assert!(editor.is_in_visual_selection(1, 7)); // 'f'
    assert!(editor.is_in_visual_selection(2, 6)); // 'i'
    assert!(editor.is_in_visual_selection(2, 7)); // 'j'
    assert!(!editor.is_in_visual_selection(3, 6)); // 'm' - outside block vertically

    // Test yank operation
    editor.yank_visual_selection().unwrap();
    assert_eq!(editor.mode, Mode::Normal);

    // Check that register contains the block content
    assert_eq!(editor.register.content, "ab\nef\nij");
    assert!(!editor.register.is_line_based);
}

#[test]
fn test_visual_block_mode_delete() {
    let mut editor = Editor::new();

    // Create buffer with content
    let buffer = Buffer::from_file("123456\n789012\nabcdef\nghijkl");
    editor.buffers[0].buffer = buffer;

    // Start at position (1, 1) - the '8'
    editor.move_cursor(1, 1);

    // Enter visual block mode and select a 2x2 block
    editor.enter_visual_block_mode();
    editor.move_cursor(2, 2); // Select "89", "bc"

    // Delete the block
    editor.delete_visual_selection().unwrap();
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.is_modified());

    // Check that the block was deleted
    assert_eq!(editor.buffer().get_line(0).unwrap(), "123456");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "7012"); // "89" removed
    assert_eq!(editor.buffer().get_line(2).unwrap(), "adef"); // "bc" removed
    assert_eq!(editor.buffer().get_line(3).unwrap(), "ghijkl");

    // Check cursor position (should be at top-left of deleted block)
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 1);

    // Test undo
    editor.undo();
    assert_eq!(editor.buffer().get_line(1).unwrap(), "789012");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "abcdef");
}

#[test]
fn test_visual_block_mode_with_unequal_line_lengths() {
    let mut editor = Editor::new();

    // Create buffer with lines of different lengths
    let buffer = Buffer::from_file("short\nlonger line\nx\nvery very long line here");
    editor.buffers[0].buffer = buffer;

    // Start at position (0, 2) and select a block
    editor.move_cursor(0, 2);
    editor.enter_visual_block_mode();
    editor.move_cursor(3, 4); // Select a 3-char wide, 4-row tall block

    // Test selection detection with varying line lengths
    assert!(editor.is_in_visual_selection(0, 2)); // 'o' in "short"
    assert!(editor.is_in_visual_selection(0, 3)); // 'r' in "short"
    assert!(editor.is_in_visual_selection(0, 4)); // 't' in "short"
    assert!(editor.is_in_visual_selection(1, 2)); // 'n' in "longer line"
    assert!(editor.is_in_visual_selection(2, 2)); // Nothing - line too short
    assert!(editor.is_in_visual_selection(3, 2)); // 'r' in "very very long line here"

    // Yank the block
    editor.yank_visual_selection().unwrap();

    // The yanked content should handle short lines appropriately
    // Lines shorter than the block should contribute empty strings
    assert_eq!(editor.register.content, "ort\nnge\n\nry ");
}

#[test]
fn test_visual_block_mode_with_multimode_check() {
    let mut editor = Editor::new();
    let buffer = Buffer::from_file("test content");
    editor.buffers[0].buffer = buffer;

    // Enter visual block mode
    editor.enter_visual_block_mode();

    // Check mode state directly
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_block_mode);
    assert!(!editor.visual_line_mode);

    // Enter normal visual mode for comparison
    editor.exit_visual_mode();
    editor.enter_visual_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(!editor.visual_block_mode);
    assert!(!editor.visual_line_mode);

    // Enter visual line mode for comparison
    editor.exit_visual_mode();
    editor.enter_visual_line_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(!editor.visual_block_mode);
    assert!(editor.visual_line_mode);
}

#[test]
fn test_visual_block_mode_composite_undo() {
    let mut editor = Editor::new();

    // Set up test content with a rectangular block to delete
    let content = "123456789\nabcdefghi\nABCDEFGHI\n987654321";
    let buffer = Buffer::from_file(content);
    editor.buffers[0].buffer = buffer;

    // Position cursor at (1, 2) - on 'c' in "abcdefghi"
    editor.move_cursor(1, 2);

    // Enter visual block mode
    editor.enter_visual_block_mode();
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_block_mode);
    assert!(!editor.visual_line_mode);

    // Move cursor to (2, 4) to select a 2x3 block (from col 2-4, rows 1-2)
    // This should select:
    // "cde" from "abcdefghi"
    // "CDE" from "ABCDEFGHI"
    editor.move_cursor(2, 4);

    // Verify selection
    if let Some((start, end)) = editor.get_visual_selection() {
        assert_eq!(start.row, 1);
        assert_eq!(start.col, 2);
        assert_eq!(end.row, 2);
        assert_eq!(end.col, 4);
    } else {
        panic!("No visual selection found");
    }

    // Delete the visual block selection
    editor.delete_visual_selection().unwrap();

    // Verify the content after deletion
    // Should be:
    // "123456789" (unchanged)
    // "abfghi" (removed "cde")
    // "ABFGHI" (removed "CDE")
    // "987654321" (unchanged)
    assert_eq!(editor.buffer().get_line(0).unwrap(), "123456789");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "abfghi");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "ABFGHI");
    assert_eq!(editor.buffer().get_line(3).unwrap(), "987654321");
    assert_eq!(editor.mode, Mode::Normal);
    assert!(!editor.visual_block_mode);

    // Test composite undo - should restore the entire block in one operation
    editor.undo();

    // Verify all content is restored exactly as it was
    assert_eq!(editor.buffer().get_line(0).unwrap(), "123456789");
    assert_eq!(editor.buffer().get_line(1).unwrap(), "abcdefghi");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "ABCDEFGHI");
    assert_eq!(editor.buffer().get_line(3).unwrap(), "987654321");

    // Test redo - should delete the block again
    editor.redo();
    assert_eq!(editor.buffer().get_line(1).unwrap(), "abfghi");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "ABFGHI");

    // Test that undo/redo can be repeated
    editor.undo();
    assert_eq!(editor.buffer().get_line(1).unwrap(), "abcdefghi");
    assert_eq!(editor.buffer().get_line(2).unwrap(), "ABCDEFGHI");
}

#[test]
fn test_visual_block_mode_with_ctrl_v_keymap() {
    let mut editor = Editor::new();

    // Test that Ctrl+V enters visual block mode
    // Set up some content
    let buffer = Buffer::from_file("line1\nline2\nline3");
    editor.buffers[0].buffer = buffer;

    // Simulate Ctrl+V key press
    let key = rustvim::input::Key::Ctrl('v');
    editor.handle_keymap_input(&key).unwrap();

    // Should be in visual block mode
    assert_eq!(editor.mode, Mode::Visual);
    assert!(editor.visual_block_mode);
    assert!(!editor.visual_line_mode);
}
