use std::collections::HashMap;
use tree_sitter::{Language, Parser, Query, QueryCursor, Tree};

/// ANSI color codes for syntax highlighting
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn ansi_code(self) -> &'static str {
        match self {
            Color::Reset => "\x1b[0m",
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::BrightRed => "\x1b[91m",
            Color::BrightGreen => "\x1b[92m",
            Color::BrightYellow => "\x1b[93m",
            Color::BrightBlue => "\x1b[94m",
            Color::BrightMagenta => "\x1b[95m",
            Color::BrightCyan => "\x1b[96m",
            Color::BrightWhite => "\x1b[97m",
        }
    }
}

/// Token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    String,
    Number,
    Comment,
    Function,
    Variable,
    Type,
    Operator,
    Punctuation,
    Constant,
    Property,
    Tag,
    Attribute,
    Error,
}

impl TokenType {
    pub fn color(self) -> Color {
        match self {
            TokenType::Keyword => Color::Blue,
            TokenType::String => Color::Green,
            TokenType::Number => Color::Magenta,
            TokenType::Comment => Color::BrightBlue,
            TokenType::Function => Color::Yellow,
            TokenType::Variable => Color::White,
            TokenType::Type => Color::Cyan,
            TokenType::Operator => Color::Red,
            TokenType::Punctuation => Color::White,
            TokenType::Constant => Color::BrightMagenta,
            TokenType::Property => Color::BrightCyan,
            TokenType::Tag => Color::BrightRed,
            TokenType::Attribute => Color::BrightGreen,
            TokenType::Error => Color::BrightRed,
        }
    }
}

/// Represents a highlighted token in the text
#[derive(Debug, Clone)]
pub struct HighlightToken {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
    pub token_type: TokenType,
}

/// Language configuration for Tree-sitter
#[derive(Debug)]
pub struct LanguageConfig {
    pub language: Language,
    pub query: Query,
}

/// Main syntax highlighter using Tree-sitter
pub struct SyntaxHighlighter {
    parser: Parser,
    languages: HashMap<String, LanguageConfig>,
    current_language: Option<String>,
    current_tree: Option<Tree>,
    tokens_cache: HashMap<usize, Vec<HighlightToken>>, // Cache tokens by line
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut highlighter = Self {
            parser: Parser::new(),
            languages: HashMap::new(),
            current_language: None,
            current_tree: None,
            tokens_cache: HashMap::new(),
        };

        // Register built-in languages
        highlighter.register_languages();
        highlighter
    }

    /// Register all supported languages with their highlight queries
    fn register_languages(&mut self) {
        // Rust
        let rust_lang = tree_sitter_rust::language();
        match Query::new(rust_lang, RUST_HIGHLIGHTS) {
            Ok(query) => {
                self.languages.insert(
                    "rust".to_string(),
                    LanguageConfig {
                        language: rust_lang,
                        query,
                    },
                );
                eprintln!("✅ Rust language registered successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to register Rust language: {e}");
            }
        }

        // JavaScript
        let js_lang = tree_sitter_javascript::language();
        match Query::new(js_lang, JAVASCRIPT_HIGHLIGHTS) {
            Ok(query) => {
                self.languages.insert(
                    "javascript".to_string(),
                    LanguageConfig {
                        language: js_lang,
                        query,
                    },
                );
                eprintln!("✅ JavaScript language registered successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to register JavaScript language: {e}");
            }
        }

        // Python
        let python_lang = tree_sitter_python::language();
        match Query::new(python_lang, PYTHON_HIGHLIGHTS) {
            Ok(query) => {
                self.languages.insert(
                    "python".to_string(),
                    LanguageConfig {
                        language: python_lang,
                        query,
                    },
                );
                eprintln!("✅ Python language registered successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to register Python language: {e}");
            }
        }

        // C
        let c_lang = tree_sitter_c::language();
        match Query::new(c_lang, C_HIGHLIGHTS) {
            Ok(query) => {
                self.languages.insert(
                    "c".to_string(),
                    LanguageConfig {
                        language: c_lang,
                        query,
                    },
                );
                eprintln!("✅ C language registered successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to register C language: {e}");
            }
        }

        // JSON
        let json_lang = tree_sitter_json::language();
        match Query::new(json_lang, JSON_HIGHLIGHTS) {
            Ok(query) => {
                self.languages.insert(
                    "json".to_string(),
                    LanguageConfig {
                        language: json_lang,
                        query,
                    },
                );
                eprintln!("✅ JSON language registered successfully");
            }
            Err(e) => {
                eprintln!("❌ Failed to register JSON language: {e}");
            }
        }

        // Markdown
        // {
        //     let md_lang = tree_sitter_markdown::language();
        //     match Query::new(md_lang, MARKDOWN_HIGHLIGHTS) {
        //         Ok(query) => {
        //             self.languages.insert(
        //                 "markdown".to_string(),
        //                 LanguageConfig {
        //                     language: md_lang,
        //                     query,
        //                 },
        //             );
        //             eprintln!("✅ Markdown language registered successfully");
        //         }
        //         Err(e) => {
        //             eprintln!("❌ Failed to register Markdown language: {}", e);
        //         }
        //     }
        // }
    }

    /// Detect language from file extension
    pub fn detect_language(&self, filename: &str) -> Option<&str> {
        let ext = std::path::Path::new(filename)
            .extension()?
            .to_str()?
            .to_lowercase();

        match ext.as_str() {
            "rs" => Some("rust"),
            "js" | "jsx" | "mjs" => Some("javascript"),
            "ts" | "tsx" => Some("javascript"), // TypeScript uses JS parser
            "py" | "pyw" => Some("python"),
            "c" | "h" => Some("c"),
            "cpp" | "cc" | "cxx" | "hpp" => Some("c"), // C++ uses C parser
            "json" => Some("json"),
            "md" | "markdown" => Some("markdown"),
            _ => None,
        }
    }

    /// Set the language for highlighting
    pub fn set_language(&mut self, language: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = self.languages.get(language) {
            self.parser.set_language(config.language)?;
            self.current_language = Some(language.to_string());
            self.current_tree = None;
            self.tokens_cache.clear();
            Ok(())
        } else {
            Err(format!("Language '{language}' not supported").into())
        }
    }

    /// Parse the given text and update the syntax tree
    pub fn parse(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_language.is_none() {
            return Ok(()); // No language set, skip parsing
        }

        let tree = self
            .parser
            .parse(text, self.current_tree.as_ref())
            .ok_or("Failed to parse text")?;
        self.current_tree = Some(tree);
        // Only clear cache for visible lines (first 100 lines) to improve performance
        // This prevents full cache clear on every keystroke while still keeping highlighting accurate
        self.tokens_cache.retain(|&line, _| line >= 100);
        Ok(())
    }

    /// Get highlight tokens for a specific line
    pub fn get_line_highlights(
        &mut self,
        line_num: usize,
        text: &str,
    ) -> Option<Vec<HighlightToken>> {
        // Check cache first
        if let Some(tokens) = self.tokens_cache.get(&line_num) {
            return Some(tokens.clone());
        }

        let language = self.current_language.as_ref()?;
        let config = self.languages.get(language)?;
        let tree = self.current_tree.as_ref()?;

        let mut cursor = QueryCursor::new();
        let captures = cursor.captures(&config.query, tree.root_node(), text.as_bytes());

        let mut line_tokens = Vec::new();

        for (m, _) in captures {
            for capture in m.captures {
                let node = capture.node;
                let start_pos = node.start_position();
                let end_pos = node.end_position();

                // Only include tokens that are on this line
                if start_pos.row == line_num || end_pos.row == line_num {
                    let token_type = self.capture_to_token_type(capture.index, &config.query);

                    line_tokens.push(HighlightToken {
                        start_row: start_pos.row,
                        start_col: start_pos.column,
                        end_row: end_pos.row,
                        end_col: end_pos.column,
                        token_type,
                    });
                }
            }
        }

        // Sort tokens by position
        line_tokens.sort_by(|a, b| a.start_col.cmp(&b.start_col));

        // Cache the result
        self.tokens_cache.insert(line_num, line_tokens.clone());
        Some(line_tokens)
    }

    /// Convert a capture index to a token type
    fn capture_to_token_type(&self, capture_index: u32, query: &Query) -> TokenType {
        if let Some(capture_name) = query.capture_names().get(capture_index as usize) {
            match capture_name.as_str() {
                "keyword" => TokenType::Keyword,
                "string" => TokenType::String,
                "number" => TokenType::Number,
                "comment" => TokenType::Comment,
                "function" => TokenType::Function,
                "variable" => TokenType::Variable,
                "type" => TokenType::Type,
                "operator" => TokenType::Operator,
                "punctuation" => TokenType::Punctuation,
                "constant" => TokenType::Constant,
                "property" => TokenType::Property,
                "tag" => TokenType::Tag,
                "attribute" => TokenType::Attribute,
                "error" => TokenType::Error,
                _ => TokenType::Variable, // Default fallback
            }
        } else {
            TokenType::Variable
        }
    }

    /// Apply syntax highlighting to a line of text
    pub fn highlight_line(&mut self, line: &str, line_num: usize) -> String {
        if let Some(tokens) = self.get_line_highlights(line_num, line) {
            let mut result = String::new();
            let chars: Vec<char> = line.chars().collect();
            let mut last_pos = 0;

            for token in tokens {
                // Only highlight tokens that are entirely on this line
                if token.start_row == line_num && token.end_row == line_num {
                    // Clamp indices to avoid out-of-bounds
                    // FIXME: This is a temporary fix, should be improved later
                    let safe_start = std::cmp::min(token.start_col, chars.len());
                    let safe_end = std::cmp::min(token.end_col, chars.len());

                    // Add text before this token
                    if safe_start > last_pos {
                        result.push_str(&chars[last_pos..safe_start].iter().collect::<String>());
                    }

                    // Add highlighted token
                    let token_text = &chars[safe_start..safe_end].iter().collect::<String>();
                    result.push_str(token.token_type.color().ansi_code());
                    result.push_str(token_text);
                    result.push_str(Color::Reset.ansi_code());

                    last_pos = safe_end;
                }
            }

            // Add remaining text
            if last_pos < chars.len() {
                result.push_str(&chars[last_pos..].iter().collect::<String>());
            }

            result
        } else {
            line.to_string() // No highlighting available
        }
    }

    /// Check if syntax highlighting is available for the current language
    pub fn is_available(&self) -> bool {
        self.current_language.is_some() && self.current_tree.is_some()
    }

    /// Get current language name
    pub fn current_language(&self) -> Option<&str> {
        self.current_language.as_deref()
    }

    /// Get list of available languages (for debugging)
    pub fn available_languages(&self) -> Vec<String> {
        self.languages.keys().cloned().collect()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

// Tree-sitter highlight queries for different languages

const RUST_HIGHLIGHTS: &str = r#"
"use" @keyword
"pub" @keyword
"fn" @keyword
"let" @keyword
"const" @keyword
"static" @keyword
"impl" @keyword
"trait" @keyword
"struct" @keyword
"enum" @keyword
"type" @keyword
"where" @keyword
"if" @keyword
"else" @keyword
"match" @keyword
"for" @keyword
"while" @keyword
"loop" @keyword
"break" @keyword
"continue" @keyword
"return" @keyword
"mod" @keyword

(mutable_specifier) @keyword
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(integer_literal) @number
(float_literal) @number
(boolean_literal) @constant

(line_comment) @comment
(block_comment) @comment

(function_item name: (identifier) @function)
(macro_invocation macro: (identifier) @macro)
(type_identifier) @type
(primitive_type) @type
"#;

const JAVASCRIPT_HIGHLIGHTS: &str = r#"
"var" @keyword
"let" @keyword
"const" @keyword
"function" @keyword
"return" @keyword
"if" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"do" @keyword
"switch" @keyword
"case" @keyword
"default" @keyword
"break" @keyword
"continue" @keyword
"try" @keyword
"catch" @keyword
"finally" @keyword
"throw" @keyword
"new" @keyword

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function)
(call_expression function: (identifier) @function)

(string) @string
(template_string) @string
(number) @number
(true) @constant
(false) @constant

(comment) @comment

(property_identifier) @property
"class" @keyword
"extends" @keyword
"import" @keyword
"export" @keyword
"from" @keyword

(string) @string
(template_string) @string
(number) @number
(true) @constant
(false) @constant
(null) @constant
(undefined) @constant

(comment) @comment

(function_declaration name: (identifier) @function)
(call_expression function: (identifier) @function)

(property_identifier) @property

"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"===" @operator
"!=" @operator
"!==" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator

"(" @punctuation
")" @punctuation
"[" @punctuation
"]" @punctuation
"{" @punctuation
"}" @punctuation
";" @punctuation
"," @punctuation
"." @punctuation
"#;

const PYTHON_HIGHLIGHTS: &str = r#"
"def" @keyword
"class" @keyword
"if" @keyword
"elif" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"try" @keyword
"except" @keyword
"finally" @keyword
"with" @keyword
"as" @keyword
"import" @keyword
"from" @keyword
"return" @keyword
"yield" @keyword
"pass" @keyword
"break" @keyword
"continue" @keyword
"global" @keyword
"nonlocal" @keyword
"lambda" @keyword
"and" @keyword
"or" @keyword
"not" @keyword
"in" @keyword
"is" @keyword

(string) @string
(integer) @number
(float) @number
(true) @constant
(false) @constant
(none) @constant

(comment) @comment

(function_definition name: (identifier) @function)
(call function: (identifier) @function)

(attribute attribute: (identifier) @property)

"+" @operator
"-" @operator
"*" @operator
"/" @operator
"//" @operator
"%" @operator
"**" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator

"(" @punctuation
")" @punctuation
"[" @punctuation
"]" @punctuation
"{" @punctuation
"}" @punctuation
"," @punctuation
"." @punctuation
":" @punctuation
"#;

const C_HIGHLIGHTS: &str = r#"
"if" @keyword
"else" @keyword
"for" @keyword
"while" @keyword
"do" @keyword
"switch" @keyword
"case" @keyword
"default" @keyword
"break" @keyword
"continue" @keyword
"return" @keyword
"goto" @keyword
"typedef" @keyword
"struct" @keyword
"union" @keyword
"enum" @keyword
"static" @keyword
"extern" @keyword
"const" @keyword
"volatile" @keyword
"auto" @keyword
"register" @keyword
"sizeof" @keyword

(string_literal) @string
(char_literal) @string
(number_literal) @number

(comment) @comment

(function_declarator declarator: (identifier) @function)
(call_expression function: (identifier) @function)

(type_identifier) @type
(primitive_type) @type

(field_identifier) @property

"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"<<" @operator
">>" @operator

"(" @punctuation
")" @punctuation
"[" @punctuation
"]" @punctuation
"{" @punctuation
"}" @punctuation
";" @punctuation
"," @punctuation
"." @punctuation
"#;

const JSON_HIGHLIGHTS: &str = r#"
(string) @string
(number) @number
(true) @constant
(false) @constant
(null) @constant

":" @punctuation
"," @punctuation
"{" @punctuation
"}" @punctuation
"[" @punctuation
"]" @punctuation
"#;

// const MARKDOWN_HIGHLIGHTS: &str = r#"
// (atx_heading) @tag
// (setext_heading) @tag
// (link_text) @string
// (link_destination) @string
// (code_span) @string
// (fenced_code_block) @string
// (emphasis) @property
// (strong_emphasis) @property
// "#;
