use serde::{Deserialize, Serialize};

use super::key_value::KeyValue;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CubeRegion {
    pub is_included: bool,
    pub key_values: Vec<KeyValue>,
}
