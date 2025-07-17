use rustvim::editor::Editor;
use rustvim::plugin::PluginRegistry;

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
    assert_eq!(commands.len(), 8);
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
