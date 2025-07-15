use std::path::PathBuf;

use clap::Parser;
use scanner::scan;
use squish::squish;

use crate::errors::Result;
use crate::filesystem::{FileCreator, FileDescriptor};

mod cli;
mod errors;
mod filesystem;
mod scanner;
mod squish;
mod types;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::Cli::parse();
    banner(!args.no_banner);

    println!("Starting file detection");
    let root = PathBuf::from(".");
    let mut result = scan(&root)?;

    let target = root.join("target");
    let filename = args.output.unwrap_or("squishy.txt".to_string());

    let creator = match args.outdir {
        Some(outdir) => FileCreator::new(&filename, &outdir).allow_create_dirs(true),
        None => FileCreator::new(&filename, &target)
            .with_fallback(&root)
            .with_fallback(&PathBuf::from("/tmp"))
            .allow_create_dirs(false),
    };

    let FileDescriptor { mut file, path } = creator.allow_truncate_file(true).create()?;
    squish(&mut result, &mut file, &path)?;

    result.summary();
    println!("Complete!");
    Ok(())
}

fn banner(enabled: bool) {
    if enabled {
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
}
