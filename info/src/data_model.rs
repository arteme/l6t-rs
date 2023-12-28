
pub struct DataModel {
    pub floats_as_ints: bool,
    pub groups: Vec<Group>,
}

pub struct Group {
    pub name: String,
    pub slots: Vec<Slot>
}

pub struct Slot {
    pub slot_id: u32,
    pub fixed_model: Option<u32>,
    pub fixed_enable: Option<bool>,
    pub params: Vec<Param>
}

#[derive(Eq, PartialEq)]
pub enum ParamType {
    Int,
    Float,
    Bool
}

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
        param_type: ParamType
    },
    IgnoreParam {
        param_id: u32,
    }
}

