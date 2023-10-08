use serde::{Deserialize, Serialize};

use super::{observation::Observation, series::Series, value::Value};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub data_set: Box<[Value]>,
    pub series: Box<[Series]>,
    pub observation: Box<[Observation]>,
}
