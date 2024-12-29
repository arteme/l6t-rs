use std::collections::HashMap;
use file::model::L6Patch;
use crate::model::{DataModel, get_name};
use crate::value;
use crate::value::Value;

pub struct ValueGroup<V = Value> {
    pub name: String,
    pub values: Vec<(String, V)>
}

pub fn group_values<V: Clone>(patch: &L6Patch, values: &HashMap<String, V>, model: &DataModel) -> Vec<ValueGroup<V>> {
    let mut groups: Vec<ValueGroup<V>> = vec![];

    for group in &model.groups {
        let mut group_values = vec![];
        let mut seen_names = vec![];

        for slot in &group.slots {
            let patch_model = patch.models.iter()
                .find(|m| value::model_matches_slot(m, slot));
            if patch_model.is_none() {
                continue;
            }

            for param in &slot.params {
                let Some(name) = get_name(param) else { continue };
                if seen_names.contains(name) { continue }
                // We allow values to contain only a portion of props defined in
                // the slot. "read_values" would have reported missing props errors,
                // but the app may have chosen to ignore them.
                if let Some(value) = values.get(name) {
                    group_values.push((name.clone(), value.clone()));
                }
                seen_names.push(name.clone());
            }
        }

        if !group_values.is_empty() {
            let group = ValueGroup {
                name: group.name.clone(),
                values: group_values
            };
            groups.push(group)
        }
    }

    groups
}

