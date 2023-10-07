use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum StructureType {
    ActualConstraint,
    Categorisation,
    CategoryScheme,
    CodeList,
    ConceptScheme,
    ContentConstraint,
    DataFlow,
    DataStructure,
    AgencyScheme,
    HierarchicalCodelist,
}

impl Display for StructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ActualConstraint => write!(f, "actualconstraint"),
            Self::Categorisation => write!(f, "categorisation"),
            Self::CategoryScheme => write!(f, "categoryscheme"),
            Self::CodeList => write!(f, "codelist"),
            Self::ConceptScheme => write!(f, "conceptscheme"),
            Self::ContentConstraint => write!(f, "contentconstraint"),
            Self::DataFlow => write!(f, "dataflow"),
            Self::DataStructure => write!(f, "datastructure"),
            Self::AgencyScheme => write!(f, "agencyscheme"),
            Self::HierarchicalCodelist => write!(f, "hierarchicalcodelist"),
        }
    }
}

impl Default for StructureType {
    fn default() -> Self {
        Self::DataFlow
    }
}
