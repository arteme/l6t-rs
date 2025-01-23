
pub struct DataGroup {
    pub title: String,
    pub values: Vec<DataItem>,
}

pub struct DataItem {
    pub label: String,
    pub value: String,
    pub tooltip: Option<String>,
}

pub mod shorthand {
    use super::*;

    pub struct GroupBuilder {
        pub data_group: DataGroup
    }

    pub fn group(title: &str) -> GroupBuilder {
        let data_group = DataGroup { title: title.into(), values: vec![] };
        GroupBuilder { data_group }
    }

    impl GroupBuilder {
        pub fn item(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
            self.data_group.values.push(DataItem {
                label: name.into(),
                value: value.into(),
                tooltip: None
            });
            self
        }
        pub fn tooltip(mut self, value: impl Into<String>) -> Self {
            if let Some(item) = self.data_group.values.last_mut() {
                item.tooltip = Some(value.into());
            }
            self
        }

        pub fn item_ne(mut self, name: &str, value: &str) -> Self {
            if !value.is_empty() {
                self.item(name, value)
            } else {
                self
            }
        }
        pub fn item_ne2(mut self, name: &str, value1: &str, sep: &str, value2: &str) -> Self {
            if !value1.is_empty() || !value2.is_empty() {
                let value = format!("{value1}{sep}{value2}");
                self.item(name, &value)
            } else {
                self
            }
        }
    }

    impl Into<DataGroup> for GroupBuilder {
        fn into(self) -> DataGroup {
            self.data_group
        }
    }
}