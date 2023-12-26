mod opts;
mod pretty;
mod pretty_model;
mod pretty_iff;

use std::fs::File;
use std::io::Read;
use std::fmt::Write;
use clap::Parser;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use crate::opts::Opts;
use crate::pretty::PrettyPrinter;

fn main() {

    let opts = Opts::parse();

    let mut v: Vec<u8> = Vec::new();
    File::open(opts.file).unwrap()
        .read_to_end(&mut v).unwrap();

    let chunk = Chunk::new(v.as_slice(), None).unwrap();
    if opts.dump_iff {
        PrettyPrinter::println(&chunk).unwrap();
    }

    let patch = Decoder::read(v.as_slice());
    match patch {
        Ok(p) => PrettyPrinter::println(&p).unwrap(),
        Err(e) => println!("ERROR: {}", e)
    }
}



