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
- Exclude irrelevant directories (like `target/` and `.git/`)
- Generate output with comprehensive processing summary
- Create output in the best available location (`target/squishy.txt`, `squishy.txt`, or `/tmp/squishy.txt`)

### Command Line Options

```bash
# Basic usage
squishy

# Custom output filename
squishy --output my-project.txt

# Custom output directory
squishy --outdir /path/to/custom/dir

# Custom directory with filename
squishy --outdir /path/to/dir --output project-snapshot.txt

# Disable banner (useful for scripting)
squishy --no-banner

# View help
squishy --help
```

### Example Console Output

```text
Squishy v0.1.8 (built: 1752554103)
 _____ _____ _____ _____ _____ _____ __ __
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|
         |__|

Starting file detection
Squishy file: ./target/squishy.txt
Scanned 9 of 17 files
Processed 9 of 9 (15914 total bytes)
  ✓ ./Cargo.toml (242 bytes)
  ✓ ./src/squish.rs (1441 bytes)
  ✓ ./src/types.rs (3143 bytes)
  ✓ ./src/filesystem.rs (6382 bytes)
  ✓ ./src/cli.rs (422 bytes)
  ✓ ./src/scanner.rs (1769 bytes)
  ✓ ./src/main.rs (1641 bytes)
  ✓ ./src/errors.rs (596 bytes)
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

- ✅ **CLI interface**: Command line argument parsing with help, version, and basic options
- ✅ **Output control**: Custom output directory and filename specification
- ✅ **Project scanning**: Recursively scans entire project directory
- ✅ **File filtering**: Includes `.rs` and `.toml` files, excludes `target/`, `.git/`
- ✅ **Processing feedback**: Shows file counts, sizes, and processing status
- ✅ **Error tracking**: Visual indicators for each file with error reporting
- ✅ **Output placement**: Automatic fallback locations with user override capability
- ✅ **Error handling**: Continues processing when individual files fail, with clear error messages
- ✅ **File headers**: Clear path separators for easy navigation
- ✅ **Project context**: Includes configuration files for complete project understanding
- ✅ **Banner control**: Option to disable banner for scripting use

## Output Location Behavior

### Default Behavior
When no custom directory is specified, Squishy uses an intelligent fallback chain:
1. `target/squishy.txt` (if `target/` directory exists and is writable)
2. `./squishy.txt` (current directory as fallback)
3. `/tmp/squishy.txt` (system temp directory as last resort)

### Custom Directory Behavior
When you specify `--outdir`, Squishy:
- Uses only the specified directory (no fallbacks)
- Creates the directory if it doesn't exist (when possible)
- Fails clearly if the custom location cannot be used

This ensures that explicit user choices are respected while providing helpful defaults for casual usage.

## Planned Improvements

### File Handling Enhancements
- [ ] `.gitignore` integration for even smarter filtering
- [ ] Support for additional file types (`.md`, etc.)
- [ ] Custom include/exclude patterns
- [ ] Configurable header formats
- [ ] Workspace support for multi-crate projects

### Advanced Features
- [ ] Configuration file support (`.squishyrc` - global and project-specific)
- [ ] Dry-run mode to preview files without generating output
- [ ] Verbose/quiet output modes
- [ ] Template support for different output formats
- [ ] Integration with Cargo as a subcommand

### User Experience
- [ ] Tree view of processed files
- [ ] Progress indication for large projects
- [ ] Summary statistics improvements
- [ ] Better handling of edge cases (symlinks, special files, etc.)

## Use Cases

- **AI/LLM Analysis**: Provide complete codebase context for analysis tools like Claude or ChatGPT
- **Code Review**: Share entire projects in a single, easily readable file
- **Documentation**: Generate comprehensive code listings with project structure
- **Learning**: Study project organization, dependencies, and file relationships
- **Backup**: Create flat file representations of project structure
- **Debugging**: Get complete project overview for troubleshooting

## Smart Filtering

Squishy processes your project by:

- **Including**: Rust source files (`.rs`), configuration files (`.toml`), build scripts
- **Excluding**: Build artifacts (`target/`), version control (`.git/`)
- **Handling**: Permission errors, unreadable files, and other edge cases
- **Reporting**: Summary of what was scanned, processed, and any failures

## Requirements

- Rust project (any structure - doesn't require specific directory layout)
- Write permissions in target directory, current directory, or `/tmp/` (for default behavior)
- Write permissions in specified directory (for custom `--outdir` usage)

## Error Handling

Squishy follows a "continue on error" approach:
- Individual file failures don't stop the entire process
- Clear error messages for issues encountered
- Summary shows what succeeded and what failed
- Always produces output even if some files can't be processed
- Clear failures for invalid custom directories (no silent fallbacks)

## License

MIT

## Contributing

This is a learning project focused on practical Rust development, but suggestions are appreciated and contributions might be considered on a case by case basis.

---

*Perfect for sharing your Rust projects with AI assistants, code reviewers, or anyone who needs to understand your complete codebase structure.*
