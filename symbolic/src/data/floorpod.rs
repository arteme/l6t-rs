use std::collections::HashMap;
use std::convert::identity;
use std::sync::OnceLock;
use maplit::{convert_args, hashmap};
use crate::data::pod2::{reverb_type};
use crate::data::shorthand::*;
use crate::model::{DataModel, Group, Slot};

/* NOTES: - Vyzex does not save "amp mode enable" to a L6T.
          - Vyzex does not save "wah enable" to a L6T.
*/

// In the order of the Vyzex Floor POD Plus amp select
fn amp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            1 => "Line 6 Clean - RED",
            28 => "Line 6 Clean - GREEN",
            2 => "Line 6 Crunch - RED",
            29 => "Line 6 Crunch - GREEN",
            3 => "Line 6 Drive - RED",
            30 => "Line 6 Drive - GREEN",
            31 => "Line 6 Insane - RED",
            4 => "Line 6 Insane - GREEN",
            5 => "Small Tweed - RED",
            21 => "Small Tweed - GREEN",
            6 => "Tweed Blues - RED",
            23 => "Tweed Blues - GREEN",
            7 => "Black Panel - RED",
            22 => "Black Panel - GREEN",
            8 => "Modern Class A - RED",
            20 => "Modern Class A - GREEN",
            9 => "Brit Class A - RED",
            19 => "Brit Class A - GREEN",
            10 => "Brit Blues - RED",
            24 => "Brit Blues - GREEN",
            11 => "Brit Classic - RED",
            25 => "Brit Classic - GREEN",
            12 => "Brit Hi Gain - RED",
            17 => "Brit Hi Gain - GREEN",
            13 => "TreadPlate - RED",
            26 => "TreadPlate - GREEN",
            14 => "Modern Hi Gain - RED",
            27 => "Modern Hi Gain - GREEN",
            15 => "Fuzz Box - RED",
            18 => "Fuzz Box - GREEN",
            0 => "Tube Preamp - RED",
            16 => "Tube Preamp - GREEN",
        ))
    })
}

// Same as Pocket POD, except for the 16777230 name
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
            16777230 => "4x12 '98 Line 6 Custom 4x12",
            16777231 => "No Cab",
        ))
    })
}

fn compression_ratio() -> &'static HashMap<u32, String> {
    static MAP: OnceLock<HashMap<u32, String>> = OnceLock::new();
    MAP.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "off",
            51 => "1.4:1",
            102 => "2:1",
            153 => "3:1",
            204 => "6:1",
            255 => "inf:1",
        ))
    })
}

fn delay_type() -> &'static HashMap<u32, String> {
    static MAP: OnceLock<HashMap<u32, String>> = OnceLock::new();
    MAP.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "tape delay",
            1 => "multi-tap delay",
            2 => "digital delay",
            3 => "reverse delay",
            4 => "sweep echo",
            5 => "analog delay",
        ))
    })
}

fn effect_select() -> &'static HashMap<u32, String> {
    static MAP: OnceLock<HashMap<u32, String>> = OnceLock::new();
    MAP.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "tremolo",
            1 => "chorus 1",
            2 => "chorus 2",
            3 => "flanger 1",
            4 => "flanger 2",
            5 => "rotary",
            6 => "phaser",
            7 => "u-vibe",
            9 => "tron-up",
            10 => "octave fuzz",
            11 => "sub octave",
            12 => "comet trails",
            13 => "ring modulator",
            14 => "otto phase",
            15 => "swell",
        ))
    })
}

fn delay_shift() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "off"),
            (4, "-12 semitones"),
            (8, "-11 semitones"),
            (12, "-10 semitones"),
            (16, "-9 semitones"),
            (20, "-8 semitones"),
            (24, "-7 semitones"),
            (28, "-6 semitones"),
            (32, "-5 semitones"),
            (36, "-4 semitones"),
            (42, "-3 semitones"),
            (48, "-2 semitones"),
            (54, "-1 semitones"),
            (60, "unison (-5)"),
            (61, "unison (-4)"),
            (62, "unison (-3)"),
            (63, "unison (-2)"),
            (64, "unison (-1)"),
            (65, "unison"),
            (66, "unison (+1)"),
            (67, "unison (+2)"),
            (68, "unison (+3)"),
            (69, "unison (+4)"),
            (70, "unison (+5)"),
            (71, "unison (+6)"),
            (72, "+1 semitones"),
            (78, "+2 semitones"),
            (84, "+3 semitones"),
            (90, "+4 semitones"),
            (96, "+5 semitones"),
            (100, "+6 semitones"),
            (104, "+7 semitones"),
            (108, "+8 semitones"),
            (112, "+9 semitones"),
            (116, "+10 semitones"),
            (120, "+11 semitones"),
            (124, "+12 semitones"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

pub fn floorpod_data_model() -> &'static DataModel {
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
                            slot("amp_select"),
                            bool(7, "drive_boost"),
                            bool(8, "volume_boost"),
                            bool(9, "presence_boost"),
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
                name: "Compressor".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x50000),
                        fixed_model: Some(0x2000000),
                        fixed_enable: Some(false),
                        params: vec![
                            ignore(0x100000),
                            fixed_int("compression_ratio", 0), // 0 = off
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50000),
                        fixed_model: Some(0x2000000),
                        fixed_enable: Some(true),
                        params: vec![
                            int(0x100000, "compression_ratio"), // 1 = 1.4:1, 2 = 2:1, 3 = 3:1, 4 = 6:1, 5 = inf:1
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
                            float(0, "gate_threshold"), // -96..0 dB
                            float(3, "gate_decay"), // %
                        ]
                    },
                ]
            },
            Group {
                name: "Delay".into(),
                slots: vec![
                    // TODO: These slots are so very similar expect
                    //   for the 1st parameter. There has got to be
                    //   a better way to model this!
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020024), // delay type = tape delay
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 0),
                            float(0x100002, "delay_depth"), // %
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020025), // delay type = multi-tap delay
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 1),
                            float(0x100002, "delay_depth"), // %
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020000), // delay type = digital delay
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 2),
                            float(0x100002, "delay_shift"),
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020026), // delay type = reverse delay
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 3),
                            float(0x100002, "delay_depth"), // %
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020027), // delay type = sweep echo
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 4),
                            float(0x100002, "delay_speed"), // %
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50004),
                        fixed_model: Some(0x2020028), // delay type = analog delay
                        fixed_enable: None,
                        params: vec![
                            slot_enable("delay_enable"),
                            fixed_int("delay_type", 5),
                            float(0x100002, "delay_depth"), // %
                            float(0x100000, "delay_time"), // 0 .. 3150.5 = 0 .. 3145.5 ms
                            float(0x100001, "delay_feedback"), // %
                            float(0x010001, "delay_level"), // %
                        ]
                    },
                ]
            },
            Group { // good
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
                        fixed_model: Some(0x2030003), // effect = tremolo
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 0),
                            slot_enable("effect_enable"),
                            float(0x100000, "trem_speed"), // 0.50 .. 6.66? Hz
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
                            fixed_int("effect_select", 1),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"), // 0.15 .. 10.00 Hz
                            float(0x100001, "chorus_depth"), // %
                            float(0x100002, "chorus_feedback"), // -1 .. 3.03 = -100 .. 100 %
                            float(0x100003, "chorus_pre_delay"), // 0 .. 25 = 0 .. 100 %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x203000f), // effect = chorus 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 2),
                            slot_enable("effect_enable"),
                            float(0x100000, "chorus_speed"), // all as above
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
                            fixed_int("effect_select", 3),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"), // 0.15 .. 10.0 Hz
                            float(0x100001, "flanger_depth"), // %
                            float(0x100002, "flanger_feedback"), // 1.01 .. 3.03 = 0 .. 100 %
                            float(0x100003, "flanger_pre_delay"), // 0 .. 25 = 0 .. 100 %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030010), // effect = flanger 2
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 4),
                            slot_enable("effect_enable"),
                            float(0x100000, "flanger_speed"), // 0.15 .. 10.0 Hz
                            float(0x100001, "flanger_depth"), // %
                            float(0x100002, "flanger_feedback"), // -1 .. 1.015 = -100 .. 0 %
                            float(0x100003, "flanger_pre_delay"), // 0 .. 25 = 0 .. 100 %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030002), // effect = rotary
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 5),
                            slot_enable("effect_enable"),
                            int(0x100000, "rotary_speed"), // 0 = slow, 1 = fast
                            float(0x100001, "rotary_fast_speed"), // 0.36 .. 10 Hz
                            float(0x100002, "rotary_slow_speed"), // 0.36 .. 10 Hz
                            float(0x100003, "rotary_depth"), // VYZEX DOESN'T WRITE?!
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030030), // effect = phaser
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 6),
                            float(0x100000, "phaser_speed"), // %
                            float(0x100001, "phaser_depth"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030031), // effect = u-vibe
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 7),
                            float(0x100000, "u_vibe_speed"), // %
                            float(0x100001, "u_vibe_level"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x20a001f), // effect = obi-wah
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 8),
                            float(0x100000, "obi_wah_speed"), // %
                            float(0x100002, "obi_wah_q"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x20a0020), // effect = tron-up
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 9),
                            float(0x100001, "tron_up_sensitivity"), // %
                            float(0x100002, "tron_up_q"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x205001b), // effect = octave fuzz
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 10),
                            float(0x100001, "octave_fuzz_level"), // %
                            //float(???, "octave_fuzz_tone"), // % // VYZEX DOESN'T WRITE :(
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x20a0022), // effect = sub octave
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 11),
                            float(0x100004, "sub_octave_mix"), // %
                            //float(???, "sub_octave_tone"), // % // VYZEX DOESN'T WRITE :(
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x20a0021), // effect = comet trails
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 12),
                            float(0x100000, "comet_trails_speed"), // %
                            float(0x100002, "comet_trails_q"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030033), // effect = ring modulator
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 13),
                            float(0x100000, "ring_modulator_freq"), // %
                            float(0x100001, "ring_modulator_depth"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2030032), // effect = otto phase
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 14),
                            float(0x100001, "otto_phase_sensitivity"), // %
                            float(0x100002, "otto_phase_q"), // %
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x50003),
                        fixed_model: Some(0x2000008), // effect = swell
                        fixed_enable: None,
                        params: vec![
                            fixed_int("effect_select", 15),
                            float(0x100000, "swell_attack_time"), // %
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
            "compression_ratio" => lookup(compression_ratio()),
            "gate_threshold" => db().range(-96.0, 0.0),
            "gate_decay" => percent(),
            "delay_type" => lookup(delay_type()),
            "delay_shift" => lookup_f(delay_shift()).convert(127.5, 0.0, 0.0),
            "delay_depth" => percent(),
            "delay_speed" => percent(),
            "delay_time" => millis1().convert(3145.5/3150.5, 0.0, 0.0).range(0.0, 3145.5),
            "delay_feedback" => percent(),
            "delay_level" => percent(),
            "reverb_type" => lookup(reverb_type()),
            "reverb_decay" => percent(),
            "reverb_density" => percent(),
            "reverb_diffusion" => percent(),
            "reverb_tone" => percent(),
            "reverb_level" => percent(),
            "effect_select" => lookup(effect_select()),
            "chorus_speed" => hz().range(0.15, 10.0),
            "chorus_depth" => percent(),
            // The k & b values are chosen for -1..3 range, whereas the real range is -1..3.03.
            // This makes the +100% in Vyzex reads 102%. However, looks like the scale is not
            // exactly linear in Vyzex. -1..1 = -100..0% and 1..3.03 = 0..100%.
            "chorus_feedback" => percent().convert(0.5, 0.0, -0.5).range(-1.0, 1.0),
            "chorus_pre_delay" => percent().convert(1.0/25.0, 0.0, 0.0),
            "flanger_speed" => hz().range(0.15, 10.0),
            "flanger_depth" => percent(),
            "flanger_feedback" => percent().convert(1.0/2.02, -1.01, -0.0).range(0.0, 1.0),
            "flanger_pre_delay" => percent().convert(1.0/25.0, 0.0, 0.0),
            "phaser_speed" => percent(),
            "phaser_depth" => percent(),
            "u_vibe_speed" => percent(),
            "u_vibe_level" => percent(),
            "obi_wah_speed" => percent(),
            "obi_wah_q" => percent(),
            "tron_up_sensitivity" => percent(),
            "tron_up_q" => percent(),
            "octave_fuzz_level" => percent(),
            //"octave_fuzz_tone" => percent(),
            "sub_octave_mix" => percent(),
            //"sub_octave_tone" => percent(),
            "comet_trails_speed" => percent(),
            "comet_trails_q" => percent(),
            "ring_modulator_freq" => percent(),
            "ring_modulator_depth" => percent(),
            "otto_phase_sensitivity" => percent(),
            "otto_phase_q" => percent(),
            "swell_attack_time" => percent(),
            "vol_min" => percent(),
            "wah_position" => percent(),
            "wah_bottom_freq" => percent(),
            "wah_top_freq" => percent(),
        ));

        DataModel {
            floats_as_ints: false,
            groups,
            info_map
        }
    })
}
