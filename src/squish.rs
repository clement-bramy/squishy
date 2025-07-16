use std::time::{Duration, Instant};
use std::{
    fs::{self, File},
    io::Write,
};

use crate::errors::Result;
use crate::timing;
use crate::types::SquishResult;

pub fn squish(result: &mut SquishResult, output: &mut File) -> Result<()> {
    let mut read_failures = Vec::new();
    let mut write_failures = Vec::new();
    let mut success = Vec::new();

    let mut total_read = Duration::default();
    let mut total_write = Duration::default();

    for scanned in result.iter_scanned() {
        let read_start = Instant::now();
        let read_result = fs::read_to_string(&scanned.path);
        total_read += read_start.elapsed();

        match read_result {
            Err(_) => read_failures.push(scanned.path.clone()),
            Ok(content) => {
                let write_start = Instant::now();
                let write_result = write!(
                    output,
                    "// ─── {} ───────────────────────────────────────────\n{content}\n\n\n",
                    scanned.path.display()
                );
                total_write += write_start.elapsed();

                match write_result.and_then(|()| output.flush()) {
                    Err(_) => write_failures.push(scanned.path.clone()),
                    Ok(_) => success.push((scanned.path.clone(), content.len())),
                }
            }
        }
    }

    timing!("Read: {total_read:?}");
    timing!("Write: {total_write:?}");

    let stats_start = Instant::now();
    read_failures
        .iter()
        .for_each(|path| result.failure(path, "Read failure"));

    write_failures
        .iter()
        .for_each(|path| result.failure(path, "Write failure"));

    success
        .iter()
        .for_each(|(path, size)| result.success(path, *size as u64));

    timing!("Stats: {:?}", stats_start.elapsed());

    Ok(())
}
