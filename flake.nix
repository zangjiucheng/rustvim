{
  description = "RustVim - A Vim-like Text Editor in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Define the Rust toolchain
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };

        # Native build inputs for cross-platform compatibility
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          llvm
          clang
        ];

        # Build inputs that may be needed for certain dependencies
        buildInputs =
          with pkgs;
          lib.optionals stdenv.isLinux [
            # Linux-specific dependencies if needed
          ]
          ++ lib.optionals stdenv.isDarwin [
            # macOS-specific dependencies
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.CoreFoundation
          ];

        # Development tools
        devTools = with pkgs; [
          # Version control
          git

          # Code coverage tools
          # cargo-llvm-cov

          # Additional Rust tools
          cargo-watch
          cargo-edit
          cargo-audit
          cargo-outdated

          # Terminal utilities
          bat
          ripgrep
          fd

          # Documentation tools
          mdbook
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          packages = devTools;

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          RUST_LOG = "debug";

          # Shell hook for initial setup
          shellHook = ''
            echo "🦀 Welcome to RustVim development environment!"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run all tests"
            echo "  cargo clippy         - Run linter"
            echo "  cargo fmt            - Format code"
            echo "  cargo run [file]     - Run RustVim"
            echo "  cargo watch -x test  - Watch and run tests"
            echo "  cargo llvm-cov --html - Generate coverage report"
            echo ""
            echo "Config example is available at .rustvimrc.example"
            echo "Copy it to ~/.rustvimrc to customize your settings"

            # Set up git hooks if not already done
            if [ ! -f .git/hooks/pre-commit ]; then
              echo "Setting up pre-commit hooks..."
              if [ -f scripts/install-pre-commit-hook.sh ]; then
                chmod +x scripts/install-pre-commit-hook.sh
                ./scripts/install-pre-commit-hook.sh
              fi
            fi
          '';

          # Additional environment setup
          env = {
            # Ensure cargo uses the correct Rust toolchain
            CARGO_HOME = "$HOME/.cargo";
            RUSTUP_HOME = "$HOME/.rustup";

            # Better backtraces for debugging
            RUST_BACKTRACE = "1";

            # Optimize build cache
            CARGO_TARGET_DIR = "./target";
          };
        };

        # Optional: Package the application
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rustvim";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = buildInputs;

          meta = with pkgs.lib; {
            description = "A Vim-like text editor implemented in Rust";
            homepage = "https://github.com/zangjiucheng/rustvim";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.all;
          };
        };

        # Formatter for `nix fmt`
        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
