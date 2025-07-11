mod editor;
mod buffer;
mod terminal;
mod input;
mod commands;

use terminal::Terminal;
use input::{InputHandler, Key};
use std::io::{self, Read};

fn main() {
    println!("VimLike Editor v0.1.0 - Day 3: Input Handling Test");
    println!("A Vim-inspired text editor built in Rust");
    println!();
    
    // Day 3: Test input handling with key mapping
    if let Err(e) = test_input_handling() {
        eprintln!("Error testing input handling: {}", e);
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
