use super::sdmx_request::SdmxRequest;
use crate::models::derived::meta_data_map::MetaDataMap;
use crate::models::derived::sdmx_response::SdmxResponse;
use crate::result::Result;

pub struct SdmxMetaRequest<'a> {
    request: SdmxRequest<'a>,
}

impl<'a> SdmxMetaRequest<'a> {
    pub fn url(&self) -> &str {
        self.request.url()
    }

    pub fn headers(&self) -> &'a [(&'a str, &'a str)] {
        self.request.headers()
    }

    pub async fn send(&self) -> Result<SdmxResponse<MetaDataMap>> {
        self.request.send::<MetaDataMap>().await
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxMetaRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxMetaRequest { request }
    }
}
