use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter)]
pub enum DimensionAtObservation {
    TimePeriod,
    All,
    Id(Box<str>),
}

impl Display for DimensionAtObservation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::TimePeriod => write!(f, "TIME_PERIOD"),
            Self::All => write!(f, "AllDimensions"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}
