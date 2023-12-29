use std::collections::HashMap;
use once_cell::sync::Lazy;
use l6t::model::L6Patch;

use crate::data::pod2::*;
use crate::data::podxt::*;
use crate::data_model::{DataModel, Group, Param, Slot};

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
            model: &PODXT_DATA_MODEL,
        }),
        (0x030005, DataModelInfo {
            name: "PODxt Pro data model",
            model: &PODXT_PRO_DATA_MODEL,
        }),
        (0x03000a, DataModelInfo {
            name: "PODxt Live data model",
            model: &PODXT_LIVE_DATA_MODEL,
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

pub fn filter_params<F>(model: &DataModel, filter_fn: F) -> DataModel
    where F: Fn(&Param) -> Option<Param>
{
    let groups = model.groups.iter().map(|g| {
        let slots = g.slots.iter().map(|s| {
            let params = s.params.iter().map(|p| {
                filter_fn(p).unwrap_or(p.clone())
            });

            Slot {
                fixed_slot: s.fixed_slot,
                fixed_model: s.fixed_model,
                fixed_enable: s.fixed_enable,
                params: params.collect()
            }
        });

        Group {
            name: g.name.clone(),
            slots: slots.collect()
        }
    });

    DataModel {
        floats_as_ints: model.floats_as_ints,
        groups: groups.collect()
    }
}

pub fn filter_params_by_prefix(model: &DataModel,
                               remove_params: &[&str],
                               strip: &[&str]
) -> DataModel {
    let should_remove = |name: &str| remove_params.iter().any(|p| name.starts_with(p));
    let should_strip = |name: &str| strip.iter().any(|p| name.starts_with(p));

    let strip_name = |name: &str| {
        for p in strip {
            if name.starts_with(p) {
                return name.strip_prefix(p).unwrap().to_string();
            }
        }
        name.to_string()
    };


    filter_params(model, |p| match p {
        Param::SlotModel { name } if should_strip(name) => {
            Some(Param::SlotModel { name: strip_name(name) })
        }
        Param::SlotEnable { name } if should_strip(name) => {
            Some(Param::SlotEnable { name: strip_name(name) })
        }
        Param::Param { name, param_id, param_type } if should_strip(name) => {
            Some(Param::Param {
                name: strip_name(name),
                param_id: *param_id,
                param_type: param_type.clone()
            })
        }
        Param::FixedParam { name, param_value, param_type, slot_id } if should_strip(name)  => {
            Some(Param::FixedParam {
                name: strip_name(name),
                param_value: *param_value,
                param_type: param_type.clone(),
                slot_id: slot_id.clone(),
            })
        }

        Param::Param { name, param_id, .. } if should_remove(name) => {
            Some(Param::IgnoreParam { param_id: *param_id })
        }
        Param::FixedParam { name, .. } if should_remove(name)  => {
            panic!("Removing Param::FixedParam not supported!")
        }

        _ => None
    })
}
