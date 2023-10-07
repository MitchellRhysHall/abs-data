use super::sdmx_request::SdmxRequest;
use crate::models::derived::{
    data_sets::{DataSetWrapper, DataSets},
    sdmx_response::SdmxResponse,
};
use crate::result::Result;

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
