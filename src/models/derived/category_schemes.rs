use serde::{Deserialize, Serialize};

use crate::traits::structure_type::StructureType;

use super::{category::Category, names::Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySchemes {
    pub category_schemes: Option<Box<[CategoryScheme]>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryScheme {
    pub id: Box<str>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub is_partial: bool,
    pub categories: Box<[Category]>,
}

impl StructureType for CategorySchemes {
    fn url_path_segment() -> &'static str {
        "conceptscheme"
    }
}
