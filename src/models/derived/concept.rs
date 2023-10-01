use serde::{Deserialize, Serialize};

use super::{annotation::Annotation, descriptions::Descriptions, names::Names};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Concept {
    pub id: Box<str>,
    pub name: Box<str>,
    pub names: Names,
    pub description: Option<String>,
    pub descriptions: Option<Descriptions>,
    #[serde(default)]
    pub annotations: Vec<Annotation>,
    pub parent: Option<Box<str>>,
}
