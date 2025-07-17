# Pre-commit Setup for RustVim

This directory contains scripts and configuration for setting up pre-commit hooks that ensure code quality before commits.

## Quick Setup (Recommended)

Run the installation script from the project root:

```bash
./scripts/install-pre-commit-hook.sh
```

This will automatically install the Git pre-commit hook that runs:
- `cargo fmt --all -- --check` (formatting)
- `cargo clippy --all-targets --all-features -- -D warnings` (linting)

## Alternative: Using pre-commit framework

If you prefer using the [pre-commit](https://pre-commit.com/) framework:

1. Install pre-commit:
   ```bash
   pip install pre-commit
   # or
   brew install pre-commit
   ```

2. Install the hooks:
   ```bash
   pre-commit install
   ```

3. Run manually on all files:
   ```bash
   pre-commit run --all-files
   ```

## What the hooks check

### Rust-specific checks:
- **Formatting**: Ensures code follows Rust formatting standards
- **Clippy**: Runs Rust linter to catch common mistakes and improvements
- **Tests**: (Optional) Runs the test suite before commit

### General checks:
- **YAML/TOML syntax**: Validates configuration files
- **Merge conflicts**: Prevents committing merge conflict markers
- **Large files**: Prevents accidentally committing large files
- **Trailing whitespace**: Removes trailing whitespace
- **End of file**: Ensures files end with newline

## Usage

Once installed, the hooks run automatically before each commit. If any check fails, the commit is blocked until you fix the issues.

### Bypass hooks (not recommended)
```bash
git commit --no-verify -m "your message"
```

### Test hooks manually
```bash
# Test the Git hook
.git/hooks/pre-commit

# Test pre-commit framework
pre-commit run --all-files
```

## Troubleshooting

### Common issues:

1. **Formatting failures**: Run `cargo fmt --all` to fix
2. **Clippy warnings**: Address the specific warnings shown
3. **Permission denied**: Make sure scripts are executable: `chmod +x scripts/*.sh`

### Disable specific checks:

Edit `.pre-commit-config.yaml` to comment out unwanted hooks or add `stages: [manual]` to disable them by default.
