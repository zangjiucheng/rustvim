use rustvim::editor::{Editor, Mode};
use rustvim::input::Key;
use rustvim::keymap::{Action, KeymapConfigBuilder, Keymap};
use rustvim::commands::{MovementCommand, EditCommand};

#[cfg(test)]
mod global_keymap_config_tests {
    use super::*;

    /// Test the main feature you requested: global keymap configuration
    #[test]
    fn test_global_keymap_configuration() {
        let mut editor = Editor::new();
        
        // Test 1: Create custom configuration using builder pattern
        let custom_config = KeymapConfigBuilder::with_defaults()
            .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand)
            .bind(Mode::Normal, Key::Char('s'), Action::Edit(EditCommand::DeleteChar))
            .build();
        
        // Test 2: Apply configuration globally to editor
        editor.configure_keymap(custom_config);
        
        // Test 3: Add individual key bindings at runtime
        editor.bind_key(Mode::Normal, Key::Char('x'), Action::Edit(EditCommand::DeleteChar));
        
        // Test 4: Get current configuration (for saving to file)
        let current_config = editor.get_keymap_config();
        
        // Verify the bindings are present
        assert!(current_config.normal.contains_key(&Key::Char('Q')));
        assert!(current_config.normal.contains_key(&Key::Char('s')));
        assert!(current_config.normal.contains_key(&Key::Char('x')));
        
        // Test 5: Reset to defaults
        editor.reset_keymap_to_defaults();
        let default_config = editor.get_keymap_config();
        
        // Should have default h,j,k,l movement but not our custom Q binding
        assert!(default_config.normal.contains_key(&Key::Char('h')));
        assert!(!default_config.normal.contains_key(&Key::Char('Q')));
    }

    /// Test that default keymaps are accessible globally as you requested
    #[test]
    fn test_global_access_to_default_keymaps() {
        // These are now public functions as you requested
        let normal_keymap = Keymap::default_normal_keymap();
        let insert_keymap = Keymap::default_insert_keymap();
        
        // Verify they have expected content
        assert!(normal_keymap.contains_key(&Key::Char('h')));
        assert!(normal_keymap.contains_key(&Key::Char('j')));
        assert!(normal_keymap.contains_key(&Key::Char('k')));
        assert!(normal_keymap.contains_key(&Key::Char('l')));
        
        // Insert mode should have arrow keys
        assert!(insert_keymap.contains_key(&Key::Left));
        assert!(insert_keymap.contains_key(&Key::Right));
    }

    /// Test builder pattern with various configuration methods
    #[test]
    fn test_configuration_builder_patterns() {
        // Pattern 1: Start with defaults and override
        let config1 = KeymapConfigBuilder::with_defaults()
            .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand)
            .build();
        
        assert!(config1.normal.contains_key(&Key::Char('h'))); // Default
        assert!(config1.normal.contains_key(&Key::Char('Q'))); // Custom
        
        // Pattern 2: Start empty and add specific mode defaults
        let config2 = KeymapConfigBuilder::new()
            .with_mode_defaults(Mode::Normal)
            .bind(Mode::Normal, Key::Char('Z'), Action::Undo)
            .build();
        
        assert!(config2.normal.contains_key(&Key::Char('h'))); // From defaults
        assert!(config2.normal.contains_key(&Key::Char('Z'))); // Custom
        assert!(config2.insert.is_empty()); // Not included in mode defaults
        
        // Pattern 3: Bulk binding multiple keys
        let bindings = vec![
            (Key::Char('1'), Action::Move(MovementCommand::LineStart)),
            (Key::Char('2'), Action::Move(MovementCommand::LineEnd)),
            (Key::Char('3'), Action::Edit(EditCommand::DeleteChar)),
        ];
        
        let config3 = KeymapConfigBuilder::new()
            .bind_multiple(Mode::Normal, bindings)
            .build();
        
        assert_eq!(config3.normal.len(), 3);
        assert!(config3.normal.contains_key(&Key::Char('1')));
        assert!(config3.normal.contains_key(&Key::Char('2')));
        assert!(config3.normal.contains_key(&Key::Char('3')));
    }

    /// Test runtime keymap updates as you requested
    #[test]
    fn test_runtime_keymap_updates() {
        let mut editor = Editor::new();
        
        // Start with defaults
        let initial_config = editor.get_keymap_config();
        let initial_count = initial_config.normal.len();
        
        // Add a custom binding at runtime
        editor.bind_key(Mode::Normal, Key::Char('@'), Action::StartSearch);
        
        // Verify it was added
        let updated_config = editor.get_keymap_config();
        assert_eq!(updated_config.normal.len(), initial_count + 1);
        assert!(updated_config.normal.contains_key(&Key::Char('@')));
        
        // Create and apply a completely new configuration
        let new_config = KeymapConfigBuilder::new()
            .bind(Mode::Normal, Key::Char('!'), Action::StartCommand)
            .build();
        
        editor.configure_keymap(new_config);
        
        // Verify the old bindings are gone and new one is present
        let final_config = editor.get_keymap_config();
        assert!(!final_config.normal.contains_key(&Key::Char('h'))); // Default gone
        assert!(!final_config.normal.contains_key(&Key::Char('@'))); // Runtime binding gone
        assert!(final_config.normal.contains_key(&Key::Char('!'))); // New config present
    }

    /// Test configuration export/import for saving to files
    #[test]
    fn test_configuration_persistence() {
        let mut editor = Editor::new();
        
        // Create custom configuration
        editor.bind_key(Mode::Normal, Key::Char('$'), Action::StartCommand);
        editor.bind_key(Mode::Insert, Key::Ctrl('x'), Action::Edit(EditCommand::DeleteChar));
        
        // Export configuration
        let exported_config = editor.get_keymap_config();
        
        // Create new editor and import the configuration
        let mut new_editor = Editor::new();
        new_editor.configure_keymap(exported_config);
        
        // Verify the configuration was applied
        let imported_config = new_editor.get_keymap_config();
        assert!(imported_config.normal.contains_key(&Key::Char('$')));
        assert!(imported_config.insert.contains_key(&Key::Ctrl('x')));
    }

    /// Test that default keymaps work correctly and are not empty
    #[test]
    fn test_default_keymap_functionality() {
        // Test that default_normal_keymap and default_insert_keymap are used
        let processor = rustvim::keymap::KeymapProcessor::new();
        let keymap = processor.keymap();
        
        // Normal mode should have comprehensive bindings
        assert!(keymap.mode_binding_count(Mode::Normal) > 10);
        
        // Essential Vim bindings should be present
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('h'))); // Left
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('j'))); // Down  
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('k'))); // Up
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('l'))); // Right
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('i'))); // Insert mode
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('x'))); // Delete char
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('u'))); // Undo
        assert!(keymap.is_bound(Mode::Normal, &Key::Char(':'))); // Command mode
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('/'))); // Search
        
        // Word movement
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('w'))); // Word forward
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('b'))); // Word backward
        
        // Line navigation
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('0'))); // Line start
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('$'))); // Line end
        
        // File navigation
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('G'))); // File end
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('g'))); // File start (gg)
    }

    /// Test mode isolation in global configuration
    #[test]
    fn test_mode_isolation_in_global_config() {
        let mut editor = Editor::new();
        
        // Bind same key to different actions in different modes
        editor.bind_key(Mode::Normal, Key::Char('d'), Action::Edit(EditCommand::DeleteChar));
        editor.bind_key(Mode::Insert, Key::Char('d'), Action::Move(MovementCommand::Down));
        editor.bind_key(Mode::Visual, Key::Char('d'), Action::Edit(EditCommand::DeleteSelection));
        
        let config = editor.get_keymap_config();
        
        // Verify each mode has the correct binding
        if let Some(Action::Edit(EditCommand::DeleteChar)) = config.normal.get(&Key::Char('d')) {
            // Expected for normal mode
        } else {
            panic!("Normal mode 'd' should be DeleteChar");
        }
        
        if let Some(Action::Move(MovementCommand::Down)) = config.insert.get(&Key::Char('d')) {
            // Expected for insert mode  
        } else {
            panic!("Insert mode 'd' should be Move(Down)");
        }
        
        if let Some(Action::Edit(EditCommand::DeleteSelection)) = config.visual.get(&Key::Char('d')) {
            // Expected for visual mode
        } else {
            panic!("Visual mode 'd' should be DeleteSelection");
        }
    }
}
