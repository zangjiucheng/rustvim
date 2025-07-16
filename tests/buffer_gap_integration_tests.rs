use rustvim::buffer::{Buffer, Position};

#[test]
fn test_buffer_newline_insertion_with_gap_buffer() {
    let mut buffer = Buffer::new();
    
    // Step 1: Insert "fn main() {" on line 0
    for (i, ch) in "fn main() {".chars().enumerate() {
        buffer.insert_char(Position::new(0, i), ch);
    }
    println!("After step 1, line 0: '{}'", buffer.get_line(0).unwrap());
    assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
    
    // Step 2: Insert newline and content on line 1
    buffer.insert_newline(Position::new(0, 11));
    for (i, ch) in "    println!(\"Hello, world!\");".chars().enumerate() {
        buffer.insert_char(Position::new(1, i), ch);
    }
    
    println!("After step 2:");
    println!("  Line 0: '{}'", buffer.get_line(0).unwrap());
    println!("  Line 1: '{}'", buffer.get_line(1).unwrap());
    println!("  Line count: {}", buffer.line_count());
    
    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.get_line(0).unwrap(), "fn main() {");
    assert_eq!(buffer.get_line(1).unwrap(), "    println!(\"Hello, world!\");");
    
    // Step 3: Add closing brace on new line
    buffer.insert_newline(Position::new(1, 31));
    buffer.insert_char(Position::new(2, 0), '}');
    
    println!("After step 3:");
    println!("  Line 0: '{}'", buffer.get_line(0).unwrap());
    println!("  Line 1: '{}'", buffer.get_line(1).unwrap());
    println!("  Line 2: '{}'", buffer.get_line(2).unwrap());
    
    assert_eq!(buffer.line_count(), 3);
    assert_eq!(buffer.get_line(2).unwrap(), "}");
    
    // Step 4: Add comment at beginning (this is where the issue happens)
    println!("Before step 4:");
    println!("  Line 0: '{}'", buffer.get_line(0).unwrap());
    println!("  Line 1: '{}'", buffer.get_line(1).unwrap());
    println!("  Line 2: '{}'", buffer.get_line(2).unwrap());
    
    // Insert comment characters one by one at beginning of line 0
    buffer.insert_char(Position::new(0, 0), '/');
    println!("After inserting first '/': '{}'", buffer.get_line(0).unwrap());
    
    buffer.insert_char(Position::new(0, 1), '/');
    println!("After inserting second '/': '{}'", buffer.get_line(0).unwrap());
    
    buffer.insert_char(Position::new(0, 2), ' ');
    println!("After inserting space: '{}'", buffer.get_line(0).unwrap());
    
    for (i, ch) in "Main function".chars().enumerate() {
        buffer.insert_char(Position::new(0, 3 + i), ch);
        println!("After inserting '{}': '{}'", ch, buffer.get_line(0).unwrap());
    }
    
    println!("After inserting comment text:");
    println!("  Line 0: '{}'", buffer.get_line(0).unwrap());
    
    // Insert newline to push the rest down
    buffer.insert_newline(Position::new(0, 16));
    
    println!("After inserting newline:");
    println!("  Line 0: '{}'", buffer.get_line(0).unwrap());
    println!("  Line 1: '{}'", buffer.get_line(1).unwrap());
    println!("  Line 2: '{}'", buffer.get_line(2).unwrap());
    println!("  Line 3: '{}'", buffer.get_line(3).unwrap());
    
    // This is where the original test fails
    assert_eq!(buffer.line_count(), 4);
    assert_eq!(buffer.get_line(0).unwrap(), "// Main function");
    assert_eq!(buffer.get_line(1).unwrap(), "fn main() {");  // This line fails with null chars
    assert_eq!(buffer.get_line(2).unwrap(), "    println!(\"Hello, world!\");");
    assert_eq!(buffer.get_line(3).unwrap(), "}");
}
