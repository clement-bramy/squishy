# Squishy ⚡🐙

A simple utility to concatenate all source files in a Rust project into a single file for easy sharing and analysis.

## What is Squishy? 🤔

Squishy recursively scans a Rust project directory, filters for relevant files (`.rs` and `.toml`), and combines them into a single output file with clear path headers. It's useful for sharing complete project context or getting an overview of your codebase.

## Installation 📥

### Option 1: Download from GitHub Releases
Download the latest binary from the [releases page](https://github.com/clement-bramy/squishy/releases).

### Option 2: Build from Source
```bash
git clone https://github.com/clement-bramy/squishy.git
cd squishy
cargo install --path .
```

## Usage 🚀

Navigate to any Rust project directory and run:

```bash
squishy
```

Or specify a directory to scan:

```bash
squishy /path/to/rust/project
```

Squishy will:
- Scan the project directory recursively
- Filter for Rust source files (`.rs`) and configuration files (`.toml`)
- Skip `target/` and `.git/` directories
- Create `squishy.txt` with the concatenated results

### Command Line Options

```bash
# Basic usage - scan current directory
squishy

# Scan specific directory
squishy /path/to/rust/project

# Custom output file
squishy -o my-project.txt

# Custom output file with path
squishy -o /path/to/output/project-snapshot.txt

# Quiet mode (no banner or summary)
squishy -q

# Enable performance tracing
squishy -t

# Combine options
squishy /path/to/project -o output.txt -q

# View help
squishy --help
```

### Available Options ⚙️

- **Positional argument**: Directory to scan (defaults to current directory)
- `-o, --out <FILE>`: Output file (default: `squishy.txt`)
- `--no-banner`: Disable ASCII banner
- `--no-summary`: Disable processing summary
- `-q, --quiet`: Disable all output (combines `--no-banner` and `--no-summary`)
- `-t, --trace`: Enable performance tracing for debugging
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Example Console Output 📺

```text
Squishy v0.1.10 (built: 1752666832)
 _____ _____ _____ _____ _____ _____ __ __
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|
         |__|

Squishy file: target/squishy.txt
Scanned 8 of 18 files
Processed 8 of 8 (10018 total bytes)
  ✓ ./Cargo.toml (298 bytes)
  ✓ ./src/squish.rs (1938 bytes)
  ✓ ./src/types.rs (2650 bytes)
  ✓ ./src/cli.rs (604 bytes)
  ✓ ./src/scanner.rs (1769 bytes)
  ✓ ./src/main.rs (1874 bytes)
  ✓ ./src/errors.rs (611 bytes)
  ✓ ./build.rs (274 bytes)
```

### Example Output File Format 📄

```rust
// ─── ./Cargo.toml ───────────────────────────────────────────
[package]
name = "example-project"
version = "0.1.0"
edition = "2021"


// ─── ./src/main.rs ───────────────────────────────────────────
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


// ─── ./src/lib.rs ───────────────────────────────────────────
pub mod utils;

pub fn helper_function() {
    // implementation
}


// ─── ./src/utils/mod.rs ───────────────────────────────────────────
pub fn utility_function() {
    // implementation
}
```

## Performance ⚡

Squishy processes files efficiently with minimal memory usage.

### Typical Performance 📊

Based on testing with real Rust projects:

| Project Size | File Count | Processing Time | Memory Usage |
|--------------|------------|-----------------|--------------|
| 📝 **Small** (personal projects) | 8-50 files | < 10ms | ~3MB |
| 📚 **Medium** (libraries like ripgrep) | 50-200 files | 10-50ms | ~3MB |
| 🏗️ **Large** (frameworks like tokio) | 200-700 files | 50-200ms | ~3MB |
| 🏭 **Enterprise** (projects like cargo) | 1000+ files | 400-600ms | ~3MB |

*Performance measured on modern hardware. Times may vary based on file sizes and directory structure.*

Memory usage stays constant because files are processed one at a time rather than loading everything into memory. 🧠

## Features ✨

- ✅ **Command line interface**: Unix-style arguments with positional directory and output options
- ✅ **Recursive scanning**: Processes all subdirectories 🔄
- ✅ **File filtering**: Includes `.rs` and `.toml` files, skips `target/` and `.git/` 🗂️
- ✅ **Progress feedback**: Shows what files were processed and any errors 📝
- ✅ **Error handling**: Continues processing when individual files fail 🛡️
- ✅ **Clear output format**: Headers show original file paths 📋
- ✅ **Quiet mode**: Can run silently for scripting 🤫
- ✅ **Performance tracing**: Optional timing information for debugging ⏱️

## Planned Improvements 🔮

### File Handling Enhancements
- [ ] `.gitignore` integration for even smarter filtering
- [ ] Support for additional file types (`.md`, etc.)
- [ ] Custom include/exclude patterns

### Advanced Features
- [ ] Configuration file support (`.squishyrc` - global and project-specific)
- [ ] Dry-run mode to preview files without generating output
- [ ] Template support for different output formats
- [ ] Integration with Cargo as a subcommand
- [ ] Parallel processing for very large projects

### User Experience
- [ ] Tree view of processed files
- [ ] Better handling of edge cases (symlinks, special files, etc.)
- [ ] Cross-platform path handling improvements

## Use Cases 🎯

- **AI/LLM Analysis** 🤖: Share complete codebase context with tools like Claude or ChatGPT
- **Code Review** 👀: Provide entire projects in a single readable file
- **Documentation** 📚: Generate code listings with project structure
- **Learning** 🎓: Study how projects are organized
- **Backup** 💾: Create flat file representations of project structure

## Platform Support 🖥️

Primarily tested on Unix-like systems (Linux, macOS). Windows compatibility not guaranteed. 🤷‍♂️

## Error Handling 🛡️

Squishy continues processing when individual files fail:
- Shows clear error messages for issues encountered ❌
- Summary indicates what succeeded and what failed ✅❌
- Always produces output even if some files can't be processed 📄

## License 📜

MIT

## Contributing 🤝

This is a learning project focused on practical Rust development. Suggestions are welcome.

---

*Useful for sharing Rust projects with AI assistants, code reviewers, or anyone who needs to understand your codebase structure.*
