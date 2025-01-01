use std::collections::HashMap;
use std::convert::identity;
use std::sync::OnceLock;
use maplit::{convert_args, hashmap};
use crate::data::shorthand::*;
use crate::data::models::filter_params_by_prefix;
use crate::model::{DataModel, Group, Param, Slot};


fn amp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            131072 => "No Amp",
            131118 => "Tube Preamp",
            131073 => "Line 6 Clean",
            131077 => "Line 6 JTS-45",
            131078 => "Line 6 Class A",
            131079 => "Line 6 Mood",
            131080 => "Spinal Puppet",
            131084 => "Line 6 Insane",
            131087 => "Line 6 ACO 2",
            131088 => "Zen Master",
            131089 => "Small Tweed",
            131090 => "Tweed B-Man",
            131091 => "Tiny Tweed",
            131092 => "Blackface Lux",
            131093 => "Double Verb",
            131096 => "Two-Tone",
            131097 => "Hiway 100",
            131098 => "Plexi 45",
            131099 => "Plexi Lead 100",
            131101 => "Plexi Jump Lead",
            131102 => "Plexi Variac",
            131104 => "Brit J-800",
            131105 => "Brit JM Pre",
            131106 => "Match Chief",
            131107 => "Match D-30",
            131108 => "Treadplate Dual",
            131110 => "Cali Crunch",
            131111 => "Jazz Clean",
            131113 => "Solo 100",
            131114 => "Super O",
            131116 => "Class A-15",
            131117 => "Class A-30 TB",
            131133 => "Line 6 Agro",
            131132 => "Line 6 Lunatic",
            131131 => "Line 6 Treadplate",
            131122 => "Variax Acoustic",
            131119 => "MS-Bomber Uber",
            131134 => "MS-Connor 50",
            131136 => "MS-Deity Lead",
            131137 => "MS-Deity's Son",
            131121 => "MS-Angel P-Ball",
            131141 => "MS-Brit Silver",
            131142 => "MS-Brit J-900 Cln",
            131143 => "MS-Brit J-900 Dst",
            131144 => "MS-Brit J-2000",
            131109 => "MS-Dismondplate",
            131145 => "MS-Criminal",
            131130 => "MS-L6 Big Bottom",
            131129 => "MS-L6 Chunk Chunk",
            131128 => "MS-L6 Fuzz",
            131185 => "MS-L6 Octone",
            131127 => "MS-L6 Smash",
            131125 => "MS-L6 Sparkle Cln",
            131181 => "MS-L6 Throttle",
            131120 => "CC-Bomber X-TC",
            131135 => "CC-Deity Crunch",
            131138 => "CC-Blackface Vibro",
            131139 => "CC-Double Show",
            131140 => "CC-Silverface Bass",
            131094 => "CC-Mini Double",
            131095 => "CC-Gibtone Expo",
            131100 => "CC-Brit Bass",
            131103 => "CC-Brit Major",
            131112 => "CC-Silver Twelve",
            131115 => "CC-Super O Thunder",
            131076 => "CC-L6 Bayou",
            131126 => "CC-L6 Crunch",
            131083 => "CC-L6 Purge",
            131074 => "CC-L6 Sparkle",
            131123 => "CC-L6 Super Cln",
            131124 => "CC-L6 Super Spark",
            131075 => "CC-L6 Twang",
            196609 => "BX-Tube Preamp",
            196610 => "BX-L6 Classic Jazz",
            196611 => "BX-L6 Brit Invader",
            196612 => "BX-L6 Super Thor",
            196613 => "BX-L6 Frankenstein",
            196614 => "BX-L6 Ebony Lux",
            196615 => "BX-L6 Doppleganger",
            196616 => "BX-L6 Sub Dub",
            196617 => "BX-Amp 360",
            196618 => "BX-Jaguar",
            196619 => "BX-Alchemist",
            196620 => "BX-Rock Classic",
            196621 => "BX-Flip Top",
            196622 => "BX-Adam and Eve",
            196623 => "BX-Tweed B-Man",
            196624 => "BX-Silverface Bass",
            196625 => "BX-Double Show",
            196626 => "BX-Eighties",
            196627 => "BX-Hiway 100",
            196628 => "BX-Hiway 200",
            196629 => "BX-British Major",
            196630 => "BX-British Bass",
            196631 => "BX-California",
            196632 => "BX-Jazz Tone",
            196633 => "BX-Stadium",
            196634 => "BX-Studio Tone",
            196635 => "BX-Motor City",
            196636 => "BX-Brit Class A100",
            131146 => "Citrus D-30",
            131147 => "L6 Modern Hi Gain",
            131148 => "L6 Boutique #1",
            131149 => "Class A-30 Fawn",
            131150 => "Brit Gain 18",
            131151 => "Brit J-2000 #2",
        ))
    })
}

fn cab_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            16908288 => "No Cab",
            16908289 => "1x6 Super O",
            16908290 => "1x8 Tweed",
            16908291 => "1x10 Gibtone",
            16908292 => "1x10 G-Brand",
            16908293 => "1x12 Line 6",
            16908294 => "1x12 Tweed",
            16908295 => "1x12 Blackface",
            16908296 => "1x12 Class A",
            16908297 => "2x2 Mini T",
            16908298 => "2x12 Line 6",
            16908299 => "2x12 Blackface",
            16908300 => "2x12 Match",
            16908301 => "2x12 Jazz",
            16908302 => "2x12 Class A",
            16908303 => "4x10 Line 6",
            16908304 => "4x10 Tweed",
            16908305 => "4x12 Line 6",
            16908306 => "4x12 Green 20's",
            16908307 => "4x12 Green 25's",
            16908308 => "4x12 Brit T75",
            16908309 => "4x12 Brit V30's",
            16908310 => "4x12 Treadplate",
            16908312 => "1x15 Thunder",
            16908314 => "2x12 Wishbook",
            16973825 => "BX-1x12 Boutique",
            16973826 => "BX-1x12 Motor City",
            16973827 => "BX-1x15 Flip Top",
            16973828 => "BX-1x15 Jazz Tone",
            16973829 => "BX-1x18 Session",
            16973830 => "BX-1x18 Amp 360",
            16973831 => "BX-1x18 California",
            16973832 => "BX-1x18+12 Stadium",
            16973833 => "BX-2x10 Modern UK",
            16973834 => "BX-2x15 Doubleshow",
            16973835 => "BX-2x15 California",
            16973836 => "BX-2x15 Class A",
            16973837 => "BX-4x10 Line 6",
            16973838 => "BX-4x10 Tweed",
            16973839 => "BX-4x10 Adam Eve",
            16973840 => "BX-4x10 Silvercone",
            16973841 => "BX-4x10 Session",
            16973842 => "BX-4x12 Hiway",
            16973843 => "BX-4x12 Green 20's",
            16973844 => "BX-4x12 Green 25's",
            16973845 => "BX-4x15 Big Boy",
            16973846 => "BX-8x10 Classic",
        ))
    })
}

fn mic_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            // TODO: some way to separate these normal/BX values
            0 => "57 on axis / BX: tube 47 close",
            1 => "57 off axis / BX: tube 74 far ",
            2 => "421 dynamic / BX: 112 dynamic",
            3 => "67 condenser / BX: 20 dynamic"
        ))
    })
}

fn stomp_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Facial Fuzz",
            1 => "Fuzz Pi",
            2 => "Screamer",
            3 => "Classic Dist",
            4 => "Octave Fuzz",
            5 => "Blue Comp",
            6 => "Red Comp",
            7 => "Vetta Comp",
            8 => "Auto Swell",
            9 => "Auto Wah",
            10 => "FX-Killer Z",
            11 => "FX-Tube Drive",
            12 => "FX-Vetta Juice",
            13 => "FX-Boost + EQ",
            14 => "FX-Blue Comp Treb",
            15 => "FX-Dingo-Tron",
            16 => "FX-Clean Sweep",
            17 => "FX-Seismik Synth",
            18 => "FX-Double Bass",
            19 => "FX-Buzz Wave",
            20 => "FX-Rez Synth",
            21 => "FX-Saturn 5 Ring M",
            22 => "FX-Synth Analog",
            23 => "FX-Synth FX",
            24 => "FX-Synth Harmony",
            25 => "FX-Synth Lead",
            26 => "FX-Synth String",
            27 => "Bass Overdrive",
            28 => "Bronze Master",
            29 => "Sub Octaces",
            30 => "Bender",
        ))
    })
}
/*
fn mod_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
        ))
    })
}
*/
fn mod_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Sine Chorus",
            1 => "Analog Chorus",
            2 => "Line 6 Flanger",
            3 => "Jet Flanger",
            4 => "Phaser",
            5 => "U-Vibe",
            6 => "Opto Trem",
            7 => "Bias Trem",
            8 => "RotaryDrum+Horn",
            9 => "Rotary Drum",
            10 => "Auto Pan",
            11 => "FX-Analog Square",
            12 => "FX-Square Chorus",
            13 => "FX-Expo Chorus",
            14 => "FX-Random Chorus",
            15 => "FX-Square Flange",
            16 => "FX-Expo Flange",
            17 => "FX-Lumpy Phase",
            18 => "FX-Hi-Talk",
            19 => "FX-Sweeper",
            20 => "FX-POD Purple X",
            21 => "FX-Random S/H",
            22 => "FX-Tape Eater",
            23 => "FX-Warble-Matic",
        ))
    })
}

fn note_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "off",
            1 => "whole",
            2 => "dotted half",
            3 => "half",
            4 => "half note triplet",
            5 => "dotted quarter",
            6 => "quarter",
            7 => "quarter note triplet",
            8 => "dotted eighth",
            9 => "eighth",
            10 => "eighth note triplet",
            11 => "dotted sixteenth",
            12 => "sixteenth",
            13 => "sixteenth note triplet",
        ))
    })
}

fn wave_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "wave 1"),
            (16, "wave 2"),
            (32, "wave 3"),
            (48, "wave 4"),
            (64, "wave 5"),
            (80, "wave 6"),
            (96, "wave 7"),
            (112, "wave 8"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

fn _1m335_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "-1 oct"),
            (16, "-maj 6th"),
            (32, "-min 6th"),
            (48, "-4th"),
            (64, "unison"),
            (80, "min 3rd"),
            (96, "maj 3rd"),
            (112, "5th"),
            (127, "1 oct"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

fn _1457_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "-1 oct"),
            (16, "-5th"),
            (32, "-4th"),
            (48, "-2nd"),
            (64, "unison"),
            (80, "4th"),
            (96, "5th"),
            (112, "7th"),
            (127, "1 oct"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

fn heel_toe_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        (-24 ..= 24).enumerate().map(|(i, n)| {
            let s = if n == 0 { "0".into() } else { format!("{n:+}") };
            if i == 0 {
                (0u32, s)
            } else {
                (16 + (i * 2) as u32, s)
            }
        })/*.map(|x| { println!("{:?}", x); x })*/
            .collect()
    })
}

fn delay_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Analog Delay",
            1 => "Analog w/Mod",
            2 => "Tube Echo",
            3 => "Multi-Head",
            4 => "Sweep Echo",
            5 => "Digital Delay",
            6 => "Stereo Delay",
            7 => "Ping Pong",
            8 => "Reverse",
            9 => "FX-Echo Platter",
            10 => "FX-Tape Echo",
            11 => "FX-Low Rez",
            12 => "FX-Phaze Eko",
            13 => "FX-Bubble Echo",
        ))
    })
}

fn reverb_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "Lux Spring",
            1 => "Std Spring",
            2 => "King Spring",
            3 => "Small Room",
            4 => "Tiled Room",
            5 => "Brite Room",
            6 => "Dark Hall",
            7 => "Medium Hall",
            8 => "Large Hall",
            9 => "Rich Chamber",
            10 => "Chamber",
            11 => "Cavernous",
            12 => "Slap Plate",
            13 => "Vintage Plate",
            14 => "Large Plate",
        ))
    })
}

fn wah_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            33947648 => "Vetta Wah",
            33947651 => "Fassel",
            33947652 => "Stomp Gain",
            33947653 => "Chrome",
            33947654 => "Chrome Custom",
            33947655 => "Throaty",
            33947656 => "Conductor",
            33947657 => "Colorful",
        ))
    })
}

fn pedal_assign_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "1-Wah 2-Vol",
            1 => "1-Tweak 2-Vol",
            2 => "1-Wah/Vol 2-Tweak"
        ))
    })
}

fn footswitch_mode_select() -> &'static HashMap<u32, String> {
    static SELECT: OnceLock<HashMap<u32, String>> = OnceLock::new();
    SELECT.get_or_init(|| {
        convert_args!(keys=identity::<u32>, values=String::from, hashmap!(
            0 => "amp",
            1 => "comp",
        ))
    })
}

fn heads_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "12--"),
            (16, "1-3-"),
            (32, "1--4"),
            (48, "-23-"),
            (64, "123-"),
            (80, "12-4"),
            (96, "1-34"),
            (112, "-234"),
            (127, "1234"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

fn bits_value() -> &'static Vec<(u32, String)> {
    static MAP: OnceLock<Vec<(u32, String)>> = OnceLock::new();
    MAP.get_or_init(|| {
        vec![
            (0, "12"),
            (16, "11"),
            (32, "10"),
            (48, "9"),
            (64, "8"),
            (80, "7"),
            (96, "6"),
            (112, "5"),
            (127, "4"),
        ]
        .into_iter().map(|(k,v)| (k, v.to_string())).collect()
    })
}

fn stomp_slot(select: u32, model: u32, params: &[Option<&str>; 5]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: Some(0x20003),
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int("stomp_select", select),
            slot_enable("stomp_enable"),

            param(0, 0x100001), // param 1
            param(1, 0x100002), // param 2
            param(2, 0x100003), // param 3
            param(3, 0x100004), // param 4
            param(4, 0x100005), // param 5
        ]
    }
}

fn mod_slot(select: u32, model: u32, params: &[Option<&str>; 4]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: None,
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int_for_slot_id("mod_position", 0, 0x20004),
            fixed_int_for_slot_id("mod_position", 1, 0x50003),
            fixed_int("mod_select", select),
            slot_enable("mod_enable"),

            float(0x100000, "mod_speed"), // 0..1 = 0.10 .. 15.00 Hz
            param(0, 0x100001), // param 1
            param(1, 0x100002), // param 2
            param(2, 0x100003), // param 3
            ignore(0x100004),
            param(3, 0x010001), // param 4
            ignore_f(0x010002),
            int(0x200000, "mod_note_select"),
        ]
    }
}

fn delay_slot(select: u32, model: u32, params: &[Option<&str>; 4]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: None,
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int_for_slot_id("delay_position", 0, 0x20005),
            fixed_int_for_slot_id("delay_position", 1, 0x50004),
            fixed_int("delay_select", select),
            slot_enable("delay_enable"),

            float(0x100000, "delay_time"), // 0..1 = 20 .. 2000 ms
            param(0, 0x100001), // param 1
            param(1, 0x100002), // param 2
            param(2, 0x100003), // param 3
            ignore(0x100004),
            param(3, 0x010001), // param 4
            float(0x010002, "di_xover"),
            int(0x200000, "delay_note_select"),
        ]
    }
}

fn reverb_slot(select: u32, model: u32, params: &[Option<&str>; 4]) -> Slot {
    let param = |i: usize, id| {
        params[i].map(|n| float(id, n)).unwrap_or_else(|| ignore(id))
    };

    Slot {
        fixed_slot: None,
        fixed_model: Some(model),
        fixed_enable: None,
        params: vec![
            fixed_int_for_slot_id("reverb_position", 0, 0x20006),
            fixed_int_for_slot_id("reverb_position", 1, 0x50005),
            fixed_int("reverb_select", select),
            slot_enable("reverb_enable"),

            param(0, 0x100001), // param 1
            param(1, 0x100000), // param 2
            param(2, 0x100002), // param 3
            param(3, 0x010002), // param 4
        ]
    }
}

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
                            float(0x200001, "tempo"), // BPM
                            float(0x200019, "di_model"), // %
                            float(0x20001a, "di_delay"), // 0 .. 1 = 0.0 .. 12.7 ms

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
                            float(0x100001, "mid"), // %
                            float(0x100002, "treble"), // %
                            float(0x100004, "presence"), // %
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
                            float(0x100000, "comp_threshold"), // 0..1 for -63..0 dB
                            float(0x100001, "comp_gain"), // 0..1 for 0..16 dB
                        ]
                    },
                ]
            },
            Group {
                name: "Stomp".into(),
                slots: vec![
                    stomp_slot(0, 0x2050000, &[
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        Some("stomp_tone"),
                        None, None
                    ]),
                    stomp_slot(1, 0x2050002, &[
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        Some("stomp_tone"),
                        None, None
                    ]),
                    stomp_slot(2, 0x2050003, &[
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        Some("stomp_tone"),
                        None, None
                    ]),
                    stomp_slot(3, 0x2050005, &[
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        Some("stomp_tone"),
                        None, None
                    ]),
                    stomp_slot(4, 0x2050004, &[
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        Some("stomp_tone"),
                        None, None
                    ]),
                    stomp_slot(5, 0x2000002, &[
                        Some("stomp_sustain"),
                        Some("stomp_level"),
                        None, None, None
                    ]),
                    stomp_slot(6, 0x2000004, &[
                        Some("stomp_sustain"),
                        Some("stomp_level"),
                        None, None, None
                    ]),
                    stomp_slot(7, 0x2000005, &[
                        Some("stomp_sens"),
                        Some("stomp_level"),
                        None, None, None
                    ]),
                    stomp_slot(8, 0x2000007, &[
                        Some("stomp_ramp"),
                        Some("stomp_depth"),
                        None, None, None
                    ]),
                    stomp_slot(9, 0x20a0000, &[
                        Some("stomp_sens"),
                        Some("stomp_q"),
                        None, None, None
                    ]),
                    stomp_slot(10, 0x2050006, &[
                        Some("stomp_drive"),
                        Some("stomp_contour"),
                        Some("stomp_gain"),
                        Some("stomp_mid"),
                        Some("stomp_midfreq"),
                    ]),
                    stomp_slot(11, 0x2050001, &[
                        Some("stomp_drive"),
                        Some("stomp_treble"),
                        Some("stomp_gain"),
                        Some("stomp_bass"),
                        None
                    ]),
                    stomp_slot(12, 0x2000006, &[
                        Some("stomp_amount"),
                        Some("stomp_level"),
                        None, None, None
                    ]),
                    stomp_slot(13, 0x2050007, &[
                        Some("stomp_gain"),
                        Some("stomp_bass"),
                        Some("stomp_treble"),
                        Some("stomp_mid"),
                        Some("stomp_midfreq"),
                    ]),
                    stomp_slot(14, 0x2000003, &[
                        Some("stomp_level"),
                        Some("stomp_sustain"),
                        None, None, None
                    ]),
                    stomp_slot(15, 0x20a000a, &[
                        None,
                        Some("stomp_sens"),
                        Some("stomp_q"),
                        None, None
                    ]),
                    stomp_slot(16, 0x20a000b, &[
                        Some("stomp_decay"),
                        Some("stomp_sens"),
                        Some("stomp_q"),
                        None, None
                    ]),
                    stomp_slot(17, 0x20a000c, &[
                        Some("stomp_wave"),
                        None,
                        None,
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(18, 0x20a0008, &[
                        Some("stomp_1octg"),
                        Some("stomp_2octg"),
                        None,
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(19, 0x20a0005, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(20, 0x20a0006, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(21, 0x20a0007, &[
                        Some("stomp_wave"),
                        None,
                        None,
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(22, 0x20a0003, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(23, 0x20a0004, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(24, 0x20a0009, &[
                        Some("stomp_1m335"),
                        Some("stomp_1457"),
                        Some("stomp_wave_p"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(25, 0x20a0001, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(26, 0x20a0002, &[
                        Some("stomp_wave"),
                        Some("stomp_filter"),
                        Some("stomp_decay"),
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(27, 0x2050008, &[
                        Some("stomp_bass"),
                        Some("stomp_treble"),
                        Some("stomp_drive"),
                        Some("stomp_gain"),
                        None
                    ]),
                    stomp_slot(28, 0x2050009, &[
                        Some("stomp_drive"),
                        Some("stomp_tone"),
                        None,
                        Some("stomp_blend"),
                        None
                    ]),
                    stomp_slot(29, 0x20a000d, &[
                        Some("stomp_1octg"),
                        Some("stomp_2octg"),
                        None,
                        Some("stomp_mix"),
                        None
                    ]),
                    stomp_slot(30, 0x20a000e, &[
                        Some("stomp_position"),
                        Some("stomp_heel"),
                        Some("stomp_toe"),
                        Some("stomp_mix"),
                        None
                    ]),
                ]
            },
            Group {
                name: "Modulation".into(),
                slots: vec![
                    mod_slot(0, 0x2030004, &[
                        Some("mod_depth"), Some("mod_bass"), Some("mod_treble"), Some("mod_mix"),
                    ]),
                    mod_slot(1, 0x2030005, &[
                        Some("mod_depth"), Some("mod_bass"), Some("mod_treble"), Some("mod_mix"),
                    ]),
                    mod_slot(2, 0x2030006, &[
                        Some("mod_depth"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(3, 0x2030007, &[
                        Some("mod_depth"), Some("mod_feedback"), Some("mod_manual"), Some("mod_mix"),
                    ]),
                    mod_slot(4, 0x2030008, &[
                        Some("mod_feedback"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(5, 0x2030009, &[
                        Some("mod_feedback"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(6, 0x203000a, &[
                        Some("mod_wave"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(7, 0x203000b, &[
                        Some("mod_wave"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(8, 0x203000c, &[
                        None, Some("mod_tone"), None, Some("mod_mix"),
                    ]),
                    mod_slot(9, 0x203000d, &[
                        None, Some("mod_tone"), None, Some("mod_mix"),
                    ]),
                    mod_slot(10, 0x203000e, &[
                        Some("mod_wave"), None, None, Some("mod_mix"),
                    ]),
                    mod_slot(11, 0x2030015, &[
                        Some("mod_depth"), Some("mod_bass"), Some("mod_treble"), Some("mod_mix"),
                    ]),
                    mod_slot(12, 0x2030012, &[
                        Some("mod_depth"), Some("mod_pre_delay"), Some("mod_feedback"), Some("mod_mix"),
                    ]),
                    mod_slot(13, 0x203001e, &[
                        Some("mod_depth"), Some("mod_pre_delay"), Some("mod_feedback"), Some("mod_mix"),
                    ]),
                    mod_slot(14, 0x2030014, &[
                        Some("mod_depth"), Some("mod_bass"), Some("mod_treble"), Some("mod_mix"),
                    ]),
                    mod_slot(15, 0x203001f, &[
                        Some("mod_depth"), Some("mod_pre_delay"), Some("mod_feedback"), Some("mod_mix"),
                    ]),
                    mod_slot(16, 0x2030013, &[
                        Some("mod_depth"), Some("mod_pre_delay"), Some("mod_feedback"), Some("mod_mix"),
                    ]),
                    mod_slot(17, 0x2030011, &[
                        Some("mod_depth"), Some("mod_bass"), Some("mod_treble"), Some("mod_mix"),
                    ]),
                    mod_slot(18, 0x203001a, &[
                        Some("mod_depth"), Some("mod_q"), None, Some("mod_mix"),
                    ]),
                    mod_slot(19, 0x203001b, &[
                        Some("mod_depth"), Some("mod_q"), Some("mod_freq"), Some("mod_mix"),
                    ]),
                    mod_slot(20, 0x2030017, &[
                        Some("mod_feedback"), Some("mod_depth"), None, Some("mod_mix"),
                    ]),
                    mod_slot(21, 0x2030018, &[
                        Some("mod_depth"), Some("mod_q"), None, Some("mod_mix"),
                    ]),
                    mod_slot(22, 0x2030019, &[
                        Some("mod_feedback"), Some("mod_flutter"), Some("mod_distortion"), Some("mod_mix"),
                    ]),
                    mod_slot(23, 0x203001c, &[
                        Some("mod_depth"), Some("mod_q"), None, Some("mod_mix"),
                    ]),
                ]
            },
            Group {
                name: "Delay".into(),
                slots: vec![
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
                    delay_slot(6, 0x2020008, &[
                        Some("delay_offset"), Some("delay_feedback_l"), Some("delay_feedback_r"), Some("delay_mix"),
                    ]),
                    delay_slot(7, 0x2020009, &[
                        Some("delay_feedback"), Some("delay_offset"), Some("delay_spread"), Some("delay_mix"),
                    ]),
                    delay_slot(8, 0x202000a, &[
                        Some("delay_feedback"), None, None, Some("delay_mix"),
                    ]),
                    delay_slot(9, 0x202000c, &[
                        Some("delay_feedback"), Some("delay_heads"), Some("delay_flutter"), Some("delay_mix"),
                    ]),
                    delay_slot(10, 0x202000b, &[
                        Some("delay_feedback"), Some("delay_bass"), Some("delay_treble"), Some("delay_mix"),
                    ]),
                    delay_slot(11, 0x202000d, &[
                        Some("delay_feedback"), Some("delay_tone"), Some("delay_bits"), Some("delay_mix"),
                    ]),
                    delay_slot(12, 0x202000e, &[
                        Some("delay_feedback"), Some("delay_mod_speed"), Some("delay_depth"), Some("delay_mix"),
                    ]),
                    delay_slot(13, 0x202000f, &[
                        Some("delay_feedback"), Some("delay_speed"), Some("delay_depth"), Some("delay_mix"),
                    ]),
                ]
            },
            Group {
                name: "Reverb".into(),
                slots: vec![
                    reverb_slot(0, 0x2040002, &[
                        None, Some("reverb_dwell"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(1, 0x2040003, &[
                        None, Some("reverb_dwell"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(2, 0x2040004, &[
                        None, Some("reverb_dwell"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(3, 0x2040005, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(4, 0x2040006, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(5, 0x2040007, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(6, 0x2040008, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(7, 0x2040009, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(8, 0x204000a, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(9, 0x204000b, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(10, 0x204000c, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(11, 0x204000d, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(12, 0x204000e, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(13, 0x204000f, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
                    reverb_slot(14, 0x2040010, &[
                        Some("reverb_pre_delay"), Some("reverb_decay"), Some("reverb_tone"), Some("reverb_mix"),
                    ]),
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

                            float(0x100000, "eq_1_freq"), // 0..1 = 50 .. 690 Hz
                            float(0x100001, "eq_1_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100002, "eq_2_freq"), // not linear, see info
                            float(0x100003, "eq_2_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100004, "eq_3_freq"), // not linear, see info
                            float(0x100005, "eq_3_gain"), // 0..1 = -12.8 .. 12.6 dB
                            float(0x100006, "eq_4_freq"), // not linear, see info
                            float(0x100007, "eq_4_gain"), // 0..1 = -12.8 .. 12.6 dB
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

        let info_map = convert_args!(hashmap!(
            "amp_select" => lookup(amp_select()),
            "drive" => percent(),
            "bass" => percent(),
            "mid" => percent(),
            "treble" => percent(),
            "presence" => percent(),
            "chan_volume" => percent(),

            "cab_select" => lookup(cab_select()),
            "mic_select" => lookup(mic_select()),
            "room" => percent(),

            "gate_threshold" => db().range(-96.0, 0.0),
            "gate_decay" => percent(),

            "comp_threshold" => db().convert(63.0, 0.0, -63.0).range(-63.0, 0.0),
            "comp_gain" => db().convert(16.0, 0.0, 0.0).range(-63.0, 0.0),

            "stomp_select" => lookup(stomp_select()),
            "stomp_drive" => percent(),
            "stomp_gain" => percent(),
            "stomp_tone" => percent(),
            "stomp_sustain" => percent(),
            "stomp_level" => percent(),
            "stomp_sens" => percent(),
            "stomp_ramp" => percent(),
            "stomp_depth" => percent(),
            "stomp_q" => percent(),
            "stomp_contour" => percent(),
            "stomp_bass" => percent(),
            "stomp_treble" => percent(),
            "stomp_mid" => percent(),
            "stomp_midfreq" => percent(),
            "stomp_amount" => percent(),
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
            "stomp_position" => percent(),
            "stomp_heel" => lookup_f(heel_toe_value()).convert(128.0, 0.0, 0.0),
            "stomp_toe" => lookup_f(heel_toe_value()).convert(128.0, 0.0, 0.0),

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
            "mod_freq" => percent(),
            "mod_flutter" => percent(),
            "mod_distortion" => percent(),

            "delay_select" => lookup(delay_select()),
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
            "delay_offset" => percent(),
            "delay_feedback_l" => percent(),
            "delay_feedback_r" => percent(),
            "delay_spread" => percent(),
            "delay_tone" => percent(),
            "delay_bits" => lookup_f(bits_value()).convert(128.0, 0.0, 0.0),

            "reverb_select" => lookup(reverb_select()),
            "reverb_dwell" => percent(),
            "reverb_tone" => percent(),
            "reverb_mix" => percent(),
            "reverb_pre_delay" => percent(),
            "reverb_decay" => percent(),

            "eq_1_freq" => hz().points_l6e(&[(0, 50.0), (128, 690.0)]),
            "eq_1_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_2_freq" => hz().points_l6e(&[(0, 50.0), (16, 130.0), (48, 450.0), (96, 2850.0), (128, 6050.0)]),
            "eq_2_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_3_freq" => hz().points_l6e(&[(0, 100.0), (32, 1700.0), (128, 11300.0)]),
            "eq_3_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),
            "eq_4_freq" => hz().points_l6e(&[(0, 500.0), (32, 1300.0), (64, 2900.0), (128, 9300.0)]),
            "eq_4_gain" => db1().from_to(0.0, -12.8, 1.0, 12.6),

            "vol_min" => percent(),

            "wah_select" => lookup(wah_select()),
            "wah_level" => percent(),

            "amp_bypass_volume" => percent(),
            "tempo" => bpm(),
            "pedal_assign" => lookup(pedal_assign_select()),
            "di_model" => percent(),
            "di_delay" => millis1().from_to(0.0, 0.0, 1.0, 12.7),
            "di_xover" => hz().points_l6e(&[(0, 0.0), (128, 800.0)]),

            "live.footswitch_mode" => lookup(footswitch_mode_select()),
        ));

        DataModel {
            floats_as_ints: false,
            groups,
            info_map
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
