use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
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
