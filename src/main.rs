use std::{io::Result, path::PathBuf};

use scanner::scan;
use squish::{squish, squish_file};

mod scanner;
mod squish;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

fn main() -> Result<()> {
    banner();
    println!("Starting file detection");
    let files = scan(PathBuf::from("src"))?;

    match squish_file() {
        Err(error) => eprintln!("failed to create squishy file: {error}"),
        Ok(output) => squish(files, output)?,
    }

    println!("Complete!");
    Ok(())
}

fn banner() {
    println!(
        r"
Squishy v{VERSION} (built: {BUILD_TIMESTAMP})
 _____ _____ _____ _____ _____ _____ __ __ 
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|  
         |__|                              
    "
    );
}
