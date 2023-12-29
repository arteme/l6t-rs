use once_cell::sync::Lazy;
use crate::data::{fixed_int, float, ignore, int, slot, slot_enable};
use crate::data_model::{DataModel, Group, Slot};

pub static PODXT_DATA_MODEL: Lazy<DataModel> = Lazy::new(|| {
    let groups = vec![
        Group {
            name: "Misc".into(),
            slots: vec![
                Slot {
                    slot_id: 0x10000,
                    fixed_model: Some(0x3e700000),
                    fixed_enable: Some(true),
                    params: vec![
                        float(0x200017, "amp_bypass_volume"),
                        float(0x200001, "tempo"),
                        float(0x200019, "di_model"),
                        float(0x20001a, "di_delay"),

                        int(0x200006, "pedal_assign"),
                        int(0x200004, "tweak_param_select"),
                    ]
                },
            ]
        },
        Group {
            name: "Amp".into(),
            slots: vec![
                Slot {
                    slot_id: 0x30000,
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
                    slot_id: 0x30001,
                    fixed_model: None,
                    fixed_enable: Some(true),
                    params: vec![
                        slot("cab_select"), // range?
                        int(0x000000, "mic_select"),
                    ]
                },
                Slot {
                    slot_id: 0x30003,
                    fixed_model: Some(0x20b0002),
                    fixed_enable: Some(false),
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
                    slot_id: 0x20000,
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
                    slot_id: 0x50000,
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
                    slot_id: 0x20003,
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
                    slot_id: 0x50003,
                    fixed_model: None,
                    fixed_enable: None,
                    params: vec![
                        slot("mod_select"),
                        slot_enable("mod_enable"),

                        float(0x100000, "mod_speed"), // 0..1
                        float(0x100001, "mod_param_1"),
                        float(0x100002, "mod_param_2"),
                        float(0x100003, "mod_param_3"),
                        ignore(0x010004),
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
                    slot_id: 0x50004,
                    fixed_model: None,
                    fixed_enable: None,
                    params: vec![
                        slot("delay_select"),
                        slot_enable("delay_enable"),

                        float(0x100000, "delay_speed"), // 0..1
                        float(0x100001, "delay_param_1"),
                        float(0x100002, "delay_param_2"),
                        float(0x100003, "delay_param_3"),
                        ignore(0x010004),
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
                    slot_id: 0x50005,
                    fixed_model: None,
                    fixed_enable: None,
                    params: vec![
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
                    slot_id: 0x30004,
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
                        ignore(0x010008),
                        ignore(0x010009),
                        ignore(0x01000a),
                        ignore(0x01000b),
                    ]
                },
            ]
        },
        Group {
            name: "Volume pedal".into(),
            slots: vec![
                Slot {
                    slot_id: 0x50002, // vol_pedal_position == 1
                    fixed_model: Some(0x2070000),
                    fixed_enable: Some(true),
                    params: vec![
                        fixed_int("vol_pedal_position", 1),
                        float(0x100004, "vol_min"),
                    ]
                },
                Slot {
                    slot_id: 0x20001, // vol_pedal_position == 0
                    fixed_model: Some(0x2070000),
                    fixed_enable: Some(true),
                    params: vec![
                        fixed_int("vol_pedal_position", 0),
                        float(0x100004, "vol_min"),
                    ]
                },
            ]
        },
        Group {
            name: "Wah pedal".into(),
            slots: vec![
                Slot {
                    slot_id: 0x20002,
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
});
