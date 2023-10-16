use serde::{Deserialize, Serialize};

use crate::models::typed::version::Version;

use super::{
    annotation::Annotation, concept::Concept, constraint_attachment::ConstraintAttachment,
    cube_region::CubeRegion, descriptions::Descriptions, link::Link, names::Names,
};

pub type MetaDataSets = Box<[MetaDataSet]>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaDataSet {
    pub id: Box<str>,
    pub links: Option<Box<[Link]>>,
    pub version: Version,
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
    pub annotations: Option<Box<[Annotation]>>,
    pub structure: Option<Box<str>>,
    #[serde(rename = "type")]
    pub type_field: Box<str>,
    pub constraint_attachment: Option<ConstraintAttachment>,
    pub cube_regions: Option<Box<[CubeRegion]>>,
}
