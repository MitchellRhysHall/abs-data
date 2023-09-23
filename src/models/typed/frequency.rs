use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
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
