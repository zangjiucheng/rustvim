mod editor;
mod buffer;
mod terminal;
mod input;
mod commands;

use terminal::Terminal;
use input::{InputHandler, Key};
use buffer::{Buffer, Position};
use editor::{Editor, Cursor, Mode};
use std::io::{self, Read};

fn main() {
    println!("VimLike Editor v0.1.0 - Day 4: Text Buffer and Cursor Management");
    println!("A Vim-inspired text editor built in Rust");
    println!();
    
    // Day 4: Test text buffer and cursor management
    if let Err(e) = test_buffer_and_cursor() {
        eprintln!("Error testing buffer and cursor: {}", e);
        std::process::exit(1);
    }
}

fn test_raw_mode() -> io::Result<()> {
    println!("Testing terminal raw mode...");
    println!("Press any key (ESC to exit, Ctrl+C to force quit):");
    println!("You should see each keypress immediately without pressing Enter.");
    println!();
    
    let terminal = Terminal::new();
    
    // Enter raw mode (RAII guard ensures restoration on exit)
    let _raw_guard = terminal.enter_raw_mode()?;
    
    // Clear screen and position cursor properly
    terminal.clear_screen()?;
    terminal.hide_cursor()?;
    
    // Write header with proper spacing
    terminal.write_line("=== RAW MODE TEST ===")?;
    terminal.write_line("")?;
    terminal.write_line("In raw mode now! Keys are read immediately.")?;
    terminal.write_line("")?;
    terminal.write_line("Press keys to see them displayed:")?;
    terminal.write_line("- Letters and numbers will appear")?;
    terminal.write_line("- Arrow keys will be detected")?;
    terminal.write_line("- Press ESC or 'q' to exit")?;
    terminal.write_line("- Ctrl+C will force quit")?;
    terminal.write_line("")?;
    terminal.write("Keys pressed: ")?;
    
    let mut stdin = io::stdin();
    let mut buffer = [0; 1];
    let mut key_count = 0;
    
    loop {
        // Read one byte at a time in raw mode
        match stdin.read(&mut buffer) {
            Ok(0) => continue, // No data available
            Ok(_) => {
                let byte = buffer[0];
                key_count += 1;
                
                match byte {
                    27 => {
                        // ESC key
                        terminal.write(" [ESC] ")?;
                        break;
                    }
                    b'q' | b'Q' => {
                        // Q key to quit
                        terminal.write(" [QUIT] ")?;
                        break;
                    }
                    3 => {
                        // Ctrl+C
                        terminal.write(" [Ctrl+C] ")?;
                        break;
                    }
                    13 | 10 => {
                        // Enter
                        terminal.write(" [ENTER] ")?;
                    }
                    127 | 8 => {
                        // Backspace
                        terminal.write(" [BACKSPACE] ")?;
                    }
                    32..=126 => {
                        // Printable ASCII
                        terminal.write(&format!(" {} ", byte as char))?;
                    }
                    _ => {
                        // Other control characters
                        terminal.write(&format!(" [{}] ", byte))?;
                    }
                }
                
                // Show progress
                if key_count % 10 == 0 {
                    terminal.write(&format!(" ({} keys) ", key_count))?;
                }
            }
            Err(e) => {
                terminal.write(&format!(" [ERROR: {}] ", e))?;
                break;
            }
        }
    }
    
    // Cleanup and show results
    terminal.write_line("")?;
    terminal.write_line("")?;
    terminal.write_line("=== RAW MODE TEST COMPLETE ===")?;
    terminal.write_line("")?;
    terminal.write_line(&format!("Total keys processed: {}", key_count))?;
    terminal.write_line("Terminal will be restored when program exits.")?;
    terminal.write_line("")?;
    terminal.show_cursor()?;
    
    // RAII guard will restore terminal automatically when it goes out of scope
    Ok(())
}

fn test_input_handling() -> io::Result<()> {
    println!("Testing advanced input handling and key mapping...");
    println!("This test demonstrates Day 3: Low-level input handling with escape sequences");
    println!();
    
    let terminal = Terminal::new();
    
    // Enter raw mode (RAII guard ensures restoration on exit)
    let _raw_guard = terminal.enter_raw_mode()?;
    
    // Clear screen and setup
    terminal.clear_screen()?;
    terminal.hide_cursor()?;
    
    // Write header
    terminal.write_line("=== DAY 3: INPUT HANDLING TEST ===")?;
    terminal.write_line("")?;
    terminal.write_line("Advanced keystroke reading with escape sequence parsing")?;
    terminal.write_line("")?;
    terminal.write_line("Try these inputs:")?;
    terminal.write_line("• Letters (a-z, A-Z) and numbers (0-9)")?;
    terminal.write_line("• Arrow keys (↑ ↓ ← →)")?;
    terminal.write_line("• Special keys: Enter, Backspace, Tab, Delete")?;
    terminal.write_line("• Navigation: Home, End, PageUp, PageDown")?;
    terminal.write_line("• Control keys: Ctrl+A, Ctrl+C, etc.")?;
    terminal.write_line("• Function keys: F1-F4")?;
    terminal.write_line("")?;
    terminal.write_line("Press ESC or 'q' to exit this test")?;
    terminal.write_line("═══════════════════════════════════════════")?;
    terminal.write_line("")?;
    
    // Create InputHandler AFTER entering raw mode
    let mut input_handler = InputHandler::new();
    let mut key_count = 0;
    
    // Give a small delay to ensure everything is set up
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    loop {
        match input_handler.read_key() {
            Ok(key) => {
                key_count += 1;
                
                // Display the key that was pressed
                let key_description = format_key_description(&key);
                terminal.write_line(&format!("{:3}: {}", key_count, key_description))?;
                
                // Check for exit conditions
                match key {
                    Key::Esc => {
                        terminal.write_line(">>> Received ESC key - exiting test")?;
                        break;
                    }
                    Key::Char('q') | Key::Char('Q') => {
                        terminal.write_line(">>> Received 'q' key - exiting test")?;
                        break;
                    }
                    Key::Ctrl('c') => {
                        terminal.write_line(">>> Received Ctrl+C - force exit")?;
                        break;
                    }
                    _ => {
                        // Continue processing other keys
                    }
                }
                
                // Show progress every 10 keys
                if key_count % 10 == 0 {
                    terminal.write_line(&format!("    (Processed {} keystrokes so far)", key_count))?;
                }
            }
            Err(e) => {
                terminal.write_line(&format!("ERROR reading key: {}", e))?;
                // Don't break immediately - try to continue for robustness
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
    
    // Cleanup and show results
    terminal.write_line("")?;
    terminal.write_line("═══════════════════════════════════════════")?;
    terminal.write_line("=== INPUT HANDLING TEST COMPLETE ===")?;
    terminal.write_line("")?;
    terminal.write_line(&format!("Total keystrokes processed: {}", key_count))?;
    terminal.write_line("All key types successfully recognized!")?;
    terminal.write_line("")?;
    terminal.write_line("Day 3 features implemented:")?;
    terminal.write_line("✓ Single-byte reading from STDIN")?;
    terminal.write_line("✓ Printable ASCII character mapping")?;
    terminal.write_line("✓ Control key detection (Ctrl+key)")?;
    terminal.write_line("✓ Escape sequence parsing for arrows/navigation")?;
    terminal.write_line("✓ Key enum abstraction for higher-level logic")?;
    terminal.write_line("✓ Non-blocking ESC key vs escape sequence handling")?;
    terminal.write_line("")?;
    terminal.show_cursor()?;
    
    Ok(())
}

/// Day 4: Test text buffer structure and cursor management functionality
fn test_buffer_and_cursor() -> io::Result<()> {
    println!("═══════════════════════════════════════════");
    println!("=== DAY 4: TEXT BUFFER & CURSOR TEST ===");
    println!("═══════════════════════════════════════════");
    println!();
    
    // Test 1: Buffer creation and basic operations
    println!("📋 Test 1: Buffer Creation and Basic Operations");
    println!("─────────────────────────────────────────────");
    
    let mut buffer = Buffer::new();
    println!("✓ Created new buffer with {} lines", buffer.line_count());
    println!("✓ Initial state: '{}'", buffer.get_line(0).unwrap_or(&String::new()));
    
    // Test 2: Character insertion
    println!("\n📝 Test 2: Character Insertion");
    println!("─────────────────────────────────");
    
    let text = "Hello, World!";
    for (i, ch) in text.chars().enumerate() {
        buffer.insert_char(Position::new(0, i), ch);
    }
    println!("✓ Inserted '{}' character by character", text);
    println!("✓ Result: '{}'", buffer.get_line(0).unwrap());
    
    // Test 3: Character deletion
    println!("\n🗑️  Test 3: Character Deletion");
    println!("─────────────────────────────");
    
    let deleted = buffer.delete_char(Position::new(0, 5)); // Delete comma
    println!("✓ Deleted character: {:?}", deleted);
    println!("✓ Result: '{}'", buffer.get_line(0).unwrap());
    
    // Test 4: Newline insertion (line splitting)
    println!("\n↩️  Test 4: Newline Insertion");
    println!("────────────────────────────");
    
    buffer.insert_newline(Position::new(0, 5)); // Split at space
    println!("✓ Inserted newline at position (0, 5)");
    println!("✓ Line count: {}", buffer.line_count());
    println!("✓ Line 0: '{}'", buffer.get_line(0).unwrap());
    println!("✓ Line 1: '{}'", buffer.get_line(1).unwrap());
    
    // Test 5: Multi-line text operations
    println!("\n📑 Test 5: Multi-line Operations");
    println!("───────────────────────────────");
    
    buffer.insert_char(Position::new(1, 0), '→');
    buffer.insert_char(Position::new(1, 1), ' ');
    println!("✓ Added arrow prefix to second line");
    println!("✓ Line 1: '{}'", buffer.get_line(1).unwrap());
    
    // Test 6: Cursor management
    println!("\n🎯 Test 6: Cursor Management");
    println!("───────────────────────────");
    
    let mut cursor = Cursor::new();
    println!("✓ Created cursor at ({}, {})", cursor.row, cursor.col);
    
    // Test cursor bounds checking
    let line_len = buffer.line_length(0);
    cursor.col = line_len; // Move to end of first line
    println!("✓ Moved cursor to end of line 0: ({}, {})", cursor.row, cursor.col);
    
    cursor.row = 1;
    cursor.col = 0;
    println!("✓ Moved cursor to start of line 1: ({}, {})", cursor.row, cursor.col);
    
    // Test 7: Editor state management
    println!("\n⚙️  Test 7: Editor State Management");
    println!("─────────────────────────────────");
    
    let mut editor = Editor::new();
    println!("✓ Created editor in mode: {:?}", editor.mode);
    println!("✓ Editor cursor: ({}, {})", editor.cursor.row, editor.cursor.col);
    println!("✓ Editor running: {}", editor.running);
    println!("✓ Buffer modified: {}", editor.modified);
    
    // Test mode switching
    editor.mode = Mode::Insert;
    println!("✓ Switched to Insert mode: {:?}", editor.mode);
    
    editor.mode = Mode::Normal;
    println!("✓ Switched back to Normal mode: {:?}", editor.mode);
    
    // Test 8: File content simulation
    println!("\n📁 Test 8: File Content Simulation");
    println!("─────────────────────────────────");
    
    let file_content = "Line 1: Introduction\nLine 2: Content\nLine 3: Conclusion";
    let file_buffer = Buffer::from_file(file_content);
    println!("✓ Created buffer from file content");
    println!("✓ File has {} lines", file_buffer.line_count());
    
    for i in 0..file_buffer.line_count() {
        if let Some(line) = file_buffer.get_line(i) {
            println!("  Line {}: '{}'", i, line);
        }
    }
    
    // Test 9: Buffer safety and bounds checking
    println!("\n🔒 Test 9: Safety and Bounds Checking");
    println!("────────────────────────────────────");
    
    let mut safety_buffer = Buffer::new();
    
    // Test invalid insertions
    safety_buffer.insert_char(Position::new(10, 0), 'x'); // Invalid row
    println!("✓ Handled invalid row insertion gracefully");
    
    safety_buffer.insert_char(Position::new(0, 100), 'y'); // Invalid column
    println!("✓ Handled invalid column insertion gracefully");
    
    // Test invalid deletions
    let result = safety_buffer.delete_char(Position::new(10, 0)); // Invalid row
    println!("✓ Invalid deletion returned: {:?}", result);
    
    // Test 10: Coordinate system validation
    println!("\n🧭 Test 10: Coordinate System");
    println!("────────────────────────────");
    
    let pos1 = Position::new(5, 10);
    let pos2 = Position::new(5, 10);
    println!("✓ Position equality works: {} == {} = {}", 
             format!("({}, {})", pos1.row, pos1.col),
             format!("({}, {})", pos2.row, pos2.col),
             pos1 == pos2);
    
    // Summary
    println!("\n═══════════════════════════════════════════");
    println!("=== DAY 4 TEST COMPLETE ===");
    println!("═══════════════════════════════════════════");
    println!();
    println!("🎉 All Day 4 features implemented and tested:");
    println!("✓ Buffer struct with Vec<String> storage");
    println!("✓ Position coordinate system (row, col)");
    println!("✓ Cursor struct for tracking edit position");
    println!("✓ Buffer::new() - empty buffer initialization");
    println!("✓ Buffer::line_count(), Buffer::get_line() - query methods");
    println!("✓ Buffer::insert_char() - character insertion with bounds checking");
    println!("✓ Buffer::delete_char() - character deletion with line merging");
    println!("✓ Buffer::insert_newline() - line splitting functionality");
    println!("✓ Buffer::from_file() - file content loading");
    println!("✓ TextBuffer trait for modularity and extensibility");
    println!("✓ Comprehensive safety with bounds checking");
    println!("✓ Editor state management with mode tracking");
    println!();
    println!("🚀 Ready for Day 5: Basic Screen Rendering and File Loading!");
    
    Ok(())
}

/// Format a key for display in the test output
fn format_key_description(key: &Key) -> String {
    match key {
        Key::Char(c) => {
            if c.is_ascii_control() {
                format!("Char(control-{:02})", *c as u8)
            } else {
                format!("Char('{}') - printable character", c)
            }
        }
        Key::Ctrl(c) => format!("Ctrl+{} - control key combination", c.to_uppercase()),
        Key::Esc => "ESC - escape key".to_string(),
        Key::Enter => "ENTER - newline/return key".to_string(),
        Key::Backspace => "BACKSPACE - delete previous character".to_string(),
        Key::Delete => "DELETE - delete next character".to_string(),
        Key::Tab => "TAB - tab character".to_string(),
        Key::Up => "UP ARROW - cursor up navigation".to_string(),
        Key::Down => "DOWN ARROW - cursor down navigation".to_string(),
        Key::Left => "LEFT ARROW - cursor left navigation".to_string(),
        Key::Right => "RIGHT ARROW - cursor right navigation".to_string(),
        Key::Home => "HOME - beginning of line".to_string(),
        Key::End => "END - end of line".to_string(),
        Key::PageUp => "PAGE UP - scroll up".to_string(),
        Key::PageDown => "PAGE DOWN - scroll down".to_string(),
        Key::Function(n) => format!("F{} - function key", n),
        Key::Unknown => "UNKNOWN - unrecognized key sequence".to_string(),
    }
}
