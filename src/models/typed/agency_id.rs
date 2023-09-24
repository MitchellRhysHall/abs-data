use std::fmt::{self, Display, Formatter};

pub enum AgencyId {
    Abs,
}

impl Display for AgencyId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Abs => write!(f, "ABS"),
        }
    }
}
