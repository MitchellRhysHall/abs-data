use std::fmt::{self, Display, Formatter};

#[derive(Default)]
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


