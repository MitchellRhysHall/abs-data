use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter, Default)]
pub enum AgencyId {
    #[default]
    Abs,
    Specific(Box<str>),
}

impl Display for AgencyId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Abs => write!(f, "ABS"),
            Self::Specific(x) => write!(f, "{}", x),
        }
    }
}

impl Serialize for AgencyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Abs => serializer.serialize_str("ABS"),
            Self::Specific(id) => serializer.serialize_str(id),
        }
    }
}

impl<'de> Deserialize<'de> for AgencyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Box<str> = Deserialize::deserialize(deserializer)?;
        Ok(match &*s {
            "ABS" => Self::Abs,
            _ => Self::Specific(s),
        })
    }
}
