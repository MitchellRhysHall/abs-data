mod builders;
mod error_code;
pub mod factories;
mod models;

use models::derived::data_sets::DataSets;
use models::typed::agency_id::AgencyId;
use models::typed::dataflow_identifier::DataflowIdentifier;
use models::typed::datakey::DataKey;
use models::typed::date_granularity::DateGranularity;
use models::typed::detail::Detail;
use models::typed::dimension_at_observation::DimensionAtObservation;
use models::typed::meta_detail::MetaDetail;
use models::typed::sdmx_client::SdmxClient;
use models::typed::structure_type::StructureType;

use crate::models::derived::dataflows::Dataflows;

use crate::error_code::Result;

pub async fn get_dataflows(agency_id: AgencyId) -> Result<Dataflows> {
    Ok(SdmxClient::new()
        .get()
        .meta(&StructureType::DataFlow, &agency_id)
        .detail(MetaDetail::All)
        .build()
        .send::<Dataflows>()
        .await?
        .data)
}

pub async fn get_data(
    dataflow_identifier: DataflowIdentifier,
    datakey: DataKey,
) -> Result<DataSets> {
    Ok(SdmxClient::new()
        .get()
        .data(dataflow_identifier, datakey)
        .start_period(DateGranularity::Year(2021))
        .end_period(DateGranularity::Year(2023))
        .detail(Detail::DataOnly)
        .dimension_at_observation(DimensionAtObservation::All)
        .build()
        .send::<DataSets>()
        .await?
        .data)
}

#[cfg(test)]
mod tests {
    use flexi_logger::{FileSpec, Logger};
    use semver::Version;

    use crate::{
        builders::{dataflow_identifier::DataflowIdentifierBuilder, datakey::DataKeyBuilder},
        models::typed::dataflow_id::DataflowId,
    };

    use super::*;

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
        let identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi)
            .agency_id(AgencyId::Abs)
            .version(Version::new(1, 0, 0))
            .build()?;

        let key = DataKey::no_filter();

        let _response = crate::get_data(identifier, key).await?;

        Ok(())
    }
}
