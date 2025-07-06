use std::{
    fs::{self, File, OpenOptions},
    io::{Result, Write},
    path::PathBuf,
};

fn main() -> Result<()> {
    banner();
    println!("Starting file detection");
    let files = scan(PathBuf::from("src"))?;

    match squish_file() {
        Err(error) => eprintln!("failed to create squishy file: {}", error),
        Ok(output) => squish(files, output)?,
    }

    println!("Complete!");
    Ok(())
}

fn scan(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for dir in fs::read_dir(dir)? {
        if let Ok(curr) = dir {
            if curr.file_type().is_ok_and(|ft| ft.is_dir()) {
                match scan(curr.path()) {
                    Ok(mut more) => files.append(&mut more),
                    Err(error) => eprintln!("Failed to scan [{:?}]: {}", curr, error),
                }
            } else {
                if curr.file_name().to_string_lossy().ends_with(".rs") {
                    files.push(curr.path());
                }
            }
        }
    }

    Ok(files)
}

fn squish_file() -> Result<File> {
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

fn squish(paths: Vec<PathBuf>, mut output: File) -> Result<()> {
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

fn banner() {
    println!(
        r"
 _____ _____ _____ _____ _____ _____ __ __ 
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|  
         |__|                              
    "
    );
}
