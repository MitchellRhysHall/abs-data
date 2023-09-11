use url::Url;

use crate::error_code::{ErrorCode, Result};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch(&self, url: Url) -> Result<String> {
        let response = self
            .client
            .get(url)
            .header("Accept", "application/vnd.smdx.data+json")
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(ErrorCode::Http(status));
        }

        let data = response.text().await?;

        Ok(data)
    }

    pub async fn fetch_json<T>(&self, url: Url) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .header("Accept", "application/vnd.smdx.data+json")
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(ErrorCode::Http(status));
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }
}
