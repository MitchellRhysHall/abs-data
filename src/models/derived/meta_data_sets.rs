use serde::{Deserialize, Serialize};

use super::{concept::Concept, descriptions::Descriptions, link::Link, names::Names};

pub type MetaDataSets = Box<[MetaDataSet]>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDataSet {
    pub id: Box<str>,
    pub links: Option<Box<[Link]>>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_external_reference: Option<bool>,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub is_partial: Option<bool>,
    pub concepts: Option<Box<[Concept]>>,
    pub description: Option<Box<str>>,
    pub descriptions: Option<Descriptions>,
}
