use std::fmt::{self, Display, Formatter};

pub enum Detail {
    Full,
    DataOnly,
    SeriesKeysOnly,
    NoData,
}

impl<'a> Display for Detail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::DataOnly => write!(f, "dataonly"),
            Self::SeriesKeysOnly => write!(f, "serieskeysonly"),
            Self::NoData => write!(f, "nodata"),
        }
    }
}
