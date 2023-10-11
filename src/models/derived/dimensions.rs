use serde::{Deserialize, Serialize};

use super::{observation::Observation, series::Series, value::Value};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub dataset: Box<[Value]>,
    pub series: Box<[Series]>,
    pub observation: Box<[Observation]>,
}
