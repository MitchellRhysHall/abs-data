pub(crate) struct Config;

impl Config {
    pub(crate) const BASE_URL: &str = "https://api.data.abs.gov.au";
    pub(crate) const DATA_PATH: &str = "data";
    pub(crate) const DATA_KEY_MAX_LENGTH: usize = 260;
    pub(crate) const DATA_KEY_REQUIRED_DOT_COUNT: usize = 4;

    pub(crate) const QUERY_START_PERIOD: &str = "startPeriod";
    pub(crate) const QUERY_END_PERIOD: &str = "endPeriod";
    pub(crate) const QUERY_DETAIL: &str = "detail";
    pub(crate) const QUERY_DIMENSION_AT_OBSERVATION: &str = "dimensionAtObservation";
    pub(crate) const QUERY_REFERENCES: &str = "references";

    pub(crate) const MIME_TYPE_SDMX_STRUCTURE_JSON: &str = "application/vnd.sdmx.structure+json";
    pub(crate) const MIME_TYPE_SDMX_DATA_JSON: &str = "application/vnd.sdmx.data+json";

    pub(crate) const HEADER_ACCEPT_KEY: &str = "Accept";
    pub(crate) const HEADER_USER_AGENT_KEY: &str = "User-Agent";
    pub(crate) const HEADER_API_KEY: &str = "x-api-key";
    pub(crate) const HEADER_USER_AGENT_VALUE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

    pub(crate) const USER_AGENT_ANONYMOUS: (&str, &str) =
        (Self::HEADER_USER_AGENT_KEY, Self::HEADER_USER_AGENT_VALUE);

    pub(crate) const ACCEPT_STRUCTURE_JSON: (&str, &str) =
        (Self::HEADER_ACCEPT_KEY, Self::MIME_TYPE_SDMX_STRUCTURE_JSON);

    pub(crate) const ACCEPT_DATA_JSON: (&str, &str) =
        (Self::HEADER_ACCEPT_KEY, Self::MIME_TYPE_SDMX_DATA_JSON);
}
