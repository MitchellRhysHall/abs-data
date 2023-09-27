use std::fmt::{self, Display, Formatter};

use crate::{models::derived::sdmx_response::SdmxResponse, traits::SdmxResponseType::ResponseType};

pub enum StructureType {
    DataFlow,
    DataStructure,
    CodeList,
    ConceptScheme,
    CategoryScheme,
    ContentConstraint,
    ActualConstraint,
    AgencyScheme,
    Categorisation,
    HierarchicalCodelist,
}

impl Display for StructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataFlow => write!(f, "dataflow"),
            Self::DataStructure => write!(f, "datastructure"),
            Self::CodeList => write!(f, "codelist"),
            Self::ConceptScheme => write!(f, "conceptscheme"),
            Self::CategoryScheme => write!(f, "categoryscheme"),
            Self::ContentConstraint => write!(f, "contentconstraint"),
            Self::ActualConstraint => write!(f, "actualconstraint"),
            Self::AgencyScheme => write!(f, "agencyscheme"),
            Self::Categorisation => write!(f, "categorisation"),
            Self::HierarchicalCodelist => write!(f, "hierarchicalcodelist"),
        }
    }
}
