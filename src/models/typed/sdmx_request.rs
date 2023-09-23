use log::info;
use reqwest::header;
use url::Url;

use crate::{
    error_code::ErrorCode, error_code::Result, models::derived::sdmx_response::SdmxResponse,
};

use super::sdmx_client::SdmxClient;

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
