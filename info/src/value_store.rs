use std::collections::HashMap;
use std::default;
use std::fmt::{Display, Formatter};
use l6t::model::{L6Patch, Model, ModelParam, Value as L6Value};
use crate::data_model::{DataModel, Param, ParamType, Slot};

#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Int(u32),
    Float(f32),
    String(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => v.fmt(f),
            Value::Int(v) => v.fmt(f),
            Value::Float(v) => v.fmt(f),
            Value::String(v) => v.fmt(f)
        }
    }
}

impl TryInto<u32> for &Value {
    type Error = String;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self {
            Value::Bool(v) => Ok(if *v { 1 } else { 0 }),
            Value::Int(v) => Ok(*v),
            Value::Float(_) => Err("Value::Float cannot be converted to u32".into()),
            Value::String(_) => Err("Value::String cannot be converted to u32".into())
        }
    }
}

impl TryInto<bool> for &Value {
    type Error = String;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(v) => Ok(*v),
            Value::Int(_) => Err("Value::Int cannot be converted to bool".into()),
            Value::Float(_) => Err("Value::Float cannot be converted to bool".into()),
            Value::String(_) => Err("Value::String cannot be converted to bool".into())
        }
    }
}

impl Value {
    pub fn to_bits(&self) -> Option<u32> {
        match self {
            Value::Bool(v) => Some(if *v { 1 } else { 0 }),
            Value::Int(v) => Some(*v),
            Value::Float(v) => Some(v.to_bits()),
            Value::String(_) => None
        }
    }
}

pub enum ValueType {
    Bool,
    Int,
    Float,
    String
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            ValueType::Bool => "bool",
            ValueType::Int => "int",
            ValueType::Float => "float",
            ValueType::String => "str"
        };
        t.fmt(f)
    }
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Int(_) => ValueType::Int,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String
        }
    }
}

pub type ValueMap = HashMap<String, Value>;

pub fn read_values(patch: &L6Patch, model: &DataModel) -> (ValueMap, Vec<String>) {
    let mut data: HashMap<String, Value> = HashMap::new();
    let mut processed_models = vec![];
    let mut errors = vec![];

    let slots = model.groups.iter().flat_map(|g| &g.slots);
    for slot in slots {
        let patch_model = patch.models.iter()
            .find(|m| model_matches_slot(m, slot));
        let Some(patch_model) = patch_model else {
            continue;
        };

        let mut missing_params = vec![];
        let mut processed_params = vec![];
        for param in &slot.params {
            let (name, value) = match param {
                Param::SlotModel { name } => {
                    (name, Value::Int(patch_model.model_id))
                },
                Param::SlotEnable { name } => {
                    (name, Value::Bool(patch_model.enabled))
                },
                Param::Param { name, param_id, param_type } => {
                    let patch_param = patch_model.params.iter()
                        .find(|p| p.param_id == *param_id);
                    let Some(patch_param) = patch_param else {
                        missing_params.push(*param_id);
                        continue;
                    };
                    processed_params.push(*param_id);
                    let value = value_from_l6(&patch_param.value, param_type, model.floats_as_ints);
                    (name, value)
                }
                Param::FixedParam { name, param_value, slot_id, .. } => {
                    match slot_id {
                        None => {
                            (name, Value::Int(*param_value))
                        }
                        Some(slot_id) if *slot_id == patch_model.slot_id => {
                            (name, Value::Int(*param_value))
                        }
                        _ => { continue; }
                    }
                }
                Param::IgnoreParam { param_id } => {
                    processed_params.push(*param_id);
                    continue;
                }
            };

            data.insert(name.clone(), value);
        }
        let unprocessed_params = patch_model.params.iter()
            .map(|p| p.param_id)
            .filter(|v| !processed_params.contains(v))
            .collect::<Vec<_>>();

        let model = patch_model; // for error reporting below
        processed_models.push(patch_model);
        if !missing_params.is_empty() {
            errors.push(
                format!("Slot {:#04x} model={:#08x} ordinal={} missing params: {}",
                        model.slot_id, model.model_id, model.ordinal,
                        missing_params.iter().map(|id| format!("{:#x}", id))
                            .collect::<Vec<_>>().join(", ")
                )
            )
        }
        if !unprocessed_params.is_empty() {
            errors.push(
                format!("Slot {:#04x} model={:#08x} ordinal={} unprocessed params: {}",
                        model.slot_id, model.model_id, model.ordinal,
                        unprocessed_params.iter().map(|id| format!("{:#x}", id))
                            .collect::<Vec<_>>().join(", ")
                )
            )
        }
    }

    let unprocessed_models = patch.models.iter().filter(|m| !processed_models.contains(m));
    for model in unprocessed_models {
        errors.push(
            format!("Slot {:#04x} model={:#08x} ordinal={} unprocessed",
                    model.slot_id, model.model_id, model.ordinal
            )
        )
    }

    (data, errors)
}

pub fn write_values(values: ValueMap, model: &DataModel) -> L6Patch {
    let floats_as_ints = model.floats_as_ints;
    let mut models: Vec<Model> = vec![];

    let slots = model.groups.iter().flat_map(|g| &g.slots);
    for slot in slots {
        let fixed_param = slot.params.iter().find(|p| match p {
                    Param::FixedParam { name, param_value, param_type, .. } => {
                        let Some(v) = values.get(name) else {
                            return false;
                        };
                        // TODO: dubious conversion to u32
                        if *param_type != ParamType::Int { todo!() }
                        let Ok(v): Result<u32, _> = v.try_into() else {
                            return false;
                        };

                        v == *param_value
                    }
                    _ => false
                });

        let slot_id = slot.fixed_slot.or(
            fixed_param.and_then(|p| match p {
                Param::FixedParam { slot_id, .. } => slot_id.clone(),
                _ => None
            })
        );
        let model = slot.fixed_model.or_else(|| {
           slot.params.iter().find_map(|p| {
               match p {
                   Param::SlotModel { name } => {
                       let Some(v) = values.get(name) else {
                           return None;
                       };
                       let v: Option<u32> = v.try_into().ok();
                       v
                   }
                   _ => None
               }
           })
        });
        let enable = slot.fixed_enable.or_else(|| {
            slot.params.iter().find_map(|p| {
                match p {
                    Param::SlotEnable { name } => {
                        let Some(v) = values.get(name) else {
                            return None;
                        };
                        let v: Option<bool> = v.try_into().ok();
                        v
                    }
                    _ => None
                }
            })
        });

        let found_by_fixed_int = fixed_param.is_some();
        let have_all = slot_id.is_some() && model.is_some() && enable.is_some();
        let slot_found = found_by_fixed_int || have_all;
        if !slot_found {
            continue;
        }
        if !have_all {
            panic!("Missing slot id, model or enable");
        }

        let mut params = vec![];
        for param in &slot.params {
            match param {
                Param::IgnoreParam { param_id } => {
                    // put a "0" for ignored parameters
                    params.push(
                        ModelParam { param_id: *param_id, value: L6Value::Int(0) }
                    )

                }
                Param::Param { name, param_id, param_type } => {
                    let Some(value) = values.get(name) else {
                        panic!("No value {:?} for param {:#x} for slot {:#x}", name, param_id, slot_id.unwrap());
                    };
                    let value = value_to_l6(value, param_type, floats_as_ints);
                    params.push(
                        ModelParam { param_id: *param_id, value }
                    )
                }
                _ => {}
            }
        }
        models.push(
            Model {
                model_id: model.unwrap(),
                slot_id: slot_id.unwrap(),
                enabled: enable.unwrap(),
                ordinal: 0,
                params,
            }
        );
    }

    L6Patch {
        models,
        ..default::Default::default()
    }
}

fn value_from_l6(value: &L6Value, param_type: &ParamType, floats_as_ints: bool) -> Value {
    match param_type {
        ParamType::Int => {
            match value {
                L6Value::Int(v) => { Value::Int(*v) }
                _ => {
                    panic!("Int value expected")
                }
            }
        }
        ParamType::Float => {
            match value {
                L6Value::Int(v) if floats_as_ints => {
                    Value::Float(f32::from_bits(*v))
                }
                // A special case for Int(0): sometimes L6E writes Int(0) for
                // unused float values, so we do a silent conversion to Float(0)
                L6Value::Int(v) if *v == 0 => {
                    Value::Float(0f32)
                }
                L6Value::Float(v) => {
                    Value::Float(*v)
                }
                _ => {
                    panic!("Float value expected, got {:?}", value)
                }
            }
        }
        ParamType::Bool => {
            match value {
                L6Value::Int(v) if *v == 0 => {
                    Value::Bool(false)
                }
                L6Value::Int(v) if *v == 1 => {
                    Value::Bool(true)
                }
                _ => {
                    panic!("Bool value expected")
                }
            }
        }
    }
}

fn value_to_l6(value: &Value, param_type: &ParamType, floats_as_ints: bool) -> L6Value {
    match param_type {
        ParamType::Int => {
            match value {
                Value::Int(v) => { L6Value::Int(*v) }
                _ => {
                    panic!("Int value expected")
                }
            }
        }
        ParamType::Float => {
            match value {
                Value::Float(v) if floats_as_ints => {
                    L6Value::Int(v.to_bits())
                }
                Value::Float(v) => {
                    L6Value::Float(*v)
                }
                _ => {
                    panic!("Float value expected")
                }
            }
        }
        ParamType::Bool => {
            match value {
                Value::Bool(v) if !v => {
                    L6Value::Int(0)
                }
                Value::Bool(v) if *v => {
                    L6Value::Int(1)
                }
                _ => {
                    panic!("Bool value expected")
                }
            }
        }
    }
}

//

pub struct ValueGroup {
    pub name: String,
    pub values: Vec<(String, Value)>
}

pub fn group_values(patch: &L6Patch, values: &ValueMap, model: &DataModel) -> Vec<ValueGroup> {
    let mut groups: Vec<ValueGroup> = vec![];

    for group in &model.groups {
        let mut group_values = vec![];
        let mut seen_names = vec![];

        for slot in &group.slots {
            let patch_model = patch.models.iter()
                .find(|m| model_matches_slot(m, slot));
            if patch_model.is_none() {
                continue;
            }

            for param in &slot.params {
                let name = match param {
                    Param::SlotModel { name, .. } => name,
                    Param::SlotEnable { name, .. } => name,
                    Param::Param { name, .. } => name,
                    Param::FixedParam { name, .. } => name,
                    Param::IgnoreParam { .. } => { continue }
                };
                if seen_names.contains(name) { continue }
                // We allow values to contain only a portion of props defined in
                // the slot. "read_values" would have reported missing props errors,
                // but the app may have chosen to ignore them.
                if let Some(value) = values.get(name) {
                    group_values.push((name.clone(), value.clone()));
                }
                seen_names.push(name.clone());
            }
        }

        if !group_values.is_empty() {
            let group = ValueGroup {
                name: group.name.clone(),
                values: group_values
            };
            groups.push(group)
        }
    }

    groups
}

fn model_matches_slot(model: &Model, slot: &Slot) -> bool {
    let mut possible_slot_ids = slot.params.iter().flat_map(|p| match p {
        Param::FixedParam { slot_id, .. } => slot_id.clone(),
        _ => None
    });

    let slot_matched = slot.fixed_slot.map(|v| model.slot_id == v)
        .unwrap_or_else(|| possible_slot_ids.find(|v| model.slot_id == *v).is_some());
    let model_matched = slot.fixed_model.map_or(true, |v| model.model_id == v);
    let enable_matched = slot.fixed_enable.map_or(true, |v| model.enabled == v);

    slot_matched && model_matched && enable_matched
}