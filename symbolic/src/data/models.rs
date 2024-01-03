use std::collections::HashMap;
use std::sync::OnceLock;
use file::model::L6Patch;

use crate::data::pod2::*;
use crate::data::podxt::*;
use crate::model::{DataModel, Group, Param, Slot};

pub struct DataModelInfo {
    pub name: &'static str,
    pub model: &'static DataModel
}

fn data_models() -> &'static HashMap<u32, DataModelInfo> {
    static MODELS: OnceLock<HashMap<u32, DataModelInfo>> = OnceLock::new();
    MODELS.get_or_init(||
        HashMap::from([
            (0x000200, DataModelInfo {
                name: "POD 2.0 / POD Pro model",
                model: pod2_data_model(),
            }),
            (0x030002, DataModelInfo {
                name: "PODxt data model",
                model: podxt_data_model(),
            }),
            (0x030005, DataModelInfo {
                name: "PODxt Pro data model",
                model: podxt_pro_data_model(),
            }),
            (0x03000a, DataModelInfo {
                name: "PODxt Live data model",
                model: podxt_live_data_model()
            })
        ])
    )
}

pub fn data_model_keys() -> &'static Vec<u32> {
    static KEYS: OnceLock<Vec<u32>> = OnceLock::new();
    KEYS.get_or_init(|| {
        let mut keys = data_models().keys().cloned().collect::<Vec<_>>();
        keys.sort();

        keys
    })
}

pub fn data_model_info_by_id(id: u32) -> Option<&'static DataModelInfo> {
    data_models()
        .get(&id)
}

pub fn data_model_by_id(id: u32) -> Option<&'static DataModel> {
    data_models()
        .get(&id)
        .map(|i| i.model)
}

pub fn data_model_by_num(num: usize) -> Option<&'static DataModel> {
    data_model_keys()
        .get(num)
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
