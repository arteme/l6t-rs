mod opts;
mod pretty;
mod pretty_model;
mod pretty_iff;

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::OnceLock;
use clap::{CommandFactory, FromArgMatches};

use l6t::iff::Chunk;
use l6t::decoder::{Decoder, DecoderResult};
use l6t::encoder::Encoder;
use l6t::model::L6Patch;
use l6t::symbolic::data::{data_model_by_num, data_model_by_patch, data_model_info_by_id, data_model_keys};
use l6t::symbolic::value::{group_values, read_values, write_values};
use crate::opts::Opts;
use crate::pretty::PrettyPrinter;

fn get_help_text() -> &'static String {
    static STR: OnceLock<String> = OnceLock::new();
    STR.get_or_init(|| {
        let mut s = String::new();

        writeln!(s, "Supported data models (-m):").unwrap();
        for (n, id) in data_model_keys().iter().enumerate() {
            let info = data_model_info_by_id(*id).unwrap();
            writeln!(s, "    [{}] {:#010x} {}", n, id, info.name).unwrap();
        }

        s
    })
}


fn main() -> Result<(), clap::error::Error> {
    let matches = Opts::command()
        .after_help(get_help_text())
        .after_long_help(get_help_text())
        .get_matches();
    let opts = Opts::from_arg_matches(&matches)?;

    let mut v: Vec<u8> = Vec::new();
    File::open(opts.file).unwrap()
        .read_to_end(&mut v).unwrap();

    let chunk = Chunk::from_data(v.as_slice(), None).unwrap();
    if opts.dump_iff {
        PrettyPrinter::println(&chunk).unwrap();
    }

    let patch = Decoder::read(v.as_slice()).unwrap();
    let patch = match patch {
        DecoderResult::Patch(patch) => patch,
        _ => {
            panic!("File decoded successfully, but info dump not supported")
        }
    };
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

    Ok(())
}

