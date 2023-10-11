use serde::{Deserialize, Serialize};

use super::{names::Names, relationship::Relationship, value::Value};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observations {
    pub observation: Box<[Observation]>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub id: Box<str>,
    pub name: Box<str>,
    pub names: Names,
    pub key_position: Option<i64>,
    pub roles: Box<[Box<str>]>,
    pub values: Box<[Value]>,
    pub relationship: Option<Relationship>,
}
