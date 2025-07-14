use std::path::PathBuf;

use scanner::scan;
use squish::squish;

use crate::errors::Result;

mod errors;
mod scanner;
mod squish;
mod types;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");

fn main() -> Result<()> {
    banner();
    println!("Starting file detection");
    let mut result = scan(PathBuf::from("."))?;
    squish(&mut result)?;

    result.summary();
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
