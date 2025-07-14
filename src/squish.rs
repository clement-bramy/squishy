use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::errors::{Result, SquishError};

pub fn squish(paths: &[PathBuf], mut output: File) -> Result<()> {
    for path in paths {
        if let Ok(content) = fs::read_to_string(path) {
            let header = format!(
                "// ─── {} ───────────────────────────────────────────",
                path.display()
            );

            if write!(output, "{header}\n{content}\n\n\n").is_err() {
                eprintln!("Failed to squish: {}", path.display());
                continue;
            }

            if let Err(error) = output.flush() {
                eprintln!("failed to flush output buffer: {error}");
                continue;
            }
        } else {
            eprintln!("Failed to read file: {}", path.display());
        }
    }

    output
        .flush()
        .map_err(|source| SquishError::SquishingError {
            message: format!("Failed to squish file: {source}"),
        })
}

pub fn squish_file() -> Result<File> {
    ["target", ".", "/tmp"]
        .iter()
        .map(PathBuf::from)
        .find(can_write_to)
        .map(|output| output.join("squishy.txt"))
        .ok_or_else(|| SquishError::SquishingError {
            message: "Could not find suitable output directory".to_string(),
        })
        .map(open_output_file)?
}

fn can_write_to(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir() && !metadata.permissions().readonly())
        .unwrap_or(false)
}

fn open_output_file(path: PathBuf) -> Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|source| SquishError::SquishingError {
            message: format!("Unable to open output file: {source}"),
        })
}
