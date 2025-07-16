use rustvim::buffer::Buffer;
use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::KeymapProcessor;

#[test]
fn test_search_functionality() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "First line with hello".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "Second line".to_string());
    editor
        .buffer_mut()
        .insert_line(2, "Third line with hello again".to_string());

    // Test search using keymap system
    // 1. Start search mode
    let result = processor.process_key(&mut editor, &Key::Char('/'));
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Search);

    // 2. Type search query
    for ch in "hello".chars() {
        let result = processor.process_key(&mut editor, &Key::Char(ch));
        assert!(result.is_ok());
    }
    assert_eq!(editor.search_input, "hello");

    // 3. Execute search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Normal);

    // Should find first occurrence
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 16);
    assert!(editor.search_match.is_some());

    // Test search next using keymap (n key)
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());

    // Should find second occurrence
    assert_eq!(editor.cursor().row, 2);
    assert_eq!(editor.cursor().col, 16);

    // Test search for non-existent pattern
    editor.search_forward("nonexistent");
    assert!(editor.search_match.is_none());
    assert!(editor.status_msg.is_some());
    assert!(editor.status_msg.as_ref().unwrap().contains("not found"));
}

#[test]
fn test_search_mode_input() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Start search mode using keymap processor
    assert_eq!(editor.mode, Mode::Normal);
    let result = processor.process_key(&mut editor, &Key::Char('/'));
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Search);
    assert!(editor.search_input.is_empty());

    // Test typing search query using keymap processor
    let result = processor.process_key(&mut editor, &Key::Char('h'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('e'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('l'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('l'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('o'));
    assert!(result.is_ok());

    assert_eq!(editor.search_input, "hello");

    // Test backspace using keymap processor
    let result = processor.process_key(&mut editor, &Key::Backspace);
    assert!(result.is_ok());
    assert_eq!(editor.search_input, "hell");

    // Test escape (cancel search) using keymap processor
    let result = processor.process_key(&mut editor, &Key::Esc);
    assert!(result.is_ok());

    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.search_input.is_empty());
    assert_eq!(editor.search_match, None);
}

#[test]
fn test_search_backward() {
    let mut editor = Editor::new();

    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "First hello line".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "Second line".to_string());
    editor
        .buffer_mut()
        .insert_line(2, "Third hello line".to_string());

    // Position cursor at end of buffer
    editor.cursor_mut().row = 2;
    editor.cursor_mut().col = 16;

    // Test backward search
    editor.search_backward("hello");

    // Should find the occurrence on line 2 first (before cursor)
    assert_eq!(editor.cursor().row, 2);
    assert_eq!(editor.cursor().col, 6);
    assert!(editor.search_match.is_some());

    // Test search previous
    editor.search_previous();

    // Should find the occurrence on line 0
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 6);
}

#[test]
fn test_search_wrap_around() {
    let mut editor = Editor::new();

    // Load some test content with only one match
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "Only one hello here".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "No match on this line".to_string());

    // Position cursor after the match
    editor.cursor_mut().row = 1;
    editor.cursor_mut().col = 10;

    // Search forward should wrap around
    editor.search_forward("hello");

    // Should find the match at the beginning
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 9);
    assert!(editor.search_match.is_some());

    // Status should indicate wrap around
    assert!(editor.status_msg.is_some());
    assert!(editor.status_msg.as_ref().unwrap().contains("wrapped"));
}

#[test]
fn test_search_no_results() {
    let mut editor = Editor::new();

    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor.buffer_mut().insert_line(0, "First line".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "Second line".to_string());

    // Search for non-existent pattern
    editor.search_forward("xyz123");

    // Should not find anything
    assert!(editor.search_match.is_none());
    assert!(editor.status_msg.is_some());
    assert!(editor.status_msg.as_ref().unwrap().contains("not found"));
}

#[test]
fn test_search_empty_query() {
    let mut editor = Editor::new();

    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor.buffer_mut().insert_line(0, "Test line".to_string());

    // Search with empty query should do nothing
    editor.search_forward("");

    // Cursor should not move
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 0);
    assert!(editor.search_match.is_none());
}

#[test]
fn test_search_backward_wrap_around() {
    let mut editor = Editor::new();

    // Load test content with multiple occurrences
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "First hello line".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "Middle line".to_string());
    editor
        .buffer_mut()
        .insert_line(2, "Another hello here".to_string());
    editor
        .buffer_mut()
        .insert_line(3, "Last hello line".to_string());

    // Position cursor at the very beginning
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 0;

    // First search forward to establish a search query
    editor.search_forward("hello");

    // Should find first occurrence
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 6);

    // Now search backward (N) - should wrap around to the last occurrence
    editor.search_previous();

    // Should wrap around to the last "hello" on line 3
    assert_eq!(editor.cursor_mut().row, 3);
    assert_eq!(editor.cursor_mut().col, 5);
    assert!(editor.search_match.is_some());

    // Status should indicate wrap around
    assert!(editor.status_msg.is_some());
    assert!(editor.status_msg.as_ref().unwrap().contains("wrapped"));
}

#[test]
fn test_search_backward_multiple_on_same_line() {
    let mut editor = Editor::new();

    // Test line with multiple occurrences
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "hello world hello again hello".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "another line".to_string());

    // Start from the end and work backwards
    editor.cursor_mut().row = 0;
    editor.cursor_mut().col = 29; // End of line

    // Search backward to find the last "hello"
    editor.search_backward("hello");

    // Should find the last "hello" at position 24
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 24);

    // Search backward again
    editor.search_previous();

    // Should find the middle "hello" at position 12
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 12);

    // Search backward again
    editor.search_previous();

    // Should find the first "hello" at position 0
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 0);

    // Search backward once more - should wrap to the last "hello"
    editor.search_previous();

    // Should wrap around to the last "hello"
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 24);
}

#[test]
fn test_search_navigation_keymap() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Load test content with multiple matches
    *editor.buffer_mut() = Buffer::new();
    editor
        .buffer_mut()
        .insert_line(0, "First test line".to_string());
    editor
        .buffer_mut()
        .insert_line(1, "Second test here".to_string());
    editor
        .buffer_mut()
        .insert_line(2, "Third test content".to_string());
    editor
        .buffer_mut()
        .insert_line(3, "Final test result".to_string());

    // Perform initial search using keymap
    let result = processor.process_key(&mut editor, &Key::Char('/'));
    assert!(result.is_ok());

    // Type "test"
    for ch in "test".chars() {
        let result = processor.process_key(&mut editor, &Key::Char(ch));
        assert!(result.is_ok());
    }

    // Execute search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());

    // Should find first match
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 6);
    assert!(editor.search_match.is_some());

    // Test 'n' key for next search
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 7);

    // Test another 'n' key
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());
    assert_eq!(editor.cursor().row, 2);
    assert_eq!(editor.cursor().col, 6);

    // Test 'N' key for previous search
    let result = processor.process_key(&mut editor, &Key::Char('N'));
    assert!(result.is_ok());
    assert_eq!(editor.cursor().row, 1);
    assert_eq!(editor.cursor().col, 7);

    // Test 'N' again
    let result = processor.process_key(&mut editor, &Key::Char('N'));
    assert!(result.is_ok());
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 6);
}

#[test]
fn test_search_navigation_no_query() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Test navigation keys without previous search
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok()); // Should not crash

    let result = processor.process_key(&mut editor, &Key::Char('N'));
    assert!(result.is_ok()); // Should not crash

    // No search match should be set
    assert!(editor.search_match.is_none());
    assert!(editor.search_query.is_none());
}
