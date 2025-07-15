use std::fs;
use std::path::Path;

use crate::errors::{Result, SquishError};
use crate::types::SquishResult;

pub fn scan(dir: &Path) -> Result<SquishResult> {
    let mut result = SquishResult::new();

    for dir in fs::read_dir(dir)
        .map_err(|source| SquishError::ScanError {
            message: format!("Failed to read directory: {source}"),
        })?
        .flatten()
    {
        match dir.file_type() {
            Err(_) => result.failure(&dir.path(), "Cannot read"),
            Ok(filetype) => match (filetype.is_file(), filetype.is_dir()) {
                (false, true) => {
                    // directory
                    if !is_ignored_dir(&dir.path()) {
                        if let Ok(other) = scan(&dir.path()) {
                            result.extend(other);
                        }
                    } else {
                        result.ignored();
                    }
                }
                (true, false) => {
                    if is_rust_adjacent(&dir.path()) {
                        result.scanned(&dir.path());
                    } else {
                        result.ignored();
                    }
                }
                // symlink and unknown
                _ => result.ignored(),
            },
        }
    }

    Ok(result)
}

fn is_ignored_dir(path: &Path) -> bool {
    let ignored = ["target".to_string(), ".git".to_string()];
    path.file_name()
        .map(|dirname| ignored.contains(&dirname.to_string_lossy().to_lowercase()))
        .unwrap_or(false)
}

fn is_rust_adjacent(path: &Path) -> bool {
    let extensions = ["rs".to_string(), "toml".to_string()];
    path.extension()
        .is_some_and(|ex| extensions.contains(&ex.to_string_lossy().to_lowercase()))
}
