use std::{cell::OnceCell, sync::OnceLock};

use crate::factories::sdmx_request::SdmxRequestBuilderFactory;

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

    pub fn get(&self) -> SdmxRequestBuilderFactory {
        SdmxRequestBuilderFactory::new(&self)
    }

    pub fn inner(&self) -> &reqwest::Client {
        self.inner.get().expect("client not initialized")
    }
}
