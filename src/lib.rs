mod builders;
mod error_code;
mod models;

use models::typed::agency_id::AgencyId;
use models::typed::dataflow_identifier::DataflowIdentifier;
use models::typed::datakey::DataKey;
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
) -> Result<Dataflows> {
    Ok(SdmxClient::new()
        .get()
        .data(dataflow_identifier, datakey) // Should be strongly typed input params
        .build()
        .send::<Dataflows>()
        .await?
        .data)
}

#[cfg(test)]
mod tests {
    use flexi_logger::{FileSpec, Logger};
    use semver::Version;

    use crate::{
        builders::dataflow_identifier::DataflowIdentifierBuilder,
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
        let response = crate::get_dataflows(AgencyId::Abs).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn data() -> Result<()> {
        init_logger();

        let key = DataKey::no_filter();
        let id = DataflowId::Cpi;

        let identifier = DataflowIdentifierBuilder::new(id)
            .agency_id(AgencyId::Abs)
            .version(Version::new(1, 0, 0))
            .build()?;

        let response = crate::get_data(identifier, key).await?;

        println!("{:?}", response);

        Ok(())
    }
}
