# Squishy 🗜️

A simple utility to concatenate all Rust source files in a project into a single file for easy sharing and analysis.

## What is Squishy?

Squishy recursively scans your Rust project's `src/` directory, finds all `.rs` files, and combines them into a single output file with clear path headers. This makes it easy to share entire codebases in a single file or get a comprehensive overview of your project structure.

## Installation

```bash
cargo install --path .
```

## Usage

Navigate to any Rust project directory and run:

```bash
squishy
```

This will create a `squishy.txt` file (or `target/squishy.txt` if a `target/` directory exists) containing all your Rust source files with clear separators.

### Example Output Format

```rust
// ─── src/main.rs ───────────────────────────────────────────
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


// ─── src/lib.rs ───────────────────────────────────────────
pub mod utils;

pub fn helper_function() {
    // implementation
}


// ─── src/utils/mod.rs ───────────────────────────────────────────
pub fn utility_function() {
    // implementation
}
```

## Current Features

- ✅ Recursively scans `src/` directory
- ✅ Filters for `.rs` files only
- ✅ Generates clean path headers for each file
- ✅ Handles directory traversal errors gracefully
- ✅ Smart output location (uses `target/` directory when available)
- ✅ Preserves file content exactly as written

## Planned Improvements

### Command Line Interface
- [ ] Custom source directory specification
- [ ] Configurable output filename
- [ ] Dry-run mode to preview files without generating output
- [ ] Verbose/quiet output modes

### File Handling
- [ ] `.gitignore` integration for intelligent file filtering
- [ ] Support for additional file types (`.toml`, `.md`, etc.)
- [ ] Include/exclude patterns for specific files or directories
- [ ] Configurable header formats

### User Experience
- [ ] Progress indication for large projects
- [ ] Summary statistics (file count, total size, etc.)
- [ ] Configuration file support
- [ ] Template support for different output formats

### Code Quality
- [ ] More specific error types and handling
- [ ] Comprehensive test suite
- [ ] Performance optimizations for large codebases
- [ ] Better handling of edge cases (symlinks, special files, etc.)

## Use Cases

- **Code Review**: Share entire projects in a single file
- **AI/LLM Analysis**: Provide complete codebase context for analysis tools
- **Documentation**: Generate comprehensive code listings
- **Backup**: Create flat file representations of project structure
- **Learning**: Study project organization and file relationships

## Requirements

- Rust project with standard `src/` directory structure
- Write permissions in the project directory (or `target/` directory)

## License

MIT

## Contributing

This is a learning project, but contributions and suggestions are welcome! Feel free to open issues or submit pull requests.
