use serde::{Deserialize, Serialize};

use super::{names::Names, relationship::Relationship, value::Value};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: Box<str>,
    pub name: Box<str>,
    pub names: Names,
    pub roles: Box<[Box<str>]>,
    pub relationship: Option<Relationship>,
    pub values: Box<[Value]>,
    pub annotations: Option<Box<[i64]>>,
    pub key_position: Option<u8>,
}
