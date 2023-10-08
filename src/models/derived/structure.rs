use serde::{Deserialize, Serialize};

use super::{
    annotation::Annotation, attributes::Attributes, descriptions::Descriptions,
    dimensions::Dimensions, names::Names,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Structure {
    pub name: Box<str>,
    pub names: Names,
    pub description: Box<str>,
    pub descriptions: Descriptions,
    pub dimensions: Dimensions,
    pub attributes: Attributes,
    pub annotations: Box<[Annotation]>,
}
