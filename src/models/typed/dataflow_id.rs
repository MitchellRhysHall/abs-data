use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use strum_macros::EnumIter;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum DataflowId {
    Capex,
    Cpi,
    Cwd,
    Ewd,
    Itgs,
    Jv,
    Lci,
    Lf,
    PopulationClock,
    Ppi,
    Qbis,
    Rppi,
    Rt,
    Wpi,
    Other(Box<str>),
}

impl fmt::Display for DataflowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capex => write!(f, "CAPEX"),
            Self::Cpi => write!(f, "CPI"),
            Self::Cwd => write!(f, "CWD"),
            Self::Ewd => write!(f, "EWD"),
            Self::Itgs => write!(f, "ITGS"),
            Self::Jv => write!(f, "JV"),
            Self::Lci => write!(f, "LCI"),
            Self::Lf => write!(f, "LF"),
            Self::PopulationClock => write!(f, "POPULATION_CLOCK"),
            Self::Ppi => write!(f, "PPI"),
            Self::Qbis => write!(f, "QBIS"),
            Self::Rppi => write!(f, "RPPI"),
            Self::Rt => write!(f, "RT"),
            Self::Wpi => write!(f, "WPI"),
            Self::Other(x) => write!(f, "{}", x),
        }
    }
}

impl Serialize for DataflowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Capex => serializer.serialize_str("CAPEX"),
            Self::Cpi => serializer.serialize_str("CPI"),
            Self::Cwd => serializer.serialize_str("CWD"),
            Self::Ewd => serializer.serialize_str("EWD"),
            Self::Itgs => serializer.serialize_str("ITGS"),
            Self::Jv => serializer.serialize_str("JV"),
            Self::Lci => serializer.serialize_str("LCI"),
            Self::Lf => serializer.serialize_str("LF"),
            Self::PopulationClock => serializer.serialize_str("POPULATION_CLOCK"),
            Self::Ppi => serializer.serialize_str("PPI"),
            Self::Qbis => serializer.serialize_str("QBIS"),
            Self::Rppi => serializer.serialize_str("RPPI"),
            Self::Rt => serializer.serialize_str("RT"),
            Self::Wpi => serializer.serialize_str("WPI"),
            Self::Other(id) => serializer.serialize_str(id),
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
            "CAPEX" => Self::Capex,
            "CPI" => Self::Cpi,
            "CWD" => Self::Cwd,
            "EWD" => Self::Ewd,
            "ITGS" => Self::Itgs,
            "JV" => Self::Jv,
            "LCI" => Self::Lci,
            "LF" => Self::Lf,
            "POPULATION_CLOCK" => Self::PopulationClock,
            "PPI" => Self::Ppi,
            "QBIS" => Self::Qbis,
            "RPPI" => Self::Rppi,
            "RT" => Self::Rt,
            "WPI" => Self::Wpi,
            _ => Self::Other(s),
        })
    }
}
