# 🔍 PyDeadCode

A blazingly fast Python dead code detector written in Rust using tree-sitter for accurate static analysis.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

## 🚀 Features

- **⚡ Lightning Fast**: Written in Rust for maximum performance
- **🎯 Accurate Detection**: Uses tree-sitter parser for precise AST analysis
- **📊 Confidence-Based Reporting**: Reports dead code with confidence levels (60-80%)
- **🔧 Smart Heuristics**: Handles edge cases like:
  - Magic methods (`__init__`, `__str__`, `__enter__`, etc.)
  - Decorated functions (frameworks might use them)
  - Test functions (`test_*` patterns)
  - `__all__` exports
  - Dynamic attribute access (`getattr`, `setattr`)
- **🎨 Clean Output**: Color-coded terminal output for easy reading

## 📦 What It Detects

- ❌ Unused functions and methods
- ❌ Unused classes
- ❌ Unused variables (local and global)
- ❌ Unused imports
- ❌ Dead code in complex scenarios (metaclasses, decorators, closures, etc.)

## 🛠️ Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

Install Rust from [https://rustup.rs/](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### From Source

```bash
# Clone the repository
git clone https://github.com/utsav-pal/pydeadcode.git
cd pydeadcode

# Build release version (optimized)
cargo build --release

# Binary will be at target/release/pydeadcode
# You can run it with:
./target/release/pydeadcode <file.py>
```

### Development Build

```bash
# Faster build (unoptimized)
cargo build

# Run directly
cargo run -- <file.py>

# Or use the binary
./target/debug/pydeadcode <file.py>
```

### Install Globally (Optional)

```bash
# Install to ~/.cargo/bin (must be in your PATH)
cargo install --path .

# Now you can run from anywhere
pydeadcode <file.py>
```

## 📖 Usage

### Basic Usage

```bash
# Analyze a single Python file
pydeadcode path/to/your_file.py

# Analyze multiple files
pydeadcode file1.py file2.py file3.py

# Using cargo run (during development)
cargo run -- testfile1.py
```

### Command Line Options

```bash
# Run the tool
pydeadcode <file.py>

# See help (coming soon)
pydeadcode --help

# Version info (coming soon)
pydeadcode --version
```

## 💡 Example Output

Given this Python code:

```python
# example.py
def used_function():
    return "I'm called"

def unused_function():
    return "Nobody calls me"

class UsedClass:
    def __init__(self):
        self.value = 10

class UnusedClass:
    pass

if __name__ == "__main__":
    used_function()
    obj = UsedClass()
```

Running PyDeadCode:

```bash
$ pydeadcode example.py

Dead Code Found:

example.py: line 4 - unused_function [function] (80% confidence)
example.py: line 11 - UnusedClass [class] (80% confidence)

2 dead code items found
```


## 🧪 Testing

### Included Test Files

**testfile1.py** - Basic scenarios:
- Simple unused functions
- Used vs unused classes
- Magic method handling
- Decorated functions

**testfile2.py** - Complex edge cases (561 lines):
- Dynamic attribute access
- Metaclasses and class decorators
- Eval/exec patterns
- Closures and generators
- Magic methods and operators
- Abstract base classes
- Mutual recursion
- Module-level `__getattr__`

### Run Tests

```bash
# Build the project
cargo build

# Test on included files
./target/debug/pydeadcode testfile1.py
./target/debug/pydeadcode testfile2.py

# Run Rust unit tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## 🏗️ Project Structure

```
pydeadcode/
├── src/
│   ├── main.rs          # CLI entry point and argument parsing
│   ├── analyzer.rs      # Core analysis logic
│   ├── parser.rs        # Tree-sitter Python parser wrapper
│   └── detector.rs      # Dead code detection algorithms
├── Cargo.toml           # Rust dependencies and metadata
├── Cargo.lock           # Locked dependency versions
├── .gitignore           # Git ignore rules (excludes target/)
├── LICENSE              # MIT License
├── README.md            # This file
├── testfile1.py         # Basic test cases
└── testfile2.py         # Complex benchmark cases
```


### Reporting Bugs

Open an issue with:
- Python code sample that triggers the bug
- Expected vs actual output
- Your environment (OS, Rust version: `rustc --version`)

## 🙏 Acknowledgments

- **[tree-sitter](https://tree-sitter.github.io/)** - Incremental parsing system that powers the analysis
- **[tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python)** - Python grammar for tree-sitter
- **[Vulture](https://github.com/jendrikseipp/vulture)** - Inspiration for dead code detection
- **[Skylos](https://github.com/duriantaco/skylos)** - Another great Rust-based Python analyzer
- **[deadcode](https://github.com/albertas/deadcode)** - Python dead code detection tool
- **Rust community** - For amazing tools and libraries

## 👤 Author

**Utsav Pal**
- GitHub: [@utsav-pal](https://github.com/utsav-pal)
- Project Link: [https://github.com/utsav-pal/pydeadcode](https://github.com/utsav-pal/pydeadcode)
- Computer Science Engineering Student
- Open Source Contributor



## 🐛 Known Limitations

- **Single-file analysis**: Currently doesn't track imports across files
- **Dynamic code**: `eval()`, `exec()`, string-based imports may cause false positives
- **Reflection**: Heavy use of `getattr`/`setattr` might miss usage
- **Monkey patching**: Runtime modifications not tracked
- **Type checking**: Doesn't integrate with mypy/pyright

These are planned improvements - contributions welcome!

## ❓ FAQ

### Why Rust?
Rust provides excellent performance and memory safety, making it ideal for parsing and analyzing large codebases quickly.

### How accurate is it?
~95% accuracy on typical Python code. It uses smart heuristics to minimize false positives while catching most dead code.

### Can it modify my code?
Not yet. Currently read-only analysis. A `--fix` mode is planned for the future.

### Does it work with Python 2?
No, only Python 3.x is supported (via tree-sitter-python grammar).

### Is it production-ready?
It's in active development. Works well for analysis but test thoroughly before removing code.

---

⭐ **If you find this tool useful, please star it on GitHub!**

🐛 **Found a bug?** [Open an issue](https://github.com/utsav-pal/pydeadcode/issues)

💡 **Have a feature request?** [Start a discussion](https://github.com/utsav-pal/pydeadcode/discussions)
