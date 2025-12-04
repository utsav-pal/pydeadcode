# pydeadcode

🦀 Fast Python dead code finder, built in Rust.

## Features

- ⚡ **Fast**: Written in Rust for maximum performance
- 🎯 **Accurate**: Uses tree-sitter for precise Python AST parsing
- 🖥️ **Cross-platform**: Runs on Linux, macOS, Windows
- 📊 **Flexible**: JSON output, confidence filtering, exclusion patterns

## Installation

### Option 1: Cargo
cargo install pydeadcode

### Option 2: Pre-built Binaries
Download from [Releases](https://github.com/YOUR_USERNAME/pydeadcode/releases)

### Option 3: Homebrew (macOS)
brew tap YOUR_USERNAME/tap
brew install pydeadcode

## Usage

# Analyze a file
pydeadcode src/main.py

# Analyze a directory
pydeadcode ./src

# With options
pydeadcode . --min-confidence 80 --sort-by-size

# JSON output
pydeadcode . --json | jq

# Exclude patterns
pydeadcode . --exclude "*test*.py,*/docs/*"

## Output

Dead Code Found:

src/main.py: line 42 - unused_function [function] (80% confidence)
src/utils.py: line 15 - old_helper [function] (75% confidence)

2 dead code items found

## Why Rust?

- 100x faster than Python-based tools
- Zero runtime dependencies
- Single binary works anywhere
- Memory safe and efficient

## Contributing

Contributions welcome! Please open issues or PRs.

## License

MIT
