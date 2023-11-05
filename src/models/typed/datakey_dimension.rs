use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DataKeyDimension<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> DataKeyDimension<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        self.key
    }

    pub fn value(&self) -> &str {
        self.value
    }
}
