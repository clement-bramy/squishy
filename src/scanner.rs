use std::{fs, path::PathBuf};

use crate::errors::{Result, SquishError};

pub fn scan(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for curr in fs::read_dir(dir)
        .map_err(|source| SquishError::ScanError {
            message: format!("Failed to read directory: {source}"),
        })?
        .flatten()
    {
        if curr.file_type().is_ok_and(|ft| ft.is_dir()) {
            match scan(curr.path()) {
                Ok(mut more) => files.append(&mut more),
                Err(error) => eprintln!("Failed to scan [{curr:?}]: {error}"),
            }
        } else if curr.file_name().to_string_lossy().ends_with(".rs") {
            files.push(curr.path());
        }
    }

    Ok(files)
}
