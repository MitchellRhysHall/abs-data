use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DataPoint(Box<str>);

impl<'de> Deserialize<'de> for DataPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataPointVisitor;

        impl<'de> Visitor<'de> for DataPointVisitor {
            type Value = DataPoint;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a data point as a float, an integer, or a string")
            }

            fn visit_f64<E>(self, data: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DataPoint(data.to_string().into()))
            }

            fn visit_u64<E>(self, data: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DataPoint(data.to_string().into()))
            }

            fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DataPoint(data.into()))
            }
        }

        deserializer.deserialize_any(DataPointVisitor)
    }
}
