use serde::{Deserialize, Serialize};

use super::texts::Texts;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    #[serde(rename = "type")]
    pub _type: Box<str>,
    pub text: Option<Box<str>>,
    pub texts: Option<Texts>,
    pub title: Option<Box<str>>,
    pub id: Option<Box<str>>,
}
