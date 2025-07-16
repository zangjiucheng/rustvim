#!/bin/bash

# RustVim Code Coverage Script
# This script generates code coverage reports for the RustVim project

set -e

echo "🔍 RustVim Code Coverage Analysis"
echo "================================="

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "❌ cargo-llvm-cov not found. Installing..."
    cargo install cargo-llvm-cov
fi

# Check if nightly toolchain is available
if ! rustup toolchain list | grep -q nightly; then
    echo "❌ Nightly toolchain not found. Installing..."
    rustup toolchain install nightly
    rustup component add llvm-tools-preview --toolchain nightly
fi

# Function to run coverage with different options
run_coverage() {
    local format=$1
    local description=$2
    
    echo ""
    echo "📊 Generating $description..."
    
    case $format in
        "html")
            cargo llvm-cov --html
            echo "✅ HTML report generated: target/llvm-cov/html/index.html"
            ;;
        "summary")
            echo ""
            cargo llvm-cov --summary-only
            ;;
        "lcov")
            cargo llvm-cov --lcov --output-path target/coverage.lcov
            echo "✅ LCOV report generated: target/coverage.lcov"
            ;;
        "detailed")
            cargo llvm-cov --show-missing-lines
            ;;
        *)
            echo "❌ Unknown format: $format"
            exit 1
            ;;
    esac
}

# Parse command line arguments
case "${1:-summary}" in
    "html")
        run_coverage "html" "HTML Coverage Report"
        if command -v open &> /dev/null; then
            echo "🌐 Opening coverage report in browser..."
            open target/llvm-cov/html/index.html
        fi
        ;;
    "summary")
        run_coverage "summary" "Coverage Summary"
        ;;
    "lcov")
        run_coverage "lcov" "LCOV Report for CI/CD"
        ;;
    "detailed")
        run_coverage "detailed" "Detailed Line Coverage"
        ;;
    "all")
        run_coverage "html" "HTML Coverage Report"
        run_coverage "summary" "Coverage Summary"
        run_coverage "lcov" "LCOV Report"
        echo ""
        echo "✅ All coverage reports generated!"
        ;;
    "clean")
        echo "🧹 Cleaning coverage data..."
        rm -rf target/llvm-cov target/coverage.lcov
        echo "✅ Coverage data cleaned"
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [OPTION]"
        echo ""
        echo "Options:"
        echo "  summary    Generate coverage summary (default)"
        echo "  html       Generate HTML coverage report"
        echo "  lcov       Generate LCOV report for CI/CD"
        echo "  detailed   Show detailed line-by-line coverage"
        echo "  all        Generate all report types"
        echo "  clean      Remove coverage data"
        echo "  help       Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0 html       # Generate and open HTML report"
        echo "  $0 summary    # Show coverage summary"
        echo "  $0 all        # Generate all reports"
        ;;
    *)
        echo "❌ Unknown option: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac

echo ""
echo "🎯 Coverage Analysis Complete!"

# Show current test count
echo ""
echo "📈 Test Statistics:"
echo "   Test files: $(find tests -name "*.rs" | wc -l | tr -d ' ')"
echo "   Total tests: $(grep -r "#\[test\]" tests/ | wc -l | tr -d ' ')"

echo ""
echo "📂 Generated Files:"
if [ -d "target/llvm-cov/html" ]; then
    echo "   HTML Report: target/llvm-cov/html/index.html"
fi
if [ -f "target/coverage.lcov" ]; then
    echo "   LCOV Report: target/coverage.lcov"
fi

echo ""
echo "💡 Tips:"
echo "   - Use 'cargo test' to run tests before coverage"
echo "   - Use './coverage.sh html' for visual coverage analysis"
echo "   - Use './coverage.sh lcov' for CI/CD integration"
