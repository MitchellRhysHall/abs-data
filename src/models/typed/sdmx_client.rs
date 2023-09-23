use std::{cell::OnceCell, sync::OnceLock};

use crate::builders::sdmx_request::SdmxRequestBuilder;

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub struct SdmxClient {
    inner: OnceCell<&'static reqwest::Client>,
}

impl SdmxClient {
    pub fn new() -> Self {
        let inner = CLIENT.get_or_init(|| reqwest::Client::new());
        Self {
            inner: inner.into(),
        }
    }

    pub fn get(&self) -> SdmxRequestBuilder {
        SdmxRequestBuilder::new(&self)
    }

    pub fn inner(&self) -> &reqwest::Client {
        self.inner.get().expect("client not initialized")
    }
}
