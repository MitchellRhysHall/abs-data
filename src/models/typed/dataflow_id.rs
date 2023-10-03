use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `DataflowId` Enum - Identifiers for Various Data Flows
///
/// Enumerates various identifiers associated with different data flows within a system or application.
/// This list is not exhaustive and serves as a subset of possible identifiers,
/// providing a set of well-known or common identifiers as predefined enum variants.
///
/// ## Variants
///
/// - `Capex`: [Description for Capex]
/// - `Cpi`: Consumer Price Index
/// - `Cwd`: [Description for Cwd]
/// - `Ewd`: [Description for Ewd]
/// - `Itgs`: [Description for Itgs]
/// - `Jv`: [Description for Jv]
/// - `Lci`: [Description for Lci]
/// - `Lf`: [Description for Lf]
/// - `PopulationClock`: [Description for PopulationClock]
/// - `Ppi`: Producer Price Index
/// - `Qbis`: [Description for Qbis]
/// - `Rppi`: [Description for Rppi]
/// - `Rt`: [Description for Rt]
/// - `Wpi`: Wholesale Price Index
/// - `Other(Box<str>)`: Any other identifier not listed as a variant
///
/// ## Usage
///
/// This enum can be used to specify, classify, or query different data flows within the application.
/// Additional data flow identifiers, which are not listed as variants, can be represented using the
/// `Other` variant by boxing the custom identifier string.
///
/// ## Note
///
/// Additional dataflow identifiers that are not enumerated here can potentially be queried
/// or retrieved via a meta request for "dataflows" from relevant data sources or services.
///
/// ```
/// use your_crate::DataflowId;
///
/// fn get_data_flow(dataflow: DataflowId) {
///     // Usage example
///     match dataflow {
///         DataflowId::Cpi => {
///             // Handle CPI related logic here
///         },
///         DataflowId::Other(id) => {
///             // Handle custom identifier related logic here
///         },
///         _ => {
///             // Handle other variants or default case here
///         }
///     }
/// }
/// ```#[derive(Debug, Clone, PartialEq, Eq)]
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
