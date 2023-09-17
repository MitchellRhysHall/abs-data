use std::{
    cell::OnceCell,
    fmt::{self, Display, Formatter},
    sync::OnceLock,
};

use log::info;
use reqwest::header;
use url::Url;

use crate::{
    error_code::{ErrorCode, Result},
    models::{
        derived::sdmx_response::SdmxResponse,
        typed::enums::{AgencyId, StructureType},
    },
};

use super::{
    client::SdmxClient, data_request::SdmxDataRequestBuilder, meta_request::SdmxMetaRequestBuilder,
};

pub struct SdmxRequestBuilder<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    key: Option<&'a str>,
}

impl<'a> SdmxRequestBuilder<'a> {
    pub fn new(client: &'a SdmxClient) -> Self {
        SdmxRequestBuilder {
            client,
            base_url: "https://api.data.abs.gov.au",
            key: None,
        }
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn data(&self, dataflow_id: &'a str, data_key: &'a str) -> SdmxDataRequestBuilder<'a> {
        SdmxDataRequestBuilder::new(
            self.client,
            self.base_url,
            "data",
            dataflow_id,
            data_key,
            self.key,
        )
    }

    pub fn meta(
        &self,
        structure_type: &'a StructureType,
        agency_id: &'a AgencyId,
    ) -> SdmxMetaRequestBuilder<'a> {
        SdmxMetaRequestBuilder::new(
            self.client,
            self.base_url,
            structure_type,
            agency_id,
            self.key,
        )
    }
}

pub struct SdmxRequest<'a> {
    client: &'a SdmxClient,
    url: Url,
    key: Option<&'a str>,
}

impl<'a> SdmxRequest<'a> {
    pub fn new(client: &'a SdmxClient, url: Url, key: Option<&'a str>) -> Self {
        Self { client, url, key }
    }

    pub async fn send<T>(&self) -> Result<SdmxResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        info!("{:?}", self.url.as_ref());

        let mut request = self
            .client
            .inner()
            .get(self.url.as_ref())
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header(header::ACCEPT, "application/vnd.sdmx.structure+json");

        if let Some(key) = &self.key {
            request = request.header("x-api-key", *key);
        }

        let response = request.send().await?;

        let status = response.status();

        if !status.is_success() {
            return Err(ErrorCode::Http(status));
        }

        let body_bytes = response.bytes().await?;

        if body_bytes.is_empty() {
            return Err(ErrorCode::HttpEmptyResponse);
        }

        info!("{}", std::str::from_utf8(&body_bytes.clone())?);

        let data: SdmxResponse<T> = serde_json::from_slice(&body_bytes)?;

        Ok(data)
    }
}
