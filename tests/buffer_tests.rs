use rustvim::buffer::{Buffer, Position};

#[test]
fn test_buffer_creation() {
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.get_line(0), Some(String::new()));
}

#[test]
fn test_insert_char() {
    let mut buffer = Buffer::new();
    buffer.insert_char(Position::new(0, 0), 'h');
    buffer.insert_char(Position::new(0, 1), 'i');

    assert_eq!(buffer.get_line(0), Some("hi".to_string()));
}

#[test]
fn test_delete_char() {
    let mut buffer = Buffer::new();
    buffer.insert_char(Position::new(0, 0), 'h');
    buffer.insert_char(Position::new(0, 1), 'i');

    let deleted = buffer.delete_char(Position::new(0, 1));
    assert_eq!(deleted, Some('i'));
    assert_eq!(buffer.get_line(0), Some("h".to_string()));
}

#[test]
fn test_insert_newline() {
    let mut buffer = Buffer::new();
    buffer.insert_char(Position::new(0, 0), 'h');
    buffer.insert_char(Position::new(0, 1), 'i');
    buffer.insert_newline(Position::new(0, 1));

    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(0), Some("h".to_string()));
    assert_eq!(buffer.get_line(1), Some("i".to_string()));
}

#[test]
fn test_yank_paste_operations() {
    let mut buffer = Buffer::new();
    buffer.insert_char(Position::new(0, 0), 'h');
    buffer.insert_char(Position::new(0, 1), 'e');
    buffer.insert_char(Position::new(0, 2), 'l');
    buffer.insert_char(Position::new(0, 3), 'l');
    buffer.insert_char(Position::new(0, 4), 'o');
    buffer.insert_newline(Position::new(0, 5));
    buffer.insert_char(Position::new(1, 0), 'w');
    buffer.insert_char(Position::new(1, 1), 'o');
    buffer.insert_char(Position::new(1, 2), 'r');
    buffer.insert_char(Position::new(1, 3), 'l');
    buffer.insert_char(Position::new(1, 4), 'd');

    // Extract "lo\nwor" - from position (0,3) which is 'l' to position (1,3) which is 'l' (exclusive)
    let yanked = buffer.extract_range((0, 3), (1, 3));
    assert_eq!(yanked, "lo\nwor");

    // Add a new line for testing
    buffer.insert_newline(Position::new(1, 5));

    // Paste at beginning of new line
    buffer.insert_char(Position::new(2, 0), 'P');
    buffer.insert_char(Position::new(2, 1), 'A');
    buffer.insert_char(Position::new(2, 2), 'S');
    buffer.insert_char(Position::new(2, 3), 'T');
    buffer.insert_char(Position::new(2, 4), 'E');

    assert_eq!(buffer.get_line(2), Some("PASTE".to_string()));
}

#[test]
fn test_newline_preservation() {
    // Test file content that ends with newline
    let content_with_newline = "hello\nworld\n";
    let buffer_with_newline = Buffer::from_file(content_with_newline);
    assert_eq!(buffer_with_newline.line_count(), 2);
    assert_eq!(buffer_with_newline.get_line(0), Some("hello".to_string()));
    assert_eq!(buffer_with_newline.get_line(1), Some("world".to_string()));
    assert!(buffer_with_newline.ends_with_newline);

    // Test file content that does NOT end with newline
    let content_without_newline = "hello\nworld";
    let buffer_without_newline = Buffer::from_file(content_without_newline);
    assert_eq!(buffer_without_newline.line_count(), 2);
    assert_eq!(
        buffer_without_newline.get_line(0),
        Some("hello".to_string())
    );
    assert_eq!(
        buffer_without_newline.get_line(1),
        Some("world".to_string())
    );
    assert!(!buffer_without_newline.ends_with_newline);

    // Test empty file
    let empty_content = "";
    let empty_buffer = Buffer::from_file(empty_content);
    assert_eq!(empty_buffer.line_count(), 1);
    assert_eq!(empty_buffer.get_line(0), Some(String::new()));
    assert!(!empty_buffer.ends_with_newline);

    // Test single newline file
    let newline_only = "\n";
    let newline_buffer = Buffer::from_file(newline_only);
    assert_eq!(newline_buffer.line_count(), 1);
    assert_eq!(newline_buffer.get_line(0), Some(String::new()));
    assert!(newline_buffer.ends_with_newline);
}

#[test]
fn test_delete_char_at_end_of_line_merges() {
    let mut buffer = Buffer::new();
    buffer.insert_char(Position::new(0, 0), 'h');
    buffer.insert_char(Position::new(0, 1), 'i');
    buffer.insert_newline(Position::new(0, 2));
    buffer.insert_char(Position::new(1, 0), 't');
    buffer.insert_char(Position::new(1, 1), 'h');
    buffer.insert_char(Position::new(1, 2), 'e');
    buffer.insert_char(Position::new(1, 3), 'r');

    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(0), Some("hi".to_string()));
    assert_eq!(buffer.get_line(1), Some("ther".to_string()));

    let deleted = buffer.delete_char(Position::new(0, 2));
    assert_eq!(deleted, Some('\n'));
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.get_line(0), Some("hither".to_string()));
}
