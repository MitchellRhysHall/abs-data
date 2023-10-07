use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Quarter {
    First,
    Second,
    Third,
    Fourth,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quarter_str = match self {
            Self::First => "Q1",
            Self::Second => "Q2",
            Self::Third => "Q3",
            Self::Fourth => "Q4",
        };
        write!(f, "{}", quarter_str)
    }
}

impl Default for Quarter {
    fn default() -> Self {
        Self::First
    }
}
