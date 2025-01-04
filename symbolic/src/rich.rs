use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::group::ValueGroup;
use crate::value::{Value, ValueMap, ValueType};
#[cfg(feature = "serde")]
use serde_map_to_array::HashMapToArray;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum FormattingType {
    Simple,
    IntLookup(
        #[cfg_attr(feature = "serde", serde(with = "HashMapToArray::<u32, String>"))]
        &'static HashMap<u32, String>
    ),
    FloatLookup(
        &'static Vec<(u32, String)>
    ),
    Percent,
    Millis(usize),
    Hertz,
    Decibel(usize),
    BPM,
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Range {
    pub min: f32,
    pub max: f32
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum  Conversion {
    /// Represents a linear value conversion, f(x) = k(a + x) + b
    Linear { k: f32, a: f32, b: f32 },
    Interpolate { points: Vec<(f32, f32)> }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct  ValueInfo {
    pub formatting_type: FormattingType,
    pub range: Option<Range>,
    pub conversion: Option<Conversion>,
}

pub type ValueInfoMap = HashMap<String, ValueInfo>;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RichValue {
    value: Value,
    formatting_type: FormattingType,
    range: Option<Range>,
    pub conversion: Option<Conversion>,
}

impl RichValue {
    pub fn get_simple(&self) -> &Value {
        &self.value
    }

    pub fn get_simple_type(&self) -> ValueType {
        self.value.get_type()
    }

    // Conversion done in f64 because we also put u32 through the same conversion
    fn convert_value(&self, value: f64) -> f64 {
        match &self.conversion {
            None => { value }
            Some(Conversion::Linear { k, a, b }) => {
                (*k as f64) * ((*a as f64) + value) + (*b as f64)
            }
            Some(Conversion::Interpolate { points }) => {
                let mut val = 0.0;
                for w in points.windows(2) {
                    let (x1, y1) = w[0];
                    let (x2, y2) = w[1];
                    let x1 = x1 as f64;
                    let x2 = x2 as f64;
                    let y1 = y1 as f64;
                    let y2 = y2 as f64;
                    if value > x2 { continue }
                    val = y1 + (value - x1) * (y2 - y1) / (x2 - x1);
                    break;
                }
                val
            }
        }
    }

    fn get_float(&self) -> Option<f32> {
        match self.value {
            Value::Float(v) => Some(self.convert_value(v as f64) as f32),
            _ => None
        }
    }

    fn get_int(&self) -> Option<u32> {
        match self.value {
            Value::Int(v) => {
                Some(self.convert_value(v as f64) as u32)
            },
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
            FormattingType::Millis(decimals) => {
                let Some(v) = self.get_float() else { return incorrect("millis") };
                write!(f, "{:.decimals$} ms", v, decimals=decimals)
            }
            FormattingType::Hertz => {
                let Some(v) = self.get_float() else { return incorrect("hertz") };
                write!(f, "{:.2} Hz", v)
            }
            FormattingType::Decibel(decimals) => {
                let Some(v) = self.get_float() else { return incorrect("decibel") };
                write!(f, "{:.decimals$} dB", v, decimals=decimals)
            }
            FormattingType::BPM => {
                let Some(v) = self.get_float() else { return incorrect("bpm") };
                write!(f, "{:.1} BPM", v)
            }
            FormattingType::IntLookup(map) => {
                let Some(v) = self.get_int() else { return incorrect("int lookup") };
                let def = "???".to_string();
                let v = map.get(&v).unwrap_or(&def);
                write!(f, "{}", v)
            }
            FormattingType::FloatLookup(map) => {
                let Some(v) = self.get_float() else { return incorrect("float lookup") };
                let v = v as u32;
                let def = "???".to_string();
                let mut r = &def;
                for (k,n) in map.iter() {
                    if *k > v { break }
                    r = n;
                }
                write!(f, "{}", r)
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
                    range: None,
                    conversion: None
                }
            }
            Some(ValueInfo { formatting_type, range, conversion }) => {
                RichValue {
                    value: v,
                    formatting_type: formatting_type.clone(),
                    range: range.clone(),
                    conversion: conversion.clone()
                }
            }
        };
        rich.insert(k, v2);
    }

    rich
}