// Bass PODxt is a variant of PODxt. It mostly follows the PODxt
// model with the following notes. Most data will be copied
// from PODxt where applicable. The notable:
//   * amps, cabs, mics that are marked with "BX";
//   * stomp, mod slots are mostly a sub-set of standard and "FX"
//     PODxt slots with a few Bass PODxt-specific models;
//   * Delay and reverb slots are mixed together in the same
//     delay slot 0x20005. While Bass PODxt reverb slots use the
//     same params and model values as PODxt, the parameter layout
//     and order is Bass PODxt-specific.
//   * The Pro/Live additions are the same as PODxt;
//
// TODO: Line6 Edit writes a dummy reverb model block to reverb
//       slot 0x20006. Should l6t-rs do the same?
//
//   [0x20006] model=0xffffffff ordinal=0 disabled
//    id=0x100001 int 0
//    id=0x100000 int 0
//    id=0x100002 int 0
//    id=0x010002 float 0
//
// NOTE: Looks like L6E in off-line mode writes Bass PODxt patch
//       as a Bass PODxt Live. It also gets the position of slots
//       wrong (0x5 vs 0x2) for some non-moving slots like delay
//       and reverb ???
//
use std::collections::HashMap;
use std::convert::identity;
use std::sync::OnceLock;
use maplit::{convert_args, hashmap};
use crate::data::shorthand::*;
use crate::data::models::filter_params_by_prefix;
use crate::data::podxt;
use crate::data::podxt::{_1457_value, _1m335_value, footswitch_mode_select, heads_value, mod_slot, mod_slot5, note_select, pedal_assign_select, podxt_data_model, stomp_slot, wave_value};
use crate::model::{DataModel, Group, Param, Slot};

/// Filter amp and cab list to contain the "BX-..." items only,
/// stripping the "BX-" prefix along the way.
fn bx_only(map: &HashMap<u32, String>) -> HashMap<u32, String> {
    map.iter()
        .flat_map(|(key, value)| {
            if !value.starts_with("BX-") { return None }
            let value = value.strip_prefix("BX-").unwrap();

            Some((*key, value.to_string()))
        })
        .collect()
}

fn amp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        let mut map = convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            196608 => "Bypass",
        ));
        map.extend(bx_only(podxt::amp_select()).into_iter());
        map
    })
}

fn cab_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        let mut map = convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            16973824 => "No Cab",
        ));
        map.extend(bx_only(podxt::cab_select()).into_iter());
        map
    })
}

fn mic_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        podxt::mic_select().iter().map(|(key, value)| {
            let value = value.split("BX: ").skip(1).next().unwrap();
            (*key, value.to_string())
        })
        .collect()
    })
}

fn stomp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Bass Overdrive",
            1 => "Screamer",
            2 => "Classic Dist",
            3 => "Facial Fuzz",
            4 => "Fuzz Pi",
            5 => "Octave Fuzz",
            6 => "Bronze Master",
            7 => "Blue Comp",
            8 => "Red Comp",
            9 => "Vetta Comp",
            10 => "Auto Wah",
            11 => "Dingo-Tron",
            12 => "Buzz Wave",
            13 => "Seismik Synth",
            14 => "Rez Synth",
            15 => "Saturn 5 Ring M",
            16 => "Synth Analog",
            17 => "Synth FX",
            18 => "Synth Harmony",
            19 => "Synth Lead",
            20 => "Synth String",
            21 => "Sub Octaves",
        ))
    })
}

fn mod_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Deluxe Chorus",
            1 => "Analog Chorus",
            2 => "Deluxe Flange",
            3 => "Jet Flanger",
            4 => "Phaser",
            5 => "U-Vibe",
            6 => "Opto Trem",
            7 => "Bias Trem",
            8 => "Rotary Drum",
            9 => "Hi-Talk",
            10 => "Line 6 Rotor",
            11 => "Random S/H",
            12 => "Tape Eater",
        ))
    })
}

fn delay_reverb_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            // delay
            0 => "Analog Delay",
            1 => "Analog w/Mod",
            2 => "Tube Echo",
            3 => "Multi-Head",
            4 => "Sweep Echo",
            5 => "Digital Delay",
            6 => "Reverse Delay",
            // reverb
            7 => "Lux Spring",
            8 => "Std Spring",
            9 => "King Spring",
            10 => "Small Room",
            11 => "Tiled Room",
            12 => "Brite Room",
            13 => "Dark Hall",
            14 => "Medium Hall",
            15 => "Large Hall",
            16 => "Rich Chamber",
            17 => "Chamber",
            18 => "Cavernous",
            19 => "Slap Plate",
            20 => "Vintage Plate",
            21 => "Large Plate",
        ))
    })
}

/// For the given select values look up PODxt slots from the provided
/// list of PODxt slots and PODxt select values. If none found and/or
/// if an override is found, that is used instead.
fn duplicate_slots(
    select_name: &str,
    select: &HashMap<u32, String>, overrides: &Vec<(&str, Slot)>,
    podxt_slots: &Vec<Slot>, podxt_select: &HashMap<u32, String>,
) -> Vec<Slot> {
    let mut slots = Vec::with_capacity(select.len());
    for (id, name) in select.iter() {
        // get PODxt slot id for this select value
        let xt_id = podxt_select.iter()
            .find_map(|(xt_id, xt_name)| {
                let xt_name = xt_name.strip_prefix("FX-").unwrap_or(xt_name);
                if xt_name != name { return None }
                Some(*xt_id)
            });
        // get the PODxt slot for this value (okay if None)
        let xt_slot = podxt_slots.iter()
            .find(|s| {
                s.params.iter().find(|p| {
                    match (p, xt_id) {
                        (Param::FixedParam { name, param_value, .. }, Some(xt_id)) => {
                            name == select_name && *param_value == xt_id
                        }
                        _ => false
                    }
                }).is_some()
            });
        // override PODxt slot with a Bass PODxt one if a specific one is found
        let xt_slot = overrides.iter()
            .find_map(|(slot_name, slot)| {
                if slot_name == name {
                    Some(slot)
                } else {
                    None
                }
            }).or(xt_slot).unwrap_or_else(|| panic!("No slot found for {:?}", &name));

        let mut params = vec![
            fixed_int(select_name, *id)
        ];
        for param in xt_slot.params.iter() {
            match param {
                Param::FixedParam { name, .. } => {
                    if name == select_name { continue }
                }
                _ => {}
            }
            params.push(param.clone());
        }

        let slot = Slot {
            fixed_slot: xt_slot.fixed_slot,
            fixed_model: xt_slot.fixed_model,
            fixed_enable: xt_slot.fixed_enable,
            params
        };
        slots.push(slot);
    }

    slots
}

fn replace_slot_params(slot: Slot, override_params: &[Param]) -> Slot {
    let mut params = Vec::with_capacity(slot.params.len());
    for param in slot.params.into_iter() {
        match param.get_id() {
            None => {
                // parameter without id, use as-is
                params.push(param);
            }
            Some(id) => {
                let new_param = override_params.iter().find(|p| {
                   p.get_id() == Some(id)
                });
                match new_param {
                    None => {
                        // no override found, use as-is
                        params.push(param);
                    }
                    Some(new_param) => {
                        // override found, use it
                        params.push(new_param.clone());
                    }
                }
            }
        }
    }
    Slot { params, ..slot }
}

fn replace_all_slot_params(slots: Vec<Slot>, override_params: &[Param]) -> Vec<Slot> {
    slots.into_iter()
        .map(|slot| replace_slot_params(slot, override_params))
        .collect()
}

fn basspodxt_specific_stomp_slots() -> &'static Vec<(&'static str, Slot)> {
    static LIST: OnceLock<Vec<(&'static str, Slot)>> = OnceLock::new();
    LIST.get_or_init(|| {
        vec![
            // Bass PODxt "Bronze Master" stomp has the same model
            // id as the PODxt one, but the "stomp_tone" control moves
            // in 8 steps like the wave control.
            ("Bronze Master", stomp_slot(0, 0x2050009, &[
                Some("stomp_drive"),
                Some("stomp_tone"), // TODO
                None,
                Some("stomp_blend"),
                None
            ])),
            // Bass PODxt "Sub Octaves" stomp has a different model number
            ("Sub Octaves", stomp_slot(0, 0x20a0008, &[
                Some("stomp_1octg"),
                Some("stomp_2octg"),
                None,
                Some("stomp_mix"),
                None
            ])),
        ]
    })
}

// Bass PODxt stomp slots are mostly the same as the PODxt ones (with a few exceptions)
fn stomp_slots() -> Vec<Slot> {
    let podxt_stomp_slots = &podxt_data_model().groups.iter()
        .find(|g| g.name == "Stomp")
        .unwrap().slots;

    duplicate_slots(
        "stomp_select", stomp_select(), basspodxt_specific_stomp_slots(),
        podxt_stomp_slots, podxt::stomp_select()
    )
}

fn basspodxt_specific_mod_slots() -> &'static Vec<(&'static str, Slot)> {
    static LIST: OnceLock<Vec<(&'static str, Slot)>> = OnceLock::new();
    LIST.get_or_init(|| {
        vec![
            ("Deluxe Chorus", mod_slot5(0, 0x2030016, &[
                Some("mod_depth"),
                Some("mod_pre_delay"),
                Some("mod_feedback"),
                Some("mod_wave"), // TODO
                Some("mod_mix"),
            ])),
            ("Deluxe Flange", mod_slot5(0, 0x2030006, &[
                Some("mod_depth"),
                Some("mod_pre_delay"),
                Some("mod_feedback"),
                Some("mod_wave"), // TODO
                Some("mod_mix"),
            ])),
            // Bass PODxt "Rotary Drum" is PODxt "RotaryDrum+Horn"
            ("Rotary Drum", mod_slot(0, 0x203000c, &[
                 None, Some("mod_tone"), None, Some("mod_mix"),
            ])),
            ("Line 6 Rotor", mod_slot(0, 0x203001b, &[
                Some("mod_depth"), Some("mod_q"), None, Some("mod_mix"),
            ])),
        ]
    })
}

// Bass PODxt mod slots are mostly the same as the PODxt ones (with a few exceptions),
// except there is also a mod-specific di-xover parameter
fn mod_slots() -> Vec<Slot> {
    let podxt_mod_slots = &podxt_data_model().groups.iter()
        .find(|g| g.name == "Modulation")
        .unwrap().slots;

    let slots = duplicate_slots(
        "mod_select", mod_select(), basspodxt_specific_mod_slots(),
        podxt_mod_slots, podxt::mod_select()
    );

    replace_all_slot_params(slots, &[
        float(0x010002, "mod_di_xover")
    ])
}


fn delay_slot(select: u32, model: u32, params: &[Option<&str>; 4]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: Some(0x20005),
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int("delay_reverb_select", select),
            slot_enable("delay_reverb_enable"),

            float(0x100000, "delay_time"), // 0..1 = 20 .. 2000 ms
            param(0, 0x100001), // param 1
            param(1, 0x100002), // param 2
            param(2, 0x100003), // param 3
            ignore(0x100004),
            param(3, 0x010001), // param 4
            float(0x010002, "delay_reverb_di_xover"),
            int(0x200000, "delay_note_select"),
        ]
    }
}

fn reverb_slot(select: u32, model: u32, params: &[Option<&str>; 4]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: Some(0x20005),
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int("delay_reverb_select", select),
            slot_enable("delay_reverb_enable"),

            ignore_f(0x100000),
            param(0, 0x100001), // param 1
            param(1, 0x100002), // param 2
            param(2, 0x100003), // param 3
            ignore(0x100004),
            param(3, 0x010001), // param 4
            float(0x010002, "delay_reverb_di_xover"),
            ignore(0x200000),
        ]
    }
}

fn basspodxt_data_model_all() -> &'static DataModel {
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
                            ignore_f(0x200017), // amp_bypass_volume on PODxt
                            float(0x200001, "tempo"), // BPM
                            float(0x200019, "di_model"), // %
                            float(0x20001a, "di_delay"), // 0 .. 1 = 0.0 .. 12.7 ms

                            int(0x200006, "pedal_assign"),
                            int(0x200004, "tweak_param_select"),

                            // Pro and Live extras are exactly the same as PODxt
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
                            int(0x200018, "live.footswitch_mode"),
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

                            float(0x100003, "drive"), // %
                            float(0x100000, "bass"), // %
                            float(0x100001, "low_mid"), // %
                            float(0x100002, "high_mid"), // %
                            float(0x100004, "treble"), // %
                            float(0x100005, "chan_volume"), // %
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
                            slot("cab_select"),
                            int(0x000000, "mic_select"),
                        ]
                    },
                    Slot {
                        fixed_slot: Some(0x30003),
                        fixed_model: Some(0x20b0002),
                        fixed_enable: None,
                        params: vec![
                            float(0x100000, "room"), // %
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
                            float(0, "gate_threshold"), // -96 .. 0 dB
                            float(3, "gate_decay"), // %
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
                            float(0x100000, "comp_amount"), // %
                        ]
                    },
                ]
            },
            Group {
                name: "Stomp".into(),
                slots: stomp_slots()
            },
            Group {
                name: "Modulation".into(),
                slots: mod_slots()
            },
            Group {
                name: "Delay / Reverb".into(),
                slots: vec![
                    // delay
                    delay_slot(0, 0x2020002, &[
                        Some("delay_feedback"), Some("delay_bass"), Some("delay_treble"), Some("delay_mix"),
                    ]),
                    delay_slot(1, 0x2020003, &[
                        Some("delay_feedback"), Some("delay_mod_speed"), Some("delay_depth"), Some("delay_mix"),
                    ]),
                    delay_slot(2, 0x2020004, &[
                        Some("delay_feedback"), Some("delay_flutter"), Some("delay_drive"), Some("delay_mix"),
                    ]),
                    delay_slot(3, 0x2020005, &[
                        Some("delay_feedback"), Some("delay_heads"), Some("delay_flutter"), Some("delay_mix"),
                    ]),
                    delay_slot(4, 0x2020006, &[
                        Some("delay_feedback"), Some("delay_speed"), Some("delay_depth"), Some("delay_mix"),
                    ]),
                    delay_slot(5, 0x2020007, &[
                        Some("delay_feedback"), Some("delay_bass"), Some("delay_treble"), Some("delay_mix"),
                    ]),
                    delay_slot(6, 0x202000a, &[
                        Some("delay_feedback"), None, None, Some("delay_mix"),
                    ]),
                    // reverb (different order of params than in PODxt)
                    reverb_slot(7, 0x2040002, &[
                        Some("reverb_dwell"), None, Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(8, 0x2040003, &[
                        Some("reverb_dwell"), None, Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(9, 0x2040004, &[
                        Some("reverb_dwell"), None, Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(10, 0x2040005, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(11, 0x2040006, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(12, 0x2040007, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(13, 0x2040008, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(14, 0x2040009, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(15, 0x204000a, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(16, 0x204000b, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(17, 0x204000c, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(18, 0x204000d, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(19, 0x204000e, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(20, 0x204000f, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(21, 0x2040010, &[
                        Some("reverb_decay"), Some("reverb_pre_delay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                ]
            },
            Group {
                name: "EQ".into(),
                slots: vec![
                    Slot {
                        fixed_slot: None,
                        fixed_model: Some(0x20c0002),
                        fixed_enable: None,
                        params: vec![
                            fixed_int_for_slot_id("eq_position", 0, 0x20007),
                            fixed_int_for_slot_id("eq_position", 1, 0x50006),
                            slot_enable("eq_enable"),

                            float(0x100000, "eq_1_freq"), // 0..1 = 50 .. 450 Hz, not linear
                            float(0x100001, "eq_1_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100002, "eq_2_freq"), // 0..1 = 20 .. 660 Hz
                            float(0x100003, "eq_2_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100004, "eq_3_freq"), // 0..1 = 50 .. 1010 Hz, not linear
                            float(0x100005, "eq_3_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100006, "eq_4_freq"), // 0..1 = 100 .. 2050 Hz, not linear
                            float(0x100007, "eq_4_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100008, "eq_5_freq"), // 0..1 = 20 .. 14200 Hz, not linear
                            float(0x100009, "eq_5_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x10000a, "eq_6_freq"), // 0..1 = 500 .. 3000 Hz, not linear
                            float(0x10000b, "eq_6_gain"), // 0..1 = -12.8 .. 12.6 dB
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
                            ignore(0x000003),
                        ]
                    },
                ]
            },
            Group {
                name: "Wah pedal".into(),
                slots: vec![
                    Slot {
                        fixed_slot: Some(0x20002),
                        fixed_model: Some(0x2060000),
                        fixed_enable: None,
                        params: vec![
                            slot_enable("wah_enable"),

                            float(1, "wah_level"),
                        ]
                    },
                ]
            },
        ];

        let info_map = convert_args!(hashmap!(
            "amp_select" => lookup(amp_select()),
            "drive" => percent(),
            "bass" => percent(),
            "low_mid" => percent(),
            "high_mid" => percent(),
            "treble" => percent(),
            "chan_volume" => percent(),

            "cab_select" => lookup(cab_select()),
            "mic_select" => lookup(mic_select()),
            "room" => percent(),

            "gate_threshold" => db().range(-96.0, 0.0),
            "gate_decay" => percent(),

            "comp_amount" => percent(),

            "stomp_select" => lookup(stomp_select()),
            "stomp_drive" => percent(),
            "stomp_gain" => percent(),
            "stomp_tone" => percent(),
            "stomp_sustain" => percent(),
            "stomp_level" => percent(),
            "stomp_sens" => percent(),
            "stomp_q" => percent(),
            "stomp_bass" => percent(),
            "stomp_treble" => percent(),
            "stomp_sustain" => percent(),
            "stomp_decay" => percent(),
            "stomp_wave" => lookup_f(wave_value()).convert(128.0, 0.0, 0.0),
            "stomp_mix" => percent(),
            "stomp_1octg" => percent(),
            "stomp_2octg" => percent(),
            "stomp_filter" => percent(),
            "stomp_1m335" => lookup_f(_1m335_value()).convert(128.0, 0.0, 0.0),
            "stomp_1457" => lookup_f(_1457_value()).convert(128.0, 0.0, 0.0),
            "stomp_wave_p" => percent(),
            "stomp_blend" => percent(),

            "mod_select" => lookup(mod_select()),
            "mod_speed" => hz().from_to(0.0, 0.1, 1.0, 15.0),
            "mod_note_select" => lookup(note_select()),
            "mod_depth" => percent(),
            "mod_bass" => percent(),
            "mod_treble" => percent(),
            "mod_mix" => percent(),
            "mod_feedback" => percent(),
            "mod_manual" => percent(),
            "mod_wave" => percent(),
            "mod_tone" => percent(),
            "mod_pre_delay" => percent(),
            "mod_q" => percent(),
            "mod_flutter" => percent(),
            "mod_distortion" => percent(),

            "delay_reverb_select" => lookup(delay_reverb_select()),
            "delay_time" => millis().from_to(0.0, 20.0, 1.0, 2000.0),
            "delay_note_select" => lookup(note_select()),
            "delay_feedback" => percent(),
            "delay_bass" => percent(),
            "delay_treble" => percent(),
            "delay_mix" => percent(),
            "delay_mod_speed" => percent(),
            "delay_depth" => percent(),
            "delay_flutter" => percent(),
            "delay_drive" => percent(),
            "delay_heads" => lookup_f(heads_value()).convert(128.0, 0.0, 0.0),
            "delay_speed" => percent(),

            "reverb_dwell" => percent(),
            "reverb_tone" => percent(),
            "reverb_mix" => percent(),
            "reverb_pre_delay" => percent(),
            "reverb_decay" => percent(),

            "eq_1_freq" => hz().points_l6e(&[
                (0, 50.0), (16, 56.25), (32, 75.0), (48, 106.25), (64, 150.0),
                (80, 206.25), (96, 275.0), (112, 356.25), (128, 450.0)]),
            "eq_1_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_2_freq" => hz().points_l6e(&[(0, 20.0), (128, 660.0)]),
            "eq_2_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_3_freq" => hz().points_l6e(&[(0, 50.0), (64, 370.0), (128, 1010.0)]),
            "eq_3_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_4_freq" => hz().points_l6e(&[(0, 100.0), (32, 260.0), (96, 900.0), (128, 2500.0)]),
            "eq_4_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_5_freq" => hz().points_l6e(&[(0, 200.0), (48, 1400.0), (80, 3000.0), (112, 6200.0), (128, 14200.0)]),
            "eq_5_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_6_freq" => hz().points_l6e(&[
                (0, 500.0), (16, 525.0), (32, 562.5), (48, 734.375), (64, 1000.0),
                (80, 1359.4), (96, 1812.5), (112, 2359.4), (128, 3000.0)]),
            "eq_6_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),

            "vol_min" => percent(),

            "wah_level" => percent(),

            "tempo" => bpm(),
            "pedal_assign" => lookup(pedal_assign_select()),
            "di_model" => percent(),
            "di_delay" => millis1().from_to(0.0, 0.0, 1.0, 12.7),
            "mod_di_xover" => hz().points_l6e(&[(0, 0.0), (128, 800.0)]),
            "delay_reverb_di_xover" => hz().points_l6e(&[(0, 0.0), (128, 800.0)]),

            "live.footswitch_mode" => lookup(footswitch_mode_select()),
        ));

        DataModel {
            floats_as_ints: false,
            groups,
            info_map
        }
    })
}

pub fn basspodxt_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(basspodxt_data_model_all(), &["pro.", "live."], &[])
    )
}

pub fn basspodxt_pro_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(basspodxt_data_model_all(), &["live."], &["pro."])
    )
}

pub fn basspodxt_live_data_model() -> &'static DataModel {
    static MODEL: OnceLock<DataModel> = OnceLock::new();
    MODEL.get_or_init(||
        filter_params_by_prefix(basspodxt_data_model_all(), &["pro."], &["live."])
    )
}
