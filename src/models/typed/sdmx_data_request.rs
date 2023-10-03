use crate::{
    error_code::Result,
    models::derived::{
        data_sets::{DataSetWrapper, DataSets},
        sdmx_response::SdmxResponse,
    },
};

use super::sdmx_request::SdmxRequest;

pub struct SdmxDataRequest<'a> {
    request: SdmxRequest<'a>,
}

impl<'a> SdmxDataRequest<'a> {
    pub async fn send(&self) -> Result<SdmxResponse<DataSets>> {
        let raw = self.request.send::<DataSetWrapper>().await?;

        let response: SdmxResponse<DataSets> = SdmxResponse {
            data: raw.data.data_sets,
            meta: raw.meta,
        };

        Ok(response)
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxDataRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxDataRequest { request }
    }
}

#[cfg(test)]
mod tests {
    use futures::future::join_all;
    use strum::IntoEnumIterator;

    use super::*;
    use crate::{
        builders::{
            dataflow_identifier::DataflowIdentifierBuilder,
            sdmx_data_request::SdmxDataRequestBuilder,
        },
        models::typed::{dataflow_id::DataflowId, detail::Detail},
    };

    #[tokio::test]
    async fn send_request_with_all_details() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(&DataflowId::Cpi).build()?;

        let futures: Vec<_> = Detail::iter()
            .map(|detail| async {
                let result = SdmxDataRequestBuilder::new(&dataflow_identifier)
                    .detail(&detail)
                    .build()
                    .send()
                    .await;
                (detail, result)
            })
            .collect();

        let results: Vec<_> = join_all(futures).await;

        results.iter().for_each(|(detail, result)| {
            assert!(
                result.is_ok(),
                "Failed for Detail::{:?} with error: {:?}",
                detail,
                result.as_ref().err().unwrap()
            )
        });

        Ok(())
    }
}
