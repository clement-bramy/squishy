use std::{
    fs::{self, File, OpenOptions},
    io::{Result, Write},
    path::PathBuf,
};

pub fn squish(paths: Vec<PathBuf>, mut output: File) -> Result<()> {
    for path in paths {
        if let Ok(content) = fs::read_to_string(&path) {
            let header = format!(
                "// ─── {} ───────────────────────────────────────────",
                path.display()
            );

            if let Err(error) = writeln!(output, "{}", header) {
                eprintln!("failed to write header: {}", error);
                continue;
            }
            if let Err(error) = writeln!(output, "{}\n\n", content) {
                eprintln!("failed to write content to [{:?}]: {}", path, error);
                continue;
            }
            if let Err(error) = output.flush() {
                eprintln!("failed to flush output buffer: {}", error);
                continue;
            }
        }
    }

    return output.flush();
}

pub fn squish_file() -> Result<File> {
    if PathBuf::from("target").exists() {
        println!("Output will be written to: target/squishy.txt");
        return OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("target/squishy.txt");
    }

    println!("Output will be written to: squishy.txt");
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("squishy.txt")
}
