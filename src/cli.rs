use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[arg(help = "Directory to scan [default: .]")]
    pub source: Option<PathBuf>,

    #[arg(short, long, help = "Output file [default: squishy.txt]")]
    pub out: Option<PathBuf>,

    #[arg(long, help = "Disables banner")]
    pub no_banner: bool,

    #[arg(long, help = "Disables report")]
    pub no_summary: bool,

    #[arg(short, long, help = "Disables stdout")]
    pub quiet: bool,

    #[arg(short, long, help = "Enables performance tracing")]
    pub trace: bool,
}
