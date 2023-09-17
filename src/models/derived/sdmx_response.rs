use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SdmxResponse<T> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Names {
    pub en: Box<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Descriptions {
    pub en: Box<str>,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texts {
    pub en: Box<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub schema: Box<str>,
    pub content_languages: Box<[Box<str>]>,
    pub id: Box<str>,
    pub prepared: Box<str>,
    pub test: bool,
    pub sender: Sender,
    pub receiver: Box<[Receiver]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
    pub id: Box<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receiver {
    pub id: Box<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStructures {
    pub data_structures: Box<[DataStructure]>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStructure {
    pub id: Box<str>,
    pub links: Box<[Link]>,
    pub version: Box<str>,
    #[serde(rename = "agencyID")]
    pub agency_id: Box<str>,
    pub is_external_reference: bool,
    pub is_final: bool,
    pub name: Box<str>,
    pub names: Names,
    pub data_structure_components: DataStructureComponents,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: Box<str>,
    pub rel: Box<str>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataStructureComponents {}
