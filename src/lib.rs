mod builders;
mod config;
mod error_code;
mod models;

#[cfg(test)]
mod tests {
    use crate::{
        builders::{
            dataflow_identifier::DataflowIdentifierBuilder,
            sdmx_data_request::SdmxDataRequestBuilder, sdmx_meta_request::SdmxMetaRequestBuilder,
        },
        models::typed::{
            dataflow_id::DataflowId, date_granularity::DateGranularity, detail::Detail,
            structure_type::StructureType,
        },
    };

    #[tokio::test]
    async fn send_request_with_all_details() -> Result<(), Box<dyn std::error::Error>> {
        let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .build()
            .send()
            .await?;

        let dataflow = &meta_response.data[10]; // Select desired dataflow

        println!("{dataflow:?}");

        let dataflow_identifier =
            DataflowIdentifierBuilder::new(&DataflowId::Specific(dataflow.id.clone()))
                .agency_id(&dataflow.agency_id)
                .version(&dataflow.version)
                .build()?;

        let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .detail(&Detail::DataOnly)
            .start_period(&DateGranularity::Year(2012))
            .end_period(&DateGranularity::Year(2022))
            .build()
            .send()
            .await?;

        println!("{response:?}");

        Ok(())
    }
}
