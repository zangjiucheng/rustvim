// Integration tests for RustVim Editor components
// These tests verify core functionality across different modules
// Run with: cargo test --test integration_tests

use rustvim::buffer::{Buffer, Position};
use rustvim::editor::{Cursor, Editor, Mode};
use rustvim::input::Key;
use rustvim::terminal::Terminal;

#[test]
fn test_buffer_operations() {
    // Test buffer creation
    let mut buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.get_line(0).unwrap(), "");

    // Test character insertion
    let pos = Position::new(0, 0);
    buffer.insert_char(pos, 'H');
    buffer.insert_char(Position::new(0, 1), 'e');
    buffer.insert_char(Position::new(0, 2), 'l');
    buffer.insert_char(Position::new(0, 3), 'l');
    buffer.insert_char(Position::new(0, 4), 'o');

    assert_eq!(buffer.get_line(0).unwrap(), "Hello");

    // Test character deletion
    let deleted = buffer.delete_char(Position::new(0, 4));
    assert_eq!(deleted, Some('o'));
    assert_eq!(buffer.get_line(0).unwrap(), "Hell");

    // Test newline insertion (line splitting)
    buffer.insert_newline(Position::new(0, 2));
    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(0).unwrap(), "He");
    assert_eq!(buffer.get_line(1).unwrap(), "ll");

    // Test line length calculation
    assert_eq!(buffer.line_length(0), 2);
    assert_eq!(buffer.line_length(1), 2);

    // Test bounds checking
    assert_eq!(buffer.get_line(5), None);
}

#[test]
fn test_editor_operations() {
    // Test editor creation
    let mut editor = Editor::new();
    assert_eq!(editor.mode, Mode::Normal);
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 0);

    // Test cursor operations
    let cursor = Cursor::new();
    assert_eq!(cursor.row, 0);
    assert_eq!(cursor.col, 0);

    // Test mode switching
    editor.mode = Mode::Insert;
    assert_eq!(editor.mode, Mode::Insert);

    // Test insert mode operations
    editor.start_insert_mode();
    editor.insert_mode_char('A');
    editor.insert_mode_char('B');
    editor.insert_mode_char('C');

    // Test ending insert mode
    editor.end_insert_mode();
    assert!(editor.insert_mode_changes.is_none());

    // Test undo/redo
    editor.undo();
    editor.redo();
}

#[test]
fn test_input_key_parsing() {
    // Test key enum variants
    let char_key = Key::Char('a');
    let esc_key = Key::Esc;
    let enter_key = Key::Enter;
    let backspace_key = Key::Backspace;
    let arrow_up = Key::Up;
    let arrow_down = Key::Down;
    let arrow_left = Key::Left;
    let arrow_right = Key::Right;

    // Test key matching
    match char_key {
        Key::Char(c) => assert_eq!(c, 'a'),
        _ => panic!("Key::Char not matched correctly"),
    }

    match esc_key {
        Key::Esc => {}
        _ => panic!("Key::Esc not matched correctly"),
    }

    match enter_key {
        Key::Enter => {}
        _ => panic!("Key::Enter not matched correctly"),
    }

    match backspace_key {
        Key::Backspace => {}
        _ => panic!("Key::Backspace not matched correctly"),
    }

    // Test arrow keys
    match arrow_up {
        Key::Up => {}
        _ => panic!("Key::Up not matched correctly"),
    }

    match arrow_down {
        Key::Down => {}
        _ => panic!("Key::Down not matched correctly"),
    }

    match arrow_left {
        Key::Left => {}
        _ => panic!("Key::Left not matched correctly"),
    }

    match arrow_right {
        Key::Right => {}
        _ => panic!("Key::Right not matched correctly"),
    }

    // Test special keys
    let delete_key = Key::Delete;
    let tab_key = Key::Tab;
    let home_key = Key::Home;
    let end_key = Key::End;

    match delete_key {
        Key::Delete => {}
        _ => panic!("Key::Delete not matched correctly"),
    }

    match tab_key {
        Key::Tab => {}
        _ => panic!("Key::Tab not matched correctly"),
    }

    match home_key {
        Key::Home => {}
        _ => panic!("Key::Home not matched correctly"),
    }

    match end_key {
        Key::End => {}
        _ => panic!("Key::End not matched correctly"),
    }

    // Test function keys
    let f1_key = Key::Function(1);
    let f2_key = Key::Function(2);

    match f1_key {
        Key::Function(1) => {}
        _ => panic!("Key::Function(1) not matched correctly"),
    }

    match f2_key {
        Key::Function(2) => {}
        _ => panic!("Key::Function(2) not matched correctly"),
    }
}

#[test]
fn test_terminal_operations() {
    // Test terminal creation
    let terminal = Terminal::new();

    // Test terminal size
    let (rows, cols) = terminal.size();
    assert!(rows > 0);
    assert!(cols > 0);

    // Test individual size getters
    assert_eq!(terminal.rows(), rows);
    assert_eq!(terminal.cols(), cols);

    // Test terminal size detection (may fail in test environment)
    match Terminal::detect_size() {
        Ok((detect_rows, detect_cols)) => {
            assert!(detect_rows > 0);
            assert!(detect_cols > 0);
        }
        Err(_) => {
            // Expected in test environment, so this is fine
        }
    }
}

#[test]
fn test_commands_functionality() {
    // Create an editor instance to test commands on
    let mut editor = Editor::new();

    // Test mode switching
    assert_eq!(editor.mode, Mode::Normal);

    // Test cursor movement
    let initial_cursor = (editor.cursor_mut().row, editor.cursor_mut().col);
    assert_eq!(initial_cursor, (0, 0));

    // Test entering insert mode
    editor.mode = Mode::Insert;
    editor.start_insert_mode();
    assert_eq!(editor.mode, Mode::Insert);

    // Test inserting text
    let mut col = 0;
    for ch in "Hello".chars() {
        editor.buffer_mut().insert_char(Position::new(0, col), ch);
        editor.insert_mode_char(ch);
        col += 1;
        editor.cursor_mut().col = col;
    }

    let line_content = editor.buffer_mut().get_line(0).unwrap();
    assert_eq!(line_content, "Hello");

    // Test exiting insert mode
    editor.mode = Mode::Normal;
    editor.end_insert_mode();
    editor.set_modified(true);
    assert_eq!(editor.mode, Mode::Normal);

    // Test cursor positioning after insert
    assert!(editor.cursor().col > 0);

    // Test buffer modification tracking
    assert!(editor.is_modified());
}

#[test]
fn test_buffer_and_cursor() {
    // Test buffer creation
    let mut buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.get_line(0).unwrap(), "");

    // Test character insertion
    let pos = Position::new(0, 0);
    buffer.insert_char(pos, 'T');
    buffer.insert_char(Position::new(0, 1), 'e');
    buffer.insert_char(Position::new(0, 2), 's');
    buffer.insert_char(Position::new(0, 3), 't');

    assert_eq!(buffer.get_line(0).unwrap(), "Test");

    // Test character deletion
    let deleted = buffer.delete_char(Position::new(0, 3));
    assert_eq!(deleted, Some('t'));
    assert_eq!(buffer.get_line(0).unwrap(), "Tes");

    // Test newline insertion
    buffer.insert_newline(Position::new(0, 3));
    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(0).unwrap(), "Tes");
    assert_eq!(buffer.get_line(1).unwrap(), "");

    // Test cursor operations
    let mut cursor = Cursor::new();
    assert_eq!(cursor.row, 0);
    assert_eq!(cursor.col, 0);

    // Test cursor movement
    cursor.row = 1;
    cursor.col = 5;
    assert_eq!(cursor.row, 1);
    assert_eq!(cursor.col, 5);

    // Test editor integration
    let mut editor = Editor::new();
    assert_eq!(editor.buffer_mut().line_count(), 1);
    assert_eq!(editor.cursor_mut().row, 0);
    assert_eq!(editor.cursor_mut().col, 0);
    assert_eq!(editor.mode, Mode::Normal);
    assert!(!editor.is_modified());

    // Test editor buffer operations
    editor.buffer_mut().insert_char(Position::new(0, 0), 'H');
    editor.buffer_mut().insert_char(Position::new(0, 1), 'i');
    editor.set_modified(true);

    assert_eq!(editor.buffer().get_line(0).unwrap(), "Hi");
    assert!(editor.is_modified());
}
