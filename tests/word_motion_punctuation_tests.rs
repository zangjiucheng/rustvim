use rustvim::buffer::Buffer;
use rustvim::editor::Editor;
use rustvim::input::Key;

// Helper function to create an editor with test content
fn create_test_editor_with_content(content: &str) -> Editor {
    let mut editor = Editor::new();
    let buffer = Buffer::from_file(content);
    editor.buffers[0].buffer = buffer;
    editor.buffers[0].modified = false;
    editor
}

#[test]
fn test_word_forward_with_punctuation() {
    let mut editor = create_test_editor_with_content("hello, world! test?");

    // Start at beginning of line
    editor.move_cursor(0, 0);
    assert_eq!(editor.cursor().col, 0); // on 'h'

    // Move forward one word - should go to ','
    editor.handle_keymap_input(&Key::Char('w')).unwrap();
    assert_eq!(editor.cursor().col, 5); // on ','

    // Move forward again - should go to 'w' in "world"
    editor.handle_keymap_input(&Key::Char('w')).unwrap();
    assert_eq!(editor.cursor().col, 7); // on 'w'

    // Move forward again - should go to '!'
    editor.handle_keymap_input(&Key::Char('w')).unwrap();
    assert_eq!(editor.cursor().col, 12); // on '!'

    // Move forward again - should go to 't' in "test"
    editor.handle_keymap_input(&Key::Char('w')).unwrap();
    assert_eq!(editor.cursor().col, 14); // on 't'

    // Move forward again - should go to '?'
    editor.handle_keymap_input(&Key::Char('w')).unwrap();
    assert_eq!(editor.cursor().col, 18); // on '?'
}

#[test]
fn test_word_backward_with_punctuation() {
    let mut editor = create_test_editor_with_content("hello, world! test?");

    // Start at end of line
    editor.move_cursor(0, 18); // on '?'

    // Move backward one word - should go to 't' in "test"
    editor.handle_keymap_input(&Key::Char('b')).unwrap();
    assert_eq!(editor.cursor().col, 14); // on 't'

    // Move backward again - should go to '!'
    editor.handle_keymap_input(&Key::Char('b')).unwrap();
    assert_eq!(editor.cursor().col, 12); // on '!'

    // Move backward again - should go to 'w' in "world"
    editor.handle_keymap_input(&Key::Char('b')).unwrap();
    assert_eq!(editor.cursor().col, 7); // on 'w'

    // Move backward again - should go to ','
    editor.handle_keymap_input(&Key::Char('b')).unwrap();
    assert_eq!(editor.cursor().col, 5); // on ','

    // Move backward again - should go to 'h' in "hello"
    editor.handle_keymap_input(&Key::Char('b')).unwrap();
    assert_eq!(editor.cursor().col, 0); // on 'h'
}

#[test]
fn test_word_end_with_punctuation() {
    let mut editor = create_test_editor_with_content("hello, world! test?");

    // Start at beginning of line
    editor.move_cursor(0, 0); // on 'h'

    // Move to end of word - should go to 'o' in "hello"
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 4); // on 'o'

    // Move to end of next word - should go to ','
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 5); // on ','

    // Move to end of next word - should go to 'd' in "world"
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 11); // on 'd'

    // Move to end of next word - should go to '!'
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 12); // on '!'

    // Move to end of next word - should go to 't' in "test"
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 17); // on 't'

    // Move to end of next word - should go to '?'
    editor.handle_keymap_input(&Key::Char('e')).unwrap();
    assert_eq!(editor.cursor().col, 18); // on '?'
}

#[test]
fn test_word_motions_with_mixed_punctuation() {
    let mut editor = create_test_editor_with_content("test() { return 42; }");

    // Start at beginning
    editor.move_cursor(0, 0); // on 't'

    // Should move: test -> () -> { -> return -> 42 -> ; -> }
    // With our improved punctuation handling:
    // "test() { return 42; }"
    //  ^   ^^ ^ ^      ^  ^ ^
    //  0   4  7 9     16 18 20 (positions)
    let expected_positions = [4, 7, 9, 16, 18, 20]; // positions after each 'w'

    for &expected_col in &expected_positions {
        editor.handle_keymap_input(&Key::Char('w')).unwrap();
        assert_eq!(
            editor.cursor().col,
            expected_col,
            "Failed at position {}",
            expected_col
        );
    }
}

#[test]
fn test_word_motions_delete_with_punctuation() {
    let mut editor = create_test_editor_with_content("hello, world!");

    // Position cursor at beginning
    editor.move_cursor(0, 0);

    // Delete word "hello" - should delete just "hello"
    editor.handle_keymap_input(&Key::Char('d')).unwrap();
    editor.handle_keymap_input(&Key::Char('w')).unwrap();

    // Should be left with ", world!"
    assert_eq!(editor.buffer().get_line(0).unwrap(), ", world!");
    assert_eq!(editor.cursor().col, 0); // cursor should be at start of deletion

    // Delete the comma and space in one motion
    editor.handle_keymap_input(&Key::Char('d')).unwrap();
    editor.handle_keymap_input(&Key::Char('w')).unwrap();

    // Should be left with "world!" (comma and space both deleted)
    assert_eq!(editor.buffer().get_line(0).unwrap(), "world!");

    // Delete "world"
    editor.handle_keymap_input(&Key::Char('d')).unwrap();
    editor.handle_keymap_input(&Key::Char('w')).unwrap();

    // Should be left with "!"
    assert_eq!(editor.buffer().get_line(0).unwrap(), "!");
}

#[test]
fn test_word_motions_across_symbols() {
    let mut editor = create_test_editor_with_content("foo->bar == baz");

    // Start at beginning
    editor.move_cursor(0, 0); // on 'f'

    // Move through: foo -> -> bar -> == -> baz
    // With our improved punctuation handling:
    // "foo->bar == baz"
    //  ^  ^  ^   ^  ^
    //  0  3  5   9 12 (positions)
    let expected_positions = [3, 5, 9, 12]; // positions after each 'w'

    for &expected_col in &expected_positions {
        editor.handle_keymap_input(&Key::Char('w')).unwrap();
        assert_eq!(
            editor.cursor().col,
            expected_col,
            "Failed at position {}",
            expected_col
        );
    }
}
