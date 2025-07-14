use std::{fs, io::Result, path::PathBuf};

pub fn scan(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for curr in fs::read_dir(dir)?.flatten() {
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
