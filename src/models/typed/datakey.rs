use crate::{config::Config, error_code::ErrorCode};
use std::fmt::{self, Display, Formatter};

use crate::result::Result;

#[derive(Debug)]
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
}

impl PartialEq for DataKey {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
    fn ne(&self, other: &Self) -> bool {
        self.inner != other.inner
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
