use std::collections::HashMap;

use super::sdmx_request::SdmxRequest;
use crate::result::Result;
use crate::{
    error_code::ErrorCode,
    models::derived::{meta_data_sets::MetaDataSets, sdmx_response::SdmxResponse},
};

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

    pub async fn send(&self) -> Result<SdmxResponse<MetaDataSets>> {
        let mut raw = self
            .request
            .send::<HashMap<Box<str>, MetaDataSets>>()
            .await?;

        let meta_data_sets = raw
            .data
            .drain()
            .next()
            .ok_or(ErrorCode::HashMapNoKeyValuesFound)?
            .1;

        let response: SdmxResponse<MetaDataSets> = SdmxResponse {
            data: meta_data_sets,
            meta: raw.meta,
            structure: raw.structure,
        };

        Ok(response)
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxMetaRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxMetaRequest { request }
    }
}
