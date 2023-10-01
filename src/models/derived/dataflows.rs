use serde::{Deserialize, Serialize};

use crate::{
    models::typed::{dataflow_id::DataflowId, structure_type::StructureType},
    traits::url_path_segment::UrlPathSegment,
};

use super::{
    annotation::Annotation, data_structures::DataStructure, descriptions::Descriptions,
    names::Names, sdmx_response::SdmxResponse,
};

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

impl UrlPathSegment for Dataflows {
    fn url_path_segment() -> &'static str {
        "dataflow"
    }
}
