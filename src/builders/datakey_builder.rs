use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    builders::sdmx_meta_request_builder::SdmxMetaRequestBuilder,
    cache::MetaCache,
    models::typed::{
        dataflow_identifier::DataflowIdentifier, datakey::DataKey, dimension_key::DimensionKey,
        reference::Reference, structure_type::StructureType,
    },
    result::Result,
};

pub struct DataKeyBuilder<'a> {
    dataflow_identifier: &'a DataflowIdentifier,
    dimensions: BTreeMap<DimensionKey<'a>, HashSet<&'a str>>,
}

impl<'a> DataKeyBuilder<'a> {
    pub async fn new(dataflow_identifier: &'a DataflowIdentifier) -> DataKeyBuilder<'_> {
        let _ = MetaCache::ensure_key_init(dataflow_identifier).await;
        Self {
            dataflow_identifier,
            dimensions: BTreeMap::new(),
        }
    }

    pub fn add(mut self, key: &'a str, value: &'a str) -> Result<Self> {
        let key_values = MetaCache::get_key_values(self.dataflow_identifier);

        if key_values.iter().find(|kv| kv.id.as_ref() == key).is_some() {
            self.dimensions
                .entry(DimensionKey::new(key, self.dataflow_identifier))
                .or_insert_with(HashSet::new)
                .insert(value);
        }

        Ok(self)
    }

    pub fn build(self) -> DataKey {
        let key = self
            .dimensions
            .into_values()
            .map(|s| s.into_iter().collect::<Vec<_>>().join("+"))
            .collect::<Vec<_>>()
            .join(".");

        DataKey::parse(&key).expect("should always be valid")
    }
}
