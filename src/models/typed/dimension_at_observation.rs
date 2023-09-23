use std::fmt::{self, Display, Formatter};

pub enum DimensionAtObservation<'a> {
    TimePeriod,
    All,
    Id(&'a str),
}

impl<'a> Display for DimensionAtObservation<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::TimePeriod => write!(f, "TIME_PERIOD"),
            Self::All => write!(f, "AllDimensions"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}
