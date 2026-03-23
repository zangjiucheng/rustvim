use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::plugin::{EditorEvent, PluginRegistry};

#[test]
fn test_plugin_system_registration() {
    let mut registry = PluginRegistry::new();

    // Register all built-in plugins
    rustvim::plugins::register_all_plugins(&mut registry);

    // Test that commands are registered
    assert!(registry.has_ex_command("wc"));
    assert!(registry.has_ex_command("hello"));
    assert!(registry.has_ex_command("sort"));
    assert!(registry.has_ex_command("charfreq"));
    assert!(registry.has_ex_command("reverse"));
    assert!(registry.has_ex_command("uniq"));
    assert!(registry.has_ex_command("time"));
    assert!(registry.has_ex_command("status"));
    assert!(!registry.has_ex_command("nonexistent"));

    // Test command listing
    let commands = registry.list_ex_commands();
    assert!(commands.contains(&&"wc".to_string()));
    assert!(commands.contains(&&"hello".to_string()));
    assert!(commands.contains(&&"sort".to_string()));
}

#[test]
fn test_plugin_system_execution() {
    let mut editor = Editor::new();

    // Execute the hello command
    let result = editor.plugin_registry.get_ex_command("hello");
    assert!(result.is_some());

    if let Some(hello_fn) = result {
        let exec_result = hello_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that the status message was set
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert_eq!(msg, "Hello from plugin system!");
        }
    }
}

#[test]
fn test_plugin_system_word_count() {
    let mut editor = Editor::new();

    // Add some content to the buffer
    let pos = rustvim::buffer::Position::new(0, 0);
    editor.buffer_mut().insert_char(pos, 'H');
    let pos = rustvim::buffer::Position::new(0, 1);
    editor.buffer_mut().insert_char(pos, 'e');
    let pos = rustvim::buffer::Position::new(0, 2);
    editor.buffer_mut().insert_char(pos, 'l');
    let pos = rustvim::buffer::Position::new(0, 3);
    editor.buffer_mut().insert_char(pos, 'l');
    let pos = rustvim::buffer::Position::new(0, 4);
    editor.buffer_mut().insert_char(pos, 'o');
    let pos = rustvim::buffer::Position::new(0, 5);
    editor.buffer_mut().insert_char(pos, ' ');
    let pos = rustvim::buffer::Position::new(0, 6);
    editor.buffer_mut().insert_char(pos, 'w');
    let pos = rustvim::buffer::Position::new(0, 7);
    editor.buffer_mut().insert_char(pos, 'o');
    let pos = rustvim::buffer::Position::new(0, 8);
    editor.buffer_mut().insert_char(pos, 'r');
    let pos = rustvim::buffer::Position::new(0, 9);
    editor.buffer_mut().insert_char(pos, 'l');
    let pos = rustvim::buffer::Position::new(0, 10);
    editor.buffer_mut().insert_char(pos, 'd');

    // Execute the word count command
    if let Some(wc_fn) = editor.plugin_registry.get_ex_command("wc") {
        let exec_result = wc_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that the status message contains word count information
        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Words: 2"));
            assert!(status.contains("Lines: 1"));
            assert!(status.contains("Characters: 11"));
        }
    }
}

#[test]
fn test_plugin_system_integration_with_ex_commands() {
    use rustvim::commands::{Command, ExCommand};

    let mut editor = Editor::new();

    // Test that unknown commands check the plugin registry
    let unknown_hello = ExCommand::Unknown {
        command: "hello".to_string(),
    };
    let result = unknown_hello.execute(&mut editor);
    assert!(result.is_ok());

    // Check that the hello plugin was executed
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert_eq!(msg, "Hello from plugin system!");
    }

    // Clear the status message for the next test
    editor.status_msg = None;

    // Test a truly unknown command
    let unknown_cmd = ExCommand::Unknown {
        command: "nonexistent".to_string(),
    };
    let result = unknown_cmd.execute(&mut editor);
    assert!(result.is_ok());

    // Check that it shows "not an editor command" message
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("E492: Not an editor command"));
    }
}

#[test]
fn test_plugin_system_key_commands() {
    let mut registry = PluginRegistry::new();

    // Register a test key command
    fn test_key_command(editor: &mut Editor) -> Result<(), String> {
        editor.set_status_message("Key command executed!".to_string());
        Ok(())
    }

    registry.register_key_command(Mode::Normal, Key::Char('x'), test_key_command);

    // Test that the key command was registered
    let key_commands = registry.list_key_commands();
    assert_eq!(key_commands.len(), 1);
    assert!(key_commands.contains(&&(Mode::Normal, Key::Char('x'))));

    // Test key command execution
    let mut editor = Editor::new();
    let result = registry.handle_key_command(Mode::Normal, &Key::Char('x'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());

    // Check that the status message was set
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert_eq!(msg, "Key command executed!");
    }

    // Test non-existent key command
    let result = registry.handle_key_command(Mode::Normal, &Key::Char('z'), &mut editor);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_plugin_system_event_handlers() {
    let mut registry = PluginRegistry::new();

    // Register event handlers
    fn file_opened_handler(editor: &mut Editor) {
        editor.set_status_message("File opened by plugin!".to_string());
    }

    fn buffer_modified_handler(editor: &mut Editor) {
        editor.set_status_message("Buffer modified by plugin!".to_string());
    }

    registry.register_event_handler(
        EditorEvent::FileOpened("test.txt".to_string()),
        file_opened_handler,
    );
    registry.register_event_handler(EditorEvent::BufferModified, buffer_modified_handler);

    // Test firing events
    let mut editor = Editor::new();

    // Fire file opened event
    registry.fire_event(EditorEvent::FileOpened("test.txt".to_string()), &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert_eq!(msg, "File opened by plugin!");
    }

    // Clear status message
    editor.status_msg = None;

    // Fire buffer modified event
    registry.fire_event(EditorEvent::BufferModified, &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert_eq!(msg, "Buffer modified by plugin!");
    }

    // Test firing non-registered event (should not crash)
    editor.status_msg = None;
    registry.fire_event(
        EditorEvent::SearchPerformed("test".to_string()),
        &mut editor,
    );
    assert!(editor.status_msg.is_none());
}

#[test]
fn test_plugin_system_text_analysis_commands() {
    let mut editor = Editor::new();

    // Add some content to test with
    let content = "hello world\nthis is a test\nhello again";
    for (line_idx, line) in content.lines().enumerate() {
        if line_idx > 0 {
            let pos = rustvim::buffer::Position::new(
                line_idx - 1,
                editor.buffer().get_line(line_idx - 1).unwrap().len(),
            );
            editor.buffer_mut().insert_newline(pos);
        }
        for (col_idx, ch) in line.chars().enumerate() {
            let pos = rustvim::buffer::Position::new(line_idx, col_idx);
            editor.buffer_mut().insert_char(pos, ch);
        }
    }

    // Test character frequency command
    if let Some(charfreq_fn) = editor.plugin_registry.get_ex_command("charfreq") {
        let exec_result = charfreq_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that character frequency was calculated
        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Total chars") || status.contains("Unique chars"));
        }
    }

    // Test status command
    editor.status_msg = None;
    if let Some(status_fn) = editor.plugin_registry.get_ex_command("status") {
        let exec_result = status_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that status information was displayed
        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Line"));
            assert!(status.contains("Col"));
            assert!(status.contains("chars"));
        }
    }
}

#[test]
fn test_plugin_system_text_manipulation_commands() {
    let mut editor = Editor::new();

    // Add some content to test with
    let content = "zebra\napple\nbanana\napple\ncherry";
    for (line_idx, line) in content.lines().enumerate() {
        if line_idx > 0 {
            let pos = rustvim::buffer::Position::new(
                line_idx - 1,
                editor.buffer().get_line(line_idx - 1).unwrap().len(),
            );
            editor.buffer_mut().insert_newline(pos);
        }
        for (col_idx, ch) in line.chars().enumerate() {
            let pos = rustvim::buffer::Position::new(line_idx, col_idx);
            editor.buffer_mut().insert_char(pos, ch);
        }
    }

    // Test sort command
    if let Some(sort_fn) = editor.plugin_registry.get_ex_command("sort") {
        let exec_result = sort_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that sort operation completed successfully
        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Sorted"));
        }
    }

    // Test reverse command with a new editor
    let mut editor2 = Editor::new();
    let content = "line1\nline2\nline3";
    for (line_idx, line) in content.lines().enumerate() {
        if line_idx > 0 {
            let pos = rustvim::buffer::Position::new(
                line_idx - 1,
                editor.buffer().get_line(line_idx - 1).unwrap().len(),
            );
            editor.buffer_mut().insert_newline(pos);
        }
        for (col_idx, ch) in line.chars().enumerate() {
            let pos = rustvim::buffer::Position::new(line_idx, col_idx);
            editor2.buffer_mut().insert_char(pos, ch);
        }
    }

    if let Some(reverse_fn) = editor2.plugin_registry.get_ex_command("reverse") {
        let exec_result = reverse_fn(&mut editor2);
        assert!(exec_result.is_ok());

        // Check that reverse operation completed successfully
        assert!(editor2.status_msg.is_some());
        if let Some(status) = &editor2.status_msg {
            assert!(status.contains("Reversed"));
        }
    }

    // Test unique command with a new editor
    let mut editor3 = Editor::new();
    let content = "apple\nbanana\napple\ncherry\nbanana";
    for (line_idx, line) in content.lines().enumerate() {
        if line_idx > 0 {
            let pos = rustvim::buffer::Position::new(
                line_idx - 1,
                editor3.buffer().get_line(line_idx - 1).unwrap().len(),
            );
            editor3.buffer_mut().insert_newline(pos);
        }
        for (col_idx, ch) in line.chars().enumerate() {
            let pos = rustvim::buffer::Position::new(line_idx, col_idx);
            editor3.buffer_mut().insert_char(pos, ch);
        }
    }

    if let Some(uniq_fn) = editor3.plugin_registry.get_ex_command("uniq") {
        let exec_result = uniq_fn(&mut editor3);
        assert!(exec_result.is_ok());

        // Check that unique operation completed successfully
        assert!(editor3.status_msg.is_some());
        if let Some(status) = &editor3.status_msg {
            assert!(status.contains("Removed") || status.contains("lines"));
        }
    }
}

#[test]
fn test_plugin_system_utility_commands() {
    let mut editor = Editor::new();

    // Test time command
    if let Some(time_fn) = editor.plugin_registry.get_ex_command("time") {
        let exec_result = time_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check that time was displayed
        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Current time"));
            assert!(status.contains("UTC"));
        }
    }
}

#[test]
fn test_plugin_system_error_handling() {
    let mut registry = PluginRegistry::new();

    // Register a command that returns an error
    fn error_command(_editor: &mut Editor) -> Result<(), String> {
        Err("Test error".to_string())
    }

    registry.register_ex_command("error".to_string(), error_command);

    let mut editor = Editor::new();

    // Test error handling in handle_ex_command
    let result = registry.handle_ex_command("error", &mut editor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Test error");

    // Test error handling in key commands
    registry.register_key_command(Mode::Normal, Key::Char('e'), error_command);
    let result = registry.handle_key_command(Mode::Normal, &Key::Char('e'), &mut editor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Test error");
}

#[test]
fn test_plugin_registry_default() {
    let registry = PluginRegistry::default();

    // Test that default registry is empty
    assert_eq!(registry.list_ex_commands().len(), 0);
    assert_eq!(registry.list_key_commands().len(), 0);
    assert!(!registry.has_ex_command("any"));
}

#[test]
fn test_editor_event_variants() {
    // Test that all EditorEvent variants can be created and used
    let events = vec![
        EditorEvent::FileOpened("test.txt".to_string()),
        EditorEvent::FileSaved("test.txt".to_string()),
        EditorEvent::ModeChanged {
            from: Mode::Normal,
            to: Mode::Insert,
        },
        EditorEvent::BufferModified,
        EditorEvent::SearchPerformed("test".to_string()),
        EditorEvent::CommandExecuted("save".to_string()),
    ];

    let mut registry = PluginRegistry::new();

    // Register handlers for all events
    fn generic_handler(editor: &mut Editor) {
        editor.set_status_message("Event handled!".to_string());
    }

    for event in &events {
        registry.register_event_handler(event.clone(), generic_handler);
    }

    let mut editor = Editor::new();

    // Fire each event and verify it works
    for event in events {
        editor.status_msg = None;
        registry.fire_event(event, &mut editor);
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert_eq!(msg, "Event handled!");
        }
    }
}

#[test]
fn test_plugin_system_key_command_examples() {
    let mut registry = PluginRegistry::new();

    // Register the example key commands
    rustvim::plugins::examples::register_example_plugins(&mut registry);

    // Test that key commands were registered
    let key_commands = registry.list_key_commands();
    assert!(key_commands.len() >= 4); // Should have at least our 4 key commands

    // Test quick save command (Ctrl+S in Normal mode)
    let mut editor = Editor::new();

    // Create a temporary file for testing
    let temp_file = std::env::temp_dir().join("rustvim_test_save.txt");
    editor.set_filename(Some(temp_file.to_string_lossy().to_string()));

    let result = registry.handle_key_command(Mode::Normal, &Key::Ctrl('s'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.is_some());

    // Clean up the temporary file
    let _ = std::fs::remove_file(&temp_file);

    // Test toggle line numbers (F2 in Normal mode)
    let initial_line_numbers = editor.show_line_numbers;
    let result = registry.handle_key_command(Mode::Normal, &Key::Function(2), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_ne!(editor.show_line_numbers, initial_line_numbers);

    // Test insert timestamp (F3 in Insert mode)
    editor.status_msg = None;
    let result = registry.handle_key_command(Mode::Insert, &Key::Function(3), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("Timestamp inserted"));
    }

    // Test delete word backward (Ctrl+W in Insert mode)
    editor.status_msg = None;
    let result = registry.handle_key_command(Mode::Insert, &Key::Ctrl('w'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.is_some());
}

#[test]
fn test_plugin_system_event_handler_examples() {
    let mut registry = PluginRegistry::new();

    // Register the example event handlers
    rustvim::plugins::examples::register_example_plugins(&mut registry);

    let mut editor = Editor::new();

    // Test SearchPerformed event - should set status message
    editor.status_msg = None;
    registry.fire_event(EditorEvent::SearchPerformed("".to_string()), &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("Search tracked"));
    }

    // Test CommandExecuted event
    editor.status_msg = None;
    registry.fire_event(EditorEvent::CommandExecuted("".to_string()), &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("Command execution tracked"));
    }

    // Test ModeChanged event
    editor.status_msg = None;
    registry.fire_event(
        EditorEvent::ModeChanged {
            from: Mode::Normal,
            to: Mode::Insert,
        },
        &mut editor,
    );
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("Mode changed"));
    }

    // Test FileOpened event - file_operation_logger should set message
    editor.status_msg = None;
    registry.fire_event(EditorEvent::FileOpened("".to_string()), &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("File operation logged"));
    }

    // Test FileSaved event
    editor.status_msg = None;
    registry.fire_event(EditorEvent::FileSaved("".to_string()), &mut editor);
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("File operation logged"));
    }
}

#[test]
fn test_plugin_system_meta_commands() {
    let mut editor = Editor::new();

    // Test bindkeys command
    if let Some(bindkeys_fn) = editor.plugin_registry.get_ex_command("bindkeys") {
        let exec_result = bindkeys_fn(&mut editor);
        assert!(exec_result.is_ok());

        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Key binding examples"));
        }
    }

    // Test events command
    editor.status_msg = None;
    if let Some(events_fn) = editor.plugin_registry.get_ex_command("events") {
        let exec_result = events_fn(&mut editor);
        assert!(exec_result.is_ok());

        assert!(editor.status_msg.is_some());
        if let Some(status) = &editor.status_msg {
            assert!(status.contains("Event handlers enabled"));
        }
    }
}

#[test]
fn test_plugin_system_key_command_modes() {
    let mut registry = PluginRegistry::new();

    // Register a test command for different modes
    fn test_normal_command(editor: &mut Editor) -> Result<(), String> {
        editor.set_status_message("Normal mode command".to_string());
        Ok(())
    }

    fn test_insert_command(editor: &mut Editor) -> Result<(), String> {
        editor.set_status_message("Insert mode command".to_string());
        Ok(())
    }

    fn test_visual_command(editor: &mut Editor) -> Result<(), String> {
        editor.set_status_message("Visual mode command".to_string());
        Ok(())
    }

    // Register same key for different modes
    registry.register_key_command(Mode::Normal, Key::Char('t'), test_normal_command);
    registry.register_key_command(Mode::Insert, Key::Char('t'), test_insert_command);
    registry.register_key_command(Mode::Visual, Key::Char('t'), test_visual_command);

    let mut editor = Editor::new();

    // Test Normal mode
    let result = registry.handle_key_command(Mode::Normal, &Key::Char('t'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.as_ref().unwrap().contains("Normal mode"));

    // Test Insert mode
    editor.status_msg = None;
    let result = registry.handle_key_command(Mode::Insert, &Key::Char('t'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.as_ref().unwrap().contains("Insert mode"));

    // Test Visual mode
    editor.status_msg = None;
    let result = registry.handle_key_command(Mode::Visual, &Key::Char('t'), &mut editor);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert!(editor.status_msg.as_ref().unwrap().contains("Visual mode"));

    // Test that wrong mode doesn't trigger
    editor.status_msg = None;
    let result = registry.handle_key_command(Mode::Command, &Key::Char('t'), &mut editor);
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Should return false for unhandled
    assert!(editor.status_msg.is_none());
}

#[test]
fn test_plugin_system_multiple_event_handlers() {
    let mut registry = PluginRegistry::new();

    // Register multiple handlers for the same event
    fn handler1(editor: &mut Editor) {
        editor.set_status_message("Handler 1 executed".to_string());
    }

    fn handler2(editor: &mut Editor) {
        editor.set_status_message("Handler 2 executed".to_string());
    }

    fn handler3(editor: &mut Editor) {
        editor.set_status_message("Handler 3 executed".to_string());
    }

    // Register all handlers for the same event
    let event = EditorEvent::BufferModified;
    registry.register_event_handler(event.clone(), handler1);
    registry.register_event_handler(event.clone(), handler2);
    registry.register_event_handler(event.clone(), handler3);

    let mut editor = Editor::new();

    // Fire the event - all handlers should execute
    registry.fire_event(event, &mut editor);

    // The last handler sets the final message
    assert!(editor.status_msg.is_some());
    if let Some(msg) = &editor.status_msg {
        assert!(msg.contains("Handler 3")); // Last handler wins
    }
}

#[test]
fn test_sort_lines_command() {
    let mut editor = Editor::new();

    // Add unsorted content (one line each)
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(0, 0), 'z');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(0, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(1, 0), 'a');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(1, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(2, 0), 'm');

    // Execute the sort command
    if let Some(sort_fn) = editor.plugin_registry.get_ex_command("sort") {
        let exec_result = sort_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check sorted content
        assert_eq!(editor.buffer().get_line(0).unwrap(), "a");
        assert_eq!(editor.buffer().get_line(1).unwrap(), "m");
        assert_eq!(editor.buffer().get_line(2).unwrap(), "z");

        // Check status message
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(msg.contains("Sorted"));
        }
    }
}

#[test]
fn test_sort_empty_buffer() {
    // Note: Buffer starts with 1 empty line by default, so we can't test the "no lines" case
    // This test verifies the sort command works with single-line buffer
    let mut editor = Editor::new();

    // The buffer has 1 line by default
    assert_eq!(editor.buffer().line_count(), 1);

    // Execute sort on single-line buffer (works but reports 1 line)
    let sort_fn = editor.plugin_registry.get_ex_command("sort");
    assert!(sort_fn.is_some());

    if let Some(sort_fn) = sort_fn {
        let exec_result = sort_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Should still have 1 line
        assert_eq!(editor.buffer().line_count(), 1);

        // Status message should say "Sorted 1 lines"
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(msg.contains("Sorted"));
        }
    }
}

#[test]
fn test_reverse_lines_command() {
    let mut editor = Editor::new();

    // Add content
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(0, 0), 'a');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(0, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(1, 0), 'b');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(1, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(2, 0), 'c');

    // Execute the reverse command
    if let Some(reverse_fn) = editor.plugin_registry.get_ex_command("reverse") {
        let exec_result = reverse_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check reversed content
        assert_eq!(editor.buffer().get_line(0).unwrap(), "c");
        assert_eq!(editor.buffer().get_line(1).unwrap(), "b");
        assert_eq!(editor.buffer().get_line(2).unwrap(), "a");

        // Check status message
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(msg.contains("Reversed"));
        }
    }
}

#[test]
fn test_reverse_empty_buffer() {
    let mut editor = Editor::new();

    // Buffer starts with 1 line by default
    assert_eq!(editor.buffer().line_count(), 1);

    let reverse_fn = editor.plugin_registry.get_ex_command("reverse");
    assert!(reverse_fn.is_some());

    if let Some(reverse_fn) = reverse_fn {
        let exec_result = reverse_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Status message
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(msg.contains("Reversed"));
        }
    }
}

#[test]
fn test_uniq_lines_command() {
    let mut editor = Editor::new();

    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(0, 0), 'a');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(0, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(1, 0), 'b');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(1, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(2, 0), 'a');
    editor
        .buffer_mut()
        .insert_newline(rustvim::buffer::Position::new(2, 1));
    editor
        .buffer_mut()
        .insert_char(rustvim::buffer::Position::new(3, 0), 'c');

    // Execute the uniq command
    if let Some(uniq_fn) = editor.plugin_registry.get_ex_command("uniq") {
        let exec_result = uniq_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Check unique content
        assert_eq!(editor.buffer().get_line(0).unwrap(), "a");
        assert_eq!(editor.buffer().get_line(1).unwrap(), "b");
        assert_eq!(editor.buffer().get_line(2).unwrap(), "c");
        assert!(editor.buffer().get_line(3).is_none());

        // Check status message
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(msg.contains("Removed"));
            assert!(msg.contains("unique"));
        }
    }
}

#[test]
fn test_uniq_empty_buffer() {
    let mut editor = Editor::new();

    // Buffer starts with 1 line by default
    assert_eq!(editor.buffer().line_count(), 1);

    let uniq_fn = editor.plugin_registry.get_ex_command("uniq");
    assert!(uniq_fn.is_some());

    if let Some(uniq_fn) = uniq_fn {
        let exec_result = uniq_fn(&mut editor);
        assert!(exec_result.is_ok());

        // Status message for single line (not truly "empty")
        assert!(editor.status_msg.is_some());
        if let Some(msg) = &editor.status_msg {
            assert!(
                msg.contains("unique"),
                "message should contain 'unique': {}",
                msg
            );
        }
    }
}
