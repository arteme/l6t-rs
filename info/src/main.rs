mod opts;
mod data_model;
mod data;
mod pretty;
mod pretty_model;
mod pretty_iff;
mod value_store;

use std::fs::File;
use std::io::{Read, Write};
use clap::Parser;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use l6t::encoder::Encoder;
use l6t::model::L6Patch;
use crate::data::models::{data_model_by_num, data_model_by_patch};
use crate::opts::Opts;
use crate::pretty::PrettyPrinter;
use crate::value_store::{group_values, read_values, write_values};

fn main() {
    let opts = Opts::parse();

    let mut v: Vec<u8> = Vec::new();
    File::open(opts.file).unwrap()
        .read_to_end(&mut v).unwrap();

    let chunk = Chunk::from_data(v.as_slice(), None).unwrap();
    if opts.dump_iff {
        PrettyPrinter::println(&chunk).unwrap();
    }

    let patch = Decoder::read(v.as_slice()).unwrap();
    if opts.dump_patch {
        PrettyPrinter::println(&patch).unwrap();
    }

    let model = opts.model
        .and_then(|num|
            data_model_by_num(num)
                .or_else(|| panic!("Data model not found by number: {}", num))
        )
        .or_else(||
            data_model_by_patch(&patch)
                .or_else(|| panic!("Data model not found by device id: {:#x}", patch.target_device.midi_id))
        )
        .unwrap();

    let (values, errors) = read_values(&patch, model);
    let groups = group_values(&patch, &values, model);

    let sep = std::iter::repeat('-').take(65).collect::<String>();
    for group in &groups {
        println!("{}\n{}", group.name, sep);

        for (name, value) in &group.values {
            println!("{:30} : {:5} : {}", name, value.get_type(), value);
        }
        println!();
    }

    if !errors.is_empty() {
        println!("ERRORS\n{}", sep);
        for error in errors.iter() {
            println!("{}", error);
        }
    }

    if let Some(write_filename) = opts.write {
        let patch = if opts.encode {
            let p = write_values(values, model);
            // for now write_values doesn't do anything target_devices and meta
            // fields, so take them from the original patch
            L6Patch {
                target_device: patch.target_device,
                models: p.models,
                meta: patch.meta,
            }
        } else {
            patch
        };

        let vec = Encoder::write(&patch).unwrap();
        File::create(write_filename).unwrap()
            .write(&vec).unwrap();
    }
}

