mod builders;
mod config;
mod error_code;
mod models;
mod result;

#[cfg(test)]
mod tests {
    use crate::{
        builders::{
            dataflow_identifier::DataflowIdentifierBuilder,
            sdmx_data_request::SdmxDataRequestBuilder, sdmx_meta_request::SdmxMetaRequestBuilder,
        },
        models::typed::{
            dataflow_id::DataflowId, detail::Detail, period::Period, structure_type::StructureType,
        },
        result::Result,
    };

    #[tokio::test]
    async fn test1() -> Result<()> {
        let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .build()
            .send()
            .await?;

        let dataflow = &meta_response.data[10]; // Select desired dataflow

        println!("{dataflow:?}");

        let dataflow_identifier = DataflowIdentifierBuilder::new(&dataflow.id)
            .agency_id(&dataflow.agency_id)
            .version(&dataflow.version)
            .build()?;

        let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .detail(&Detail::DataOnly)
            .start_period(&Period::Year(2012))
            .end_period(&Period::Year(2022))
            .build()
            .send()
            .await?;

        println!("{response:?}");

        Ok(())
    }

    #[tokio::test]
    async fn test2() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::CPI).build()?;

        let request_builder = SdmxDataRequestBuilder::new(&dataflow_identifier);

        let request = request_builder.build();

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }
}
