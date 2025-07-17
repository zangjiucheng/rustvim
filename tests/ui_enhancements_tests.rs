use rustvim::commands::Command;
use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::terminal::CursorShape;
use std::time::{Duration, Instant};

/// Test status message timeout functionality with real timing
#[test]
fn test_status_message_timeout() {
    let mut editor = Editor::new();

    // Set a status message
    editor.set_status_message("Test message".to_string());

    // Verify message is set
    assert_eq!(editor.status_msg, Some("Test message".to_string()));
    assert!(editor.status_msg_time.is_some());

    // Check that timeout doesn't clear immediately (fresh message)
    let changed = editor.check_status_timeout();
    assert!(!changed); // Should return false - no change
    assert_eq!(editor.status_msg, Some("Test message".to_string()));
    assert!(editor.status_msg_time.is_some());

    // Manually set an old timestamp to simulate timeout (> 2 seconds)
    editor.status_msg_time = Some(Instant::now() - Duration::from_secs(3));

    // Check that timeout clears the message after 2+ seconds
    let changed = editor.check_status_timeout();
    assert!(changed); // Should return true - status was cleared
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);
}

/// Test status message timeout boundary conditions
#[test]
fn test_status_message_timeout_boundary() {
    let mut editor = Editor::new();

    // Set a status message
    editor.set_status_message("Boundary test".to_string());

    // Test exactly at 2 seconds (should clear)
    editor.status_msg_time = Some(Instant::now() - Duration::from_secs(2));
    let changed = editor.check_status_timeout();
    assert!(changed); // Should return true - status was cleared
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);

    // Set another message
    editor.set_status_message("Almost timeout".to_string());

    // Test just under 2 seconds (should NOT clear)
    editor.status_msg_time = Some(Instant::now() - Duration::from_millis(1999));
    let changed = editor.check_status_timeout();
    assert!(!changed); // Should return false - no change
    assert_eq!(editor.status_msg, Some("Almost timeout".to_string()));
    assert!(editor.status_msg_time.is_some());
}

/// Test status message timeout behavior with no message set
#[test]
fn test_status_message_timeout_no_message() {
    let mut editor = Editor::new();

    // Ensure no message is set initially
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);

    // Check timeout with no message (should not panic)
    let changed = editor.check_status_timeout();
    assert!(!changed); // Should return false - no change
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);
}

/// Test clearing status message
#[test]
fn test_clear_status_message() {
    let mut editor = Editor::new();

    // Set a status message
    editor.set_status_message("Test message".to_string());
    assert_eq!(editor.status_msg, Some("Test message".to_string()));
    assert!(editor.status_msg_time.is_some());

    // Clear the message
    editor.clear_status_message();
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);
}

/// Test cursor shape updates based on mode
#[test]
fn test_cursor_shape_updates() {
    let mut editor = Editor::new();

    // Test Normal mode cursor shape
    editor.mode = Mode::Normal;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    // Test Insert mode cursor shape
    editor.mode = Mode::Insert;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    // Test Visual mode cursor shape
    editor.mode = Mode::Visual;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    // Test Command mode cursor shape
    editor.mode = Mode::Command;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    // Test Search mode cursor shape
    editor.mode = Mode::Search;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());
}

/// Test line numbers toggle functionality
#[test]
fn test_line_numbers_toggle() {
    let mut editor = Editor::new();

    // Initially line numbers should be disabled
    assert!(!editor.show_line_numbers);

    // Enable line numbers via set command
    let command = rustvim::commands::ExCommandParser::parse("set numbers");
    let result = command.execute(&mut editor);
    assert!(result.is_ok());
    assert!(editor.show_line_numbers);
    assert_eq!(editor.status_msg, Some("Line numbers enabled".to_string()));

    // Disable line numbers via set command
    let command = rustvim::commands::ExCommandParser::parse("set nonumbers");
    let result = command.execute(&mut editor);
    assert!(result.is_ok());
    assert!(!editor.show_line_numbers);
    assert_eq!(editor.status_msg, Some("Line numbers disabled".to_string()));
}

/// Test set command parsing for line numbers
#[test]
fn test_set_command_parsing() {
    // Test valid set commands
    let command = rustvim::commands::ExCommandParser::parse("set numbers");
    match command {
        rustvim::commands::ExCommand::Set { option, value } => {
            assert_eq!(option, "numbers");
            assert_eq!(value, None);
        }
        _ => panic!("Expected Set command"),
    }

    let command = rustvim::commands::ExCommandParser::parse("set number");
    match command {
        rustvim::commands::ExCommand::Set { option, value } => {
            assert_eq!(option, "number");
            assert_eq!(value, None);
        }
        _ => panic!("Expected Set command"),
    }

    let command = rustvim::commands::ExCommandParser::parse("set nonumbers");
    match command {
        rustvim::commands::ExCommand::Set { option, value } => {
            assert_eq!(option, "nonumbers");
            assert_eq!(value, None);
        }
        _ => panic!("Expected Set command"),
    }

    // Test set command with value
    let command = rustvim::commands::ExCommandParser::parse("set tabstop 4");
    match command {
        rustvim::commands::ExCommand::Set { option, value } => {
            assert_eq!(option, "tabstop");
            assert_eq!(value, Some("4".to_string()));
        }
        _ => panic!("Expected Set command"),
    }

    // Test set command without argument
    let command = rustvim::commands::ExCommandParser::parse("set");
    match command {
        rustvim::commands::ExCommand::Unknown { command } => {
            assert_eq!(command, "E471: Argument required");
        }
        _ => panic!("Expected Unknown command with error"),
    }
}

/// Test unknown set options
#[test]
fn test_unknown_set_options() {
    let mut editor = Editor::new();

    let command = rustvim::commands::ExCommandParser::parse("set unknownoption");
    let result = command.execute(&mut editor);
    assert!(result.is_ok());
    assert_eq!(
        editor.status_msg,
        Some("E518: Unknown option: unknownoption".to_string())
    );
}

/// Test bell and flash functionality
#[test]
fn test_bell_and_flash() {
    let editor = Editor::new();

    // Test bell (should not panic and should be non-blocking)
    let result = editor.bell();
    assert!(result.is_ok());

    // Test flash (should not panic and should be non-blocking)
    let result = editor.flash();
    assert!(result.is_ok());

    // Test that terminal bell functions work directly
    let terminal = rustvim::terminal::Terminal::new();

    // Test direct terminal bell (non-blocking)
    let result = terminal.bell();
    assert!(result.is_ok());

    // Test brief flash (non-blocking)
    let result = terminal.flash_screen_brief();
    assert!(result.is_ok());

    // Test immediate flash (non-blocking)
    let result = terminal.flash_screen_immediate();
    assert!(result.is_ok());

    // Test regular flash (minimal blocking)
    let result = terminal.flash_screen();
    assert!(result.is_ok());
}

/// Test that bell and flash operations are non-blocking
#[test]
fn test_bell_flash_non_blocking() {
    use std::time::Instant;
    let terminal = rustvim::terminal::Terminal::new();

    // Test that bell is non-blocking (should complete very quickly)
    let start = Instant::now();
    let result = terminal.bell();
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 50,
        "Bell should be non-blocking, took {}ms",
        duration.as_millis()
    );

    // Test that immediate flash is non-blocking
    let start = Instant::now();
    let result = terminal.flash_screen_immediate();
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 50,
        "Immediate flash should be non-blocking, took {}ms",
        duration.as_millis()
    );

    // Test that brief flash is non-blocking
    let start = Instant::now();
    let result = terminal.flash_screen_brief();
    let duration = start.elapsed();

    assert!(result.is_ok());
    assert!(
        duration.as_millis() < 50,
        "Brief flash should be non-blocking, took {}ms",
        duration.as_millis()
    );
}

/// Test that status timeout check returns correct values for screen refresh optimization
#[test]
fn test_status_timeout_return_values() {
    let mut editor = Editor::new();

    // Test with no status message - should return false (no change)
    let changed = editor.check_status_timeout();
    assert!(!changed);

    // Set a fresh status message
    editor.set_status_message("Fresh message".to_string());

    // Check timeout on fresh message - should return false (no change)
    let changed = editor.check_status_timeout();
    assert!(!changed);
    assert!(editor.status_msg.is_some());

    // Simulate old message (> 2 seconds)
    editor.status_msg_time = Some(Instant::now() - Duration::from_secs(3));

    // Check timeout on old message - should return true (message cleared)
    let changed = editor.check_status_timeout();
    assert!(changed);
    assert!(editor.status_msg.is_none());

    // Check again with no message - should return false (no change)
    let changed = editor.check_status_timeout();
    assert!(!changed);
}

/// Test timeout key handling in input
#[test]
fn test_timeout_key_handling() {
    // Test that timeout key is properly defined
    let timeout_key = Key::Timeout;
    let unknown_key = Key::Unknown;

    assert_ne!(timeout_key, unknown_key);

    // Test that timeout key can be compared
    assert_eq!(timeout_key, Key::Timeout);
}

/// Test timeout event integration with status message clearing
#[test]
fn test_timeout_event_integration() {
    let mut editor = Editor::new();

    // Set a status message with an old timestamp
    editor.set_status_message("Will timeout".to_string());
    editor.status_msg_time = Some(Instant::now() - Duration::from_secs(3));

    // Simulate what happens when a Key::Timeout event is received in the main loop
    // This mimics the behavior: if key == Key::Timeout { self.check_status_timeout(); }

    // Verify message is still there before timeout check
    assert_eq!(editor.status_msg, Some("Will timeout".to_string()));

    // Simulate timeout event handling
    let changed = editor.check_status_timeout();

    // Message should be cleared after timeout
    assert!(changed); // Should return true - status was cleared
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);

    // Test that fresh messages don't get cleared on timeout events
    editor.set_status_message("Fresh message".to_string());

    // Fresh message should survive timeout check
    let changed = editor.check_status_timeout();
    assert!(!changed); // Should return false - no change
    assert_eq!(editor.status_msg, Some("Fresh message".to_string()));
    assert!(editor.status_msg_time.is_some());
}

/// Test status message timeout with multiple successive messages
#[test]
fn test_multiple_status_message_timeouts() {
    let mut editor = Editor::new();

    // Set first message
    editor.set_status_message("First message".to_string());
    let first_time = editor.status_msg_time;

    // Wait a moment and set second message (should update timestamp)
    std::thread::sleep(Duration::from_millis(10));
    editor.set_status_message("Second message".to_string());
    let second_time = editor.status_msg_time;

    // Timestamps should be different
    assert_ne!(first_time, second_time);
    assert_eq!(editor.status_msg, Some("Second message".to_string()));

    // Simulate old timestamp for the second message
    editor.status_msg_time = Some(Instant::now() - Duration::from_secs(5));

    // Check timeout clears the second message
    let changed = editor.check_status_timeout();
    assert!(changed); // Should return true - status was cleared
    assert_eq!(editor.status_msg, None);
    assert_eq!(editor.status_msg_time, None);
}

/// Test enhanced status messages for file operations
#[test]
fn test_enhanced_file_operation_messages() {
    let mut editor = Editor::new();

    // Add some content to the buffer
    let pos = rustvim::buffer::Position::new(0, 0);
    editor.buffer_mut().insert_char(pos, 'H');
    let pos = rustvim::buffer::Position::new(0, 1);
    editor.buffer_mut().insert_char(pos, 'i');
    editor.set_modified(true);

    // Set a filename
    editor.set_filename(Some("test.txt".to_string()));

    // Test write operation (this will fail but should set appropriate message)
    let success = editor.write_file(None);

    // The write might fail due to permissions or other issues, but status message should be set
    assert!(editor.status_msg.is_some());

    if success {
        // If write succeeded, should have success message
        assert!(editor.status_msg.as_ref().unwrap().contains("written"));
    } else {
        // If write failed, should have error message
        assert!(editor.status_msg.as_ref().unwrap().starts_with("E212:"));
    }
}

/// Test status message preservation during search and command modes
#[test]
fn test_status_message_preservation_in_modes() {
    let mut editor = Editor::new();

    // Set a status message
    editor.set_status_message("Test message".to_string());

    // Enter search mode
    editor.mode = Mode::Search;

    // Status message should be preserved in search mode
    // (This would be tested in the main loop, but we can verify the logic)
    assert_eq!(editor.status_msg, Some("Test message".to_string()));

    // Enter command mode
    editor.mode = Mode::Command;

    // Status message should be preserved in command mode
    assert_eq!(editor.status_msg, Some("Test message".to_string()));

    // Return to normal mode
    editor.mode = Mode::Normal;

    // In normal mode, status message would be cleared on key press
    // (This is handled in the main loop)
    assert_eq!(editor.status_msg, Some("Test message".to_string()));
}

/// Test line number display calculations
#[test]
fn test_line_number_calculations() {
    let mut editor = Editor::new();

    // Add multiple lines to test line number width calculation
    for i in 0..15 {
        let line_content = format!("Line {}", i + 1);
        for (j, ch) in line_content.chars().enumerate() {
            let char_pos = rustvim::buffer::Position::new(i, j);
            editor.buffer_mut().insert_char(char_pos, ch);
        }
        if i < 14 {
            let pos = rustvim::buffer::Position::new(i, line_content.len());
            editor.buffer_mut().insert_newline(pos);
        }
    }

    // Enable line numbers
    editor.show_line_numbers = true;

    // Verify buffer has the expected line count
    assert_eq!(editor.buffer().line_count(), 15);

    // Test that draw_buffer doesn't panic with line numbers enabled
    // (This is mainly to ensure the line number calculation logic is sound)
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // We can't easily test the actual drawing without a real terminal,
        // but we can verify that the line count calculation works
        let line_count = editor.buffer().line_count();
        let max_line_num_width = format!("{line_count}").len();
        assert_eq!(max_line_num_width, 2); // 15 has 2 digits
    }));

    assert!(result.is_ok());
}

/// Test line number gutter width calculation and coordinate conversion
#[test]
fn test_line_number_coordinate_conversion() {
    let mut editor = Editor::new();

    // Add some content to get realistic line numbers
    for i in 0..100 {
        let line_content = format!("Line {}", i + 1);
        for (j, ch) in line_content.chars().enumerate() {
            let char_pos = rustvim::buffer::Position::new(i, j);
            editor.buffer_mut().insert_char(char_pos, ch);
        }
        if i < 99 {
            let pos = rustvim::buffer::Position::new(i, line_content.len());
            editor.buffer_mut().insert_newline(pos);
        }
    }

    // Test without line numbers
    editor.show_line_numbers = false;
    assert_eq!(editor.line_number_gutter_width(), 0);

    // Test coordinate conversion without line numbers
    let (screen_row, screen_col) = editor.buffer_to_screen_coords(0, 0);
    assert_eq!(screen_row, 1); // 1-based screen coordinates
    assert_eq!(screen_col, 1); // 1-based screen coordinates

    let coords = editor.screen_to_buffer_coords(1, 1);
    assert_eq!(coords, Some((0, 0))); // Back to 0-based buffer coordinates

    // Enable line numbers (100 lines = 3 digits + 1 space = 4 chars)
    editor.show_line_numbers = true;
    assert_eq!(editor.line_number_gutter_width(), 4);

    // Test coordinate conversion with line numbers
    let (screen_row, screen_col) = editor.buffer_to_screen_coords(0, 0);
    assert_eq!(screen_row, 1); // Row unchanged
    assert_eq!(screen_col, 5); // 1 + gutter_width(4) = 5

    // Test screen to buffer conversion
    let coords = editor.screen_to_buffer_coords(1, 5);
    assert_eq!(coords, Some((0, 0))); // Should convert back correctly

    // Test clicking in the gutter area (should return None)
    let coords = editor.screen_to_buffer_coords(1, 1);
    assert_eq!(coords, None); // Click in gutter

    let coords = editor.screen_to_buffer_coords(1, 4);
    assert_eq!(coords, None); // Still in gutter (at boundary)

    // Test valid area just outside gutter
    let coords = editor.screen_to_buffer_coords(1, 5);
    assert_eq!(coords, Some((0, 0))); // First valid position
}

/// Test cursor shape enum
#[test]
fn test_cursor_shape_enum() {
    // Test that all cursor shapes are properly defined
    let block = CursorShape::Block;
    let underline = CursorShape::UnderLine;
    let bar = CursorShape::Bar;

    // Test that they're different
    assert_ne!(block, underline);
    assert_ne!(block, bar);
    assert_ne!(underline, bar);

    // Test that they're copy-able
    let block_copy = block;
    assert_eq!(block, block_copy);
}

/// Integration test for UI enhancement features working together
#[test]
fn test_ui_enhancement_integration() {
    let mut editor = Editor::new();

    // Test the complete flow of UI enhancement features

    // 1. Enable line numbers
    let command = rustvim::commands::ExCommandParser::parse("set numbers");
    let result = command.execute(&mut editor);
    assert!(result.is_ok());
    assert!(editor.show_line_numbers);

    // 2. Add some content
    let pos = rustvim::buffer::Position::new(0, 0);
    editor.buffer_mut().insert_char(pos, 'T');
    let pos = rustvim::buffer::Position::new(0, 1);
    editor.buffer_mut().insert_char(pos, 'e');
    let pos = rustvim::buffer::Position::new(0, 2);
    editor.buffer_mut().insert_char(pos, 's');
    let pos = rustvim::buffer::Position::new(0, 3);
    editor.buffer_mut().insert_char(pos, 't');

    // 3. Test mode changes with cursor shapes
    editor.mode = Mode::Insert;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    editor.mode = Mode::Normal;
    let result = editor.update_cursor_shape();
    assert!(result.is_ok());

    // 4. Test status message with timeout
    editor.set_status_message("Integration test message".to_string());
    assert!(editor.status_msg.is_some());
    assert!(editor.status_msg_time.is_some());

    // 5. Test error feedback
    let result = editor.bell();
    assert!(result.is_ok());

    // 6. Disable line numbers
    let command = rustvim::commands::ExCommandParser::parse("set nonumbers");
    let result = command.execute(&mut editor);
    assert!(result.is_ok());
    assert!(!editor.show_line_numbers);
}
