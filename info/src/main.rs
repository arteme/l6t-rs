mod opts;
mod data_model;
mod data;
mod pretty;
mod pretty_model;
mod pretty_iff;
mod value_store;

use std::fs::File;
use std::io::Read;
use std::fmt::Write;
use clap::Parser;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use crate::data::POD2_DATA_MODEL;
use crate::opts::Opts;
use crate::pretty::PrettyPrinter;
use crate::value_store::{group_values, read_values};

fn main() {
    let opts = Opts::parse();

    let mut v: Vec<u8> = Vec::new();
    File::open(opts.file).unwrap()
        .read_to_end(&mut v).unwrap();

    let chunk = Chunk::new(v.as_slice(), None).unwrap();
    if opts.dump_iff {
        PrettyPrinter::println(&chunk).unwrap();
    }

    let patch = Decoder::read(v.as_slice()).unwrap();
    if opts.dump_patch {
        PrettyPrinter::println(&patch).unwrap();
    }

    let values = read_values(&patch, &POD2_DATA_MODEL);
    let groups = group_values(&patch, &values, &POD2_DATA_MODEL);

    for group in &groups {
        println!("{}", group.name);
        println!("{}", std::iter::repeat('-').take(65).collect::<String>());

        for (name, value) in &group.values {
            println!("{:30} : {:5} : {}", name, value.get_type(), value);
        }
        println!();
    }
}

