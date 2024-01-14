
pub struct DataModel {
    pub floats_as_ints: bool,
    pub groups: Vec<Group>,
}

#[derive(Clone)]
pub struct Group {
    pub name: String,
    pub slots: Vec<Slot>
}

#[derive(Clone)]
pub struct Slot {
    pub fixed_slot: Option<u32>,
    pub fixed_model: Option<u32>,
    pub fixed_enable: Option<bool>,
    pub params: Vec<Param>
}

#[derive(Clone, Eq, PartialEq)]
pub enum ParamType {
    Int,
    Float,
    Bool
}

#[derive(Clone)]
pub enum Param {
    SlotModel {
        name: String,
    },
    SlotEnable {
        name: String,
    },
    Param {
        name: String,
        param_id: u32,
        param_type: ParamType
    },
    FixedParam {
        name: String,
        param_value: u32,
        param_type: ParamType,
        slot_id: Option<u32>
    },
    IgnoreParam {
        param_id: u32,
        param_type: ParamType,
    }
}

