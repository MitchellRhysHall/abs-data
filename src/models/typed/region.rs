use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Region {
    Aus,
    Usa,
    Eu,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Region::Aus => write!(f, "AUS"),
            Region::Usa => write!(f, "USA"),
            Region::Eu => write!(f, "EU"),
        }
    }
}
