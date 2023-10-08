use serde::{Deserialize, Serialize};

use super::names::Names;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub start: Option<Box<str>>,
    pub end: Option<Box<str>>,
    pub id: Box<str>,
    pub name: Box<str>,
    pub names: Names,
}
