use std::fmt;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(
    Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
pub enum Month {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let month_number = match self {
            Self::Jan => "01",
            Self::Feb => "02",
            Self::Mar => "03",
            Self::Apr => "04",
            Self::May => "05",
            Self::Jun => "06",
            Self::Jul => "07",
            Self::Aug => "08",
            Self::Sep => "09",
            Self::Oct => "10",
            Self::Nov => "11",
            Self::Dec => "12",
        };
        write!(f, "{}", month_number)
    }
}

impl Default for Month {
    fn default() -> Self {
        Self::Jan
    }
}
