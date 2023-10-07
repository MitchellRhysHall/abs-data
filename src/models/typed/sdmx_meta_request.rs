use std::collections::HashMap;

use crate::{
    error_code::ErrorCode,
    error_code::Result,
    models::derived::{meta_data_sets::MetaDataSets, sdmx_response::SdmxResponse},
};

use super::sdmx_request::SdmxRequest;

pub struct SdmxMetaRequest<'a> {
    request: SdmxRequest<'a>,
}

impl<'a> SdmxMetaRequest<'a> {
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
        };

        Ok(response)
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxMetaRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxMetaRequest { request }
    }
}
