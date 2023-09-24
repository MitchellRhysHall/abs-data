use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataflowId {
    Cpi,
    Ppi,
    Wpi,
    Other(Box<str>),
}

impl fmt::Display for DataflowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cpi => write!(f, "CPI"),
            Self::Ppi => write!(f, "PPI"),
            Self::Wpi => write!(f, "WPI"),
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
            Self::Cpi => serializer.serialize_str("CPI"),
            Self::Ppi => serializer.serialize_str("PPI"),
            Self::Wpi => serializer.serialize_str("WPI"),
            Self::Other(id) => serializer.serialize_str(&id),
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
            "CPI" => Self::Cpi,
            "PPI" => Self::Ppi,
            "WPI" => Self::Wpi,
            _ => Self::Other(s),
        })
    }
}
