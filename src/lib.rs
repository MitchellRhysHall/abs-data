mod builders;
mod config;
mod error_code;
mod models;

#[cfg(test)]
mod tests {
    use crate::{
        builders::{
            dataflow_identifier::DataflowIdentifierBuilder,
            sdmx_data_request::SdmxDataRequestBuilder,
        },
        models::typed::{
            dataflow_id::DataflowId, date_granularity::DateGranularity, detail::Detail,
        },
    };

    use crate::error_code::Result;

    #[tokio::test]
    async fn send_request_with_all_details() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(&DataflowId::Cpi).build()?;

        let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
            .detail(&Detail::DataOnly)
            .start_period(&DateGranularity::Year(1960))
            .end_period(&DateGranularity::Year(2022))
            .build()
            .send()
            .await?;

        println!("{response:?}");

        Ok(())
    }
}
