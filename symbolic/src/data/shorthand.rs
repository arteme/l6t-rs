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
        .formatting_type(FormattingType::IntLookup(map))
}

pub fn lookup_f(map: &'static Vec<(u32, String)>) -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::FloatLookup(map))
}

pub fn percent() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Percent)
        .range(0.0, 1.0)
}

pub fn millis() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Millis(0))
        .range(0.0, f32::INFINITY)
}

pub fn millis1() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Millis(1))
        .range(0.0, f32::INFINITY)
}

pub fn hz() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Hertz)
        .range(0.0, f32::INFINITY)
}

pub fn bpm() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::BPM)
        .range(30.0, 240.0)
}

pub fn db() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Decibel(0))
}

pub fn db1() -> ValueInfoBuilder {
    ValueInfoBuilder::new()
        .formatting_type(FormattingType::Decibel(1))
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
        self.conversion = Some(Conversion::Linear { k, a, b });
        self
    }

    pub fn points(mut self, points: &[(f32, f32)]) -> ValueInfoBuilder {
        self.conversion = Some(Conversion::Interpolate { points: points.to_vec() });
        self
    }

    // Line6 Edit deals with MIDI control, where the typical range is
    // 0..127. Dividing by 127 yields not so pretty fractions, so most
    // interpolations divide by 128, and skip value 127 altogether.
    // The float values saved to L6T files still use a 1/127 step.
    // This points-function treats the x-axis as 128 fixed steps from
    // 0 to 1, inserting a special point at 126/128.
    pub fn points_l6e(mut self, points: &[(u32, f32)]) -> ValueInfoBuilder {
        let mut ret = Vec::new();
        for w in points.windows(2) {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];

            ret.push(((x1 as f32)/127.0, y1));

            if x2 == 128 {
                let x = 126.0/127.0;
                let y = y1 + (126 - x1) as f32 * (y2 - y1)/(x2 - x1) as f32;
                ret.push((x, y));
                ret.push((1.0, y2));
            }
        }

        self.conversion = Some(Conversion::Interpolate { points: ret });
        self
    }

    pub fn from_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> ValueInfoBuilder {
        let k = (y2 - y1)/(x2 - x1);
        let b = (x2 * y1 - x1 * y2)/(x2 - x1);
        self.convert(k, 0.0, b).range(y1, y2)
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