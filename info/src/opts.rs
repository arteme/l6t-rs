use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Opts {
    #[clap(short = 'i', long)]
    /// Dump IFF chunks loaded from the files
    pub dump_iff: bool,

    /// File to print out the info for
    pub file: PathBuf
}