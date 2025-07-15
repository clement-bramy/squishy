use std::path::Path;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::errors::Result;
use crate::types::SquishResult;

pub fn squish(result: &mut SquishResult, output: &mut File, path: &Path) -> Result<()> {
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

    result.with_output(path);

    Ok(())
}
