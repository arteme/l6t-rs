pub mod models;

mod podxt;
mod pod2;

use crate::data_model::{Param, ParamType};

fn slot(name: &str) -> Param {
    Param::SlotModel {
        name: name.into()
    }
}
fn slot_enable(name: &str) -> Param {
    Param::SlotEnable {
        name: name.into()
    }
}

fn bool(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Bool
    }
}
fn fixed_int(name: &str, value: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: None
    }
}
fn fixed_int_for_slot_id(name: &str, value: u32, slot_id: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: Some(slot_id)
    }
}

fn int(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Int
    }
}

fn float(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Float
    }
}

fn ignore(id: u32) -> Param {
    Param::IgnoreParam {
        param_id: id,
    }
}
