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
    
    // Parse command line arguments for file loading
    let args: Vec<String> = std::env::args().collect();
    
    // Create and run the editor
    let mut editor = Editor::new();
    
    // Load file if specified
    if args.len() > 1 {
        let filename = &args[1];
        match editor.load_file(filename) {
            Ok(()) => {
                println!("Loaded file: {} ({} lines)", filename, editor.buffer.line_count());
            }
            Err(e) => {
                eprintln!("Warning: Could not load file '{}': {}", filename, e);
                editor.filename = Some(filename.clone());
            }
        }
    }
    
    // Run the editor
    if let Err(e) = editor.run() {
        eprintln!("Editor error: {}", e);
        std::process::exit(1);
    }
}
