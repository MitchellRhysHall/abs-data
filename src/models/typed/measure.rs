use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Measure {
    M1,
    M2,
}

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Measure::M1 => write!(f, "M1"),
            Measure::M2 => write!(f, "M2"),
        }
    }
}
