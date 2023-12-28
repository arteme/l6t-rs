mod opts;
mod data_model;
mod data;
mod pretty;
mod pretty_model;
mod pretty_iff;
mod value_store;

use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;
use clap::Parser;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use l6t::encoder::Encoder;
use l6t::model::{L6Patch, Model};
use crate::data::POD2_DATA_MODEL;
use crate::opts::Opts;
use crate::pretty::PrettyPrinter;
use crate::value_store::{group_values, read_values_full, write_values};

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

    let errors = Rc::new(RefCell::new(vec![]));
    let missing_prop_cb = |model: &Model, missing_props: &Vec<u32>| {
        let mut errors = errors.borrow_mut();
        errors.push(
            format!("Slot {:#04x} model={:#08x} ordinal={} missing params: {}",
                    model.slot_id, model.model_id, model.ordinal,
                    missing_props.iter().map(|id| format!("{:#x}", id))
                        .collect::<Vec<_>>().join(", ")
            )
        )
    };
    let unprocessed_cb = |model: &Model| {
        let mut errors = errors.borrow_mut();
        errors.push(
            format!("Slot {:#04x} model={:#08x} ordinal={} unprocessed",
                    model.slot_id, model.model_id, model.ordinal
            )
        )
    };

    let values = read_values_full(&patch, &POD2_DATA_MODEL,
                                  missing_prop_cb, unprocessed_cb);
    let groups = group_values(&patch, &values, &POD2_DATA_MODEL);

    let sep = std::iter::repeat('-').take(65).collect::<String>();
    for group in &groups {
        println!("{}\n{}", group.name, sep);

        for (name, value) in &group.values {
            println!("{:30} : {:5} : {}", name, value.get_type(), value);
        }
        println!();
    }

    let errors = errors.borrow();
    if !errors.is_empty() {
        println!("ERRORS\n{}", sep);
        for error in errors.iter() {
            println!("{}", error);
        }
    }

    if let Some(write_filename) = opts.write {
        let patch = if opts.encode {
            let p = write_values(values, &POD2_DATA_MODEL);
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

