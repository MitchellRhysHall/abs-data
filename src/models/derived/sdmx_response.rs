use serde::{Deserialize, Serialize};

use super::{meta::Meta, structure::Structure};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdmxResponse<T> {
    pub data: T,
    pub meta: Meta,
    pub structure: Option<Structure>,
}
