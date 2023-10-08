use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::annotation::Annotation;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub attributes: Box<[Option<i64>]>,
    pub annotations: Box<[Annotation]>,
    pub observations: HashMap<Box<str>, Box<[Option<f64>]>>,
}
