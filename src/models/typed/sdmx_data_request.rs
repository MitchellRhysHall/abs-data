use super::sdmx_request::SdmxRequest;
use crate::models::derived::{data_sets::DataSets, sdmx_response::SdmxResponse};
use crate::result::Result;

pub struct SdmxDataRequest<'a> {
    request: SdmxRequest<'a>,
}

impl<'a> SdmxDataRequest<'a> {
    pub fn url(&self) -> &str {
        self.request.url()
    }

    pub fn headers(&self) -> &'a [(&'a str, &'a str)] {
        self.request.headers()
    }

    pub async fn send(&self) -> Result<SdmxResponse<DataSets>> {
        self.request.send::<DataSets>().await
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxDataRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxDataRequest { request }
    }
}
