use std::sync::atomic::{AtomicBool, Ordering};

use quick_cache::{
    sync::{Cache, DefaultLifecycle},
    DefaultHashBuilder, OptionsBuilder, UnitWeighter,
};
use tokio::sync::OnceCell;

use crate::{
    builders::sdmx_meta_request_builder::SdmxMetaRequestBuilder,
    models::{
        derived::{key_value::KeyValue, meta_data_map::MetaDataMap, meta_data_sets::MetaDataSet},
        typed::{
            dataflow_identifier::DataflowIdentifier, reference::Reference,
            structure_type::StructureType, version::Version,
        },
    },
    result::Result,
};

static META_CACHE: OnceCell<Cache<DataflowIdentifier, MetaDataMap>> = OnceCell::const_new();
static IS_INIT: AtomicBool = AtomicBool::new(false);

pub(crate) struct MetaCache;

impl MetaCache {
    fn init() -> Cache<DataflowIdentifier, MetaDataMap> {
        Cache::with_options(
            OptionsBuilder::new()
                .estimated_items_capacity(10000)
                .weight_capacity(10000)
                .build()
                .unwrap(),
            UnitWeighter,
            DefaultHashBuilder::default(),
            DefaultLifecycle::default(),
        )
    }

    fn ensure_cache_init() {
        if !IS_INIT.load(Ordering::Relaxed) {
            META_CACHE.set(Self::init()).expect("Couldn't set cache");
            IS_INIT.store(true, Ordering::Relaxed);
        }
    }

    pub async fn ensure_key_init(key: &DataflowIdentifier) -> Result<()> {
        Self::ensure_cache_init();
        if META_CACHE
            .get()
            .expect("must init cache")
            .get(key)
            .is_none()
        {
            let response = Self::get_meta(key).await?;
            let cache = META_CACHE.get().expect("must init cache");
            cache.insert_with_lifecycle(key.clone(), response);
        }
        Ok(())
    }

    pub fn get(key: &DataflowIdentifier) -> Option<MetaDataMap> {
        META_CACHE.get()?.get(key)
    }

    pub fn get_key_values(key: &DataflowIdentifier) -> Box<[KeyValue]> {
        Self::get(key)
            .expect("key not found")
            .content_constraints
            .as_ref()
            .expect("must populate content constraint in cache")
            .first()
            .expect("content constraint empty")
            .cube_regions
            .as_ref()
            .expect("cube regions empty")
            .first()
            .expect("cub regions is empty")
            .key_values
            .as_ref()
            .to_vec()
            .into()
    }

    async fn get_meta(id: &DataflowIdentifier) -> Result<MetaDataMap> {
        Ok(SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .structure_id(&id.structure_id())
            .reference(&Reference::StructureType(StructureType::ContentConstraint))
            .agency_id(id.agency_id().unwrap_or("ABS"))
            .structure_version(id.version().unwrap_or(&Version::default()))
            .build()
            .send()
            .await?
            .data)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builders::dataflow_identifier_builder::DataflowIdentifierBuilder, cache::MetaCache,
        result::Result,
    };

    #[tokio::test]
    async fn test1() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

        let cache = MetaCache::get(&dataflow_identifier);

        println!("{:?}", cache);

        Ok(())
    }
}
