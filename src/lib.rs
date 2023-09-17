mod client;
mod error_code;
mod models;

use client::{AgencyId, SdmxClient, StructureType, Stub};
use models::{DataStructures, Dataflows};

use crate::error_code::Result;

pub async fn get_dataflow_ids(agency_id: AgencyId) -> Result<Vec<String>> {
    Ok(SdmxClient::new()
        .get()
        .meta(&StructureType::DataFlow, &agency_id)
        .stub(Stub::All)
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
    use log::info;
    use std::io::Write;

    use crate::client::{SdmxClient, Stub};

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
    async fn data() {
        let client = SdmxClient::new();

        let response = client
            .get()
            .data("ABS,CPI,1.0.0", "1.1.1.1.1")
            .build()
            .send::<Dataflows>()
            .await;

        assert!(response.is_ok())
    }
}
