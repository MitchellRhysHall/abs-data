use serde::{Deserialize, Serialize};

use super::names::Names;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: Box<str>,
    pub name: Box<str>,
    pub names: Names,
    #[serde(default)]
    pub categories: Option<Box<[Category]>>,
}
