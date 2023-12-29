use std::collections::HashMap;
use once_cell::sync::Lazy;
use l6t::model::L6Patch;

use crate::data::POD2_DATA_MODEL;
use crate::data::podxt::PODXT_DATA_MODEL;
use crate::data_model::DataModel;

pub struct DataModelInfo {
    name: &'static str,
    model: &'static DataModel
}

pub static DATA_MODELS: Lazy<HashMap<u32, DataModelInfo>> = Lazy::new(|| {
    HashMap::from([
        (0x000200, DataModelInfo {
            name: "POD 2.0 / POD Pro model",
            model: &POD2_DATA_MODEL
        }),
        (0x030002, DataModelInfo {
            name: "PODxt data model",
            model: &PODXT_DATA_MODEL
        })
    ])
});

pub static DATA_MODEL_KEYS: Lazy<Vec<u32>> = Lazy::new(|| {
    let mut keys = DATA_MODELS.keys().cloned().collect::<Vec<_>>();
    keys.sort();

    keys
});

pub fn data_model_by_id(id: u32) -> Option<&'static DataModel> {
    DATA_MODELS
        .get(&id)
        .map(|i| i.model)
}

pub fn data_model_by_num(num: usize) -> Option<&'static DataModel> {
    DATA_MODEL_KEYS.get(num)
        .and_then(|key| data_model_by_id(*key))
}

pub fn data_model_by_patch(patch: &L6Patch) -> Option<&DataModel> {
    data_model_by_id(patch.target_device.midi_id)
}