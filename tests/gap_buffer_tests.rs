use rustvim::gap_buffer::GapBufferLine;

#[test]
fn test_gap_buffer_insert_sequence() {
    let mut buf = GapBufferLine::new();

    // Simulate the failing case: inserting characters one by one
    buf.insert(0, 'f');
    buf.insert(1, 'n');
    buf.insert(2, ' ');
    buf.insert(3, 'm');
    buf.insert(4, 'a');
    buf.insert(5, 'i');
    buf.insert(6, 'n');
    buf.insert(7, '(');
    buf.insert(8, ')');
    buf.insert(9, ' ');
    buf.insert(10, '{');

    // Now insert at the beginning to test gap movement
    buf.insert(0, '/');
    buf.insert(1, '/');
    buf.insert(2, ' ');

    assert_eq!(buf.to_string(), "// fn main() {");
}

#[test]
fn test_gap_buffer_new() {
    let buf = GapBufferLine::new();
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
    assert_eq!(buf.to_string(), "");
}

#[test]
fn test_gap_buffer_from_string() {
    let buf = GapBufferLine::from_string("hello");
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.to_string(), "hello");
}

#[test]
fn test_insert_at_gap() {
    let mut buf = GapBufferLine::new();
    buf.insert(0, 'h');
    buf.insert(1, 'e');
    buf.insert(2, 'l');
    buf.insert(3, 'l');
    buf.insert(4, 'o');

    assert_eq!(buf.to_string(), "hello");
    assert_eq!(buf.len(), 5);
}

#[test]
fn test_insert_middle() {
    let mut buf = GapBufferLine::from_string("hllo");
    buf.insert(1, 'e');
    assert_eq!(buf.to_string(), "hello");
}

#[test]
fn test_delete() {
    let mut buf = GapBufferLine::from_string("hello");
    let deleted = buf.delete(1);
    assert_eq!(deleted, Some('e'));
    assert_eq!(buf.to_string(), "hllo");
}

#[test]
fn test_get_char() {
    let buf = GapBufferLine::from_string("hello");
    assert_eq!(buf.get_char(0), Some('h'));
    assert_eq!(buf.get_char(1), Some('e'));
    assert_eq!(buf.get_char(4), Some('o'));
    assert_eq!(buf.get_char(5), None);
}

#[test]
fn test_complex_operations() {
    let mut buf = GapBufferLine::from_string("abcdef");

    // Insert in middle
    buf.insert(3, 'X');
    assert_eq!(buf.to_string(), "abcXdef");

    // Delete from middle
    buf.delete(1);
    assert_eq!(buf.to_string(), "acXdef");

    // Insert at beginning
    buf.insert(0, 'Z');
    assert_eq!(buf.to_string(), "ZacXdef");

    // Insert at end
    buf.insert(buf.len(), 'Y');
    assert_eq!(buf.to_string(), "ZacXdefY");
}

#[test]
fn test_newline_buffer_operations() {
    // Test for the specific case that was failing in the history test
    let mut buf = GapBufferLine::new();

    // Build "fn main() {" at position 0
    for (i, ch) in "fn main() {".chars().enumerate() {
        buf.insert(i, ch);
    }
    assert_eq!(buf.to_string(), "fn main() {");

    // Now test what happens when we insert characters at the beginning
    // This simulates adding a comment "// " at the start
    buf.insert(0, '/');
    buf.insert(1, '/');
    buf.insert(2, ' ');

    // The line should now be "// fn main() {"
    assert_eq!(buf.to_string(), "// fn main() {");

    // Test substring operations (used by buffer.insert_newline)
    // If we have "// fn main() {" and want to split at position 16
    // But our string is only 14 characters, so test a valid split
    let before_14 = buf.substring(0, 14);
    assert_eq!(before_14, "// fn main() {");

    let after_14 = buf.substring(14, buf.len());
    assert_eq!(after_14, "");
}

#[test]
fn test_substring_operations() {
    let buf = GapBufferLine::from_string("Hello, World!");

    // Test various substring operations
    assert_eq!(buf.substring(0, 5), "Hello");
    assert_eq!(buf.substring(7, 12), "World");
    assert_eq!(buf.substring(0, buf.len()), "Hello, World!");
    assert_eq!(buf.substring(5, 5), ""); // Empty substring
    assert_eq!(buf.substring(10, 5), ""); // Invalid range
    assert_eq!(buf.substring(0, 100), "Hello, World!"); // Beyond end
}

#[test]
fn test_insert_str() {
    let mut buf = GapBufferLine::from_string("Hello");
    buf.insert_str(5, ", World!");
    assert_eq!(buf.to_string(), "Hello, World!");
}

#[test]
fn test_insert_str_middle() {
    let mut buf = GapBufferLine::from_string("Hello, World!");

    // Insert " there" at position 5 (after "Hello")
    buf.insert_str(5, " there");

    assert_eq!(buf.to_string(), "Hello there, World!");
}

#[test]
fn test_delete_range() {
    let mut buf = GapBufferLine::from_string("Hello, World!");

    // Delete ", World"
    let deleted = buf.delete_range(5, 12);
    assert_eq!(deleted, ", World");
    assert_eq!(buf.to_string(), "Hello!");
}

#[test]
fn test_clear() {
    let mut buf = GapBufferLine::from_string("Hello, World!");
    buf.clear();
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
    assert_eq!(buf.to_string(), "");
}

#[test]
fn test_insert_middle_with_existing_content() {
    // Reproduce the exact failing case step by step
    let mut buf = GapBufferLine::from_string("fn main() {");

    buf.insert(0, '/');
    buf.insert(1, '/');
    buf.insert(2, ' ');
    // Now we have "// fn main() {"

    // Insert characters one by one to find the exact failure point
    buf.insert(3, 'M');
    assert_eq!(buf.to_string(), "// Mfn main() {");

    buf.insert(4, 'a');
    assert_eq!(buf.to_string(), "// Mafn main() {");

    buf.insert(5, 'i');
    assert_eq!(buf.to_string(), "// Maifn main() {");

    buf.insert(6, 'n');
    assert_eq!(buf.to_string(), "// Mainfn main() {");

    buf.insert(7, ' ');
    assert_eq!(buf.to_string(), "// Main fn main() {");

    buf.insert(8, 'f');
    assert_eq!(buf.to_string(), "// Main ffn main() {");

    buf.insert(9, 'u');
    assert_eq!(buf.to_string(), "// Main fufn main() {");

    buf.insert(10, 'n');
    assert_eq!(buf.to_string(), "// Main funfn main() {");

    // This is where it was failing - inserting 'c' at position 11
    buf.insert(11, 'c');
    assert_eq!(buf.to_string(), "// Main funcfn main() {");
}
