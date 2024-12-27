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

    #[clap(short = 's', long)]
    /// Print simple (underlying) values together with the decoded rich values
    pub dump_simple: bool,

    #[clap(short = 'e', long)]
    /// Re-encode the values before writing
    pub encode: bool,

    #[clap(short = 'w', long)]
    /// File to write L&T patch to
    pub write: Option<PathBuf>,

    #[clap(short = 'm', long)]
    /// Data model number
    pub model: Option<usize>,


    /// File to print out the info for
    pub file: PathBuf
}