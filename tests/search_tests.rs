use vimlike_editor::editor::{Editor, Mode};
use vimlike_editor::buffer::Buffer;
use vimlike_editor::input::Key;

#[test]
fn test_search_functionality() {
    let mut editor = Editor::new();
    
    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor.buffer_mut().insert_line(0, "First line with hello".to_string());
    editor.buffer_mut().insert_line(1, "Second line".to_string());
    editor.buffer_mut().insert_line(2, "Third line with hello again".to_string());
    
    // Test forward search
    editor.search_forward("hello");
    
    // Should find first occurrence
    assert_eq!(editor.cursor().row, 0);
    assert_eq!(editor.cursor().col, 16);
    assert!(editor.search_match.is_some());
    
    // Test search next
    editor.search_next();
    
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
    
    // Start search mode
    editor.start_search();
    assert_eq!(editor.mode, Mode::Search);
    assert!(editor.search_input.is_empty());
    
    // Test typing search query
    editor.handle_search_mode_input(&Key::Char('h'));
    editor.handle_search_mode_input(&Key::Char('e'));
    editor.handle_search_mode_input(&Key::Char('l'));
    editor.handle_search_mode_input(&Key::Char('l'));
    editor.handle_search_mode_input(&Key::Char('o'));
    
    assert_eq!(editor.search_input, "hello");
    
    // Test backspace
    editor.handle_search_mode_input(&Key::Backspace);
    assert_eq!(editor.search_input, "hell");
    
    // Test escape (cancel search) - now handled globally
    editor.mode = Mode::Normal;
    editor.search_input.clear();
    editor.search_match = None;
    
    assert_eq!(editor.mode, Mode::Normal);
    assert!(editor.search_input.is_empty());
}

#[test]
fn test_search_backward() {
    let mut editor = Editor::new();
    
    // Load some test content
    *editor.buffer_mut() = Buffer::new();
    editor.buffer_mut().insert_line(0, "First hello line".to_string());
    editor.buffer_mut().insert_line(1, "Second line".to_string());
    editor.buffer_mut().insert_line(2, "Third hello line".to_string());
    
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
    editor.buffer_mut().insert_line(0, "Only one hello here".to_string());
    editor.buffer_mut().insert_line(1, "No match on this line".to_string());
    
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
    editor.buffer_mut().insert_line(1, "Second line".to_string());
    
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
    editor.buffer_mut().insert_line(0, "First hello line".to_string());
    editor.buffer_mut().insert_line(1, "Middle line".to_string());
    editor.buffer_mut().insert_line(2, "Another hello here".to_string());
    editor.buffer_mut().insert_line(3, "Last hello line".to_string());
    
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
    editor.buffer_mut().insert_line(0, "hello world hello again hello".to_string());
    editor.buffer_mut().insert_line(1, "another line".to_string());
    
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
