use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{data::Data, link::Link, structure::Structure};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSets {
    pub data_sets: Box<[DataSet]>,
    pub structure: Structure,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
    pub action: Box<str>,
    pub links: Box<[Link]>,
    pub annotations: Vec<i64>,
    pub series: HashMap<Box<str>, Data>,
}
