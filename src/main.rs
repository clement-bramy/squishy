use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Instant;

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

pub static TRACE_TIMING_ENABLED: OnceLock<bool> = OnceLock::new();

#[macro_export]
macro_rules! timing {
    ($($arg:tt)*) => {
        if *$crate::TRACE_TIMING_ENABLED.get_or_init(|| false) {
            eprintln!("TIMING: {}", format!($($arg)*));
        }
    };
}

fn main() {
    let args = cli::Cli::parse();
    let _ = TRACE_TIMING_ENABLED.set(args.enable_tracing);

    if let Err(error) = run(args) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(args: cli::Cli) -> Result<()> {
    banner(!args.no_banner);

    let scan_start = Instant::now();
    let root = PathBuf::from(".");
    let mut result = scan(&root)?;
    // timing!(format!("Scan duration: {:?}", scan_start.elapsed()));
    timing!("Scan: {:?}", scan_start.elapsed());

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

    if !args.no_summary {
        result.summary();
    }
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
