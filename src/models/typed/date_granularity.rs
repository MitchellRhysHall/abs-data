use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::{month::Month, quarter::Quarter, semester::Semester};

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum DateGranularity {
    Year(u16),
    YearSemester(u16, Semester),
    YearQuarter(u16, Quarter),
    YearMonth(u16, Month),
}

impl Display for DateGranularity {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Year(year) => write!(f, "{:04}", year),
            Self::YearSemester(year, semester) => write!(f, "{:04}-{}", year, semester),
            Self::YearQuarter(year, quarter) => write!(f, "{:04}-{}", year, quarter),
            Self::YearMonth(year, month) => write!(f, "{:04}-{}", year, month),
        }
    }
}
