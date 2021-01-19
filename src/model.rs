use std::collections::HashMap;

#[derive(Debug)]
pub struct TargetDevice {
    pub midi_id: u32,
    pub name: String,
    pub version: u32
}

#[derive(Debug)]
pub struct Model {
    pub model_id: u32,
    pub slot_id: u32,
    pub enabled: bool,
    pub ordinal: u8,
    pub params: Vec<ModelParam>
}

#[derive(Debug)]
pub struct ModelParam {
    pub param_id: u32,
    pub value_type: ValueType,
    pub value: u32
}

#[derive(Debug)]
pub enum ValueType {
    Int = 0,
    Float = 1
}

#[derive(Debug)]
pub struct MetaTags {
    pub author: String,
    pub guitarist: String,
    pub band: String,
    pub song: String,
    pub style: String,
    pub pickup_style: String,
    pub pickup_position: String,
    pub date: usize,
    pub amp_name: String,
    pub creator_app: String,
    pub creator_app_version: String,
    pub comments: String
}

#[derive(Debug)]
pub struct L6Patch {
    pub target_device: TargetDevice,
    pub models: HashMap<u32, Model>,
    pub meta: MetaTags
}

impl Default for MetaTags {
    fn default() -> Self {
        MetaTags {
            author: "".to_string(),
            guitarist: "".to_string(),
            band: "".to_string(),
            song: "".to_string(),
            style: "".to_string(),
            pickup_style: "".to_string(),
            pickup_position: "".to_string(),
            date: 0,
            amp_name: "".to_string(),
            creator_app: "".to_string(),
            creator_app_version: "".to_string(),
            comments: "".to_string()
        }
    }
}

impl Default for TargetDevice {
    fn default() -> Self {
        TargetDevice {
            midi_id: 0,
            name: "".to_string(),
            version: 0
        }
    }
}

impl Default for L6Patch {
    fn default() -> Self {
        L6Patch {
            target_device: Default::default(),
            models: Default::default(),
            meta: Default::default()
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            model_id: 0,
            slot_id: 0,
            enabled: false,
            ordinal: 0,
            params: vec![]
        }
    }
}

impl Default for ModelParam {
    fn default() -> Self {
        ModelParam {
            param_id: 0,
            value_type: 0.into(),
            value: 0
        }
    }
}

impl From<u32> for ValueType {
    fn from(v: u32) -> Self {
        match v {
            0 => ValueType::Int,
            1 => ValueType::Float,
            _ => panic!("Unknown value: {}", v)
        }
    }
}