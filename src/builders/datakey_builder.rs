use std::collections::{HashMap, HashSet};

use crate::{
    builders::sdmx_meta_request_builder::SdmxMetaRequestBuilder,
    error_code::ErrorCode,
    models::typed::{
        dataflow_identifier::DataflowIdentifier, datakey::DataKey, reference::Reference,
        structure_type::StructureType,
    },
    result::Result,
};

pub struct DataKeyBuilder<'a> {
    dataflow_identifier: &'a DataflowIdentifier<'a>,
    map_values: HashMap<&'a str, HashSet<&'a str>>,
    map_key_index: HashMap<&'a str, u8>,
    dimensions: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> DataKeyBuilder<'a> {
    async fn get_meta(&self) -> Result<()> {
        let mut builder = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .structure_id(&self.dataflow_identifier.structure_id())
            .reference(&Reference::StructureType(StructureType::ContentConstraint));

        if let Some(agency_id) = self.dataflow_identifier.agency_id() {
            builder = builder.agency_id(agency_id);
        }
        if let Some(version) = &self.dataflow_identifier.version() {
            builder = builder.structure_version(version);
        }

        let response = builder.build().send().await?;

        Ok(())
    }

    pub fn new(dataflow_identifier: &'a DataflowIdentifier<'a>) -> Self {
        Self {
            dataflow_identifier,
            map_values: HashMap::new(),
            map_key_index: HashMap::new(),
            dimensions: HashMap::new(),
        }
    }

    pub fn add(mut self, key: &'a str, value: &'a str) -> Result<Self> {
        let set = self.map_values.get(key).unwrap();

        if set.contains(value) {
            self.dimensions
                .entry(key)
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
