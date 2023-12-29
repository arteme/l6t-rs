mod podxt;
pub mod models;

use once_cell::sync::Lazy;
use crate::data_model::{DataModel, Group, Param, ParamType, Slot};

fn slot(name: &str) -> Param {
    Param::SlotModel {
        name: name.into()
    }
}
fn slot_enable(name: &str) -> Param {
    Param::SlotEnable {
        name: name.into()
    }
}

fn bool(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Bool
    }
}
fn fixed_int(name: &str, value: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: None
    }
}
fn fixed_int_for_slot_id(name: &str, value: u32, slot_id: u32) -> Param {
    Param::FixedParam {
        name: name.into(),
        param_type: ParamType::Int,
        param_value: value,
        slot_id: Some(slot_id)
    }
}

fn int(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Int
    }
}

fn float(id: u32, name: &str) -> Param {
    Param::Param {
        name: name.into(),
        param_id: id,
        param_type: ParamType::Float
    }
}

fn ignore(id: u32) -> Param {
    Param::IgnoreParam {
        param_id: id,
    }
}


pub static POD2_DATA_MODEL: Lazy<DataModel> = Lazy::new(|| {
    DataModel {
        floats_as_ints: true,
        groups: vec![
            Group {
                name: "Amp".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x30000),
                        fixed_model: None,
                        fixed_enable: Some(true),
                        params: vec![
                            slot("amp_select"), // range?
                            bool(7, "distortion_enable"),
                            bool(8, "drive_enable"),
                            bool(9, "eq_enable"),
                            bool(10, "bright_enable"),

                            float(0x100003, "drive"),
                            float(0x100000, "bass"),
                            float(0x100001, "mid"),
                            float(0x100002, "treble"),
                            float(0x100004, "presence"),
                            float(0x100005, "chan_volume"),
                            float(0x100006, "drive2"),
                        ],
                    },

                ]
            },
            Group {
                name: "Cab".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x30001),
                        fixed_model: None,
                        fixed_enable: Some(true),
                        params: vec![
                            slot("cab_select"), // range?
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x30003),
                        fixed_model: Some(0x20b0002),
                        fixed_enable: Some(true),
                        params: vec![
                            float(0x100000, "air"), // format?
                        ]
                    },
                ]
            },
            Group {
                name: "Noise gate".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20000),
                        fixed_model: Some(0x2000001),
                        fixed_enable: None,
                        params: vec![
                            slot_enable("gate_enable"),
                            float(0, "gate_threshold"), // format?
                            float(3, "gate_decay"), // format?
                        ]
                    },
                ]
            },
            Group {
                name: "Delay".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020000),
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            float(0x100000, "delay_time"), // format?
                            float(0x100001, "delay_feedback"), // format?
                            float(0x010001, "delay_level"), // format?
                        ]
                    },
                ]
            },
            Group {
                name: "Reverb".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x50005),
                        fixed_model: Some(0x2040000), // reverb type = room
                        fixed_enable: None,
                        params: vec![
                            slot_enable("reverb_enable"),
                            fixed_int("reverb_type", 1),
                            float(0x100000, "reverb_decay"),
                            float(0x100004, "reverb_density"),
                            float(0x100003, "reverb_diffusion"),
                            float(0x100002, "reverb_tone"),
                            float(0x010002, "reverb_level"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50005),
                        fixed_model: Some(0x2040001), // reverb type = spring
                        fixed_enable: None,
                        params: vec![
                            slot_enable("reverb_enable"),
                            fixed_int("reverb_type", 0),
                            float(0x100000, "reverb_decay"),
                            float(0x100004, "reverb_density"),
                            float(0x100003, "reverb_diffusion"),
                            float(0x100002, "reverb_tone"),
                            float(0x010002, "reverb_level"),
                        ]
                    },
                ]
            },
            Group {
                name: "Effects".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x7fffffff), // effect = bypass
                        fixed_enable: Some(false),
                        params: vec![
                            fixed_int("effect_select", 0),
                            float(0x100000, "reverb_decay"),
                            ignore(0x100001),
                            ignore(0x100002),
                            ignore(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2000000), // effect = compressor
                        fixed_enable: Some(false),
                        params: vec![
                            fixed_int("effect_select", 1),
                            float(0x100000, "compression_ratio"),
                            ignore(0x100001),
                            ignore(0x100002),
                            ignore(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2000008), // effect = auto swell
                        fixed_enable: Some(false),
                        params: vec![
                            fixed_int("effect_select", 2),
                            float(0x100000, "volume_swell_time"),
                            ignore(0x100001),
                            ignore(0x100002),
                            ignore(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030000), // effect = chorus 1
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 3),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"),
                            float(0x100001, "chorus_depth"),
                            float(0x100002, "chorus_feedback"),
                            float(0x100003, "chorus_pre_delay"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x203000f), // effect = chorus 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 4),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"),
                            float(0x100001, "chorus_depth"),
                            float(0x100002, "chorus_feedback"),
                            float(0x100003, "chorus_pre_delay"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030001), // effect = flanger 1
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 5),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"),
                            float(0x100001, "flanger_depth"),
                            float(0x100002, "flanger_feedback"),
                            float(0x100003, "flanger_pre_delay"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030010), // effect = flanger 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 6),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"),
                            float(0x100001, "flanger_depth"),
                            float(0x100002, "flanger_feedback"),
                            float(0x100003, "flanger_pre_delay"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030003), // effect = tremolo
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 7),
                            slot_enable("effect_enable"),
                            float(0x100000, "trem_speed"),
                            float(0x100001, "trem_depth"),
                            ignore(0x100002),
                            ignore(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030002), // effect = rotary
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 8),
                            slot_enable("effect_enable"),
                            float(0x100000, "rotary_speed"),
                            float(0x100001, "rotary_fast_speed"),
                            float(0x100002, "rotary_slow_speed"),
                            float(0x100003, "rotary_depth"),
                        ]
                    },
                ]
            },
            Group {
                name: "Volume pedal".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: Some(0x2070001),
                        fixed_enable: Some(true),
                        params: vec![
                            fixed_int_for_slot_id("vol_pedal_position", 0, 0x20001),
                            fixed_int_for_slot_id("vol_pedal_position", 1, 0x50002),
                            float(4, "vol_min"),
                        ]
                    },
                ]
            },
            Group {
                name: "Wah pedal".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20002),
                        fixed_model: Some(0x2060001),
                        fixed_enable: Some(false), // wah enable?
                        params: vec![
                            float(3, "wah_bottom_freq"),
                            float(2, "wah_top_freq"),
                        ]
                    },
                ]
            },
        ]
    }
});