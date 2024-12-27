mod opts;
mod pretty;
mod pretty_model;
mod pretty_iff;
mod pretty_info;

use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::OnceLock;
use clap::{CommandFactory, FromArgMatches};

use l6t::iff::Chunk;
use l6t::decoder::{Decoder, DecoderResult};
use l6t::model::L6Patch;
use l6t::symbolic::data::{data_model_by_id, data_model_by_num, data_model_info_by_id, data_model_keys};
use l6t::symbolic::model::DataModel;
use l6t::symbolic::value::read_values;
use l6t::symbolic::group::group_values;
use l6t::symbolic::rich::{enrich_values, RichValueGroup};
use crate::opts::Opts;
use crate::pretty::{Pretty, PrettyPrinter};

pub struct DecodedPatch {
    patch: L6Patch,
    values: Vec<RichValueGroup>,
    errors: Vec<String>
}

pub struct DecodedBank {
    name: String,
    patches: Vec<DecodedPatch>
}

pub struct DecodedBundle {
    is_bundle: bool,
    banks: Vec<DecodedBank>
}

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

fn get_model(patch: &L6Patch, model_num: &Option<usize>) -> &'static DataModel {
    model_num
        .and_then(|num|
            data_model_by_num(num)
                .or_else(|| panic!("Data model not found by number: {}", num))
        )
        .or_else(|| {
            let id = patch.target_device.midi_id;
            data_model_by_id(id)
                .or_else(|| panic!("Data model not found by device id: {:#x}", id))
        })
        .unwrap()

}

fn decoder_result_to_bundle(dr: DecoderResult, model_num: Option<usize>) -> DecodedBundle {
    let patch_to_decoded = |patch: L6Patch| {
        let model = get_model(&patch, &model_num);

        let (values, errors) = read_values(&patch, model);
        let values = enrich_values(values, &model.info_map);
        let values = group_values(&patch, &values, model);

        DecodedPatch { patch, values, errors }
    };


    match dr {
        DecoderResult::Patch(p) => {
            let p = patch_to_decoded(p);
            let bank = DecodedBank { name: "".into(), patches: vec![ p ] };
            DecodedBundle {
                is_bundle: false,
                banks: vec![ bank ]
            }
        }
        DecoderResult::Bundle(b) => {
            let mut banks = vec![];
            for b in b.banks {
                let name = b.name;
                let patches = b.patches.into_iter()
                    .map(patch_to_decoded)
                    .collect();
                banks.push(DecodedBank { name, patches });
            }
            DecodedBundle { is_bundle: true, banks }
        }
    }

}


fn main() -> Result<(), clap::error::Error> {
    let matches = Opts::command()
        .after_help(get_help_text())
        .after_long_help(get_help_text())
        .get_matches();
    let opts = Opts::from_arg_matches(&matches)?;
    let mut pp = PrettyPrinter::with_simple(opts.dump_simple);

    let mut v: Vec<u8> = Vec::new();
    File::open(opts.file).unwrap()
        .read_to_end(&mut v).unwrap();

    if opts.dump_iff {
        let chunk = Chunk::from_data(v.as_slice(), None).unwrap();
        pp.println(&chunk).unwrap();
    }

    let decoded = Decoder::read(v.as_slice()).unwrap();
    if opts.dump_patch {
        let patch: &dyn Pretty = match &decoded {
            DecoderResult::Patch(v) => v,
            DecoderResult::Bundle(v) => v,
        };
        pp.println(patch).unwrap();
    }

    let bundle = decoder_result_to_bundle(decoded, opts.model);
    pp.println(&bundle).unwrap();

/*
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
*/
    Ok(())
}

