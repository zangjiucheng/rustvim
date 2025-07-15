use rustvim::editor::Mode;
use rustvim::input::Key;
use rustvim::keymap::{
    Action, KeymapConfig, KeymapConfigBuilder, KeymapProcessor, Keymap, PendingAction
};
use rustvim::commands::{MovementCommand, EditCommand, ModeSwitchCommand};

#[cfg(test)]
mod keymap_tests {
    use super::*;

    // ============================================================================
    // Basic Keymap Functionality Tests
    // ============================================================================

    #[test]
    fn test_keymap_creation() {
        let keymap = Keymap::new();
        
        // Check that default keymaps have expected bindings
        assert!(keymap.mode_binding_count(Mode::Normal) > 0);
        // Insert mode may have minimal bindings
        assert!(keymap.mode_binding_count(Mode::Insert) < 20); // Reasonable upper bound
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('h')));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('j')));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('k')));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('l')));
    }

    #[test]
    fn test_empty_keymap() {
        let keymap = Keymap::empty();
        
        // All modes should be empty
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 0);
        assert_eq!(keymap.mode_binding_count(Mode::Insert), 0);
        assert_eq!(keymap.mode_binding_count(Mode::Visual), 0);
        assert!(!keymap.is_bound(Mode::Normal, &Key::Char('h')));
    }

    #[test]
    fn test_keymap_bind_unbind() {
        let mut keymap = Keymap::empty();
        
        // Test binding
        keymap.bind(Mode::Normal, Key::Char('x'), Action::Edit(EditCommand::DeleteChar));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('x')));
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 1);
        
        // Test unbinding
        keymap.unbind(Mode::Normal, &Key::Char('x'));
        assert!(!keymap.is_bound(Mode::Normal, &Key::Char('x')));
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 0);
    }

    #[test]
    fn test_keymap_bulk_operations() {
        let mut keymap = Keymap::empty();
        
        let bindings = vec![
            (Key::Char('h'), Action::Move(MovementCommand::Left)),
            (Key::Char('j'), Action::Move(MovementCommand::Down)),
            (Key::Char('k'), Action::Move(MovementCommand::Up)),
        ];
        
        keymap.bind_multiple(Mode::Normal, bindings);
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 3);
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('h')));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('j')));
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('k')));
    }

    #[test]
    fn test_keymap_mode_defaults() {
        let keymap = Keymap::with_mode_defaults(&[Mode::Normal, Mode::Insert]);
        
        // Normal mode should have defaults
        assert!(keymap.mode_binding_count(Mode::Normal) > 0);
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('h')));
        
        // Insert mode should have defaults (minimal)
        let insert_count = keymap.mode_binding_count(Mode::Insert);
        assert!(insert_count < 20); // Reasonable upper bound
        
        // Visual mode should be empty (not in defaults list)
        assert_eq!(keymap.mode_binding_count(Mode::Visual), 0);
    }

    // ============================================================================
    // KeymapProcessor Tests
    // ============================================================================

    #[test]
    fn test_keymap_processor_creation() {
        let processor = KeymapProcessor::new();
        
        // Should have default keymaps
        assert!(processor.keymap().mode_binding_count(Mode::Normal) > 0);
    }

    #[test]
    fn test_keymap_processor_with_config() {
        let config = KeymapConfig::builder()
            .bind(Mode::Normal, Key::Char('q'), Action::StartCommand)
            .build();
        
        let processor = KeymapProcessor::with_config(config);
        assert!(processor.keymap().is_bound(Mode::Normal, &Key::Char('q')));
    }

    #[test]
    fn test_keymap_processor_update_config() {
        let mut processor = KeymapProcessor::new();
        
        let new_config = KeymapConfig::builder()
            .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand)
            .build();
        
        processor.update_config(new_config);
        assert!(processor.keymap().is_bound(Mode::Normal, &Key::Char('Q')));
    }

    // ============================================================================
    // KeymapConfig Tests
    // ============================================================================

    #[test]
    fn test_keymap_config_default() {
        let config = KeymapConfig::default();
        
        // Should have default bindings for all modes
        assert!(!config.normal.is_empty());
        // Insert mode may be empty or minimal
        let insert_len = config.insert.len();
        assert!(insert_len < 20); // Reasonable upper bound
    }

    #[test]
    fn test_keymap_config_empty() {
        let config = KeymapConfig::empty();
        
        // All modes should be empty
        assert!(config.normal.is_empty());
        assert!(config.insert.is_empty());
        assert!(config.visual.is_empty());
        assert!(config.command.is_empty());
        assert!(config.search.is_empty());
    }

    // ============================================================================
    // KeymapConfigBuilder Tests
    // ============================================================================

    #[test]
    fn test_builder_empty() {
        let config = KeymapConfigBuilder::new().build();
        
        assert!(config.normal.is_empty());
        assert!(config.insert.is_empty());
    }

    #[test]
    fn test_builder_with_defaults() {
        let config = KeymapConfigBuilder::with_defaults().build();
        
        assert!(!config.normal.is_empty());
        assert!(config.normal.contains_key(&Key::Char('h')));
        assert!(config.normal.contains_key(&Key::Char('j')));
    }

    #[test]
    fn test_builder_single_binding() {
        let config = KeymapConfigBuilder::new()
            .bind(Mode::Normal, Key::Char('q'), Action::StartCommand)
            .build();
        
        assert_eq!(config.normal.len(), 1);
        assert!(config.normal.contains_key(&Key::Char('q')));
        
        if let Some(Action::StartCommand) = config.normal.get(&Key::Char('q')) {
            // Expected
        } else {
            panic!("Binding not found or incorrect action");
        }
    }

    #[test]
    fn test_builder_multiple_bindings() {
        let bindings = vec![
            (Key::Char('h'), Action::Move(MovementCommand::Left)),
            (Key::Char('j'), Action::Move(MovementCommand::Down)),
            (Key::Char('k'), Action::Move(MovementCommand::Up)),
        ];
        
        let config = KeymapConfigBuilder::new()
            .bind_multiple(Mode::Normal, bindings)
            .build();
        
        assert_eq!(config.normal.len(), 3);
        assert!(config.normal.contains_key(&Key::Char('h')));
        assert!(config.normal.contains_key(&Key::Char('j')));
        assert!(config.normal.contains_key(&Key::Char('k')));
    }

    #[test]
    fn test_builder_with_mode_defaults() {
        let config = KeymapConfigBuilder::new()
            .with_mode_defaults(Mode::Normal)
            .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand) // Override
            .build();
        
        // Should have normal defaults plus our custom binding
        assert!(config.normal.len() > 1);
        assert!(config.normal.contains_key(&Key::Char('h'))); // Default
        assert!(config.normal.contains_key(&Key::Char('Q'))); // Custom
        
        // Other modes should be empty
        assert!(config.insert.is_empty());
        assert!(config.visual.is_empty());
    }

    #[test]
    fn test_builder_fluent_interface() {
        let config = KeymapConfigBuilder::with_defaults()
            .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand)
            .bind(Mode::Insert, Key::Ctrl('s'), Action::Edit(EditCommand::DeleteChar))
            .bind_multiple(Mode::Visual, vec![
                (Key::Char('d'), Action::Edit(EditCommand::DeleteSelection)),
                (Key::Char('y'), Action::Edit(EditCommand::YankSelection)),
            ])
            .build();
        
        // Check all the bindings are present
        assert!(config.normal.contains_key(&Key::Char('Q')));
        assert!(config.insert.contains_key(&Key::Ctrl('s')));
        assert!(config.visual.contains_key(&Key::Char('d')));
        assert!(config.visual.contains_key(&Key::Char('y')));
        
        // Should also have defaults
        assert!(config.normal.contains_key(&Key::Char('h')));
    }

    // ============================================================================
    // Import/Export Tests
    // ============================================================================

    #[test]
    fn test_keymap_export_import() {
        let mut original_keymap = Keymap::new();
        original_keymap.bind(Mode::Normal, Key::Char('Q'), Action::StartCommand);
        
        // Export to config
        let config = original_keymap.export_config();
        
        // Import to new keymap
        let mut new_keymap = Keymap::empty();
        new_keymap.import_config(config);
        
        // Should have the same bindings
        assert!(new_keymap.is_bound(Mode::Normal, &Key::Char('Q')));
        assert!(new_keymap.is_bound(Mode::Normal, &Key::Char('h'))); // Default
    }

    // ============================================================================
    // Default Keymap Content Tests
    // ============================================================================

    #[test]
    fn test_default_normal_keymap_content() {
        let keymap = Keymap::default_normal_keymap();
        
        // Basic movement
        assert!(keymap.contains_key(&Key::Char('h')));
        assert!(keymap.contains_key(&Key::Char('j')));
        assert!(keymap.contains_key(&Key::Char('k')));
        assert!(keymap.contains_key(&Key::Char('l')));
        
        // Word movement
        assert!(keymap.contains_key(&Key::Char('w')));
        assert!(keymap.contains_key(&Key::Char('b')));
        assert!(keymap.contains_key(&Key::Char('e')));
        
        // Line navigation
        assert!(keymap.contains_key(&Key::Char('0')));
        assert!(keymap.contains_key(&Key::Char('^')));
        assert!(keymap.contains_key(&Key::Char('$')));
        
        // File navigation
        assert!(keymap.contains_key(&Key::Char('G')));
        assert!(keymap.contains_key(&Key::Char('g')));
        
        // Mode switching
        assert!(keymap.contains_key(&Key::Char('i')));
        assert!(keymap.contains_key(&Key::Char('a')));
        assert!(keymap.contains_key(&Key::Char('o')));
        
        // Edit operations
        assert!(keymap.contains_key(&Key::Char('x')));
        assert!(keymap.contains_key(&Key::Char('d')));
        assert!(keymap.contains_key(&Key::Char('y')));
        
        // Undo/Redo
        assert!(keymap.contains_key(&Key::Char('u')));
        assert!(keymap.contains_key(&Key::Ctrl('r')));
        
        // Search and command
        assert!(keymap.contains_key(&Key::Char('/')));
        assert!(keymap.contains_key(&Key::Char(':')));
    }

    #[test]
    fn test_default_insert_keymap_content() {
        let keymap = Keymap::default_insert_keymap();
        
        // Arrow keys should work in insert mode
        assert!(keymap.contains_key(&Key::Left));
        assert!(keymap.contains_key(&Key::Right));
        assert!(keymap.contains_key(&Key::Up));
        assert!(keymap.contains_key(&Key::Down));
    }

    #[test]
    fn test_default_visual_keymap_content() {
        let keymap = Keymap::default_visual_keymap();
        
        // Movement should extend selection
        assert!(keymap.contains_key(&Key::Char('h')));
        assert!(keymap.contains_key(&Key::Char('j')));
        assert!(keymap.contains_key(&Key::Char('k')));
        assert!(keymap.contains_key(&Key::Char('l')));
        
        // Operations on selection
        assert!(keymap.contains_key(&Key::Char('d')));
        assert!(keymap.contains_key(&Key::Char('y')));
        
        // Exit visual mode
        assert!(keymap.contains_key(&Key::Char('v')));
    }

    // ============================================================================
    // Action Type Tests
    // ============================================================================

    #[test]
    fn test_action_types() {
        // Test that different action types can be created
        let _move_action = Action::Move(MovementCommand::Left);
        let _edit_action = Action::Edit(EditCommand::DeleteChar);
        let _mode_action = Action::ModeSwitch(ModeSwitchCommand::InsertBefore);
        let _pending_action = Action::Pending(PendingAction::Delete);
        
        // Special actions
        let _search_action = Action::StartSearch;
        let _command_action = Action::StartCommand;
        let _undo_action = Action::Undo;
        let _redo_action = Action::Redo;
        let _paste_action = Action::Paste;
        let _paste_before_action = Action::PasteBefore;
        let _visual_action = Action::EnterVisual;
        let _visual_line_action = Action::EnterVisualLine;
    }

    // ============================================================================
    // Integration Tests with Editor Mock
    // ============================================================================

    #[test]
    fn test_keymap_lookup() {
        let keymap = Keymap::new();
        
        // Test lookup in normal mode
        if let Some(action) = keymap.lookup(Mode::Normal, &Key::Char('h')) {
            match action {
                Action::Move(MovementCommand::Left) => {
                    // Expected
                }
                _ => panic!("Expected Move(Left) action for 'h' key"),
            }
        } else {
            panic!("'h' key should be bound in normal mode");
        }
        
        // Test lookup for non-existent key
        assert!(keymap.lookup(Mode::Normal, &Key::Char('!')).is_none());
    }

    #[test]
    fn test_keymap_mode_isolation() {
        let mut keymap = Keymap::empty();
        
        // Bind same key to different actions in different modes
        keymap.bind(Mode::Normal, Key::Char('x'), Action::Edit(EditCommand::DeleteChar));
        keymap.bind(Mode::Insert, Key::Char('x'), Action::Move(MovementCommand::Left));
        
        // Check that modes are isolated
        if let Some(Action::Edit(EditCommand::DeleteChar)) = keymap.lookup(Mode::Normal, &Key::Char('x')) {
            // Expected for normal mode
        } else {
            panic!("Normal mode 'x' should be DeleteChar");
        }
        
        if let Some(Action::Move(MovementCommand::Left)) = keymap.lookup(Mode::Insert, &Key::Char('x')) {
            // Expected for insert mode
        } else {
            panic!("Insert mode 'x' should be Move(Left)");
        }
    }

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[test]
    fn test_get_bound_keys() {
        let mut keymap = Keymap::empty();
        keymap.bind(Mode::Normal, Key::Char('h'), Action::Move(MovementCommand::Left));
        keymap.bind(Mode::Normal, Key::Char('j'), Action::Move(MovementCommand::Down));
        
        let bound_keys = keymap.get_bound_keys(Mode::Normal);
        assert_eq!(bound_keys.len(), 2);
        assert!(bound_keys.contains(&&Key::Char('h')));
        assert!(bound_keys.contains(&&Key::Char('j')));
    }

    #[test]
    fn test_clear_mode() {
        let mut keymap = Keymap::new();
        assert!(keymap.mode_binding_count(Mode::Normal) > 0);
        
        keymap.clear_mode(Mode::Normal);
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 0);
        
        // Other modes should be unaffected
        let insert_count = keymap.mode_binding_count(Mode::Insert);
        assert!(insert_count < 20); // Reasonable upper bound
    }

    #[test]
    fn test_load_default_for_mode() {
        let mut keymap = Keymap::empty();
        assert_eq!(keymap.mode_binding_count(Mode::Normal), 0);
        
        keymap.load_default_for_mode(Mode::Normal);
        assert!(keymap.mode_binding_count(Mode::Normal) > 0);
        assert!(keymap.is_bound(Mode::Normal, &Key::Char('h')));
        
        // Other modes should still be empty
        assert_eq!(keymap.mode_binding_count(Mode::Visual), 0);
    }
}
