use serde::{Deserialize, Serialize};

use crate::traits::structure_type::StructureType;

use super::{link::Link, names::Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Codelists {
    pub codelists: Box<[Codelist]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Codelist {
    pub id: Box<str>,
    pub links: Box<[Link]>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_external_reference: bool,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub is_partial: bool,
}

impl StructureType for Codelists {
    fn url_path_segment() -> &'static str {
        "codelist"
    }
}
