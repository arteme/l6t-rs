use std::collections::HashSet;
use std::sync::OnceLock;

use regex::Regex;

use crate::data::{data_models, DataModelInfo};

const OK_PARAMS: &[&str] = &[
    // known booleans
    ".*_enable", ".*_position", ".*_boost",
    // TODO: these should be gone through and have info added some day
    "tweak_param_select", "variax_*",
];

fn ok_params_res() -> &'static [Regex] {
    static RES: OnceLock<Vec<Regex>> = OnceLock::new();
    RES.get_or_init(|| {
        OK_PARAMS.iter()
            .map(|p| Regex::new(p).unwrap())
            .collect::<Vec<_>>()
    })
}

fn partition<'a>(data: &Vec<&'a String>, res: &[Regex]) -> (Vec<&'a String>, Vec<&'a String>) {
    let mut t = vec![];
    let mut f = vec![];
    for item in data.iter() {
        if res.iter().any(|re| re.is_match(item)) {
            t.push(*item)
        } else {
            f.push(*item)
        }
    }

    (t, f)
}

#[test]
fn test_param_info_maps() {

    fn test_data_model(model_info: &DataModelInfo) -> bool {
        let mut ok = true;
        eprintln!("Data model: {} ------------------------", model_info.name);

        let all_named_params =
            model_info.model.groups.iter()
                .flat_map(|g| g.slots.iter())
                .flat_map(|s| s.params.iter())
                .filter(|p| p.get_name().is_some())
                .collect::<Vec<_>>();
        let all_param_names = all_named_params.iter()
            .map(|p| p.get_name().unwrap())
            .collect::<HashSet<&String>>();

        let info_map_param_names = model_info.model.info_map.keys()
            .collect::<HashSet<&String>>();

        let params_without_info =
            all_param_names.difference(&info_map_param_names)
                .cloned()
                .collect::<Vec<&String>>();

        let (mut params_ok, mut params_nok) = partition(&params_without_info, ok_params_res());
        params_ok.sort();
        params_nok.sort();
        ok &= params_nok.is_empty();
        eprintln!("Params without info: {}", params_without_info.len());
        eprintln!("   OK: {:?}", params_ok);
        eprintln!("  NOK: {:?}", params_nok);

        let mut info_without_params =
            info_map_param_names.difference(&all_param_names)
                .map(|s| *s)
                .collect::<Vec<&String>>();

        eprintln!("Info without params: {}", info_without_params.len());
        if !info_without_params.is_empty() {
            info_without_params.sort();
            ok = false;
            eprintln!("  NOK: {:?}", info_without_params);
        }

        eprintln!("\n");
        ok
    }

    let mut nok_models = vec![];
    for (_, model_info) in data_models() {
        if !test_data_model(model_info) {
            nok_models.push(model_info.name);
        }
    }
    assert!(nok_models.is_empty(),
            "Errors in {} data models: {:?}", nok_models.len(), nok_models);
}
