use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::link::Link;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSets {
    pub data_sets: Box<[DataSet]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
    pub action: Box<str>,
    pub links: Box<[Link]>,
    pub annotations: Vec<i64>,
    pub observations: HashMap<Box<str>, Box<[Option<f64>]>>,
}
