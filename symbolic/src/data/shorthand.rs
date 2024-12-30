use std::collections::HashMap;
use crate::model::{Param, ParamType};
use crate::rich::{Conversion, FormattingType, Range, ValueInfo};

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

//

pub fn lookup(map: &'static HashMap<u32, String>) -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::StringLookup(map))
}

pub fn percent() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Percent)
        .range(0.0, 1.0)
}

pub fn millis() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Millis)
        .range(0.0, f32::INFINITY)
}

pub fn millis1() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Millis1)
        .range(0.0, f32::INFINITY)
}

pub fn hz() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Hertz)
        .range(0.0, f32::INFINITY)
}

pub fn db() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Hertz)
}

pub struct ValueInfoBuilder {
    formatting_type: FormattingType,
    range: Option<Range>,
    conversion: Option<Conversion>
}

impl ValueInfoBuilder {
    pub fn new() -> ValueInfoBuilder {
        ValueInfoBuilder {
            formatting_type: FormattingType::Simple,
            range: None,
            conversion: None
        }
    }

    pub fn formatting_type(mut self, formatting_type: FormattingType) -> ValueInfoBuilder {
        self.formatting_type = formatting_type;
        self
    }

    pub fn min(mut self, min: f32) -> ValueInfoBuilder {
        let max = self.range.map(|r| r.max).unwrap_or(f32::INFINITY);
        self.range = Some(Range { min, max });
        self
    }

    pub fn max(mut self, max: f32) -> ValueInfoBuilder {
        let min = self.range.map(|r| r.min).unwrap_or(f32::NEG_INFINITY);
        self.range = Some(Range { min, max });
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> ValueInfoBuilder {
        self.min(min).max(max)
    }

    pub fn convert(mut self, k: f32, a: f32, b: f32) -> ValueInfoBuilder {
        self.conversion = Some(Conversion { k, a, b });
        self
    }
}

impl Into<ValueInfo> for ValueInfoBuilder {
    fn into(self) -> ValueInfo {
        ValueInfo {
            formatting_type: self.formatting_type,
            range: self.range,
            conversion: self.conversion
        }
    }
}