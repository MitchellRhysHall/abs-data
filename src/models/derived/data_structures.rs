use serde::{Deserialize, Serialize};

use super::{data_structure_components::DataStructureComponents, link::Link, names::Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStructures {
    pub data_structures: Box<[DataStructure]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStructure {
    pub id: Box<str>,
    pub links: Box<[Link]>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_external_reference: bool,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub data_structure_components: DataStructureComponents,
}
