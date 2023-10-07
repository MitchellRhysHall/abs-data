use crate::error_code::ErrorCode;

pub type Result<T> = std::result::Result<T, ErrorCode>;
