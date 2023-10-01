use serde::{Deserialize, Serialize};

use crate::traits::structure_type::StructureType;

use super::{concept::Concept, descriptions::Descriptions, link::Link, names::Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptSchemes {
    pub concept_schemes: Box<[ConceptScheme]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptScheme {
    pub id: Box<str>,
    pub links: Option<Box<[Link]>>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_external_reference: Option<bool>,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub is_partial: bool,
    pub concepts: Box<[Concept]>,
    pub description: Option<Box<str>>,
    pub descriptions: Option<Descriptions>,
}

impl StructureType for ConceptSchemes {
    fn url_path_segment() -> &'static str {
        "conceptscheme"
    }
}
