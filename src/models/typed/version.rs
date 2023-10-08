use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::boxed::Box;
use std::fmt;

use crate::error_code::ErrorCode;
use crate::result::Result;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    str: Box<str>,
}

impl Version {
    pub const ONE: &str = "1.0.0";

    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Version {
            major,
            minor,
            patch,
            str: format!("{}{}{}", major, minor, patch).into(),
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
            str: Self::ONE.into(),
        }
    }
}

impl AsRef<str> for Version {
    fn as_ref(&self) -> &str {
        &self.str
    }
}

impl TryFrom<Box<str>> for Version {
    type Error = ErrorCode;

    fn try_from(s: Box<str>) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(ErrorCode::VersionStringNotCorrectLength(parts.len()));
        }
        let major = parts[0]
            .parse()
            .map_err(ErrorCode::VersionStringContainsUnknownChar)?;

        let minor = parts[1]
            .parse()
            .map_err(ErrorCode::VersionStringContainsUnknownChar)?;

        let patch = parts[2]
            .parse()
            .map_err(ErrorCode::VersionStringContainsUnknownChar)?;

        Ok(Version::new(major, minor, patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Box<str> = Deserialize::deserialize(deserializer)?;
        s.try_into().map_err(de::Error::custom)
    }
}
