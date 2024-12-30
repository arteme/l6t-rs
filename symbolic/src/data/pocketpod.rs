use std::collections::HashMap;
use std::convert::identity;
use std::sync::OnceLock;
use maplit::{convert_args, hashmap};
use crate::data::pod2::{compression_ratio, effect_select, reverb_type, rotary_speed};
use crate::data::shorthand::*;
use crate::model::{DataModel, Group, Slot};

/* NOTES: Vyzex does not save "wah enable" to a L6T,
   it is also a MIDI-only control.
*/

// In the order of the Vyzex Pocket POD amp select, some names
// different from POD 2.0
fn amp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Tube Preamp",
            16 => "Jazz Clean",
            5 => "Small Tweed",
            21 => "Small Tweed 2",
            6 => "Tweed Blues",
            10 => "Brit Blues",
            7 => "Black Panel",
            22 => "Black Panel 2",
            17 => "Boutique 1",
            18 => "Boutique 2",
            23 => "Boutique 3",
            24 => "Cali Church 1",
            25 => "Cali Church 2",
            9 => "Brit Class A",
            19 => "Brit Class A 2",
            20 => "Brit Class A 3",
            11 => "Brit Classic",
            12 => "Brit Hi Gain",
            8 => "Modern Class A",
            13 => "Treadplate",
            26 => "Treadplate 2",
            14 => "Modern Hi Gain",
            27 => "Modern Hi Gain 2",
            15 => "Fuzz Box",
            1 => "Line 6 Crunch",
            28 => "Line 6 Twang",
            30 => "Line 6 Blues",
            2 => "Line 6 Crunch",
            29 => "Line 6 Crunch 2",
            3 => "Line 6 Drive",
            4 => "Line 6 Layer",
            31 => "Line 6 Insane",
        ))
    })
}

// Supposedly, these ids are unique, but Pocket POD uses the same
// ids as POD 2.0 with completely different cabs
fn cab_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            16777216 => "1x8 Small Tweed #2",
            16777217 => "1x12 Small Tweed",
            16777218 => "1x12 Brit Class A #3",
            16777219 => "1x12 Black Panel",
            16777220 => "1x12 '98 Line 6 Flextone",
            16777221 => "2x12 Black Panel #2",
            16777222 => "2x12 Brit Class A",
            16777223 => "2x12 Modern Class A",
            16777224 => "2x12 '98 Line 6 Custom 2x12",
            16777225 => "4x10 Tweed Blues",
            16777226 => "4x10 '98 Line 6 Custom 4x10",
            16777227 => "4x12 Brit High Gain",
            16777228 => "4x12 Brit High Gain #2",
            16777229 => "4x12 Brit High Gain #3",
            16777230 => "4x12 Line 6",
            16777231 => "No Cab",
        ))
    })
}



pub fn pocketpod_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(|| {
        let groups = vec![
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

                            float(0x100003, "drive"), // %
                            float(0x100000, "bass"), // %
                            float(0x100001, "mid"), // %
                            float(0x100002, "treble"), // %
                            float(0x100004, "presence"), // %
                            float(0x100005, "chan_volume"), // %
                            float(0x100006, "drive2"), // %
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
                            slot("cab_select"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x30003),
                        fixed_model: Some(0x20b0002),
                        fixed_enable: Some(true),
                        params: vec![
                            float(0x100000, "air"), // %
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
                            float(0, "gate_threshold"), // 0..-96 = -96..0 dB
                            float(3, "gate_decay"), // %
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
                            float(0x100000, "delay_time"), // 0 .. 3150 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
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
                            float(0x100000, "reverb_decay"), // %
                            float(0x100002, "reverb_tone"), // %
                            float(0x100003, "reverb_diffusion"), // %
                            float(0x100004, "reverb_density"), // %
                            // "Reverb" knob in the "Amp" group in Vyzex
                            float(0x010002, "reverb_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50005),
                        fixed_model: Some(0x2040001), // reverb type = spring
                        fixed_enable: None,
                        params: vec![
                            slot_enable("reverb_enable"),
                            fixed_int("reverb_type", 0),
                            float(0x100000, "reverb_decay"), // %
                            float(0x100002, "reverb_tone"), // %
                            float(0x100003, "reverb_diffusion"), // %
                            float(0x100004, "reverb_density"), // %
                            // "Reverb" knob in the "Amp" group in Vyzex
                            float(0x010002, "reverb_level"), // %
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
                        fixed_enable: Some(true),
                        params: vec![
                            fixed_int("effect_select", 0),
                            ignore_f(0x100000),
                            ignore_f(0x100001),
                            ignore_f(0x100002),
                            ignore_f(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2000000), // effect = compressor
                        fixed_enable: Some(true),
                        params: vec![
                            fixed_int("effect_select", 1),
                            int(0x100000, "compression_ratio"), // 0 = off, 1 = 1.4:1, 2 = 2:1, 3 = 3:1, 4 = 6:1, 5 = inf:1
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030003), // effect = tremolo
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 7),
                            slot_enable("effect_enable"),
                            float(0x100000, "trem_speed"), // 0.33 .. 6.66 Hz
                            float(0x100001, "trem_depth"), // %
                            ignore_f(0x100002),
                            ignore_f(0x100003),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030000), // effect = chorus 1
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 3),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"), // 0.16 .. 5 Hz
                            float(0x100001, "chorus_depth"), // %
                            float(0x100002, "chorus_feedback"), // -1 .. 1 %
                            float(0x100003, "chorus_pre_delay"), // 0 .. 25 = 0 .. 100%
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x203000f), // effect = chorus 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 4),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"), // 0.16 .. 5 Hz
                            float(0x100001, "chorus_depth"), // %
                            float(0x100002, "chorus_feedback"), // -1 .. 1 %
                            float(0x100003, "chorus_pre_delay"), // 0 .. 25 = 0 .. 100%
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030001), // effect = flanger 1
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 5),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"), // 0.16 .. 5 Hz
                            float(0x100001, "flanger_depth"), // %
                            float(0x100002, "flanger_feedback"), // -1 .. 1 %
                            float(0x100003, "flanger_pre_delay"), // 0 .. 25 = 0 .. 100%
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030010), // effect = flanger 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 6),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"), // 0.16 .. 5 Hz
                            float(0x100001, "flanger_depth"), // %
                            float(0x100002, "flanger_feedback"), // -1 .. 1 %
                            float(0x100003, "flanger_pre_delay"), // 0 .. 25 = 0 .. 100%
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030002), // effect = rotary
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 8),
                            slot_enable("effect_enable"),
                            int(0x100000, "rotary_speed"), // 0 = slow, 1 = fast
                            float(0x100001, "rotary_fast_speed"), // 0.36 .. 10 Hz
                            float(0x100002, "rotary_slow_speed"), // 0.36 .. 10 Hz
                            float(0x100003, "rotary_depth"), // VYZEX DOESN'T WRITE?!
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2000008), // effect = auto swell
                        fixed_enable: Some(true),
                        params: vec![
                            fixed_int("effect_select", 2),
                            float(0x100000, "volume_swell_time"), // %
                            ignore_f(0x100001),
                            ignore_f(0x100002),
                            ignore_f(0x100003),
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
                            //float(3, "???"), -- float ignore, always set to 1 ?
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
                        fixed_enable: Some(true),
                        params: vec![
                            float(1, "wah_position"),
                            float(2, "wah_bottom_freq"),
                            float(3, "wah_top_freq"),
                        ]
                    },
                ]
            },
        ];

        let info_map = convert_args!(hashmap!(
            "amp_select" => lookup(amp_select()),
            "drive" => percent(),
            "bass" => percent(),
            "mid" => percent(),
            "treble" => percent(),
            "presence" => percent(),
            "chan_volume" => percent(),
            "drive2" => percent(),
            "cab_select" => lookup(cab_select()),
            "air" => percent(),
            "gate_threshold" => db().convert(-1.0, 0.0, -96.0).range(-96.0, 0.0),
            "gate_decay" => percent(),
            "delay_time" => millis().range(0.0, 3150.0),
            "delay_feedback" => percent(),
            "delay_level" => percent(),
            "effect_select" => lookup(effect_select()),
            "compression_ratio" => lookup(compression_ratio()),
            "volume_swell_time" => percent(),
             "chorus_speed" => hz().range(0.16, 5.0),
            "chorus_depth" => percent(),
            "chorus_feedback" => percent().range(-1.0, 1.0),
            "chorus_pre_delay" => percent().convert(1.0/25.0, 0.0, 0.0),
            "flanger_speed" => hz().range(0.16, 5.0),
            "flanger_depth" => percent(),
            "flanger_feedback" => percent().range(-1.0, 1.0),
            "flanger_pre_delay" => percent().convert(1.0/25.0, 0.0, 0.0),
            "trem_speed" => hz().range(0.33, 6.67),
            "trem_depth" => percent(),
            "rotary_speed" => lookup(rotary_speed()),
            "rotary_fast_speed" => hz().range(0.36, 10.0),
            "rotary_slow_speed" => hz().range(0.36, 10.0),
            "rotary_depth" => percent(), //???
            "vol_min" => percent(),
            "wah_position" => percent(),
            "wah_bottom_freq" => percent(),
            "wah_top_freq" => percent(),
            "reverb_type" => lookup(reverb_type()),
            "reverb_decay" => percent(),
            "reverb_density" => percent(),
            "reverb_diffusion" => percent(),
            "reverb_tone" => percent(),
            "reverb_level" => percent(),
        ));

        DataModel {
            floats_as_ints: false,
            groups,
            info_map
        }
    })
}
