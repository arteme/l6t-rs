use crate::model::{ModelParam, Value, L6Patch, TargetDevice};

fn knob_param(param_id: u32, value: i8) -> ModelParam {
    ModelParam { param_id, value: Value::Float(f32::from(value) / 127.0) }
}

fn half_knob_param(param_id: u32, value: i8) -> ModelParam {
    ModelParam { param_id, value: Value::Float(f32::from(value) / 63.0) }
}


fn data_to_patch(data: &[u8; 55], name: &str) -> L6Patch {
    let models: Vec<ModelParam> = vec![];



    L6Patch {
        target_device: TargetDevice {
            midi_id: 0x0300,
            name: name.into(),
            version: 0
        },
        models: Default::default(),
        meta: Default::default()
    }
}