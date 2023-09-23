use crate::{
    builders::datakey::DataKeyBuilder,
    error_code::{ErrorCode, Result},
};
use std::fmt::{self, Display, Formatter};

use super::datakey_dimensions::DataKeyDimensions;

const MAX_LENGTH: usize = 260;
const REQUIRED_DOT_COUNT: usize = 4;

pub struct DataKey {
    inner: Box<str>,
}

impl DataKey {
    pub fn builder() -> DataKeyBuilder {
        DataKeyBuilder::new()
    }

    pub fn no_filter() -> Self {
        Self {
            inner: "all".into(),
        }
    }

    fn format_dimension<T: Display>(items: &[T]) -> String {
        items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
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
            dimensions_vec.push(DataKey::format_dimension(&measure));
        }

        if !region.is_empty() {
            dimensions_vec.push(DataKey::format_dimension(&region));
        }

        if !frequency.is_empty() {
            dimensions_vec.push(DataKey::format_dimension(&frequency));
        }

        let mut dimensions_str = dimensions_vec.join(".");

        let dot_count = dimensions_str.matches('.').count();

        if dot_count < REQUIRED_DOT_COUNT {
            dimensions_str.push_str(&".".repeat(4 - dot_count));
        }

        let key = Self {
            inner: dimensions_str.into(),
        };

        if format!("{}", key).len() > MAX_LENGTH {
            return Err(ErrorCode::DataKeyLengthExceeded(MAX_LENGTH));
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
