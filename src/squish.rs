use std::path::Path;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::errors::{Result, SquishError};
use crate::types::SquishResult;

pub fn squish(result: &mut SquishResult) -> Result<()> {
    let (path, mut output) = squish_file()?;

    let mut read_failures = Vec::new();
    let mut write_failures = Vec::new();
    let mut success = Vec::new();

    for scanned in result.iter_scanned() {
        match fs::read_to_string(&scanned.path) {
            Err(_) => read_failures.push(scanned.path.clone()),
            Ok(content) => {
                match write!(
                    output,
                    "// ─── {} ───────────────────────────────────────────\n{content}\n\n\n",
                    scanned.path.display()
                )
                .and_then(|()| output.flush())
                {
                    Err(_) => write_failures.push(scanned.path.clone()),
                    Ok(_) => success.push((scanned.path.clone(), content.len())),
                }
            }
        }
    }

    read_failures
        .iter()
        .for_each(|path| result.failure(path, "Read failure"));

    write_failures
        .iter()
        .for_each(|path| result.failure(path, "Write failure"));

    success
        .iter()
        .for_each(|(path, size)| result.success(path, *size as u64));

    result.with_output(&path);

    Ok(())
}

fn squish_file() -> Result<(PathBuf, File)> {
    let path = ["target", ".", "/tmp"]
        .iter()
        .map(PathBuf::from)
        .find(|path| can_write_to(&path))
        .map(|output| output.join("squishy.txt"))
        .ok_or_else(|| SquishError::SquishingError {
            message: "Could not find suitable output directory".to_string(),
        })?;

    Ok((path.clone(), open_output_file(&path)?))
}

fn can_write_to(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir() && !metadata.permissions().readonly())
        .unwrap_or(false)
}

fn open_output_file(path: &Path) -> Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|source| SquishError::SquishingError {
            message: format!("Unable to open output file: {source}"),
        })
}
