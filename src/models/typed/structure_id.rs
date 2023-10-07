use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
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
