use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::{Action, Keymap, KeymapProcessor};

#[test]
fn test_search_mode_keymap_default() {
    let search_keymap = Keymap::default_search_keymap();

    // Check that basic search keys are bound
    assert!(search_keymap.contains_key(&Key::Enter));
    assert!(search_keymap.contains_key(&Key::Backspace));
    assert!(search_keymap.contains_key(&Key::Esc));

    // Verify the correct actions
    if let Some(action) = search_keymap.get(&Key::Enter) {
        matches!(action, Action::SearchExecute);
    }
    if let Some(action) = search_keymap.get(&Key::Backspace) {
        matches!(action, Action::SearchBackspace);
    }
    if let Some(action) = search_keymap.get(&Key::Esc) {
        matches!(action, Action::SearchCancel);
    }
}

#[test]
fn test_search_mode_keymap_processor() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Start search mode
    editor.mode = Mode::Search;

    // Test Enter key - should execute search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Normal);

    // Test character input - should add to search input
    editor.mode = Mode::Search;
    editor.search_input.clear();

    let result = processor.process_key(&mut editor, &Key::Char('h'));
    assert!(result.is_ok());
    assert_eq!(editor.search_input, "h");

    let result = processor.process_key(&mut editor, &Key::Char('e'));
    assert!(result.is_ok());
    assert_eq!(editor.search_input, "he");

    // Test backspace - should remove character
    let result = processor.process_key(&mut editor, &Key::Backspace);
    assert!(result.is_ok());
    assert_eq!(editor.search_input, "h");

    // Test escape - should cancel search
    let result = processor.process_key(&mut editor, &Key::Esc);
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(editor.search_input, "");
    assert_eq!(editor.search_match, None);
}

#[test]
fn test_search_mode_character_accumulation() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    editor.mode = Mode::Search;
    editor.search_input.clear();

    // Type a search query character by character
    let chars = ['t', 'e', 's', 't'];
    for ch in chars {
        let result = processor.process_key(&mut editor, &Key::Char(ch));
        assert!(result.is_ok());
    }

    assert_eq!(editor.search_input, "test");
    assert_eq!(editor.mode, Mode::Search); // Should still be in search mode
}

#[test]
fn test_search_mode_execute_with_content() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Replace buffer with content to search in
    use rustvim::buffer::Buffer;
    *editor.buffer_mut() = Buffer::from_file("hello world\ntest line\nmore content");

    editor.mode = Mode::Search;
    editor.search_input = "test".to_string();

    // Execute search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());

    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Should have set the search query
    assert_eq!(editor.search_query, Some("test".to_string()));

    // Should have cleared search input
    assert_eq!(editor.search_input, "");
}

#[test]
fn test_search_mode_special_characters() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    editor.mode = Mode::Search;
    editor.search_input.clear();

    // Test various special characters that should be added to search
    let special_chars = [
        '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=',
    ];
    for ch in special_chars {
        let result = processor.process_key(&mut editor, &Key::Char(ch));
        assert!(result.is_ok());
    }

    let expected: String = special_chars.iter().collect();
    assert_eq!(editor.search_input, expected);
}

#[test]
fn test_search_mode_empty_search_execute() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    editor.mode = Mode::Search;
    editor.search_input.clear(); // Empty search

    // Execute empty search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());

    // Should be back in normal mode
    assert_eq!(editor.mode, Mode::Normal);

    // Search query should remain None since search was empty
    assert_eq!(editor.search_query, None);
}

#[test]
fn test_search_mode_backspace_on_empty() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    editor.mode = Mode::Search;
    editor.search_input.clear(); // Empty search input

    // Backspace on empty search should not cause issues
    let result = processor.process_key(&mut editor, &Key::Backspace);
    assert!(result.is_ok());

    // Should still be in search mode
    assert_eq!(editor.mode, Mode::Search);
    assert_eq!(editor.search_input, "");
}

#[test]
fn test_search_mode_integration_with_global_config() {
    use rustvim::keymap::KeymapConfigBuilder;

    let mut editor = Editor::new();

    // Create custom search keymap configuration
    let config = KeymapConfigBuilder::new()
        .with_mode_defaults(Mode::Search)
        .bind(Mode::Search, Key::Char('q'), Action::SearchCancel) // Custom: 'q' to cancel
        .build();

    editor.configure_keymap(config);

    // Test that custom binding works
    editor.mode = Mode::Search;

    // Extract keymap processor to test custom binding
    let mut processor = std::mem::take(&mut editor.keymap_processor);
    let result = processor.process_key(&mut editor, &Key::Char('q'));
    editor.keymap_processor = processor;

    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Normal); // Should have cancelled search
}

#[test]
fn test_search_navigation_next() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Add content with multiple occurrences
    use rustvim::buffer::Buffer;
    *editor.buffer_mut() = Buffer::from_file("test line\ntest again\nmore test content");

    // Set up an existing search query
    editor.search_query = Some("test".to_string());
    editor.mode = Mode::Normal;

    // Test search next (n key)
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());

    // Should have found a match and moved cursor
    assert!(editor.search_match.is_some());
}

#[test]
fn test_search_navigation_previous() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Add content with multiple occurrences
    use rustvim::buffer::Buffer;
    *editor.buffer_mut() = Buffer::from_file("test line\ntest again\nmore test content");

    // Set up an existing search query and position cursor at end
    editor.search_query = Some("test".to_string());
    editor.mode = Mode::Normal;
    editor.cursor_mut().row = 2; // Move to last line

    // Test search previous (N key)
    let result = processor.process_key(&mut editor, &Key::Char('N'));
    assert!(result.is_ok());

    // Should have found a match and moved cursor backward
    assert!(editor.search_match.is_some());
}

#[test]
fn test_search_navigation_without_query() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // No existing search query
    editor.search_query = None;
    editor.mode = Mode::Normal;

    // Test search next with no previous query
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());

    // Should not crash or cause issues
    assert_eq!(editor.search_match, None);
}

#[test]
fn test_search_full_workflow() {
    let mut editor = Editor::new();
    let mut processor = KeymapProcessor::new();

    // Add content with multiple test occurrences
    use rustvim::buffer::Buffer;
    *editor.buffer_mut() = Buffer::from_file("first test\nsecond test\nthird test");

    // 1. Start search mode
    editor.mode = Mode::Normal;
    let result = processor.process_key(&mut editor, &Key::Char('/'));
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Search);

    // 2. Type search query
    let result = processor.process_key(&mut editor, &Key::Char('t'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('e'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('s'));
    assert!(result.is_ok());
    let result = processor.process_key(&mut editor, &Key::Char('t'));
    assert!(result.is_ok());

    assert_eq!(editor.search_input, "test");

    // 3. Execute search
    let result = processor.process_key(&mut editor, &Key::Enter);
    assert!(result.is_ok());
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(editor.search_query, Some("test".to_string()));

    // 4. Navigate to next match
    let first_match = editor.search_match;
    let result = processor.process_key(&mut editor, &Key::Char('n'));
    assert!(result.is_ok());

    // Should have different match position
    assert!(editor.search_match.is_some());
    assert_ne!(editor.search_match, first_match);
}
