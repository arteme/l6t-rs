use crate::model::{Param, ParamType};

pub fn slot(name: &str) -> Param {
    Param::SlotModel {
        name: name.into()
    }
}
pub fn slot_enable(name: &str) -> Param {
    Param::SlotEnable {
        name: name.into()
    }
}

pub fn bool(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Bool
    }
}
pub fn fixed_int(name: &str, value: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: None
    }
}
pub fn fixed_int_for_slot_id(name: &str, value: u32, slot_id: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: Some(slot_id)
    }
}

pub fn int(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Int
    }
}
pub fn float(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Float
    }
}

pub fn ignore(id: u32) -> Param {
    Param::IgnoreParam {
        param_id: id,
        param_type: ParamType::Int
    }
}

pub fn ignore_f(id: u32) -> Param {
    Param::IgnoreParam {
        param_id: id,
        param_type: ParamType::Float
    }
}
