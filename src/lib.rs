mod builders;
mod error_code;
mod models;
use builders::client::SdmxClient;
use models::typed::enums::{AgencyId, MetaDetail, StructureType};

use crate::models::derived::dataflows::Dataflows;

use crate::error_code::Result;

pub async fn get_dataflow_ids(agency_id: AgencyId) -> Result<Vec<String>> {
    Ok(SdmxClient::new()
        .get()
        .meta(&StructureType::DataFlow, &agency_id)
        .detail(MetaDetail::All)
        .build()
        .send::<Dataflows>()
        .await?
        .data
        .dataflows
        .iter()
        .map(|dataflow| dataflow.id.to_string())
        .collect())
}

pub async fn get_data(dataflow_id: &str, datakey: &str) -> Result<Vec<String>> {
    Ok(SdmxClient::new()
        .get()
        .data(dataflow_id, datakey)
        .build()
        .send::<Dataflows>()
        .await?
        .data
        .dataflows
        .iter()
        .map(|dataflow| dataflow.id.to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use flexi_logger::{FileSpec, Logger};

    use crate::{
        builders::datakey::{DataKey, DataKeyBuilder},
        models::typed::enums::{Frequency, Measure, Region},
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
        let response = crate::get_dataflow_ids(AgencyId::ABS).await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn data() -> Result<()> {
        init_logger();
        let key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .add_frequency(Frequency::Quarterly)
            .add_region(Region::Aus)
            .build()?;

        let response = crate::get_data("CPI", key.as_ref()).await?;

        println!("{:?}", response);

        Ok(())
    }
}
