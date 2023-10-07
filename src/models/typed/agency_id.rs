use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumIter,
    Default,
)]
pub enum AgencyId {
    #[default]
    Abs,
}

impl Display for AgencyId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Abs => write!(f, "ABS"),
        }
    }
}
