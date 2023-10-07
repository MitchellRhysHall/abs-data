pub struct Config;

impl Config {
    pub const BASE_URL: &str = "https://api.data.abs.gov.au";
    pub const DATA_PATH: &str = "data";
    pub const DATA_KEY_MAX_LENGTH: usize = 260;
    pub const DATA_KEY_REQUIRED_DOT_COUNT: usize = 4;

    pub const QUERY_START_PERIOD: &str = "startPeriod";
    pub const QUERY_END_PERIOD: &str = "endPeriod";
    pub const QUERY_DETAIL: &str = "detail";
    pub const QUERY_DIMENSION_AT_OBSERVATION: &str = "dimensionAtObservation";
    pub const QUERY_REFERENCES: &str = "references";

    pub const MIME_TYPE_SDMX_STRUCTURE_JSON: &str = "application/vnd.sdmx.structure+json";
    pub const MIME_TYPE_SDMX_DATA_JSON: &str = "application/vnd.sdmx.data+json";

    pub const HEADER_ACCEPT_KEY: &str = "Accept";
    pub const HEADER_USER_AGENT_KEY: &str = "User-Agent";
    pub const HEADER_API_KEY: &str = "x-api-key";
    pub const HEADER_USER_AGENT_VALUE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

    pub const USER_AGENT_ANONYMOUS: (&str, &str) =
        (Self::HEADER_USER_AGENT_KEY, Self::HEADER_USER_AGENT_VALUE);

    pub const ACCEPT_STRUCTURE_JSON: (&str, &str) =
        (Self::HEADER_ACCEPT_KEY, Self::MIME_TYPE_SDMX_STRUCTURE_JSON);

    pub const ACCEPT_DATA_JSON: (&str, &str) =
        (Self::HEADER_ACCEPT_KEY, Self::MIME_TYPE_SDMX_DATA_JSON);
}
