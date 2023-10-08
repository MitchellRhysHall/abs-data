use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct AgencyId<'a> {
    value: &'a str,
}

impl<'a> AgencyId<'a> {
    pub const ABS: &str = "ABS";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for AgencyId<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
