use serde::{Deserialize, Serialize};

use super::{receiver::Receiver, sender::Sender};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub schema: Box<str>,
    pub content_languages: Box<[Box<str>]>,
    pub id: Box<str>,
    pub prepared: Box<str>,
    pub test: bool,
    pub sender: Sender,
    pub receiver: Option<Box<[Receiver]>>,
}
