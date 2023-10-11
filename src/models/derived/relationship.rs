use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub dimensions: Option<Box<[Box<str>]>>,
    pub primary_measure: Option<Box<str>>,
}
