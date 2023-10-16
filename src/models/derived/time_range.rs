use serde::{Deserialize, Serialize};

use super::period::Period;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub start_period: Period,
    pub end_period: Period,
}
