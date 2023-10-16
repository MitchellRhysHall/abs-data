use serde::{Deserialize, Serialize};

use super::time_range::TimeRange;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValue {
    pub id: String,
    #[serde(default)]
    pub values: Box<[Box<str>]>,
    pub time_range: Option<TimeRange>,
}
