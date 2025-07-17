use rustvim::buffer::Position;
use rustvim::editor::Editor;

#[test]
fn test_clamp_cursor_to_buffer_basic() {
    let mut editor = Editor::new();

    // Enable line numbers
    editor.show_line_numbers = true;

    // Add some content
    let buffer = &mut editor.buffers[0].buffer;
    buffer.insert_char(Position::new(0, 0), 'H');
    buffer.insert_char(Position::new(0, 1), 'e');
    buffer.insert_char(Position::new(0, 2), 'l');
    buffer.insert_char(Position::new(0, 3), 'l');
    buffer.insert_char(Position::new(0, 4), 'o');

    // Position cursor beyond buffer bounds
    editor.buffers[0].cursor.row = 5; // Beyond line count
    editor.buffers[0].cursor.col = 20; // Beyond line length

    // Test cursor clamping
    editor.clamp_cursor_to_buffer();

    // Verify cursor is now within bounds
    assert!(editor.buffers[0].cursor.row < editor.buffers[0].buffer.line_count());
    assert!(
        editor.buffers[0].cursor.col
            <= editor.buffers[0]
                .buffer
                .line_length(editor.buffers[0].cursor.row)
    );
}

#[test]
fn test_clamp_cursor_to_buffer_empty_buffer() {
    let mut editor = Editor::new();

    // Enable line numbers
    editor.show_line_numbers = true;

    // Position cursor beyond empty buffer bounds
    editor.buffers[0].cursor.row = 5;
    editor.buffers[0].cursor.col = 10;

    // Test cursor clamping on empty buffer
    editor.clamp_cursor_to_buffer();

    // Verify cursor is at valid position
    assert_eq!(editor.buffers[0].cursor.row, 0);
    assert_eq!(editor.buffers[0].cursor.col, 0);
}

#[test]
fn test_coordinate_conversion_with_line_numbers() {
    let mut editor = Editor::new();

    // Enable line numbers
    editor.show_line_numbers = true;

    // Add some content to ensure line numbers are calculated
    let buffer = &mut editor.buffers[0].buffer;
    for i in 1..=5 {
        let line_text = format!("Line {i}");
        for (j, ch) in line_text.chars().enumerate() {
            buffer.insert_char(Position::new(i - 1, j), ch);
        }
        if i < 5 {
            buffer.insert_newline(Position::new(i - 1, line_text.len()));
        }
    }

    // Test gutter width calculation
    let gutter_width = editor.line_number_gutter_width();
    assert!(gutter_width > 0); // Should have gutter for line numbers

    // Test screen to buffer conversion
    let screen_col: usize = 10;
    println!("Gutter width: {gutter_width}");
    if let Some((_, buffer_col)) = editor.screen_to_buffer_coords(0, screen_col) {
        println!(
            "Screen col: {}, Buffer col: {}, Expected: {}",
            screen_col,
            buffer_col,
            screen_col.saturating_sub(1 + gutter_width)
        );
        assert_eq!(buffer_col, screen_col.saturating_sub(1 + gutter_width));
    }

    // Test buffer to screen conversion
    let buffer_pos = (0, 3);
    let screen_pos = editor.buffer_to_screen_coords(buffer_pos.0, buffer_pos.1);
    assert_eq!(screen_pos.1, buffer_pos.1 + 1 + gutter_width);
}

#[test]
fn test_undo_redo_status_messages() {
    let mut editor = Editor::new();

    // Enable line numbers
    editor.show_line_numbers = true;

    // Test undo when no history exists
    editor.undo();
    assert!(editor.status_msg.is_some());
    let msg = editor.status_msg.as_ref().unwrap();
    println!("Undo message: {msg}");
    assert!(
        msg.contains("oldest change")
            || msg.contains("no changes")
            || msg.contains("Nothing to undo")
    );

    // Test redo when no history exists
    editor.redo();
    assert!(editor.status_msg.is_some());
    let msg = editor.status_msg.as_ref().unwrap();
    println!("Redo message: {msg}");
    assert!(msg.contains("newest change") || msg.contains("Nothing to redo"));
}

#[test]
fn test_cursor_bounds_validation() {
    let mut editor = Editor::new();

    // Enable line numbers
    editor.show_line_numbers = true;

    // Add content with multiple lines of different lengths
    {
        let buffer = &mut editor.buffers[0].buffer;

        // Line 0: "Short" (5 chars)
        buffer.insert_char(Position::new(0, 0), 'S');
        buffer.insert_char(Position::new(0, 1), 'h');
        buffer.insert_char(Position::new(0, 2), 'o');
        buffer.insert_char(Position::new(0, 3), 'r');
        buffer.insert_char(Position::new(0, 4), 't');
        buffer.insert_newline(Position::new(0, 5));

        // Line 1: "This is much longer" (19 chars)
        let long_text = "This is much longer";
        for (i, ch) in long_text.chars().enumerate() {
            buffer.insert_char(Position::new(1, i), ch);
        }
    }

    // Test clamping cursor with valid position
    editor.buffers[0].cursor.row = 1;
    editor.buffers[0].cursor.col = 10; // Valid position within long line
    editor.clamp_cursor_to_buffer();
    assert_eq!(editor.buffers[0].cursor.row, 1);
    assert_eq!(editor.buffers[0].cursor.col, 10);

    // Test clamping cursor beyond line length
    editor.buffers[0].cursor.row = 0;
    editor.buffers[0].cursor.col = 20; // Beyond "Short" line
    editor.clamp_cursor_to_buffer();
    assert_eq!(editor.buffers[0].cursor.row, 0);
    assert_eq!(editor.buffers[0].cursor.col, 5); // Should be clamped to line length

    // Test clamping cursor beyond line count
    editor.buffers[0].cursor.row = 10; // Beyond buffer
    editor.buffers[0].cursor.col = 5;
    editor.clamp_cursor_to_buffer();
    assert_eq!(editor.buffers[0].cursor.row, 1); // Should be clamped to last line
    let line_len = editor.buffers[0].buffer.line_length(1);
    assert!(editor.buffers[0].cursor.col <= line_len);
}
