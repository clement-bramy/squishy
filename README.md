# Squishy 🗜️

A smart utility to concatenate all source files in a Rust project into a single file for easy sharing and analysis, with comprehensive tracking and intelligent filtering.

## What is Squishy?

Squishy recursively scans your entire Rust project directory, intelligently filters relevant files (`.rs` and `.toml`), and combines them into a single output file with clear path headers. It provides detailed feedback about what was processed, making it perfect for sharing complete project context or getting a comprehensive overview of your codebase.

## Installation

```bash
cargo install --path .
```

## Usage

Navigate to any Rust project directory and run:

```bash
squishy
```

Squishy will automatically:
- Scan your entire project directory
- Filter for Rust source files (`.rs`) and configuration files (`.toml`)
- Exclude irrelevant directories (like `target/`)
- Generate output with comprehensive processing summary
- Create output in the best available location (`target/squishy.txt`, `squishy.txt`, or `/tmp/squishy.txt`)

### Example Console Output

```text
Squishy v0.1.6 (built: 1752506856)
 _____ _____ _____ _____ _____ _____ __ __
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|
         |__|

Starting file detection
Squishy file: target/squishy.txt
Scanned 7 of 162 files
Processed 7 of 7 (9030 total bytes)
  ✓ ./Cargo.toml (110 bytes)
  ✓ ./src/squish.rs (2393 bytes)
  ✓ ./src/types.rs (3185 bytes)
  ✓ ./src/scanner.rs (1767 bytes)
  ✓ ./src/main.rs (786 bytes)
  ✓ ./src/errors.rs (511 bytes)
  ✓ ./build.rs (278 bytes)
Complete!
```

### Example Output File Format

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

## Current Features

- ✅ **Smart project scanning**: Recursively scans entire project directory
- ✅ **Intelligent file filtering**: Includes `.rs` and `.toml` files, excludes irrelevant directories
- ✅ **Comprehensive feedback**: Shows file counts, sizes, and processing status
- ✅ **Success/failure tracking**: Visual indicators for each file with error reporting
- ✅ **Multiple output locations**: Automatically chooses best location (`target/`, `.`, or `/tmp/`)
- ✅ **Graceful error handling**: Continues processing even when individual files fail
- ✅ **Clean file headers**: Clear path separators for easy navigation
- ✅ **Project-wide context**: Includes configuration files for complete project understanding

## Potential Improvements

### Command Line Interface
- [ ] Custom source directory specification
- [ ] Configurable output filename
- [ ] Dry-run mode to preview files without generating output
- [ ] Verbose/quiet output modes
- [ ] File type inclusion/exclusion options

### File Handling
- [ ] `.gitignore` integration for even smarter filtering
- [ ] Support for additional file types (`.md`, etc.)
- [ ] Custom include/exclude patterns
- [ ] Configurable header formats
- [ ] Workspace support for multi-crate projects

### User Experience
- [ ] Tree view of processed files
- [ ] Configuration file support (`.squishyrc`)
- [ ] Template support for different output formats
- [ ] Integration with Cargo as a subcommand

### Code Quality
- [ ] More specific error types and messaging
- [ ] Comprehensive test suite
- [ ] Performance optimizations for large codebases
- [ ] Better handling of edge cases (symlinks, special files, etc.)

## Use Cases

- **AI/LLM Analysis**: Provide complete codebase context for analysis tools like Claude or ChatGPT
- **Code Review**: Share entire projects in a single, easily readable file
- **Documentation**: Generate comprehensive code listings with project structure
- **Learning**: Study project organization, dependencies, and file relationships
- **Backup**: Create flat file representations of project structure
- **Debugging**: Get complete project overview for troubleshooting

## Smart Filtering

Squishy intelligently processes your project:

- **Includes**: Rust source files (`.rs`), configuration files (`.toml`), build scripts
- **Excludes**: Build artifacts (`target/`), temporary files, irrelevant directories
- **Handles**: Permission errors, unreadable files, and other edge cases gracefully
- **Reports**: Detailed summary of what was scanned, processed, and any failures

## Requirements

- Rust project (any structure - doesn't require specific directory layout)
- Write permissions in project directory, `target/` directory, or `/tmp/`

## Error Handling

Squishy follows a "fail-safe" approach:
- Individual file failures don't stop the entire process
- Clear error reporting for any issues encountered
- Comprehensive summary shows exactly what succeeded and what failed
- Always produces output even if some files can't be processed

## License

MIT

## Contributing

This is a learning project focused on practical Rust development, but suggestions are appreciated and contributions might be considered on a case by case basis.

---

*Perfect for sharing your Rust projects with AI assistants, code reviewers, or anyone who needs to understand your complete codebase structure.*
