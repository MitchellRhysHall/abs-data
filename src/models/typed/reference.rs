use std::fmt::{self, Display, Formatter};

use super::structure_type::StructureType;

#[derive(Debug, Clone)]
pub enum Reference {
    None,
    Parents,
    ParentsAndSimplings,
    Children,
    Descendants,
    All,
    Specific(StructureType),
}

impl<'a> Display for Reference {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Parents => write!(f, "parents"),
            Self::ParentsAndSimplings => write!(f, "parentsandsiblings"),
            Self::Children => write!(f, "children"),
            Self::Descendants => write!(f, "descendants"),
            Self::All => write!(f, "all"),
            Self::Specific(structure_type) => write!(f, "{}", structure_type),
        }
    }
}
