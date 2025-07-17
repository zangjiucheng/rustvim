# Contributing to RustVim

Thank you for your interest in contributing to RustVim! This guide will help you get started with contributing to our Vim-like text editor written in Rust.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
4. **Make your changes** with tests
5. **Run quality checks** (see below)
6. **Commit your changes** (`git commit -m 'Add amazing feature'`)
7. **Push to your branch** (`git push origin feature/amazing-feature`)
8. **Open a Pull Request** on GitHub

## 🛠️ Development Setup

### Prerequisites
- **Rust 1.85+** (install from [rustup.rs](https://rustup.rs/))
- **Git** for version control

### Local Development
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/rustvim.git
cd rustvim

# Build the project
cargo build

# Run tests
cargo test

# Run the editor
cargo run [filename]
```

## ✅ Quality Assurance

Before submitting your pull request, ensure all quality checks pass:

### 1. Run Tests
```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test visual_block_mode_tests
cargo test --test buffer_tests

# Run with output for debugging
cargo test -- --nocapture
```

### 2. Code Coverage
```bash
# Make the script executable (first time only)
chmod +x coverage.sh

# Generate coverage summary
./coverage.sh

# Generate HTML report
./coverage.sh html
```

### 3. Code Formatting
```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all
```

### 4. Linting
```bash
# Run clippy for code quality
cargo clippy --all-targets --all-features -- -D warnings
```

## 📋 Development Guidelines

### Code Standards
- **Follow Rust naming conventions** (snake_case for functions/variables, PascalCase for types)
- **Write comprehensive tests** for new functionality
- **Update documentation** for user-facing changes
- **Ensure `cargo clippy` passes** without warnings
- **Maintain or improve code coverage**
- **Use descriptive commit messages**

### Testing Requirements
- **Unit tests** for individual functions and modules
- **Integration tests** for complete workflows
- **Edge case testing** for error conditions
- **Performance considerations** for large files

### Documentation Standards
- **Inline documentation** for public APIs using `///`
- **Code comments** for complex algorithms
- **README updates** for user-facing features
- **Architecture documentation** for significant changes

## 🔄 Automated Quality Checks

When you open a pull request, GitHub Actions will automatically run:

- ✅ **Cross-platform Build**: Linux, Windows, and macOS compilation
- ✅ **Test Suite**: Complete test suite (171+ tests)
- ✅ **Code Formatting**: `cargo fmt` validation
- ✅ **Linting**: `cargo clippy` analysis
- ✅ **Coverage Analysis**: Test coverage reporting
- ✅ **PR Comments**: Coverage summary posted on your PR

**All checks must pass before merging.**

## 📝 Contribution Types

### 🐛 Bug Reports
When reporting bugs, please include:
- **Clear description** of the issue
- **Steps to reproduce** the problem
- **Expected vs actual behavior**
- **Environment details** (OS, Rust version)
- **Minimal code example** if applicable

### ✨ Feature Requests
For new features, please provide:
- **Clear use case** and motivation
- **Detailed description** of proposed functionality
- **Examples** of how it would work
- **Consideration of alternatives**

### 🔧 Code Contributions
- **Start with issues** labeled `good first issue` for beginners
- **Discuss major changes** in an issue first
- **Keep pull requests focused** on a single feature/fix
- **Write tests** for your changes
- **Update documentation** as needed

## 🏗️ Project Architecture

Understanding the codebase structure:

```
src/
├── main.rs           # Application entry point
├── lib.rs           # Module exports
├── editor.rs        # Core editor state and event loop
├── buffer.rs        # Text buffer management (Gap Buffer)
├── terminal.rs      # Terminal I/O and rendering
├── input.rs         # Key input handling
├── commands.rs      # Ex-command processing
├── keymap.rs        # Key mapping system
├── io.rs           # File I/O operations
└── history.rs       # Undo/redo system

tests/              # Comprehensive test suite
├── visual_block_mode_tests.rs
├── visual_mode_tests.rs
├── gap_buffer_tests.rs
└── ... (28+ test files)
```

### Key Components
- **Modal Architecture**: Clean separation between editing modes
- **Gap Buffer**: Efficient text storage for O(1) insertions
- **Composable Commands**: Operators combine with motions
- **Comprehensive Testing**: 171+ tests ensure reliability

## 🎯 Areas for Contribution

### High Priority
- **Input Processing** (`input.rs`) - Currently has low test coverage
- **Command Execution** (`commands.rs`) - Needs more ex-command tests
- **Terminal Operations** (`terminal.rs`) - Rendering and UI tests needed

### Medium Priority
- **Performance Optimizations** - Large file handling
- **Error Handling** - Edge case scenarios
- **Documentation** - Code examples and tutorials

### Future Features
- **Syntax Highlighting** - Language-aware coloring
- **Configuration System** - User customizable settings
- **Plugin Architecture** - Extensibility framework
- **Regular Expressions** - Pattern matching in search
- **Multiple Windows** - Split view functionality

## 🚦 Pull Request Process

### Before Submitting
1. **Rebase your branch** on the latest main
2. **Run all quality checks** locally
3. **Write/update tests** for your changes
4. **Update documentation** if needed
5. **Ensure clean commit history**

### PR Description Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests pass locally
- [ ] Coverage maintained/improved
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### Review Process
1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Discussion and feedback** addressed
4. **Final approval** and merge

## 📞 Getting Help

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Pull Request Comments**: Code-specific discussions

### Resources
- **[Architecture Documentation](ARCHITECTURE.md)**: System design overview
- **[Daily Summaries](docs/daily-summaries/)**: Implementation progress
- **[Rust Book](https://doc.rust-lang.org/book/)**: Rust language reference
- **[Vim Documentation](https://vimhelp.org/)**: Vim behavior reference

## 🏆 Recognition

Contributors are recognized in:
- **README acknowledgments**
- **Git commit history**
- **Release notes** for significant contributions
- **GitHub contributor graphs**

## 📜 Code of Conduct

This project adheres to a code of conduct based on respect, inclusivity, and collaboration:

- **Be respectful** in all interactions
- **Welcome newcomers** and help them learn
- **Focus on constructive feedback**
- **Assume good intentions**
- **Follow community guidelines**

## 📄 License

By contributing to RustVim, you agree that your contributions will be licensed under the same [MIT License](LICENSE) that covers the project.

---

Thank you for contributing to RustVim! Your efforts help make this project better for everyone. 🎉
