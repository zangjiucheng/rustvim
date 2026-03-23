use rustvim::config::EditorConfig;
use tempfile::tempdir;

#[test]
fn test_default_config() {
    let config = EditorConfig::default();
    assert_eq!(config.tab_size, 4);
    assert!(config.expand_tabs);
    assert!(!config.show_line_numbers);
    assert!(config.auto_indent);
}

#[test]
fn test_set_option_tabstop() {
    let mut config = EditorConfig::default();

    // Valid tab size
    assert!(config.set_option("tabstop", Some("8")).is_ok());
    assert_eq!(config.tab_size, 8);

    // Invalid tab size
    assert!(config.set_option("tabstop", Some("0")).is_err());
    assert!(config.set_option("tabstop", Some("20")).is_err());
    assert!(config.set_option("tabstop", Some("abc")).is_err());

    // Test short form
    assert!(config.set_option("ts", Some("2")).is_ok());
    assert_eq!(config.tab_size, 2);

    // Test without value (should return current value)
    let result = config.set_option("tabstop", None).unwrap();
    assert_eq!(result, "tabstop=2");
}

#[test]
fn test_set_option_boolean() {
    let mut config = EditorConfig::default();

    // Enable number
    assert!(config.set_option("number", None).is_ok());
    assert!(config.show_line_numbers);
    assert!(!config.show_relative_numbers);

    // Disable number
    assert!(config.set_option("nonumber", None).is_ok());
    assert!(!config.show_line_numbers);

    // Test expandtab
    assert!(config.set_option("noexpandtab", None).is_ok());
    assert!(!config.expand_tabs);

    assert!(config.set_option("expandtab", None).is_ok());
    assert!(config.expand_tabs);
}

#[test]
fn test_set_option_search_settings() {
    let mut config = EditorConfig::default();

    // Test ignore case
    assert!(config.set_option("ignorecase", None).is_ok());
    assert!(config.ignore_case);

    assert!(config.set_option("noignorecase", None).is_ok());
    assert!(!config.ignore_case);

    // Test smart case
    assert!(config.set_option("nosmartcase", None).is_ok());
    assert!(!config.smart_case);

    assert!(config.set_option("smartcase", None).is_ok());
    assert!(config.smart_case);

    // Test highlight search
    assert!(config.set_option("nohlsearch", None).is_ok());
    assert!(!config.highlight_search);

    assert!(config.set_option("hlsearch", None).is_ok());
    assert!(config.highlight_search);
}

#[test]
fn test_set_option_scrolloff() {
    let mut config = EditorConfig::default();

    // Valid scroll offset
    assert!(config.set_option("scrolloff", Some("5")).is_ok());
    assert_eq!(config.scroll_offset, 5);

    // Test short form
    assert!(config.set_option("so", Some("10")).is_ok());
    assert_eq!(config.scroll_offset, 10);

    // Invalid scroll offset (too large)
    assert!(config.set_option("scrolloff", Some("100")).is_err());

    // Invalid scroll offset (not a number)
    assert!(config.set_option("scrolloff", Some("abc")).is_err());

    // Test without value (should return current value)
    let result = config.set_option("scrolloff", None).unwrap();
    assert_eq!(result, "scrolloff=10");
}

#[test]
fn test_set_option_wrap_settings() {
    let mut config = EditorConfig::default();

    // Test wrap
    assert!(config.set_option("wrap", None).is_ok());
    assert!(config.wrap_lines);

    assert!(config.set_option("nowrap", None).is_ok());
    assert!(!config.wrap_lines);
}

#[test]
fn test_set_option_autoindent() {
    let mut config = EditorConfig::default();

    // Test autoindent
    assert!(config.set_option("noautoindent", None).is_ok());
    assert!(!config.auto_indent);

    assert!(config.set_option("autoindent", None).is_ok());
    assert!(config.auto_indent);

    // Test short forms
    assert!(config.set_option("noai", None).is_ok());
    assert!(!config.auto_indent);

    assert!(config.set_option("ai", None).is_ok());
    assert!(config.auto_indent);
}

#[test]
fn test_set_option_unknown() {
    let mut config = EditorConfig::default();

    // Unknown option should return error
    let result = config.set_option("unknownoption", None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("E518: Unknown option"));
}

#[test]
fn test_get_tab_string() {
    let mut config = EditorConfig {
        tab_size: 4,
        expand_tabs: true,
        ..Default::default()
    };
    // With tab expansion
    assert_eq!(config.get_tab_string(), "    ");

    // Without tab expansion
    config.expand_tabs = false;
    assert_eq!(config.get_tab_string(), "\t");

    // Different tab size
    config.tab_size = 8;
    config.expand_tabs = true;
    assert_eq!(config.get_tab_string(), "        ");
}

#[test]
fn test_expand_tab_at_column() {
    let config = EditorConfig {
        tab_size: 4,
        expand_tabs: true,
        ..Default::default()
    };

    // At column 0, should expand to 4 spaces
    assert_eq!(config.expand_tab_at_column(0), "    ");

    // At column 1, should expand to 3 spaces
    assert_eq!(config.expand_tab_at_column(1), "   ");

    // At column 2, should expand to 2 spaces
    assert_eq!(config.expand_tab_at_column(2), "  ");

    // At column 3, should expand to 1 space
    assert_eq!(config.expand_tab_at_column(3), " ");

    // At column 4, should expand to 4 spaces (next tab stop)
    assert_eq!(config.expand_tab_at_column(4), "    ");

    // At column 5, should expand to 3 spaces
    assert_eq!(config.expand_tab_at_column(5), "   ");
}

#[test]
fn test_expand_tab_different_sizes() {
    // Test with tab size 2
    let config2 = EditorConfig {
        tab_size: 2,
        expand_tabs: true,
        ..Default::default()
    };

    assert_eq!(config2.expand_tab_at_column(0), "  ");
    assert_eq!(config2.expand_tab_at_column(1), " ");
    assert_eq!(config2.expand_tab_at_column(2), "  ");

    // Test with tab size 8
    let config8 = EditorConfig {
        tab_size: 8,
        expand_tabs: true,
        ..Default::default()
    };

    assert_eq!(config8.expand_tab_at_column(0), "        ");
    assert_eq!(config8.expand_tab_at_column(7), " ");
    assert_eq!(config8.expand_tab_at_column(8), "        ");
}

#[test]
fn test_config_file_operations() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("test_config.toml");

    // Create and save config
    let config = EditorConfig {
        tab_size: 8,
        show_line_numbers: true,
        wrap_lines: true,
        ignore_case: true,
        scroll_offset: 5,
        ..Default::default()
    };

    config.save_to_file(&config_path).unwrap();
    assert!(config_path.exists());

    // Load config back
    let loaded_config = EditorConfig::load_from_file(&config_path).unwrap();
    assert_eq!(loaded_config.tab_size, 8);
    assert!(loaded_config.show_line_numbers);
    assert!(loaded_config.wrap_lines);
    assert!(loaded_config.ignore_case);
    assert_eq!(loaded_config.scroll_offset, 5);
}

#[test]
fn test_config_file_nonexistent() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("nonexistent.toml");

    // Loading non-existent file should return default config
    let config = EditorConfig::load_from_file(&config_path).unwrap();
    let default_config = EditorConfig::default();

    assert_eq!(config.tab_size, default_config.tab_size);
    assert_eq!(config.expand_tabs, default_config.expand_tabs);
    assert_eq!(config.show_line_numbers, default_config.show_line_numbers);
}

#[test]
fn test_config_save_creates_directories() {
    let dir = tempdir().unwrap();
    let nested_path = dir.path().join("nested").join("dir").join("config.toml");

    let config = EditorConfig::default();
    config.save_to_file(&nested_path).unwrap();

    assert!(nested_path.exists());
    assert!(nested_path.parent().unwrap().exists());
}

#[test]
fn test_list_settings() {
    let config = EditorConfig::default();
    let settings = config.list_settings();

    assert!(!settings.is_empty());
    assert!(settings.iter().any(|s| s.starts_with("tabstop=")));
    assert!(settings.iter().any(|s| s.contains("expandtab")));
    assert!(settings.iter().any(|s| s.contains("number")));
    assert!(settings.iter().any(|s| s.contains("wrap")));
    assert!(settings.iter().any(|s| s.contains("ignorecase")));
    assert!(settings.iter().any(|s| s.contains("smartcase")));
    assert!(settings.iter().any(|s| s.contains("hlsearch")));
    assert!(settings.iter().any(|s| s.starts_with("scrolloff=")));
    assert!(settings.iter().any(|s| s.contains("autoindent")));

    // Settings should be sorted
    let mut expected_sorted = settings.clone();
    expected_sorted.sort();
    assert_eq!(settings, expected_sorted);
}

#[test]
fn test_list_settings_reflects_changes() {
    let mut config = EditorConfig::default();

    // Change some settings
    config.set_option("number", None).unwrap();
    config.set_option("noexpandtab", None).unwrap();
    config.set_option("tabstop", Some("8")).unwrap();

    let settings = config.list_settings();

    assert!(settings.iter().any(|s| s == "number"));
    assert!(settings.iter().any(|s| s == "noexpandtab"));
    assert!(settings.iter().any(|s| s == "tabstop=8"));
}

#[test]
fn test_config_new() {
    let config = EditorConfig::new();
    let default_config = EditorConfig::default();

    assert_eq!(config.tab_size, default_config.tab_size);
    assert_eq!(config.expand_tabs, default_config.expand_tabs);
    assert_eq!(config.show_line_numbers, default_config.show_line_numbers);
}

#[test]
fn test_config_default_path() {
    // This test might fail in some environments where home directory is not available
    // but it's still good to test the functionality
    let path = EditorConfig::default_config_path();
    if let Some(p) = path {
        assert!(p.to_string_lossy().contains(".rustvimrc"));
    }
}

#[test]
fn test_relative_line_numbers() {
    let mut config = EditorConfig::default();

    // Enable relative numbers
    assert!(config.set_option("relativenumber", None).is_ok());
    assert!(config.show_relative_numbers);
    assert!(!config.show_line_numbers); // Should disable regular numbers

    // Disable relative numbers
    assert!(config.set_option("norelativenumber", None).is_ok());
    assert!(!config.show_relative_numbers);

    // Test short form
    assert!(config.set_option("rnu", None).is_ok());
    assert!(config.show_relative_numbers);

    assert!(config.set_option("nornu", None).is_ok());
    assert!(!config.show_relative_numbers);
}

#[test]
fn test_config_clone() {
    let config1 = EditorConfig {
        tab_size: 8,
        show_line_numbers: true,
        ..Default::default()
    };

    let config2 = config1.clone();
    assert_eq!(config1.tab_size, config2.tab_size);
    assert_eq!(config1.show_line_numbers, config2.show_line_numbers);
}

#[test]
fn test_edge_case_tab_sizes() {
    let mut config = EditorConfig::default();

    // Test minimum valid tab size
    assert!(config.set_option("tabstop", Some("1")).is_ok());
    assert_eq!(config.tab_size, 1);

    // Test maximum valid tab size
    assert!(config.set_option("tabstop", Some("16")).is_ok());
    assert_eq!(config.tab_size, 16);

    // Test boundary conditions
    assert!(config.set_option("tabstop", Some("0")).is_err());
    assert!(config.set_option("tabstop", Some("17")).is_err());
}

#[test]
fn test_edge_case_scrolloff() {
    let mut config = EditorConfig::default();

    // Test minimum valid scrolloff
    assert!(config.set_option("scrolloff", Some("0")).is_ok());
    assert_eq!(config.scroll_offset, 0);

    // Test maximum valid scrolloff
    assert!(config.set_option("scrolloff", Some("50")).is_ok());
    assert_eq!(config.scroll_offset, 50);

    // Test boundary condition
    assert!(config.set_option("scrolloff", Some("51")).is_err());
}

#[test]
fn test_fill_missing_with_defaults() {
    // Test calling fill_missing_with_defaults (just to cover the function)
    let mut config = EditorConfig::default();
    config.fill_missing_with_defaults();

    // After filling with defaults, should still have valid values
    assert!(config.tab_size > 0);
    assert!(config.scroll_offset <= 50);
}
