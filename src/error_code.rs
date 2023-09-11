pub type Result<T> = std::result::Result<T, ErrorCode>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode {
    #[error("HTTP status error: {0}")]
    Http(reqwest::StatusCode),

    #[error("HTTP request error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Deserialization error: {0}")]
    SerdeError(#[from] serde::de::value::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}
