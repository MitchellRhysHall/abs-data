use std::collections::HashMap;

use super::{annotation::Annotation, data_point::DataPoint};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Box<[Option<i64>]>,
    pub annotations: Box<[Annotation]>,
    pub observations: HashMap<Box<str>, Box<[Option<DataPoint>]>>,
}
