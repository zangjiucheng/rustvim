use crate::editor::{Editor, Mode};
use crate::input::Key;
use std::collections::HashMap;

/// Type alias for plugin command functions
type PluginCommand = fn(&mut Editor) -> Result<(), String>;

/// Type alias for plugin event handlers
type EventHandler = fn(&mut Editor);

/// Registry for dynamically registered plugin commands
pub struct PluginRegistry {
    /// Ex commands (colon commands like :wc, :hello)
    ex_commands: HashMap<String, PluginCommand>,
    /// Key-based commands in different modes  
    key_commands: HashMap<(Mode, Key), PluginCommand>,
    /// Event handlers for editor events
    event_handlers: HashMap<EditorEvent, Vec<EventHandler>>,
}

/// Events that plugins can hook into
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EditorEvent {
    FileOpened(String),
    FileSaved(String),
    ModeChanged { from: Mode, to: Mode },
    BufferModified,
    SearchPerformed(String),
    CommandExecuted(String),
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self {
            ex_commands: HashMap::new(),
            key_commands: HashMap::new(),
            event_handlers: HashMap::new(),
        }
    }

    /// Register a new Ex command (colon command)
    pub fn register_ex_command(&mut self, name: String, command: PluginCommand) {
        self.ex_commands.insert(name, command);
    }

    /// Register a key command for a specific mode
    pub fn register_key_command(&mut self, mode: Mode, key: Key, command: PluginCommand) {
        self.key_commands.insert((mode, key), command);
    }

    /// Register an event handler
    pub fn register_event_handler(&mut self, event: EditorEvent, handler: EventHandler) {
        self.event_handlers.entry(event).or_default().push(handler);
    }

    /// Try to handle an unknown Ex command
    pub fn handle_ex_command(&self, name: &str, editor: &mut Editor) -> Result<bool, String> {
        if let Some(command) = self.ex_commands.get(name) {
            command(editor)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if an Ex command exists in the registry
    pub fn has_ex_command(&self, name: &str) -> bool {
        self.ex_commands.contains_key(name)
    }

    /// Get an Ex command function if it exists
    pub fn get_ex_command(&self, name: &str) -> Option<PluginCommand> {
        self.ex_commands.get(name).copied()
    }

    /// Try to handle a key command
    pub fn handle_key_command(
        &self,
        mode: Mode,
        key: &Key,
        editor: &mut Editor,
    ) -> Result<bool, String> {
        if let Some(command) = self.key_commands.get(&(mode, key.clone())) {
            command(editor)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Fire an event to all registered handlers
    pub fn fire_event(&mut self, event: EditorEvent, editor: &mut Editor) {
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
                handler(editor);
            }
        }
    }

    /// List all registered Ex commands
    pub fn list_ex_commands(&self) -> Vec<&String> {
        self.ex_commands.keys().collect()
    }

    /// List all registered key commands
    pub fn list_key_commands(&self) -> Vec<&(Mode, Key)> {
        self.key_commands.keys().collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
