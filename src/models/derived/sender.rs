use serde::{Deserialize, Serialize};

use super::names::Names;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub id: Box<str>,
    pub name: Option<Box<str>>,
    pub names: Option<Names>,
}
