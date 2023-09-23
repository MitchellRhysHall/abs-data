use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
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
