#!/bin/bash
# Script to install the pre-commit hook for RustVim
# Run this from the project root: ./scripts/install-pre-commit-hook.sh

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${YELLOW}[INSTALL]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[INSTALL]${NC} ✅ $1"
}

print_error() {
    echo -e "${RED}[INSTALL]${NC} ❌ $1"
}

# Ensure we're in the project root
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory."
    exit 1
fi

# Check if .git directory exists
if [ ! -d ".git" ]; then
    print_error "Not a Git repository. Please run 'git init' first."
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy the pre-commit hook
print_info "Installing pre-commit hook..."
cp scripts/pre-commit-hook.sh .git/hooks/pre-commit

# Make it executable
chmod +x .git/hooks/pre-commit

print_success "Pre-commit hook installed successfully!"
echo ""
print_info "The hook will now run automatically before each commit."
print_info "It will check:"
echo "  • Rust code formatting (cargo fmt)"
echo "  • Clippy lints (cargo clippy)"
echo ""
print_info "To test the hook manually, run:"
echo "  git commit --allow-empty -m 'test commit'"
echo ""
print_info "To temporarily skip the hook, use:"
echo "  git commit --no-verify -m 'your message'"
