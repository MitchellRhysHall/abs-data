use serde::{Deserialize, Serialize};

use super::meta_data_sets::MetaDataSet;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDataMap {
    pub dataflows: Option<Box<[MetaDataSet]>>,
    pub content_constraints: Option<Box<[MetaDataSet]>>,
}
