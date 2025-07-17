//! Plugin modules for RustVim
//!
//! This module contains all the built-in plugin examples and utilities.
//! Each plugin is organized in its own module for better maintainability.

use crate::plugin::PluginRegistry;

pub mod text_analysis;
pub mod text_manipulation;
pub mod utils;

/// Register all built-in plugins
pub fn register_all_plugins(registry: &mut PluginRegistry) {
    text_analysis::register_plugins(registry);
    text_manipulation::register_plugins(registry);
    utils::register_plugins(registry);
}
