use anyhow::*;
use gtk4::gio;
use gtk4::gio::Cancellable;
use gtk4::prelude::{FileExt, InputStreamExtManual};
use log::warn;
use l6t::decoder::{Decoder, DecoderResult};
use l6t::model::L6Patch;
use l6t::symbolic::data::data_model_by_id;
use l6t::symbolic::value::{read_values, ValueMap};
use crate::file::{Bank, Bundle, File, Patch};

pub fn load_file(file: gio::File) -> Result<File> {
    let (data, _) = file.load_bytes(None::<&Cancellable>)
        .context("Read failed")?;

    let process_patch = |patch: L6Patch| {
        let Some(model) = data_model_by_id(patch.target_device.midi_id) else {
            warn!("Model not found: {:04x?}", patch.target_device.midi_id);
            return Patch { patch, values: ValueMap::new() }
        };
        let (values, errors) = read_values(&patch, &model);

        Patch { patch, values }
    };

    let contents = match Decoder::read(&data)? {
        DecoderResult::Patch(p) => {
            File::Patch(process_patch(p))
        }
        DecoderResult::Bundle(b) => {
            let banks = b.banks.into_iter()
                .map(|b| {
                    let name = b.name;
                    let patches = b.patches.into_iter()
                        .map(process_patch)
                        .collect();
                    Bank { name, patches }
                })
                .collect();
            File::Bundle(Bundle { banks })
        }
    };

    Ok(contents)
}