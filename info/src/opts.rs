use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Opts {
    #[clap(short = 'i', long)]
    /// Dump IFF chunks loaded from the file
    pub dump_iff: bool,

    #[clap(short = 'p', long)]
    /// Dump L6T model loaded from the file
    pub dump_patch: bool,

    /// File to print out the info for
    pub file: PathBuf
}