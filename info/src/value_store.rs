use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use l6t::model::{L6Patch, ModelParam, Value as L6Value};
use crate::data_model::{DataModel, Param, ParamType};

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

pub fn read_values(patch: &L6Patch, model: &DataModel) -> ValueMap {
    let mut data: HashMap<String, Value> = HashMap::new();

    let slots = model.groups.iter().flat_map(|g| &g.slots);
    for slot in slots {
        let patch_model = patch.models.iter().find(|m| {
            let slot_matched = m.slot_id == slot.slot_id;
            let model_matched = slot.fixed_model.map_or(true, |v| m.model_id == v);
            let enable_matched = slot.fixed_enable.map_or(true, |v| m.enabled == v);

            slot_matched && model_matched && enable_matched
        });
        let Some(patch_model) = patch_model else {
            continue;
        };

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
                        continue;
                    };
                    let value = param_to_value(patch_param, param_type, model.floats_as_ints);
                    (name, value)
                }
                Param::FixedParam { name, param_value, param_type } => {
                    (name, Value::Int(*param_value))
                }
                Param::IgnoreParam { .. } => { continue; }
            };

            data.insert(name.clone(), value);
        }
    }

    data
}

fn param_to_value(param: &ModelParam, param_type: &ParamType, floats_as_ints: bool) -> Value {
    match param_type {
        ParamType::Int => {
            match param.value {
                L6Value::Int(v) => { Value::Int(v) }
                _ => {
                    panic!("Int value expected")
                }
            }
        }
        ParamType::Float => {
            match param.value {
                L6Value::Int(v) if floats_as_ints => {
                    Value::Float(f32::from_bits(v))
                }
                L6Value::Float(v) => {
                    Value::Float(v)
                }
                _ => {
                    panic!("Float value expected")
                }
            }
        }
        ParamType::Bool => {
            match param.value {
                L6Value::Int(v) if v == 0 => {
                    Value::Bool(false)
                }
                L6Value::Int(v) if v == 1 => {
                    Value::Bool(true)
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

        for slot in &group.slots {
            let patch_model = patch.models.iter().find(|m| {
                let slot_matched = m.slot_id == slot.slot_id;
                let model_matched = slot.fixed_model.map_or(true, |v| m.model_id == v);
                let enable_matched = slot.fixed_enable.map_or(true, |v| m.enabled == v);

                slot_matched && model_matched && enable_matched
            });
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
                group_values.push((name.clone(), values[name].clone()));
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
