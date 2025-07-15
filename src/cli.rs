use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[arg(long, help = "Disables the cool banner :(")]
    pub no_banner: bool,

    #[arg(long, help = "Output directory [default: target, current, /tmp]")]
    pub outdir: Option<PathBuf>,

    #[arg(short, long, help = "Output filename [default: squishy.txt]")]
    pub output: Option<String>,
}

