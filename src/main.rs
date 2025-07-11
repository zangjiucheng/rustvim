mod editor;
mod buffer;
mod terminal;
mod input;
mod commands;

use editor::Editor;

fn main() {
    println!("VimLike Editor v0.1.0");
    println!("A Vim-inspired text editor built in Rust");
    println!();
    println!("Usage: vimlike_editor [filename]");
    println!();
    println!("This is Day 1 - basic project structure is set up!");
    println!("The editor loop will be implemented in the coming days.");
    
    // TODO: Initialize and run the editor
    // let mut editor = Editor::new();
    // editor.run();
}
