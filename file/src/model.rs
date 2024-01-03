use std::fmt::{Display, Formatter, Result};

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

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.slot_id == other.slot_id &&
            self.model_id == other.model_id &&
            self.ordinal == other.ordinal
    }
}

#[derive(Debug)]
pub struct ModelParam {
    pub param_id: u32,
    pub value: Value
}

#[derive(Debug)]
pub enum Value {
    Int(u32),
    Float(f32)
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

#[derive(Debug, Default, PartialEq, Eq)]
pub enum PatchType {
    #[default]
    Patch,
    AmpSetup,
    FxSetup
}

#[derive(Debug, Default)]
pub struct L6Patch {
    pub patch_type: PatchType,
    pub target_device: TargetDevice,
    pub models: Vec<Model>,
    pub meta: MetaTags
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum BundleType {
    #[default]
    Bundle,
    Collection
}

#[derive(Debug, Default)]
pub struct L6Bundle {
    pub bundle_type: BundleType,
    pub banks: Vec<Bank>
}

pub(crate) struct BatchHead {
    pub version: u32
}

#[derive(Debug, Default)]
pub struct Bank {
    pub name: String,
    pub patches: Vec<L6Patch>
}

pub(crate) struct BankInfo {
    pub name: String
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
            value: Value::Int(0)
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Int(v) => write!(f, "int {}", v),
            Value::Float(v) => write!(f, "float {}", v)
        }
    }
}