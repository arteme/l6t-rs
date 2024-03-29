use std::sync::OnceLock;
use crate::data::shorthand::*;
use crate::data::models::filter_params_by_prefix;
use crate::model::{DataModel, Group, Slot};

fn podxt_data_model_all() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(|| {
        let groups = vec![
            Group {
                name: "Misc".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x10000),
                        fixed_model: Some(0x3e700000),
                        fixed_enable: Some(true),
                        params: vec![
                            float(0x200017, "amp_bypass_volume"),
                            float(0x200001, "tempo"),
                            float(0x200019, "di_model"),
                            float(0x20001a, "di_delay"),

                            int(0x200006, "pedal_assign"),
                            int(0x200004, "tweak_param_select"),

                            bool(0x200007, "pro.loop_enable"),

                            int(0x200009, "live.variax_300_500_700.model_select"),
                            int(0x20000a, "live.variax_300_500_700.tone"),
                            int(0x20000b, "live.variax_acoustic.model_select"),
                            int(0x20000c, "live.variax_acoustic.mic_pos"),
                            int(0x20000d, "live.variax_acoustic.comp"),
                            int(0x20000e, "live.variax_bass.model_select"),
                            int(0x20000f, "live.variax_bass.blend"),
                            int(0x200010, "live.variax_bass.bass"),
                            int(0x200011, "live.variax_bass.treble"),
                        ]
                    },
                ]
            },
            Group {
                name: "Amp".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x30000),
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            slot("amp_select"),
                            slot_enable("amp_enable"),

                            float(0x100003, "drive"),
                            float(0x100000, "bass"),
                            float(0x100001, "mid"),
                            float(0x100002, "treble"),
                            float(0x100004, "presence"),
                            float(0x100005, "chan_volume"),
                        ]
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
                            int(0x000000, "mic_select"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x30003),
                        fixed_model: Some(0x20b0002),
                        fixed_enable: None,
                        params: vec![
                            float(0x100000, "room"),
                        ]
                    },
                ]
            },
            Group {
                name: "Noise gate".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20000),
                        fixed_model: Some(0x20b0000),
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
                name: "Compressor".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x50000),
                        fixed_model: Some(0x20b0001),
                        fixed_enable: None,
                        params: vec![
                            slot_enable("comp_enable"),
                            float(0x100000, "comp_threshold"), // 0..1 for -63..0 dB
                            float(0x100001, "comp_gain"), // 0..1 for 0..16 dB
                        ]
                    },
                ]
            },
            Group {
                name: "Stomp".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20003),
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            slot("stomp_select"),
                            slot_enable("stomp_enable"),

                            float(0x100001, "stomp_param_1"),
                            float(0x100002, "stomp_param_2"),
                            float(0x100003, "stomp_param_3"),
                            float(0x100004, "stomp_param_4"),
                            float(0x100005, "stomp_param_5"),
                        ]
                    },
                ]
            },
            Group {
                name: "Modulation".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            fixed_int_for_slot_id("mod_position", 0, 0x20004),
                            fixed_int_for_slot_id("mod_position", 1, 0x50003),
                            slot("mod_select"),
                            slot_enable("mod_enable"),

                            float(0x100000, "mod_speed"), // 0..1
                            float(0x100001, "mod_param_1"),
                            float(0x100002, "mod_param_2"),
                            float(0x100003, "mod_param_3"),
                            ignore(0x100004),
                            float(0x010001, "mod_param_4"),
                            ignore(0x010002), // TODO: float!
                            int(0x200000, "mod_note_select"),
                        ]
                    },
                ]
            },
            Group {
                name: "Delay".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            fixed_int_for_slot_id("delay_position", 0, 0x20005),
                            fixed_int_for_slot_id("delay_position", 1, 0x50004),
                            slot("delay_select"),
                            slot_enable("delay_enable"),

                            float(0x100000, "delay_speed"), // 0..1
                            float(0x100001, "delay_param_1"),
                            float(0x100002, "delay_param_2"),
                            float(0x100003, "delay_param_3"),
                            ignore(0x100004),
                            float(0x010001, "delay_param_4"),
                            float(0x010002, "di_xover"),
                            int(0x200000, "delay_note_select"),
                        ]
                    },
                ]
            },
            Group {
                name: "Reverb".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            fixed_int_for_slot_id("reverb_position", 0, 0x20006),
                            fixed_int_for_slot_id("reverb_position", 1, 0x50005),
                            slot("reverb_select"),
                            slot_enable("reverb_enable"),

                            float(0x100000, "reverb_dwell"), // 0..1
                            // TODO: written as "int 0" when not initialized
                            float(0x100001, "reverb_pre_delay"),
                            float(0x100002, "reverb_tone"),
                            float(0x010002, "reverb_mix"),
                        ]
                    },
                ]
            },
            Group {
                name: "EQ".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x30004),
                        fixed_model: Some(0x20c0002),
                        fixed_enable: None,
                        params: vec![
                            slot_enable("eq_enable"),

                            float(0x100000, "eq_1_freq"), // 0..1
                            float(0x100001, "eq_1_gain"),
                            float(0x100002, "eq_2_freq"),
                            float(0x100003, "eq_2_gain"),
                            float(0x100004, "eq_3_freq"),
                            float(0x100005, "eq_3_gain"),
                            float(0x100006, "eq_4_freq"),
                            float(0x100007, "eq_4_gain"),
                            ignore(0x100008),
                            ignore(0x100009),
                            ignore(0x10000a),
                            ignore(0x10000b),
                        ]
                    },
                ]
            },
            Group {
                name: "Volume pedal".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: Some(0x2070000),
                        fixed_enable: Some(true),
                        params: vec![
                            fixed_int_for_slot_id("vol_pedal_position", 0, 0x20001),
                            fixed_int_for_slot_id("vol_pedal_position", 1, 0x50002),
                            float(0x100004, "vol_min"),
                        ]
                    },
                ]
            },
            Group {
                name: "Wah pedal".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20002),
                        fixed_model: None,
                        fixed_enable: None,
                        params: vec![
                            slot("wah_select"),
                            slot_enable("wah_enable"),

                            float(1, "wah_level"),
                        ]
                    },
                ]
            },
        ];

        DataModel {
            floats_as_ints: false,
            groups
        }
    })
}

pub fn podxt_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(podxt_data_model_all(), &["pro.", "live."], &[])
    )
}

pub fn podxt_pro_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(podxt_data_model_all(), &["live."], &["pro."])
    )
}

pub fn podxt_live_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(podxt_data_model_all(), &["pro."], &["live."])
    )
}
