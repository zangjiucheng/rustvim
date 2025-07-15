//! Keymap system for the Vim-like editor
//! 
//! This module provides a centralized, configurable key mapping system that maps
//! key combinations to actions across different modes. This replaces scattered
//! match statements with a clean, extensible table-driven approach.
//! 
//! # Features
//! 
//! - **Mode-specific keymaps**: Different key bindings for Normal, Insert, Visual, etc.
//! - **Configurable bindings**: Runtime modification and custom configurations  
//! - **Builder pattern**: Easy keymap configuration with fluent API
//! - **Vim compatibility**: Default bindings match standard Vim behavior
//! - **Operator-pending mode**: Support for complex commands like `d2w`, `y3j`
//! - **Multi-key sequences**: Support for commands like `gg`, `dd`, `yy`
//! 
//! # Examples
//! 
//! ```rust
//! use vimlike_editor::keymap::*;
//! use vimlike_editor::editor::Mode;
//! use vimlike_editor::input::Key;
//! 
//! // Create with defaults
//! let processor = KeymapProcessor::new();
//! 
//! // Custom configuration
//! let config = KeymapConfigBuilder::with_defaults()
//!     .bind(Mode::Normal, Key::Char('Q'), Action::StartCommand)
//!     .build();
//! let processor = KeymapProcessor::with_config(config);
//! ```

use std::collections::HashMap;
use crate::input::Key;
use crate::editor::{Editor, Mode};
use crate::commands::{MovementCommand, EditCommand, ModeSwitchCommand, Command};

// ============================================================================
// Core Types and Actions
// ============================================================================

/// Represents an action that can be performed in the editor
#[derive(Debug, Clone)]
pub enum Action {
    /// Movement actions
    Move(MovementCommand),
    /// Edit actions  
    Edit(EditCommand),
    /// Mode switching actions
    ModeSwitch(ModeSwitchCommand),
    /// Special actions
    StartSearch,
    StartCommand,
    Undo,
    Redo,
    Paste,
    PasteBefore,
    /// Search mode actions
    SearchExecute,
    SearchBackspace,
    SearchAddChar(char),
    SearchCancel,
    /// Search navigation actions
    SearchNext,
    SearchPrevious,
    /// Insert mode actions
    InsertChar(char),
    InsertBackspace,
    InsertNewline,
    InsertNavLeft,
    InsertNavRight,
    InsertNavUp,
    InsertNavDown,
    /// Command mode actions
    CommandExecute,
    CommandBackspace,
    CommandAddChar(char),
    CommandCancel,
    /// Visual mode actions
    EnterVisual,
    EnterVisualLine,
    /// Multi-character sequences
    Pending(PendingAction),
}

/// Represents a pending action that requires additional input
#[derive(Debug, Clone)]
pub enum PendingAction {
    /// Waiting for second 'g' for 'gg' command
    FirstG,
    /// Delete operator waiting for motion
    Delete,
    /// Yank operator waiting for motion  
    Yank,
    /// Change operator waiting for motion
    Change,
}

/// Represents an operator waiting for a motion
#[derive(Debug, Clone)]
pub enum Operator {
    Delete,    // d
    Yank,      // y
    Change,    // c
}

/// Multi-key command state
#[derive(Debug, Clone)]
pub enum MultiKeyState {
    G,  // Waiting for second 'g' in 'gg'
}

/// Key mapping for a specific mode
pub type ModeKeymap = HashMap<Key, Action>;

// ============================================================================
// Keymap Configuration
// ============================================================================

/// Serializable keymap configuration for saving/loading
#[derive(Debug, Clone)]
pub struct KeymapConfig {
    pub normal: ModeKeymap,
    pub insert: ModeKeymap,
    pub visual: ModeKeymap,
    pub command: ModeKeymap,
    pub search: ModeKeymap,
}

/// Complete keymap for all modes
#[derive(Debug)]
pub struct Keymap {
    normal: ModeKeymap,
    insert: ModeKeymap,
    visual: ModeKeymap,
    command: ModeKeymap,
    search: ModeKeymap,
}

// ============================================================================
// Default Keymap Definitions
// ============================================================================

impl Keymap {
    /// Create a new keymap with default Vim bindings
    pub fn new() -> Self {
        Self {
            normal: Self::default_normal_keymap(),
            insert: Self::default_insert_keymap(),
            visual: Self::default_visual_keymap(),
            command: Self::default_command_keymap(),
            search: Self::default_search_keymap(),
        }
    }
    
    /// Create an empty keymap (for custom configuration)
    pub fn empty() -> Self {
        Self {
            normal: HashMap::new(),
            insert: HashMap::new(),
            visual: HashMap::new(),
            command: HashMap::new(),
            search: HashMap::new(),
        }
    }
    
    /// Create a keymap with only specific mode defaults
    pub fn with_mode_defaults(modes: &[Mode]) -> Self {
        let mut keymap = Self::empty();
        for mode in modes {
            match mode {
                Mode::Normal => keymap.normal = Self::default_normal_keymap(),
                Mode::Insert => keymap.insert = Self::default_insert_keymap(),
                Mode::Visual => keymap.visual = Self::default_visual_keymap(),
                Mode::Command => keymap.command = Self::default_command_keymap(),
                Mode::Search => keymap.search = Self::default_search_keymap(),
            }
        }
        keymap
    }
    
    /// Load default keymap for a specific mode (public access)
    pub fn load_default_for_mode(&mut self, mode: Mode) {
        match mode {
            Mode::Normal => self.normal = Self::default_normal_keymap(),
            Mode::Insert => self.insert = Self::default_insert_keymap(),
            Mode::Visual => self.visual = Self::default_visual_keymap(),
            Mode::Command => self.command = Self::default_command_keymap(),
            Mode::Search => self.search = Self::default_search_keymap(),
        }
    }
    
    /// Clear all bindings for a specific mode
    pub fn clear_mode(&mut self, mode: Mode) {
        self.get_mode_keymap_mut(mode).clear();
    }
    
    /// Get the number of bindings in a specific mode
    pub fn mode_binding_count(&self, mode: Mode) -> usize {
        self.get_mode_keymap(mode).len()
    }
    
    /// Get all keys bound in a specific mode
    pub fn get_bound_keys(&self, mode: Mode) -> Vec<&Key> {
        self.get_mode_keymap(mode).keys().collect()
    }
    
    /// Check if a key is bound in a specific mode
    pub fn is_bound(&self, mode: Mode, key: &Key) -> bool {
        self.get_mode_keymap(mode).contains_key(key)
    }
    
    /// Bulk bind multiple keys at once
    pub fn bind_multiple(&mut self, mode: Mode, bindings: Vec<(Key, Action)>) {
        let mode_map = self.get_mode_keymap_mut(mode);
        for (key, action) in bindings {
            mode_map.insert(key, action);
        }
    }
    
    /// Export current keymap as a configuration (for saving to file)
    pub fn export_config(&self) -> KeymapConfig {
        KeymapConfig {
            normal: self.normal.clone(),
            insert: self.insert.clone(),
            visual: self.visual.clone(),
            command: self.command.clone(),
            search: self.search.clone(),
        }
    }
    
    /// Import keymap from configuration (for loading from file)
    pub fn import_config(&mut self, config: KeymapConfig) {
        self.normal = config.normal;
        self.insert = config.insert;
        self.visual = config.visual;
        self.command = config.command;
        self.search = config.search;
    }
    
    /// Get the keymap for a specific mode
    pub fn get_mode_keymap(&self, mode: Mode) -> &ModeKeymap {
        match mode {
            Mode::Normal => &self.normal,
            Mode::Insert => &self.insert,
            Mode::Visual => &self.visual,
            Mode::Command => &self.command,
            Mode::Search => &self.search,
        }
    }
    
    /// Get a mutable reference to the keymap for a specific mode
    pub fn get_mode_keymap_mut(&mut self, mode: Mode) -> &mut ModeKeymap {
        match mode {
            Mode::Normal => &mut self.normal,
            Mode::Insert => &mut self.insert,
            Mode::Visual => &mut self.visual,
            Mode::Command => &mut self.command,
            Mode::Search => &mut self.search,
        }
    }
    
    /// Look up an action for a key in a specific mode
    pub fn lookup(&self, mode: Mode, key: &Key) -> Option<&Action> {
        self.get_mode_keymap(mode).get(key)
    }
    
    /// Add or override a key binding
    pub fn bind(&mut self, mode: Mode, key: Key, action: Action) {
        self.get_mode_keymap_mut(mode).insert(key, action);
    }
    
    /// Remove a key binding
    pub fn unbind(&mut self, mode: Mode, key: &Key) {
        self.get_mode_keymap_mut(mode).remove(key);
    }
    
    /// Default Normal mode keymap (Vim-compatible) - Public for global access
    pub fn default_normal_keymap() -> ModeKeymap {
        let mut keymap = HashMap::new();
        
        // === Basic Movement ===
        keymap.insert(Key::Char('h'), Action::Move(MovementCommand::Left));
        keymap.insert(Key::Char('j'), Action::Move(MovementCommand::Down));
        keymap.insert(Key::Char('k'), Action::Move(MovementCommand::Up));
        keymap.insert(Key::Char('l'), Action::Move(MovementCommand::Right));
        keymap.insert(Key::Left, Action::Move(MovementCommand::Left));
        keymap.insert(Key::Down, Action::Move(MovementCommand::Down));
        keymap.insert(Key::Up, Action::Move(MovementCommand::Up));
        keymap.insert(Key::Right, Action::Move(MovementCommand::Right));
        
        // === Word Movement ===
        keymap.insert(Key::Char('w'), Action::Move(MovementCommand::WordForward));
        keymap.insert(Key::Char('b'), Action::Move(MovementCommand::WordBackward));
        keymap.insert(Key::Char('e'), Action::Move(MovementCommand::WordEnd));
        
        // === Line Navigation ===
        keymap.insert(Key::Char('0'), Action::Move(MovementCommand::LineStart));
        keymap.insert(Key::Char('^'), Action::Move(MovementCommand::LineFirstChar));
        keymap.insert(Key::Char('$'), Action::Move(MovementCommand::LineEnd));
        
        // === File Navigation ===
        keymap.insert(Key::Char('G'), Action::Move(MovementCommand::FileEnd));
        keymap.insert(Key::Char('g'), Action::Pending(PendingAction::FirstG));
        keymap.insert(Key::Ctrl('u'), Action::Move(MovementCommand::PageUp));
        keymap.insert(Key::Ctrl('d'), Action::Move(MovementCommand::PageDown));
        
        // === Mode Switching ===
        keymap.insert(Key::Char('i'), Action::ModeSwitch(ModeSwitchCommand::InsertBefore));
        keymap.insert(Key::Char('a'), Action::ModeSwitch(ModeSwitchCommand::InsertAfter));
        keymap.insert(Key::Char('A'), Action::ModeSwitch(ModeSwitchCommand::InsertLineEnd));
        keymap.insert(Key::Char('o'), Action::ModeSwitch(ModeSwitchCommand::OpenLineBelow));
        keymap.insert(Key::Char('O'), Action::ModeSwitch(ModeSwitchCommand::OpenLineAbove));
        
        // === Edit Operations ===
        keymap.insert(Key::Char('x'), Action::Edit(EditCommand::DeleteChar));
        keymap.insert(Key::Char('d'), Action::Pending(PendingAction::Delete));
        keymap.insert(Key::Char('y'), Action::Pending(PendingAction::Yank));
        keymap.insert(Key::Char('c'), Action::Pending(PendingAction::Change));
        
        // === Paste Operations ===
        keymap.insert(Key::Char('p'), Action::Paste);
        keymap.insert(Key::Char('P'), Action::PasteBefore);
        
        // === Undo/Redo ===
        keymap.insert(Key::Char('u'), Action::Undo);
        keymap.insert(Key::Ctrl('r'), Action::Redo);
        
        // === Search and Command ===
        keymap.insert(Key::Char('/'), Action::StartSearch);
        keymap.insert(Key::Char('n'), Action::SearchNext);
        keymap.insert(Key::Char('N'), Action::SearchPrevious);
        keymap.insert(Key::Char(':'), Action::StartCommand);
        
        // === Visual Mode ===
        keymap.insert(Key::Char('v'), Action::EnterVisual);
        keymap.insert(Key::Char('V'), Action::EnterVisualLine);
        
        keymap
    }
    
    /// Default Insert mode keymap - Public for global access
    pub fn default_insert_keymap() -> ModeKeymap {
        let mut keymap = HashMap::new();
        
        // === Core Insert Mode Operations ===
        keymap.insert(Key::Enter, Action::InsertNewline);
        keymap.insert(Key::Backspace, Action::InsertBackspace);
        
        // === Navigation in Insert Mode ===
        // Arrow keys work in insert mode for convenience
        keymap.insert(Key::Left, Action::InsertNavLeft);
        keymap.insert(Key::Right, Action::InsertNavRight);
        keymap.insert(Key::Up, Action::InsertNavUp);
        keymap.insert(Key::Down, Action::InsertNavDown);
        
        // Note: Regular character insertion is handled as a fallback
        // through InsertChar action when no specific key binding exists
        
        keymap
    }
    
    /// Default Visual mode keymap - Public for global access
    pub fn default_visual_keymap() -> ModeKeymap {
        let mut keymap = HashMap::new();
        
        // === Movement extends selection ===
        keymap.insert(Key::Char('h'), Action::Move(MovementCommand::Left));
        keymap.insert(Key::Char('j'), Action::Move(MovementCommand::Down));
        keymap.insert(Key::Char('k'), Action::Move(MovementCommand::Up));
        keymap.insert(Key::Char('l'), Action::Move(MovementCommand::Right));
        keymap.insert(Key::Left, Action::Move(MovementCommand::Left));
        keymap.insert(Key::Down, Action::Move(MovementCommand::Down));
        keymap.insert(Key::Up, Action::Move(MovementCommand::Up));
        keymap.insert(Key::Right, Action::Move(MovementCommand::Right));
        
        // === Word movements ===
        keymap.insert(Key::Char('w'), Action::Move(MovementCommand::WordForward));
        keymap.insert(Key::Char('b'), Action::Move(MovementCommand::WordBackward));
        keymap.insert(Key::Char('e'), Action::Move(MovementCommand::WordEnd));
        
        // === Line movements ===
        keymap.insert(Key::Char('0'), Action::Move(MovementCommand::LineStart));
        keymap.insert(Key::Char('^'), Action::Move(MovementCommand::LineFirstChar));
        keymap.insert(Key::Char('$'), Action::Move(MovementCommand::LineEnd));
        
        // === Operations on selection ===
        keymap.insert(Key::Char('d'), Action::Edit(EditCommand::DeleteSelection));
        keymap.insert(Key::Char('y'), Action::Edit(EditCommand::YankSelection));
        
        // === Exit visual mode ===
        keymap.insert(Key::Char('v'), Action::ModeSwitch(ModeSwitchCommand::ExitVisual));
        
        keymap
    }
    
    /// Default Command mode keymap - Public for global access
    pub fn default_command_keymap() -> ModeKeymap {
        let mut keymap = HashMap::new();
        
        // Enter - execute command
        keymap.insert(Key::Enter, Action::CommandExecute);
        
        // Backspace - remove character from command input
        keymap.insert(Key::Backspace, Action::CommandBackspace);
        
        // Escape is handled globally in editor.rs, but we can also handle it here for completeness  
        keymap.insert(Key::Esc, Action::CommandCancel);
        
        // Regular characters are handled dynamically in the processor
        // since we need to capture any character for command input
        
        keymap
    }
    
    /// Default Search mode keymap - Public for global access
    pub fn default_search_keymap() -> ModeKeymap {
        let mut keymap = HashMap::new();
        
        // Enter - execute search
        keymap.insert(Key::Enter, Action::SearchExecute);
        
        // Backspace - remove character from search query
        keymap.insert(Key::Backspace, Action::SearchBackspace);
        
        // Escape is handled globally in editor.rs, but we can also handle it here for completeness
        keymap.insert(Key::Esc, Action::SearchCancel);
        
        // Regular characters are handled dynamically in the processor
        // since we need to capture any character for search input
        
        keymap
    }
}

impl Default for Keymap {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// KeymapProcessor - Main Input Handler
// ============================================================================

/// KeymapProcessor handles key input using the keymap system
pub struct KeymapProcessor {
    keymap: Keymap,
    pending_action: Option<PendingAction>,
    pending_count: Option<usize>,
    pending_operator: Option<Operator>,
    multi_key_state: Option<MultiKeyState>,
}

// ============================================================================
// KeymapProcessor Implementation
// ============================================================================

impl KeymapProcessor {
    // ---- Construction and Configuration ----

    /// Create a new keymap processor with default bindings
    pub fn new() -> Self {
        Self {
            keymap: Keymap::new(),
            pending_action: None,
            pending_count: None,
            pending_operator: None,
            multi_key_state: None,
        }
    }
    
    /// Create a keymap processor with custom keymap
    pub fn with_keymap(keymap: Keymap) -> Self {
        Self {
            keymap,
            pending_action: None,
            pending_count: None,
            pending_operator: None,
            multi_key_state: None,
        }
    }
    
    /// Create a processor with a custom keymap configuration
    pub fn with_config(config: KeymapConfig) -> Self {
        let mut keymap = Keymap::empty();
        keymap.import_config(config);
        Self::with_keymap(keymap)
    }
    
    /// Update the keymap configuration at runtime
    pub fn update_config(&mut self, config: KeymapConfig) {
        self.keymap.import_config(config);
    }
    
    /// Get current configuration for saving
    pub fn get_config(&self) -> KeymapConfig {
        self.keymap.export_config()
    }
    
    /// Get a reference to the keymap
    pub fn keymap(&self) -> &Keymap {
        &self.keymap
    }
    
    /// Get a mutable reference to the keymap
    pub fn keymap_mut(&mut self) -> &mut Keymap {
        &mut self.keymap
    }
    
    // ---- Key Processing ----
    
    /// Process a key input and execute the corresponding action
    pub fn process_key(&mut self, editor: &mut Editor, key: &Key) -> Result<bool, String> {
        // Special handling for search mode
        if editor.mode == Mode::Search {
            // First check for specific search keys in the keymap
            if let Some(action) = self.keymap.lookup(Mode::Search, key) {
                let result = self.execute_action(editor, action.clone())?;
                return Ok(result);
            }
            
            // For any character not in the search keymap, add it to search input
            if let Key::Char(c) = key {
                let result = self.execute_action(editor, Action::SearchAddChar(*c))?;
                return Ok(result);
            }
            
            // Unknown key in search mode
            return Ok(false);
        }
        
        // Special handling for command mode
        if editor.mode == Mode::Command {
            // First check for specific command keys in the keymap (Enter, Backspace, Escape)
            if let Some(action) = self.keymap.lookup(Mode::Command, key) {
                let result = self.execute_action(editor, action.clone())?;
                return Ok(result);
            }
            
            // For any character not in the command keymap, add it to command input
            if let Key::Char(c) = key {
                let result = self.execute_action(editor, Action::CommandAddChar(*c))?;
                return Ok(result);
            }
            
            // Unknown key in command mode
            return Ok(false);
        }
        
        // Special handling for insert mode
        if editor.mode == Mode::Insert {
            // First check for specific insert keys in the keymap (Enter, Backspace, arrows)
            if let Some(action) = self.keymap.lookup(Mode::Insert, key) {
                let result = self.execute_action(editor, action.clone())?;
                return Ok(result);
            }
            
            // For any printable character not in the insert keymap, insert it
            if let Key::Char(c) = key {
                let result = self.execute_action(editor, Action::InsertChar(*c))?;
                return Ok(result);
            }
            
            // Unknown key in insert mode
            return Ok(false);
        }
        
        // Handle digit inputs for count accumulation
        if let Key::Char(c) = key {
            if c.is_ascii_digit() && (*c != '0' || self.pending_count.is_some()) {
                let digit = c.to_digit(10).unwrap() as usize;
                self.pending_count = Some(self.pending_count.unwrap_or(0) * 10 + digit);
                return Ok(true);
            }
        }
        
        // Handle multi-key state first
        if let Some(multi_state) = &self.multi_key_state {
            return self.handle_multi_key_state(editor, key, multi_state.clone());
        }
        
        // Handle pending operators (operator-pending mode)
        if let Some(operator) = &self.pending_operator {
            return self.handle_pending_operator(editor, key, operator.clone());
        }
        
        // Handle operator keys that start operator-pending mode
        match key {
            Key::Char('d') => {
                self.pending_operator = Some(Operator::Delete);
                return Ok(true);
            }
            Key::Char('y') => {
                self.pending_operator = Some(Operator::Yank);
                return Ok(true);
            }
            Key::Char('g') => {
                self.multi_key_state = Some(MultiKeyState::G);
                return Ok(true);
            }
            _ => {}
        }
        
        // Handle pending multi-character sequences
        if let Some(pending) = &self.pending_action {
            return self.handle_pending_action(editor, key, pending.clone());
        }
        
        // Look up the action for this key in the current mode
        if let Some(action) = self.keymap.lookup(editor.mode.clone(), key) {
            let result = self.execute_action(editor, action.clone())?;
            
            // Clear count after successful action
            if result {
                self.pending_count = None;
            }
            
            Ok(result)
        } else {
            // Key not found in keymap
            Ok(false)
        }
    }
    
    // ---- Special Input Handlers ----
    
    /// Handle multi-key state (like gg)
    fn handle_multi_key_state(&mut self, editor: &mut Editor, key: &Key, state: MultiKeyState) -> Result<bool, String> {
        self.multi_key_state = None; // Clear multi-key state
        
        match state {
            MultiKeyState::G => {
                if key == &Key::Char('g') {
                    // Execute 'gg' - go to top of file
                    let count = self.pending_count.unwrap_or(1);
                    crate::commands::MovementExecutor::execute_movement(editor, MovementCommand::FileStart, count);
                    self.pending_count = None;
                    Ok(true)
                } else {
                    // Invalid sequence, ignore
                    Ok(false)
                }
            }
        }
    }
    
    /// Handle pending operators waiting for motion
    fn handle_pending_operator(&mut self, editor: &mut Editor, key: &Key, operator: Operator) -> Result<bool, String> {
        self.pending_operator = None; // Clear pending operator
        let count = self.pending_count.unwrap_or(1);
        self.pending_count = None;
        
        match key {
            // Handle operator doubling (dd, yy)
            Key::Char('d') if matches!(operator, Operator::Delete) => {
                crate::commands::OperatorExecutor::execute_delete_line(editor, count);
                Ok(true)
            }
            Key::Char('y') if matches!(operator, Operator::Yank) => {
                crate::commands::OperatorExecutor::execute_yank_line(editor, count);
                Ok(true)
            }
            // Handle 'g' prefix for motions like dgg, ygg
            Key::Char('g') => {
                self.multi_key_state = Some(MultiKeyState::G);
                self.pending_operator = Some(operator); // Restore operator
                self.pending_count = Some(count); // Restore count
                Ok(true)
            }
            // Handle regular motions
            _ => {
                if let Some(Action::Move(motion)) = self.keymap.lookup(Mode::Normal, key) {
                    match operator {
                        Operator::Delete => {
                            crate::commands::OperatorExecutor::execute_delete_motion(editor, motion.clone(), count);
                        }
                        Operator::Yank => {
                            crate::commands::OperatorExecutor::execute_yank_motion(editor, motion.clone(), count);
                        }
                        Operator::Change => {
                            // TODO: Implement change operator
                        }
                    }
                    Ok(true)
                } else {
                    // Invalid motion for operator
                    Ok(false)
                }
            }
        }
    }
    
    /// Handle pending actions that require additional input
    fn handle_pending_action(&mut self, editor: &mut Editor, key: &Key, pending: PendingAction) -> Result<bool, String> {
        self.pending_action = None; // Clear pending action
        
        match pending {
            PendingAction::FirstG => {
                if key == &Key::Char('g') {
                    // Execute 'gg' - go to top of file
                    crate::commands::MovementExecutor::execute_movement(editor, MovementCommand::FileStart, 1);
                    Ok(true)
                } else {
                    // Invalid sequence, ignore
                    Ok(false)
                }
            }
            PendingAction::Delete => {
                // Handle delete operator + motion
                if key == &Key::Char('d') {
                    // 'dd' - delete line
                    EditCommand::DeleteLine.execute(editor)?;
                    Ok(true)
                } else if let Some(action) = self.keymap.lookup(Mode::Normal, key) {
                    if let Action::Move(movement) = action {
                        // Delete from cursor to end of movement
                        self.execute_delete_motion(editor, movement.clone())?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            PendingAction::Yank => {
                // Handle yank operator + motion
                if key == &Key::Char('y') {
                    // 'yy' - yank line
                    EditCommand::YankLine.execute(editor)?;
                    Ok(true)
                } else if let Some(action) = self.keymap.lookup(Mode::Normal, key) {
                    if let Action::Move(movement) = action {
                        // Yank from cursor to end of movement
                        self.execute_yank_motion(editor, movement.clone())?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            PendingAction::Change => {
                // TODO: Implement change operator for future
                Ok(false)
            }
        }
    }
    
    // ---- Action Execution ----
    
    /// Execute an action
    fn execute_action(&mut self, editor: &mut Editor, action: Action) -> Result<bool, String> {
        match action {
            Action::Move(movement) => {
                let count = self.pending_count.unwrap_or(1);
                crate::commands::MovementExecutor::execute_movement(editor, movement, count);
                Ok(true)
            }
            Action::Edit(edit) => {
                // Store count in editor temporarily for commands that need it
                let old_count = editor.pending_count;
                editor.pending_count = self.pending_count;
                let result = edit.execute(editor);
                editor.pending_count = old_count; // Restore
                result?;
                Ok(true)
            }
            Action::ModeSwitch(mode_switch) => {
                mode_switch.execute(editor)?;
                Ok(true)
            }
            Action::StartSearch => {
                editor.mode = Mode::Search;
                editor.search_input.clear();
                Ok(true)
            }
            Action::StartCommand => {
                editor.mode = Mode::Command;
                editor.command_input.clear();
                Ok(true)
            }
            Action::Undo => {
                editor.undo();
                Ok(true)
            }
            Action::Redo => {
                editor.redo();
                Ok(true)
            }
            Action::Paste => {
                crate::commands::OperatorExecutor::execute_paste_after(editor);
                Ok(true)
            }
            Action::PasteBefore => {
                crate::commands::OperatorExecutor::execute_paste_before(editor);
                Ok(true)
            }
            Action::SearchExecute => {
                // Execute search
                if !editor.search_input.is_empty() {
                    editor.search_query = Some(editor.search_input.clone());
                    editor.search_forward(&editor.search_input.clone());
                }
                editor.mode = Mode::Normal;
                editor.search_input.clear();
                Ok(true)
            }
            Action::SearchBackspace => {
                // Remove last character from search query
                editor.search_input.pop();
                Ok(true)
            }
            Action::SearchAddChar(c) => {
                // Add character to search query
                editor.search_input.push(c);
                Ok(true)
            }
            Action::SearchCancel => {
                // Cancel search and return to normal mode
                editor.mode = Mode::Normal;
                editor.search_input.clear();
                editor.search_match = None;
                Ok(true)
            }
            Action::SearchNext => {
                // Search for next occurrence of last query
                editor.search_next();
                Ok(true)
            }
            Action::SearchPrevious => {
                // Search for previous occurrence of last query
                editor.search_previous();
                Ok(true)
            }
            Action::InsertChar(c) => {
                // Insert character at cursor position
                let pos = crate::buffer::Position::new(editor.cursor().row, editor.cursor().col);
                editor.buffer_mut().insert_char(pos, c);
                editor.insert_mode_char(c);
                editor.cursor_mut().col += 1;
                editor.set_modified(true);
                Ok(true)
            }
            Action::InsertNewline => {
                // Insert newline and move cursor
                let pos = crate::buffer::Position::new(editor.cursor().row, editor.cursor().col);
                editor.buffer_mut().insert_newline(pos);
                editor.insert_mode_newline();
                editor.cursor_mut().row += 1;
                editor.cursor_mut().col = 0;
                editor.set_modified(true);
                editor.update_scroll();
                Ok(true)
            }
            Action::InsertBackspace => {
                // Handle backspace in insert mode
                if editor.cursor().col > 0 {
                    // Delete character to the left in current line
                    editor.cursor_mut().col -= 1;
                    let pos = crate::buffer::Position::new(editor.cursor().row, editor.cursor().col);
                    let deleted_char = editor.buffer_mut().delete_char(pos);
                    editor.insert_mode_backspace(deleted_char, Some(pos));
                    editor.set_modified(true);
                } else if editor.cursor().row > 0 {
                    // At beginning of line - join with previous line
                    editor.cursor_mut().row -= 1;
                    editor.cursor_mut().col = editor.buffer().line_length(editor.cursor().row);
                    
                    // Delete the newline (which will merge the lines)
                    let pos = crate::buffer::Position::new(editor.cursor().row, editor.cursor().col);
                    let deleted_char = editor.buffer_mut().delete_char(pos);
                    
                    editor.insert_mode_backspace(deleted_char, Some(pos));
                    editor.set_modified(true);
                    editor.update_scroll();
                }
                Ok(true)
            }
            Action::InsertNavLeft => {
                // Navigate left in insert mode
                if editor.cursor().col > 0 {
                    editor.cursor_mut().col -= 1;
                }
                Ok(true)
            }
            Action::InsertNavRight => {
                // Navigate right in insert mode
                let line_len = editor.buffer().line_length(editor.cursor().row);
                // In insert mode, allow cursor to go to line_len (after last character)
                if editor.cursor().col < line_len {
                    editor.cursor_mut().col += 1;
                }
                Ok(true)
            }
            Action::InsertNavUp => {
                // Navigate up in insert mode
                if editor.cursor().row > 0 {
                    editor.cursor_mut().row -= 1;
                    let line_len = editor.buffer().line_length(editor.cursor().row);
                    if editor.cursor().col > line_len {
                        editor.cursor_mut().col = line_len;
                    }
                }
                editor.update_scroll();
                Ok(true)
            }
            Action::InsertNavDown => {
                // Navigate down in insert mode
                if editor.cursor().row + 1 < editor.buffer().line_count() {
                    editor.cursor_mut().row += 1;
                    let line_len = editor.buffer().line_length(editor.cursor().row);
                    if editor.cursor().col > line_len {
                        editor.cursor_mut().col = line_len;
                    }
                }
                editor.update_scroll();
                Ok(true)
            }
            Action::CommandExecute => {
                // Execute command and return to normal mode
                let command = editor.command_input.trim().to_string();
                editor.execute_ex_command(&command);
                editor.mode = Mode::Normal;
                editor.command_input.clear();
                Ok(true)
            }
            Action::CommandBackspace => {
                // Remove last character from command input
                editor.command_input.pop();
                Ok(true)
            }
            Action::CommandAddChar(ch) => {
                // Add character to command input
                editor.command_input.push(ch);
                Ok(true)
            }
            Action::CommandCancel => {
                // Cancel command and return to normal mode
                editor.mode = Mode::Normal;
                editor.command_input.clear();
                Ok(true)
            }
            Action::EnterVisual => {
                // TODO: Implement for Day 17
                editor.mode = Mode::Visual;
                Ok(true)
            }
            Action::EnterVisualLine => {
                // TODO: Implement for Day 17  
                editor.mode = Mode::Visual;
                Ok(true)
            }
            Action::Pending(pending) => {
                self.pending_action = Some(pending);
                Ok(true)
            }
        }
    }
    
    // ---- Motion Execution Helpers ----
    
    /// Execute delete with motion
    fn execute_delete_motion(&self, editor: &mut Editor, movement: MovementCommand) -> Result<(), String> {
        let count = self.pending_count.unwrap_or(1);
        crate::commands::OperatorExecutor::execute_delete_motion(editor, movement, count);
        Ok(())
    }
    
    /// Execute yank with motion
    fn execute_yank_motion(&self, editor: &mut Editor, movement: MovementCommand) -> Result<(), String> {
        let count = self.pending_count.unwrap_or(1);
        crate::commands::OperatorExecutor::execute_yank_motion(editor, movement, count);
        Ok(())
    }
}

impl Default for KeymapProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Configuration Builder Pattern
// ============================================================================

/// Global keymap configuration helpers and builders
impl KeymapConfig {
    /// Create a default configuration
    pub fn default() -> Self {
        Self {
            normal: Keymap::default_normal_keymap(),
            insert: Keymap::default_insert_keymap(),
            visual: Keymap::default_visual_keymap(),
            command: Keymap::default_command_keymap(),
            search: Keymap::default_search_keymap(),
        }
    }
    
    /// Create an empty configuration
    pub fn empty() -> Self {
        Self {
            normal: HashMap::new(),
            insert: HashMap::new(),
            visual: HashMap::new(),
            command: HashMap::new(),
            search: HashMap::new(),
        }
    }
    
    /// Builder pattern for custom configuration
    pub fn builder() -> KeymapConfigBuilder {
        KeymapConfigBuilder::new()
    }
}

/// Builder for keymap configuration
pub struct KeymapConfigBuilder {
    config: KeymapConfig,
}

impl KeymapConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: KeymapConfig::empty(),
        }
    }
    
    /// Start with default configuration
    pub fn with_defaults() -> Self {
        Self {
            config: KeymapConfig::default(),
        }
    }
    
    /// Add a binding for a specific mode
    pub fn bind(mut self, mode: Mode, key: Key, action: Action) -> Self {
        let mode_map = match mode {
            Mode::Normal => &mut self.config.normal,
            Mode::Insert => &mut self.config.insert,
            Mode::Visual => &mut self.config.visual,
            Mode::Command => &mut self.config.command,
            Mode::Search => &mut self.config.search,
        };
        mode_map.insert(key, action);
        self
    }
    
    /// Add multiple bindings for a mode
    pub fn bind_multiple(mut self, mode: Mode, bindings: Vec<(Key, Action)>) -> Self {
        let mode_map = match mode {
            Mode::Normal => &mut self.config.normal,
            Mode::Insert => &mut self.config.insert,
            Mode::Visual => &mut self.config.visual,
            Mode::Command => &mut self.config.command,
            Mode::Search => &mut self.config.search,
        };
        for (key, action) in bindings {
            mode_map.insert(key, action);
        }
        self
    }
    
    /// Load default bindings for a specific mode
    pub fn with_mode_defaults(mut self, mode: Mode) -> Self {
        match mode {
            Mode::Normal => self.config.normal = Keymap::default_normal_keymap(),
            Mode::Insert => self.config.insert = Keymap::default_insert_keymap(),
            Mode::Visual => self.config.visual = Keymap::default_visual_keymap(),
            Mode::Command => self.config.command = Keymap::default_command_keymap(),
            Mode::Search => self.config.search = Keymap::default_search_keymap(),
        }
        self
    }
    
    /// Build the final configuration
    pub fn build(self) -> KeymapConfig {
        self.config
    }
}

impl Default for KeymapConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
