mod builders;
mod error_code;
mod factories;
mod models;

use builders::sdmx_data_request::SdmxDataRequestBuilder;
use builders::sdmx_meta_request::SdmxMetaRequestBuilder;
use models::derived::data_sets::DataSets;
use models::typed::agency_id::AgencyId;
use models::typed::dataflow_identifier::DataflowIdentifier;
use models::typed::datakey::DataKey;
use models::typed::date_granularity::DateGranularity;
use models::typed::detail::Detail;
use models::typed::dimension_at_observation::DimensionAtObservation;
use models::typed::meta_detail::MetaDetail;
use models::typed::structure_type::StructureType;

use crate::models::derived::dataflows::Dataflows;

use crate::error_code::Result;

// Should work out type T dataflows from the structure type internally.
pub async fn get_dataflows(agency_id: AgencyId) -> Result<Dataflows> {
    Ok(
        SdmxMetaRequestBuilder::new(&StructureType::DataFlow, &agency_id)
            .detail(MetaDetail::All)
            .send::<Dataflows>()
            .await?
            .data,
    )
}

pub async fn get_data(
    dataflow_identifier: DataflowIdentifier,
    datakey: DataKey,
) -> Result<DataSets> {
    Ok(SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
        .start_period(DateGranularity::Year(2021))
        .end_period(DateGranularity::Year(2023))
        .detail(Detail::DataOnly)
        .dimension_at_observation(DimensionAtObservation::TimePeriod)
        .send::<DataSets>()
        .await?
        .data)
}

#[cfg(test)]
mod tests {
    use flexi_logger::{FileSpec, Logger};
    use semver::Version;

    use super::*;
    use crate::{
        builders::{dataflow_identifier::DataflowIdentifierBuilder, datakey::DataKeyBuilder},
        models::typed::dataflow_id::DataflowId,
    };
    use std::io::Write;

    fn init_logger() {
        Logger::try_with_env_or_str("info, mycrate=debug")
            .expect("could not create logs")
            .log_to_file(FileSpec::default())
            .start()
            .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    }

    #[tokio::test]
    async fn get_dataflow_ids() -> Result<()> {
        let _response = crate::get_dataflows(AgencyId::Abs).await?;

        Ok(())
    }

    #[tokio::test]
    async fn data() -> Result<()> {
        let identifier = DataflowIdentifierBuilder::new(DataflowId::Ppi).build()?;

        let key = DataKey::no_filter();

        let _response = crate::get_data(identifier, key).await?;

        Ok(())
    }

    #[tokio::test]
    async fn dynamic_data_key() -> Result<()> {
        let dataflows = crate::get_dataflows(AgencyId::Abs).await?;

        let identifier = DataflowIdentifierBuilder::new(dataflows.dataflows[0].id.clone())
            .agency_id(AgencyId::Abs)
            .version(Version::new(1, 0, 0))
            .build()?;

        let key = DataKey::no_filter();
        let _response = crate::get_data(identifier, key).await?;

        Ok(())
    }
}
