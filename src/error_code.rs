pub type Result<T> = std::result::Result<T, ErrorCode>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode {
    #[error("HTTP status error: {0}")]
    Http(reqwest::StatusCode),

    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    Serde(#[from] serde::de::value::Error),

    #[error("JSON slice deserialization error: {0}")]
    JsonSliceDeserialization(#[from] serde_json::Error),

    #[error("Empty response body")]
    HttpEmptyResponse,

    #[error("Url parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("URL cannot be a base")]
    UrlCannotBeABase,

    #[error("UTF-8 decode error: {0}")]
    Utf8Decode(#[from] std::str::Utf8Error),

    #[error("Custom error: {0}")]
    Custom(String),
}
