mod editor;
mod buffer;
mod terminal;
mod input;
mod commands;

use terminal::Terminal;
use std::io::{self, Read};

fn main() {
    println!("VimLike Editor v0.1.0 - Day 2: Raw Mode Test");
    println!("A Vim-inspired text editor built in Rust");
    println!();
    
    // Day 2: Test raw mode functionality
    if let Err(e) = test_raw_mode() {
        eprintln!("Error testing raw mode: {}", e);
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
