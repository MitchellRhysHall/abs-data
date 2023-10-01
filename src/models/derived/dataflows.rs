use serde::{Deserialize, Serialize};

use crate::{models::typed::dataflow_id::DataflowId, traits::structure_type::StructureType};

use super::{annotation::Annotation, descriptions::Descriptions, names::Names};

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

impl StructureType for Dataflows {
    fn url_path_segment() -> &'static str {
        "dataflow"
    }
}
