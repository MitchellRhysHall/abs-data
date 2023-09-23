use serde::{Deserialize, Serialize};

use crate::models::typed::dataflow_id::DataflowId;

use super::sdmx_response::{Annotation, Descriptions, Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataflows {
    pub dataflows: Box<[Dataflow]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataflow {
    pub id: DataflowId,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub description: Option<Box<str>>,
    pub descriptions: Option<Descriptions>,
    pub annotations: Option<Vec<Annotation>>,
    pub structure: Option<Box<str>>,
}
