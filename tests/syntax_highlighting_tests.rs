use rustvim::syntax::{Color, SyntaxHighlighter, TokenType};

#[test]
fn test_syntax_highlighter_creation() {
    let highlighter = SyntaxHighlighter::new();
    assert!(!highlighter.is_available()); // No language set initially
    assert!(highlighter.current_language().is_none());
}

#[test]
fn test_language_detection() {
    let highlighter = SyntaxHighlighter::new();

    // Test Rust files
    assert_eq!(highlighter.detect_language("main.rs"), Some("rust"));
    assert_eq!(highlighter.detect_language("lib.rs"), Some("rust"));

    // Test JavaScript files
    assert_eq!(highlighter.detect_language("app.js"), Some("javascript"));
    assert_eq!(
        highlighter.detect_language("component.jsx"),
        Some("javascript")
    );
    assert_eq!(
        highlighter.detect_language("module.mjs"),
        Some("javascript")
    );

    // Test TypeScript (uses JavaScript parser)
    assert_eq!(highlighter.detect_language("app.ts"), Some("javascript"));
    assert_eq!(
        highlighter.detect_language("component.tsx"),
        Some("javascript")
    );

    // Test Python files
    assert_eq!(highlighter.detect_language("script.py"), Some("python"));
    assert_eq!(highlighter.detect_language("gui.pyw"), Some("python"));

    // Test C files
    assert_eq!(highlighter.detect_language("main.c"), Some("c"));
    assert_eq!(highlighter.detect_language("header.h"), Some("c"));
    assert_eq!(highlighter.detect_language("main.cpp"), Some("c"));
    assert_eq!(highlighter.detect_language("header.hpp"), Some("c"));

    // Test JSON files
    assert_eq!(highlighter.detect_language("config.json"), Some("json"));

    // Test Markdown files
    assert_eq!(highlighter.detect_language("README.md"), Some("markdown"));
    assert_eq!(
        highlighter.detect_language("doc.markdown"),
        Some("markdown")
    );

    // Test unsupported files
    assert_eq!(highlighter.detect_language("file.txt"), None);
    assert_eq!(highlighter.detect_language("file.unknown"), None);
    assert_eq!(highlighter.detect_language("file"), None); // No extension
}

#[test]
fn test_set_language_rust() {
    let mut highlighter = SyntaxHighlighter::new();

    // Test setting Rust language
    assert!(highlighter.set_language("rust").is_ok());
    assert_eq!(highlighter.current_language(), Some("rust"));

    // Test setting unsupported language
    assert!(highlighter.set_language("unsupported").is_err());
    assert_eq!(highlighter.current_language(), Some("rust")); // Should remain unchanged
}

#[test]
fn test_set_language_javascript() {
    let mut highlighter = SyntaxHighlighter::new();

    assert!(highlighter.set_language("javascript").is_ok());
    assert_eq!(highlighter.current_language(), Some("javascript"));
}

#[test]
fn test_set_language_python() {
    let mut highlighter = SyntaxHighlighter::new();

    assert!(highlighter.set_language("python").is_ok());
    assert_eq!(highlighter.current_language(), Some("python"));
}

#[test]
fn test_set_language_c() {
    let mut highlighter = SyntaxHighlighter::new();

    assert!(highlighter.set_language("c").is_ok());
    assert_eq!(highlighter.current_language(), Some("c"));
}

#[test]
fn test_set_language_json() {
    let mut highlighter = SyntaxHighlighter::new();

    assert!(highlighter.set_language("json").is_ok());
    assert_eq!(highlighter.current_language(), Some("json"));
}

#[test]
fn test_parse_rust_code() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let rust_code = r#"
fn main() {
    let x = 42;
    println!("Hello, world!");
}
"#;

    assert!(highlighter.parse(rust_code).is_ok());
    assert!(highlighter.is_available());
}

#[test]
fn test_parse_javascript_code() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("javascript").unwrap();

    let js_code = r#"
function greet(name) {
    const message = `Hello, ${name}!`;
    console.log(message);
}
"#;

    assert!(highlighter.parse(js_code).is_ok());
    assert!(highlighter.is_available());
}

#[test]
fn test_parse_python_code() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("python").unwrap();

    let python_code = r#"
def greet(name):
    message = f"Hello, {name}!"
    print(message)
"#;

    assert!(highlighter.parse(python_code).is_ok());
    assert!(highlighter.is_available());
}

#[test]
fn test_parse_c_code() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("c").unwrap();

    let c_code = r#"
#include <stdio.h>

int main() {
    int x = 42;
    printf("Hello, world!\n");
    return 0;
}
"#;

    assert!(highlighter.parse(c_code).is_ok());
    assert!(highlighter.is_available());
}

#[test]
fn test_parse_json_code() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("json").unwrap();

    let json_code = r#"
{
    "name": "test",
    "version": "1.0.0",
    "active": true,
    "count": 42
}
"#;

    assert!(highlighter.parse(json_code).is_ok());
    assert!(highlighter.is_available());
}

#[test]
fn test_highlight_rust_line() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let rust_code = r#"fn main() {
    let x = 42;
    println!("Hello, world!");
}"#;

    highlighter.parse(rust_code).unwrap();

    // Test highlighting the first line
    let highlighted = highlighter.highlight_line("fn main() {", 0);

    // Should contain ANSI color codes
    assert!(highlighted.contains('\x1b'));
    assert!(highlighted.contains("fn"));
    assert!(highlighted.contains("main"));
}

#[test]
fn test_highlight_javascript_line() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("javascript").unwrap();

    let js_code = r#"function greet(name) {
    const message = "Hello!";
    console.log(message);
}"#;

    highlighter.parse(js_code).unwrap();

    // Test highlighting the first line
    let highlighted = highlighter.highlight_line("function greet(name) {", 0);

    // Should contain ANSI color codes
    assert!(highlighted.contains('\x1b'));
    assert!(highlighted.contains("function"));
    assert!(highlighted.contains("greet"));
}

#[test]
fn test_color_ansi_codes() {
    assert_eq!(Color::Reset.ansi_code(), "\x1b[0m");
    assert_eq!(Color::Red.ansi_code(), "\x1b[31m");
    assert_eq!(Color::Green.ansi_code(), "\x1b[32m");
    assert_eq!(Color::Blue.ansi_code(), "\x1b[34m");
    assert_eq!(Color::BrightRed.ansi_code(), "\x1b[91m");
}

#[test]
fn test_token_type_colors() {
    assert_eq!(
        TokenType::Keyword.color().ansi_code(),
        Color::Blue.ansi_code()
    );
    assert_eq!(
        TokenType::String.color().ansi_code(),
        Color::Green.ansi_code()
    );
    assert_eq!(
        TokenType::Number.color().ansi_code(),
        Color::Magenta.ansi_code()
    );
    assert_eq!(
        TokenType::Comment.color().ansi_code(),
        Color::BrightBlue.ansi_code()
    );
    assert_eq!(
        TokenType::Function.color().ansi_code(),
        Color::Yellow.ansi_code()
    );
}

#[test]
fn test_parse_without_language() {
    let mut highlighter = SyntaxHighlighter::new();

    // Should succeed but do nothing when no language is set
    assert!(highlighter.parse("some code").is_ok());
    assert!(!highlighter.is_available());
}

#[test]
fn test_highlight_without_language() {
    let mut highlighter = SyntaxHighlighter::new();

    let line = "fn main() {}";
    let highlighted = highlighter.highlight_line(line, 0);

    // Should return the original line unchanged
    assert_eq!(highlighted, line);
}

#[test]
fn test_language_switching() {
    let mut highlighter = SyntaxHighlighter::new();

    // Start with Rust
    highlighter.set_language("rust").unwrap();
    assert_eq!(highlighter.current_language(), Some("rust"));

    // Switch to JavaScript
    highlighter.set_language("javascript").unwrap();
    assert_eq!(highlighter.current_language(), Some("javascript"));

    // Switch to Python
    highlighter.set_language("python").unwrap();
    assert_eq!(highlighter.current_language(), Some("python"));
}

#[test]
fn test_complex_rust_highlighting() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let rust_code = r#"use std::collections::HashMap;

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    
    fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old", self.name, self.age)
    }
}

fn main() {
    let mut people = HashMap::new();
    let person = Person::new("Alice".to_string(), 30);
    people.insert(1, person);
    
    if let Some(p) = people.get(&1) {
        println!("{}", p.greet());
    }
}"#;

    highlighter.parse(rust_code).unwrap();

    // Test different lines
    let use_line = highlighter.highlight_line("use std::collections::HashMap;", 0);
    assert!(use_line.contains('\x1b')); // Should have color codes

    let struct_line = highlighter.highlight_line("struct Person {", 2);
    assert!(struct_line.contains('\x1b')); // Should have color codes

    let fn_line = highlighter.highlight_line("    fn new(name: String, age: u32) -> Self {", 7);
    assert!(fn_line.contains('\x1b')); // Should have color codes
}

#[test]
fn test_json_highlighting() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("json").unwrap();

    let json_code = r#"{
    "name": "test-project",
    "version": "1.0.0",
    "active": true,
    "count": 42,
    "description": null
}"#;

    highlighter.parse(json_code).unwrap();

    // Test string highlighting
    let name_line = highlighter.highlight_line("    \"name\": \"test-project\",", 1);
    assert!(name_line.contains('\x1b')); // Should have color codes

    // Test boolean highlighting
    let bool_line = highlighter.highlight_line("    \"active\": true,", 3);
    assert!(bool_line.contains('\x1b')); // Should have color codes

    // Test number highlighting
    let num_line = highlighter.highlight_line("    \"count\": 42,", 4);
    assert!(num_line.contains('\x1b')); // Should have color codes
}

#[test]
fn test_default_trait() {
    let highlighter = SyntaxHighlighter::default();
    assert!(!highlighter.is_available());
    assert!(highlighter.current_language().is_none());
}

#[test]
fn test_cache_behavior() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let rust_code = "fn main() {\n    let x = 42;\n}";
    highlighter.parse(rust_code).unwrap();

    // First call should compute and cache
    let highlighted1 = highlighter.highlight_line("fn main() {", 0);

    // Second call should use cache
    let highlighted2 = highlighter.highlight_line("fn main() {", 0);

    assert_eq!(highlighted1, highlighted2);
}

#[test]
fn test_empty_line_highlighting() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let rust_code = "fn main() {\n\n    let x = 42;\n}";
    highlighter.parse(rust_code).unwrap();

    // Test empty line
    let highlighted = highlighter.highlight_line("", 1);
    assert_eq!(highlighted, "");
}

#[test]
fn test_long_line_highlighting() {
    let mut highlighter = SyntaxHighlighter::new();
    highlighter.set_language("rust").unwrap();

    let long_line = "fn very_long_function_name_that_goes_on_and_on(param1: String, param2: u32, param3: bool) -> Result<String, Box<dyn std::error::Error>> {";
    let rust_code = format!("{long_line}\n    Ok(\"result\".to_string())\n}}");

    highlighter.parse(&rust_code).unwrap();

    let highlighted = highlighter.highlight_line(long_line, 0);
    assert!(highlighted.contains('\x1b')); // Should have color codes
    assert!(highlighted.len() >= long_line.len()); // Should be longer due to color codes
}
