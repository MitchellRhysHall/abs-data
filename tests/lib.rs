#[cfg(test)]
mod tests {
    use abs_data::{
        builders::{
            dataflow_identifier_builder::DataflowIdentifierBuilder,
            datakey_builder::DataKeyBuilder, sdmx_data_request_builder::SdmxDataRequestBuilder,
            sdmx_meta_request_builder::SdmxMetaRequestBuilder,
        },
        models::typed::{
            datakey::DataKey, detail::Detail, period::Period, structure_type::StructureType,
        },
        result::Result,
    };

    #[tokio::test]
    async fn test1() -> Result<()> {
        let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .build()
            .send()
            .await?;

        let dataflow = &meta_response.data.dataflows.unwrap()[10]; // Select desired dataflow

        let dataflow_identifier = DataflowIdentifierBuilder::new(&dataflow.id)
            .agency_id(&dataflow.agency_id)
            .version(&dataflow.version)
            .build();

        let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .detail(&Detail::DataOnly)
            .start_period(&Period::Year(2012))
            .end_period(&Period::Year(2022))
            .build()
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test2() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

        let request_builder = SdmxDataRequestBuilder::new(&dataflow_identifier);

        let request = request_builder.build();

        let response = request.send().await?;

        println!("{:?}", response);

        Ok(())
    }

    #[tokio::test]
    async fn get_datakeys_for_dataflow() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

        let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .detail(&Detail::SeriesKeysOnly)
            .build()
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn with_parse_datakey() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

        let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .data_key(&DataKey::parse("1.40066.10.8.Q")?)
            .detail(&Detail::DataOnly)
            .build()
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn with_datakey_builder() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

        let key = DataKeyBuilder::new(&dataflow_identifier)
            .await
            .add("MEASURE", "1")?
            .add("INDEX", "40066")?
            .add("REGION", "8")?
            .add("FREQ", "Q")?
            .add("TSEST", "10")?
            .build();

        assert_eq!(key, DataKey::parse("1.40066.10.8.Q")?);

        let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .data_key(&key)
            .detail(&Detail::DataOnly)
            .build()
            .send()
            .await?;

        Ok(())
    }
}
