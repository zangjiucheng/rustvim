#!/bin/bash
# Pre-commit hook for RustVim
# This script runs formatting checks and lints before allowing commits
#
# To install this hook:
# 1. Copy this file to .git/hooks/pre-commit
# 2. Make it executable: chmod +x .git/hooks/pre-commit
#
# Or use the install script: ./scripts/install-pre-commit-hook.sh

set -e  # Exit on any error

echo "🔍 Running pre-commit checks for RustVim..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${YELLOW}[PRE-COMMIT]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PRE-COMMIT]${NC} ✅ $1"
}

print_error() {
    echo -e "${RED}[PRE-COMMIT]${NC} ❌ $1"
}

# Change to the repository root
cd "$(git rev-parse --show-toplevel)"

# Check if this is a Rust project
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. This hook is designed for Rust projects."
    exit 1
fi

# 1. Check Rust formatting
print_status "Checking Rust code formatting..."
if cargo fmt --all -- --check; then
    print_success "Code formatting is correct"
else
    print_error "Code formatting check failed!"
    echo "Run 'cargo fmt --all' to fix formatting issues."
    exit 1
fi

# 2. Run Clippy lints
print_status "Running Clippy lints..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy lints passed"
else
    print_error "Clippy found issues!"
    echo "Fix the warnings above before committing."
    exit 1
fi

# 3. Run tests (optional - uncomment to enable)
# print_status "Running tests..."
# if cargo test --all-features; then
#     print_success "All tests passed"
# else
#     print_error "Tests failed!"
#     echo "Fix failing tests before committing."
#     exit 1
# fi

print_success "All pre-commit checks passed! 🎉"
echo ""
