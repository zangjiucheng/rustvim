#![allow(unused)]

use crate::editor::{Editor, Mode};
use crate::input::Key;

/// Represents different types of commands in Normal mode
#[derive(Debug, Clone)]
#[allow(unused)]
pub enum NormalCommand {
    /// Movement commands
    Movement(MovementCommand),
    
    /// Edit commands
    Edit(EditCommand),
    
    /// Mode switch commands
    ModeSwitch(ModeSwitchCommand),
    
    /// File operations
    File(FileCommand),
}

/// Movement commands for cursor navigation
#[derive(Debug, Clone)]
pub enum MovementCommand {
    /// Basic movements
    Left,
    Right,
    Up,
    Down,
    
    /// Word movements
    WordForward,      // w
    WordBackward,     // b
    WordEnd,          // e
    
    /// Line movements
    LineStart,        // 0
    LineFirstChar,    // ^
    LineEnd,          // $
    
    /// File movements
    FileStart,        // gg
    FileEnd,          // G
    
    /// Page movements
    PageUp,           // Ctrl-U
    PageDown,         // Ctrl-D
}

/// Edit commands for text modification
#[derive(Debug, Clone)]
pub enum EditCommand {
    /// Delete commands
    DeleteChar,       // x
    DeleteLine,       // dd
    Delete(MovementCommand), // d{motion}
    
    /// Yank (copy) commands
    YankLine,         // yy
    Yank(MovementCommand), // y{motion}
    
    /// Paste commands
    PasteAfter,       // p
    PasteBefore,      // P
    
    /// Undo/Redo
    Undo,             // u
    Redo,             // Ctrl-R
}

/// Mode switching commands
#[derive(Debug, Clone)]
pub enum ModeSwitchCommand {
    /// Insert mode variants
    InsertBefore,     // i
    InsertAfter,      // a
    InsertLineEnd,    // A
    InsertLineStart,  // I
    
    /// Open line commands
    OpenLineBelow,    // o
    OpenLineAbove,    // O
    
    /// Command mode
    CommandMode,      // :
    
    /// Search mode
    SearchForward,    // /
    SearchBackward,   // ?
}

/// File operation commands
#[derive(Debug, Clone)]
pub enum FileCommand {
    Save,             // :w
    Quit,             // :q
    SaveQuit,         // :wq
    ForceQuit,        // :q!
    Open(String),     // :e filename
}

/// Command parser and executor
pub struct CommandProcessor;

impl CommandProcessor {
    /// Parse a key into a command for Normal mode
    pub fn parse_normal_command(key: &Key, count: Option<usize>) -> Option<NormalCommand> {
        match key {
            // Movement commands
            Key::Char('h') | Key::Left => Some(NormalCommand::Movement(MovementCommand::Left)),
            Key::Char('j') | Key::Down => Some(NormalCommand::Movement(MovementCommand::Down)),
            Key::Char('k') | Key::Up => Some(NormalCommand::Movement(MovementCommand::Up)),
            Key::Char('l') | Key::Right => Some(NormalCommand::Movement(MovementCommand::Right)),
            
            Key::Char('w') => Some(NormalCommand::Movement(MovementCommand::WordForward)),
            Key::Char('b') => Some(NormalCommand::Movement(MovementCommand::WordBackward)),
            Key::Char('e') => Some(NormalCommand::Movement(MovementCommand::WordEnd)),
            
            Key::Char('0') => Some(NormalCommand::Movement(MovementCommand::LineStart)),
            Key::Char('^') => Some(NormalCommand::Movement(MovementCommand::LineFirstChar)),
            Key::Char('$') => Some(NormalCommand::Movement(MovementCommand::LineEnd)),
            
            // Edit commands
            Key::Char('x') => Some(NormalCommand::Edit(EditCommand::DeleteChar)),
            Key::Char('u') => Some(NormalCommand::Edit(EditCommand::Undo)),
            Key::Ctrl('r') => Some(NormalCommand::Edit(EditCommand::Redo)),
            
            Key::Char('p') => Some(NormalCommand::Edit(EditCommand::PasteAfter)),
            Key::Char('P') => Some(NormalCommand::Edit(EditCommand::PasteBefore)),
            
            // Mode switch commands
            Key::Char('i') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertBefore)),
            Key::Char('a') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertAfter)),
            Key::Char('A') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertLineEnd)),
            Key::Char('I') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::InsertLineStart)),
            
            Key::Char('o') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::OpenLineBelow)),
            Key::Char('O') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::OpenLineAbove)),
            
            Key::Char(':') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::CommandMode)),
            Key::Char('/') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::SearchForward)),
            Key::Char('?') => Some(NormalCommand::ModeSwitch(ModeSwitchCommand::SearchBackward)),
            
            _ => None,
        }
    }
    
    /// Execute a command on the editor
    pub fn execute_command(editor: &mut Editor, command: NormalCommand) {
        match command {
            NormalCommand::Movement(mov) => {
                Self::execute_movement(editor, mov);
            }
            NormalCommand::Edit(edit) => {
                Self::execute_edit(editor, edit);
            }
            NormalCommand::ModeSwitch(mode_switch) => {
                Self::execute_mode_switch(editor, mode_switch);
            }
            NormalCommand::File(file) => {
                Self::execute_file_command(editor, file);
            }
        }
    }
    
    /// Execute movement command (to be implemented)
    fn execute_movement(editor: &mut Editor, command: MovementCommand) {
        // TODO: Implement movement logic
    }
    
    /// Execute edit command (to be implemented)
    fn execute_edit(editor: &mut Editor, command: EditCommand) {
        // TODO: Implement edit logic
    }
    
    /// Execute mode switch command (to be implemented)
    fn execute_mode_switch(editor: &mut Editor, command: ModeSwitchCommand) {
        match command {
            ModeSwitchCommand::InsertBefore => {
                editor.mode = Mode::Insert;
            }
            ModeSwitchCommand::InsertAfter => {
                editor.mode = Mode::Insert;
                // TODO: Move cursor forward one position
            }
            ModeSwitchCommand::CommandMode => {
                editor.mode = Mode::Command;
            }
            _ => {
                // TODO: Implement other mode switches
            }
        }
    }
    
    /// Execute file command (to be implemented)
    fn execute_file_command(editor: &mut Editor, command: FileCommand) {
        // TODO: Implement file operations
    }
}
