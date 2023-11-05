use std::collections::{HashMap, HashSet};

use crate::{
    error_code::ErrorCode,
    models::typed::{
        dataflow_identifier::DataflowIdentifier, datakey::DataKey,
        datakey_dimension::DataKeyDimension, reference::Reference, structure_type::StructureType,
    },
    result::Result,
};

use super::sdmx_meta_request_builder::SdmxMetaRequestBuilder;

pub struct DataKeyBuilder<'a> {
    dataflow_identifier: &'a DataflowIdentifier,
    dimensions: HashMap<Box<str>, HashSet<Box<str>>>,
}

impl<'a> DataKeyBuilder<'a> {
    async fn get_constraints(
        id: &DataflowIdentifier,
    ) -> Result<(HashMap<Box<str>, u8>, HashMap<Box<str>, HashSet<Box<str>>>)> {
        let mut builder = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .structure_id(&id.structure_id())
            .reference(&Reference::StructureType(StructureType::ContentConstraint));

        if let Some(agency_id) = id.agency_id() {
            builder = builder.agency_id(agency_id)
        }
        if let Some(version) = id.version() {
            builder = builder.structure_version(version)
        }

        let data = builder.build().send().await?.data;

        let key_values = data
            .content_constraints
            .as_ref()
            .ok_or(ErrorCode::Custom("cannot find content constraints".into()))?
            .first()
            .ok_or(ErrorCode::Custom("content constraint empty".into()))?
            .cube_regions
            .as_ref()
            .ok_or(ErrorCode::Custom("cube regions empty".into()))?
            .first()
            .ok_or(ErrorCode::Custom("cub regions is empty".into()))?
            .key_values
            .as_ref();

        let mut key_order = HashMap::new();
        let mut constraints = HashMap::new();

        for (i, kv) in key_values.into_iter().enumerate() {
            let index = i as u8;
            let id_clone = kv.id.clone();
            let values_set: HashSet<Box<str>> = kv.values.clone().into_vec().into_iter().collect();

            key_order.insert(id_clone.clone(), index);
            constraints.insert(id_clone, values_set);
        }

        Ok((key_order, constraints))
    }

    fn is_subset(
        dimensions: &HashMap<Box<str>, HashSet<Box<str>>>,
        constraints: &HashMap<Box<str>, HashSet<Box<str>>>,
    ) -> bool {
        dimensions.iter().all(|(key, value_set)| {
            constraints
                .get(key)
                .map_or(false, |dimension_set| value_set.is_subset(dimension_set))
        })
    }

    pub fn new(dataflow_identifier: &'a DataflowIdentifier) -> Self {
        Self {
            dataflow_identifier,
            dimensions: HashMap::new(),
        }
    }

    pub fn add(mut self, dimension: &'a DataKeyDimension) -> Self {
        self.dimensions
            .entry(dimension.key().into())
            .or_insert_with(HashSet::new)
            .insert(dimension.value().into());

        self
    }

    pub async fn build(self) -> Result<DataKey> {
        let (key_order, constraints) = Self::get_constraints(self.dataflow_identifier).await?;

        if !Self::is_subset(&self.dimensions, &constraints) {
            return Err(ErrorCode::Custom("Invalid datakey dimensions".into()));
        }

        let mut key = self
            .dimensions
            .into_iter()
            .map(|(m, s)| (m, s.into_iter().collect::<Vec<_>>().join("+")))
            .collect::<Vec<_>>();

        key.sort_by_key(|(m, _)| key_order.get(m));

        let key_string = key
            .iter()
            .map(|(_, s)| s.to_owned())
            .collect::<Vec<_>>()
            .join(".");

        Ok(DataKey::parse(&key_string).expect("should always be valid"))
    }
}
