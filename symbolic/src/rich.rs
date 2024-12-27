use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::group::ValueGroup;
use crate::value::{Value, ValueMap, ValueType};

#[derive(Clone)]
pub enum FormattingType {
    Simple,
    StringLookup(&'static HashMap<u32, String>),
    Percent,
    Millis,
    Hertz,
    Decibel,
}

#[derive(Clone)]
pub struct Range {
    pub min: f32,
    pub max: f32
}

#[derive(Clone)]
pub struct  ValueInfo {
    pub formatting_type: FormattingType,
    pub range: Option<Range>,
}

pub type ValueInfoMap = HashMap<String, ValueInfo>;

#[derive(Clone)]
pub struct RichValue {
    value: Value,
    formatting_type: FormattingType,
    range: Option<Range>
}

impl RichValue {
    pub fn get_simple(&self) -> &Value {
        &self.value
    }

    pub fn get_simple_type(&self) -> ValueType {
        self.value.get_type()
    }

    fn get_float(&self) -> Option<f32> {
        match self.value {
            Value::Float(v) => Some(v),
            _ => None
        }
    }

    fn get_int(&self) -> Option<u32> {
        match self.value {
            Value::Int(v) => Some(v),
            _ => None
        }
    }
}

impl Display for RichValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut incorrect = |type_: &str| {
            write!(f, "{} (incorrect type '{}' for {})", &self.value, &self.value.get_type(), type_)
        };


        match self.formatting_type {
            FormattingType::Simple => {
                write!(f, "{}", &self.value)
            }
            FormattingType::Percent => {
                let Some(v) = self.get_float() else { return incorrect("percent") };
                write!(f, "{:.0} %", v * 100.0)
            }
            FormattingType::Millis => {
                let Some(v) = self.get_float() else { return incorrect("millis") };
                write!(f, "{:.0} ms", v)
            }
            FormattingType::Hertz => {
                let Some(v) = self.get_float() else { return incorrect("hertz") };
                write!(f, "{:.2} Hz", v)
            }
            FormattingType::Decibel => {
                let Some(v) = self.get_float() else { return incorrect("decibel") };
                write!(f, "{:.0} dB", v)
            }
            FormattingType::StringLookup(map) => {
                let Some(v) = self.get_int() else { return incorrect("string lookup") };
                let def = "???".to_string();
                let v = map.get(&v).unwrap_or(&def);
                write!(f, "{}", v)
            }
        }
    }
}

pub type RichValueMap = HashMap<String, RichValue>;
pub type RichValueGroup = ValueGroup<RichValue>;

pub fn enrich_values(map: ValueMap, rich_type_map: &ValueInfoMap) -> RichValueMap {
    let mut rich = RichValueMap::with_capacity(map.len());
    for (k, v) in map.into_iter() {
        let v2 = match rich_type_map.get(&k) {
            None => {
                RichValue {
                    value: v,
                    formatting_type: FormattingType::Simple,
                    range: None
                }
            }
            Some(ValueInfo { formatting_type, range }) => {
                RichValue {
                    value: v,
                    formatting_type: formatting_type.clone(),
                    range: range.clone()
                }
            }
        };
        rich.insert(k, v2);
    }

    rich
}