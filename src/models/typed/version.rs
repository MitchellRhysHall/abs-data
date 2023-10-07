use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::boxed::Box;
use std::fmt;

use crate::error_code::{ErrorCode, Result};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }
}

impl TryFrom<Box<str>> for Version {
    type Error = ErrorCode;

    fn try_from(s: Box<str>) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(ErrorCode::VersionStringNotCorrectLength(parts.len()));
        }
        Ok(Version {
            major: parts[0]
                .parse()
                .map_err(ErrorCode::VersionStringContainsUnknownChar)?,
            minor: parts[1]
                .parse()
                .map_err(ErrorCode::VersionStringContainsUnknownChar)?,
            patch: parts[2]
                .parse()
                .map_err(ErrorCode::VersionStringContainsUnknownChar)?,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
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
