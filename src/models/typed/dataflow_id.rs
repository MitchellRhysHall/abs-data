use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataflowId {
    Cpi,
    Other(Box<str>),
}

impl fmt::Display for DataflowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cpi => write!(f, "CPI"),
            Self::Other(x) => write!(f, "{}", x),
        }
    }
}

impl Default for DataflowId {
    fn default() -> Self {
        DataflowId::Other(Box::from(""))
    }
}

impl Serialize for DataflowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DataflowId::Cpi => serializer.serialize_str("CPI"),
            DataflowId::Other(id) => serializer.serialize_str(&id),
        }
    }
}

impl<'de> Deserialize<'de> for DataflowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Box<str> = Deserialize::deserialize(deserializer)?;
        Ok(match &*s {
            "CPI" => DataflowId::Cpi,
            _ => DataflowId::Other(s),
        })
    }
}

impl AsRef<str> for DataflowId {
    fn as_ref(&self) -> &str {
        match self {
            DataflowId::Cpi => "CPI",
            DataflowId::Other(id) => &id,
        }
    }
}
