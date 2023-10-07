pub type Result<T> = std::result::Result<T, ErrorCode>;

use std::num::ParseIntError;

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

    #[error("Data key length cannot be larger than {0}")]
    DataKeyLengthExceeded(usize),

    #[error("Data key contains non number: {0}")]
    DataKeyContainsNonNumber(Box<str>),

    #[error("UTF-8 decode error: {0}")]
    Utf8Decode(#[from] std::str::Utf8Error),

    #[error("Hashmap contains no keys or values")]
    HashMapNoKeyValuesFound,

    #[error("Version string unknown character: {0}")]
    VersionStringContainsUnknownChar(#[from] ParseIntError),

    #[error("Version string length not exactly three: {0}")]
    VersionStringNotCorrectLength(usize),

    #[error("Custom error: {0}")]
    Custom(Box<str>),
}
