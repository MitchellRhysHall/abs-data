use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Semester {
    First,
    Second,
}

impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let semester_str = match self {
            Self::First => "S1",
            Self::Second => "S2",
        };
        write!(f, "{}", semester_str)
    }
}

impl Default for Semester {
    fn default() -> Self {
        Self::First
    }
}
