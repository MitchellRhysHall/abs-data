use std::fmt::{self, Display, Formatter};

use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum StructureId {
    All,
}

impl Display for StructureId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
        }
    }
}
