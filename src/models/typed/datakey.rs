use crate::{config::Config, error_code::ErrorCode};
use std::fmt::{self, Display, Formatter};

use crate::result::Result;

use super::datakey_dimensions::DataKeyDimensions;

pub struct DataKey {
    inner: Box<str>,
}

impl DataKey {
    fn new(str: &str) -> Self {
        Self { inner: str.into() }
    }

    pub fn parse(str: &str) -> Result<Self> {
        Self::try_from(str)
    }

    pub fn no_filter() -> Self {
        Self {
            inner: "all".into(),
        }
    }

    fn format_dimension<T: AsRef<str>>(items: &[T]) -> String {
        items
            .iter()
            .map(|item| item.as_ref())
            .collect::<Vec<&str>>()
            .join("+")
    }
}

impl TryFrom<DataKeyDimensions<'_>> for DataKey {
    type Error = ErrorCode;

    fn try_from(dimensions: DataKeyDimensions) -> Result<Self> {
        let DataKeyDimensions {
            measure,
            region,
            frequency,
        } = dimensions;

        if measure.is_empty() && region.is_empty() && frequency.is_empty() {
            return Ok(DataKey::no_filter());
        }

        let mut dimensions_vec = Vec::new();

        if !measure.is_empty() {
            dimensions_vec.push(DataKey::format_dimension(measure));
        }

        if !region.is_empty() {
            dimensions_vec.push(DataKey::format_dimension(region));
        }

        if !frequency.is_empty() {
            dimensions_vec.push(DataKey::format_dimension(frequency));
        }

        let mut dimensions_str = dimensions_vec.join(".");

        let dot_count = dimensions_str.matches('.').count();

        if dot_count < Config::DATA_KEY_REQUIRED_DOT_COUNT {
            dimensions_str.push_str(&".".repeat(4 - dot_count));
        }

        let key = Self {
            inner: dimensions_str.into(),
        };

        let byte_len = format!("{}", key).len();

        if byte_len > Config::DATA_KEY_MAX_LENGTH {
            return Err(ErrorCode::DataKeyLengthIncorrect(byte_len));
        };

        Ok(key)
    }
}

impl TryFrom<&str> for DataKey {
    type Error = ErrorCode;

    fn try_from(str: &str) -> Result<Self> {
        let dot_count = str.matches('.').count();

        if dot_count != Config::DATA_KEY_REQUIRED_DOT_COUNT {
            return Err(ErrorCode::DataKeyLengthIncorrect(str.len()));
        }

        let key = Self { inner: str.into() };

        let byte_len = format!("{}", key).len();

        if byte_len > Config::DATA_KEY_MAX_LENGTH {
            return Err(ErrorCode::DataKeyLengthIncorrect(byte_len));
        };

        Ok(key)
    }
}

impl Display for DataKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl AsRef<str> for DataKey {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl Default for DataKey {
    fn default() -> Self {
        Self::no_filter()
    }
}
