mod opts;
mod data_model;
mod data;
mod pretty;
mod pretty_model;
mod pretty_iff;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::fmt::Write;
use clap::Parser;

use l6t::iff::Chunk;
use l6t::decoder::Decoder;
use l6t::model::L6Patch;
use crate::data::MODEL_DATA;
use crate::data_model::{Param, Slot};
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

    let patch = Decoder::read(v.as_slice()).unwrap();
    if opts.dump_patch {
        PrettyPrinter::println(&patch).unwrap();
    }

    let m = patch_to_model(&patch, &MODEL_DATA);
    for (k, v) in m.iter() {
        println!("{}: {}", k, v);

    }
}

fn patch_to_model(patch: &L6Patch, model: &[Slot]) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    let mut not_found_slots: Vec<&Slot> = vec![];

    for slot in model {
        let patch_model = patch.models.iter().find(|m| {
            let slot_matched = m.slot_id == slot.slot_id;
            let model_matched = slot.fixed_model.map_or(true, |v| m.model_id == v);
            let enable_matched = slot.fixed_enable.map_or(true, |v| m.enabled == v);

            slot_matched && model_matched && enable_matched
        });
        let Some(patch_model) = patch_model else {
            not_found_slots.push(slot);
            continue;
        };

        for param in &slot.params {
            match param {
                Param::SlotModel { name } => {
                    data.insert(name.clone(), format!("{}", patch_model.model_id));
                }
                Param::SlotEnable { name } => {
                    data.insert(name.clone(), format!("{}", patch_model.enabled));
                }
                Param::Param { name, param_id, param_type } => {
                    let patch_param = patch_model.params.iter()
                        .find(|p| p.param_id == *param_id);
                    let Some(patch_param) = patch_param else {
                        continue;
                    };
                    data.insert(name.clone(), format!("{}", patch_param.value));
                }
                Param::FixedParam { name, param_value, param_type } => {
                    data.insert(name.clone(), format!("{}", param_value));
                }
                Param::IgnoreParam { .. } => {}
            }
        }


    }

    data
}