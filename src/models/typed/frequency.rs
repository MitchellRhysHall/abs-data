use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Frequency {
    Quarterly,
    Monthly,
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Frequency::Quarterly => write!(f, "Q"),
            Frequency::Monthly => write!(f, "M"),
        }
    }
}
