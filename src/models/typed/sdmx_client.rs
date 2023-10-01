use std::{cell::OnceCell, sync::OnceLock};

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub struct SdmxClient {
    inner: OnceCell<&'static reqwest::Client>,
}

impl SdmxClient {
    pub fn get_or_init() -> Self {
        let inner = CLIENT.get_or_init(reqwest::Client::new);
        Self {
            inner: inner.into(),
        }
    }

    pub fn inner(&self) -> &reqwest::Client {
        self.inner.get().expect("client not initialized")
    }
}
