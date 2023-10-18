use crate::{cache::MetaCache, models::derived::key_value};

use super::dataflow_identifier::DataflowIdentifier;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct DimensionKey<'a> {
    id: &'a DataflowIdentifier,
    key: &'a str,
}

impl<'a> DimensionKey<'a> {
    pub fn new(key: &'a str, id: &'a DataflowIdentifier) -> Self {
        Self { key, id }
    }
}

impl Ord for DimensionKey<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let key_values = MetaCache::get_key_values(&self.id);

        let self_index = key_values
            .iter()
            .position(|key_value| self.key == key_value.id.as_ref())
            .expect("key not found");

        let other_index = key_values
            .iter()
            .position(|key_value| other.key == key_value.id.as_ref())
            .expect("key not found");

        self_index.cmp(&other_index)
    }
}
