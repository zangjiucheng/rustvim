use rustvim::editor::Editor;

#[allow(dead_code)]
fn main() {
    println!("RustVim Editor v0.1.0");
    println!("A Vim-inspired text editor built in Rust");
    println!();

    // Parse command line arguments for file loading
    let args: Vec<String> = std::env::args().collect();

    // Create and run the editor
    let mut editor = Editor::new();

    // Load files if specified
    if args.len() > 1 {
        let filenames = args[1..].to_vec(); // Get all filenames after the program name
        let results = editor.load_files(&filenames);

        // Report results
        for (filename, result) in filenames.iter().zip(results.iter()) {
            match result {
                Ok(()) => {
                    // Get line count for the loaded buffer
                    if let Some((_, line_count)) = editor.get_buffer_info(filename) {
                        println!("Loaded file: {filename} ({line_count} lines)");
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Could not load file '{filename}': {e}");
                }
            }
        }

        if filenames.len() > 1 {
            println!("Loaded {} files. Use :bn and :bp to switch between buffers, :ls to list all buffers.", filenames.len());
        }
    }

    // Run the editor
    if let Err(e) = editor.run() {
        eprintln!("Editor error: {e}");
        std::process::exit(1);
    }
}
