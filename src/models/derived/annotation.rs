use serde::{Deserialize, Serialize};

use super::texts::Texts;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    #[serde(rename = "type")]
    pub type_field: Box<str>,
    pub text: Option<Box<str>>,
    pub texts: Option<Texts>,
    pub title: Option<Box<str>>,
    pub id: Option<Box<str>>,
}