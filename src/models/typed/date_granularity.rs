use std::fmt::{self, Display, Formatter};

pub enum DateGranularity<'a> {
    Year(u16),
    YearSemester(u16, &'a str),
    YearQuarter(u16, &'a str),
    YearMonth(u16, u8),
}

impl<'a> Display for DateGranularity<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Year(year) => write!(f, "{:04}", year),
            Self::YearSemester(year, semester) => write!(f, "{:04}-{}", year, semester),
            Self::YearQuarter(year, quarter) => write!(f, "{:04}-{}", year, quarter),
            Self::YearMonth(year, month) => write!(f, "{:04}-{:02}", year, month),
        }
    }
}
