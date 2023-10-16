use serde::{Deserialize, Serialize};

use super::{
    annotation::Annotation, constraint_attachment::ConstraintAttachment, cube_region::CubeRegion,
    names::Names,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentConstraint {
    pub id: Box<str>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub annotations: Box<[Annotation]>,
    #[serde(rename = "type")]
    pub type_field: Box<str>,
    pub constraint_attachment: ConstraintAttachment,
    pub cube_regions: Box<[CubeRegion]>,
}
