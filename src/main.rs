use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

use clap::Parser;
use scanner::scan;
use squish::squish;

use crate::errors::{Result, SquishError};

mod cli;
mod errors;
mod scanner;
mod squish;
mod types;

const GIT_HASH: &str = env!("GIT_HASH");
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
    let _ = TRACE_TIMING_ENABLED.set(args.trace);

    if let Err(error) = run(args) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(args: cli::Cli) -> Result<()> {
    let out = args.out.unwrap_or(PathBuf::from("squishy.txt"));
    let source = args.source.unwrap_or(PathBuf::from("."));
    if !args.no_banner && !args.quiet {
        banner(&source, &out);
    }
    let scan_start = Instant::now();
    let mut result = scan(&source)?;
    timing!("Scan: {:?}", scan_start.elapsed());

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out)
        .map_err(|_| SquishError::MissingOutputDirectory { out: out.clone() })?;

    squish(&mut result, &mut file)?;

    if !args.no_summary && !args.quiet {
        result.summary();
    }

    Ok(())
}

fn banner(source: &Path, out: &Path) {
    let source = source
        .canonicalize()
        .unwrap_or_else(|_| source.to_path_buf());

    println!(
        r"
Squishy v{VERSION} (built: {BUILD_TIMESTAMP}, git: {GIT_HASH})
 _____ _____ _____ _____ _____ _____ __ __ 
|   __|     |  |  |     |   __|  |  |  |  |
|__   |  |  |  |  |-   -|__   |     |_   _|
|_____|__  _|_____|_____|_____|__|__| |_|  
         |__|                              
📁 Scanning: {}
📄 Output: {}
    ",
        source.display(),
        out.display()
    );
}
