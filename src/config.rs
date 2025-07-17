//! Configuration system for RustVim
//!
//! This module provides a comprehensive configuration system that supports:
//! - Built-in default settings
//! - Runtime configuration via `:set` commands
//! - File-based configuration loading (future: ~/.rustvimrc)
//! - Editor behavior customization
//!
//! # Examples
//!
//! ```rust
//! use rustvim::config::EditorConfig;
//!
//! // Create with defaults
//! let config = EditorConfig::default();
//! assert_eq!(config.tab_size, 4);
//! assert!(config.expand_tabs);
//!
//! // Load from file (future)
//! // let config = EditorConfig::load_from_file("~/.rustvimrc")?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Main configuration structure for the editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// Tab-related settings
    pub tab_size: usize,
    pub expand_tabs: bool, // Convert tabs to spaces

    /// Display settings
    pub show_line_numbers: bool,
    pub show_relative_numbers: bool,
    pub wrap_lines: bool,

    /// Editor behavior
    pub auto_indent: bool,
    pub highlight_current_line: bool,
    pub show_whitespace: bool,

    /// Search settings
    pub ignore_case: bool,
    pub smart_case: bool,
    pub highlight_search: bool,

    /// Cursor and scrolling
    pub scroll_offset: usize, // Keep cursor this many lines from edge
    pub cursor_line_highlight: bool,

    /// File handling
    pub auto_save: bool,
    pub backup_files: bool,

    /// Custom key mappings (for future extension)
    pub custom_mappings: HashMap<String, String>,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            // Tab settings - commonly used defaults
            tab_size: 4,
            expand_tabs: true, // Convert tabs to spaces (modern preference)

            // Display settings
            show_line_numbers: false,
            show_relative_numbers: false,
            wrap_lines: false, // Vim default is nowrap

            // Editor behavior
            auto_indent: true,
            highlight_current_line: false,
            show_whitespace: false,

            // Search settings
            ignore_case: false,
            smart_case: true, // Case-sensitive if uppercase present
            highlight_search: true,

            // Cursor and scrolling
            scroll_offset: 3, // Keep 3 lines above/below cursor when scrolling
            cursor_line_highlight: false,

            // File handling
            auto_save: false,
            backup_files: false,

            // Custom mappings
            custom_mappings: HashMap::new(),
        }
    }
}

impl EditorConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a file
    /// File format is TOML for human readability
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();

        if !path.exists() {
            // Return default config if file doesn't exist
            return Ok(Self::default());
        }

        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::FileRead(path.to_string_lossy().into(), e))?;

        let config: EditorConfig =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let path = path.as_ref();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::FileWrite(path.to_string_lossy().into(), e))?;
        }

        let content =
            toml::to_string_pretty(self).map_err(|e| ConfigError::SerializeError(e.to_string()))?;

        fs::write(path, content)
            .map_err(|e| ConfigError::FileWrite(path.to_string_lossy().into(), e))?;

        Ok(())
    }

    /// Get the default config file path
    pub fn default_config_path() -> Option<std::path::PathBuf> {
        dirs::home_dir().map(|home| home.join(".rustvimrc"))
    }

    /// Load configuration from the default location (~/.rustvimrc)
    pub fn load_default() -> Result<Self, ConfigError> {
        match Self::default_config_path() {
            Some(path) => Self::load_from_file(path),
            None => Ok(Self::default()), // Fallback to defaults
        }
    }

    /// Save configuration to the default location
    pub fn save_default(&self) -> Result<(), ConfigError> {
        match Self::default_config_path() {
            Some(path) => self.save_to_file(path),
            None => Err(ConfigError::NoHomeDirectory),
        }
    }

    /// Set a configuration option by name (used by `:set` command)
    pub fn set_option(&mut self, option: &str, value: Option<&str>) -> Result<String, String> {
        match option {
            // Tab settings
            "tabstop" | "ts" => {
                if let Some(val) = value {
                    match val.parse::<usize>() {
                        Ok(size) if size > 0 && size <= 16 => {
                            self.tab_size = size;
                            Ok(format!("tabstop={size}"))
                        }
                        Ok(_) => Err("tabstop must be between 1 and 16".to_string()),
                        Err(_) => Err("tabstop requires a number".to_string()),
                    }
                } else {
                    Ok(format!("tabstop={}", self.tab_size))
                }
            }
            "expandtab" | "et" => {
                self.expand_tabs = true;
                Ok("expandtab enabled".to_string())
            }
            "noexpandtab" | "noet" => {
                self.expand_tabs = false;
                Ok("expandtab disabled".to_string())
            }

            // Line number settings
            "number" | "nu" | "numbers" => {
                self.show_line_numbers = true;
                self.show_relative_numbers = false;
                Ok("Line numbers enabled".to_string())
            }
            "nonumber" | "nonu" | "nonumbers" => {
                self.show_line_numbers = false;
                self.show_relative_numbers = false;
                Ok("Line numbers disabled".to_string())
            }
            "relativenumber" | "rnu" => {
                self.show_relative_numbers = true;
                self.show_line_numbers = false;
                Ok("relative line numbers enabled".to_string())
            }
            "norelativenumber" | "nornu" => {
                self.show_relative_numbers = false;
                Ok("relative line numbers disabled".to_string())
            }

            // Display settings
            "wrap" => {
                self.wrap_lines = true;
                Ok("line wrapping enabled".to_string())
            }
            "nowrap" => {
                self.wrap_lines = false;
                Ok("line wrapping disabled".to_string())
            }

            // Search settings
            "ignorecase" | "ic" => {
                self.ignore_case = true;
                Ok("ignore case in search enabled".to_string())
            }
            "noignorecase" | "noic" => {
                self.ignore_case = false;
                Ok("ignore case in search disabled".to_string())
            }
            "smartcase" | "scs" => {
                self.smart_case = true;
                Ok("smart case search enabled".to_string())
            }
            "nosmartcase" | "noscs" => {
                self.smart_case = false;
                Ok("smart case search disabled".to_string())
            }
            "hlsearch" | "hls" => {
                self.highlight_search = true;
                Ok("search highlighting enabled".to_string())
            }
            "nohlsearch" | "nohls" => {
                self.highlight_search = false;
                Ok("search highlighting disabled".to_string())
            }

            // Scroll settings
            "scrolloff" | "so" => {
                if let Some(val) = value {
                    match val.parse::<usize>() {
                        Ok(offset) if offset <= 50 => {
                            self.scroll_offset = offset;
                            Ok(format!("scrolloff={offset}"))
                        }
                        Ok(_) => Err("scrolloff must be 50 or less".to_string()),
                        Err(_) => Err("scrolloff requires a number".to_string()),
                    }
                } else {
                    Ok(format!("scrolloff={}", self.scroll_offset))
                }
            }

            // Auto-indent
            "autoindent" | "ai" => {
                self.auto_indent = true;
                Ok("auto-indent enabled".to_string())
            }
            "noautoindent" | "noai" => {
                self.auto_indent = false;
                Ok("auto-indent disabled".to_string())
            }

            _ => Err(format!("E518: Unknown option: {option}")),
        }
    }

    /// Get the tab string (either a tab character or spaces)
    pub fn get_tab_string(&self) -> String {
        if self.expand_tabs {
            " ".repeat(self.tab_size)
        } else {
            "\t".to_string()
        }
    }

    /// Expand a tab character at a given column position
    pub fn expand_tab_at_column(&self, column: usize) -> String {
        if self.expand_tabs {
            let spaces_to_next_stop = self.tab_size - (column % self.tab_size);
            " ".repeat(spaces_to_next_stop)
        } else {
            "\t".to_string()
        }
    }

    /// List all current settings (for `:set` with no args)
    pub fn list_settings(&self) -> Vec<String> {
        let mut settings = Vec::new();

        settings.push(format!("tabstop={}", self.tab_size));
        settings.push(
            if self.expand_tabs {
                "expandtab"
            } else {
                "noexpandtab"
            }
            .to_string(),
        );
        settings.push(
            if self.show_line_numbers {
                "number"
            } else {
                "nonumber"
            }
            .to_string(),
        );
        settings.push(
            if self.show_relative_numbers {
                "relativenumber"
            } else {
                "norelativenumber"
            }
            .to_string(),
        );
        settings.push(if self.wrap_lines { "wrap" } else { "nowrap" }.to_string());
        settings.push(
            if self.ignore_case {
                "ignorecase"
            } else {
                "noignorecase"
            }
            .to_string(),
        );
        settings.push(
            if self.smart_case {
                "smartcase"
            } else {
                "nosmartcase"
            }
            .to_string(),
        );
        settings.push(
            if self.highlight_search {
                "hlsearch"
            } else {
                "nohlsearch"
            }
            .to_string(),
        );
        settings.push(format!("scrolloff={}", self.scroll_offset));
        settings.push(
            if self.auto_indent {
                "autoindent"
            } else {
                "noautoindent"
            }
            .to_string(),
        );

        settings.sort();
        settings
    }
}

/// Configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file '{0}': {1}")]
    FileRead(String, std::io::Error),

    #[error("Failed to write config file '{0}': {1}")]
    FileWrite(String, std::io::Error),

    #[error("Failed to parse config file: {0}")]
    ParseError(String),

    #[error("Failed to serialize config: {0}")]
    SerializeError(String),

    #[error("Could not determine home directory")]
    NoHomeDirectory,
}
