use super::mime_type::MimeTypeFactory;

pub struct RequestHeaderFactory;

impl RequestHeaderFactory {
    const ACCEPT_KEY: &str = "Accept";
    const USER_AGENT_KEY: &str = "User-Agent";

    const USER_AGENT_ANON: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

    pub const USER_AGENT_ANONYMOUS: (&str, &str) = (Self::USER_AGENT_KEY, Self::USER_AGENT_ANON);

    pub const ACCEPT_STRUCTURE_JSON: (&str, &str) =
        (Self::ACCEPT_KEY, MimeTypeFactory::SDMX_STRUCTURE_JSON);

    pub const ACCEPT_DATA_JSON: (&str, &str) = (Self::ACCEPT_KEY, MimeTypeFactory::SDMX_DATA_JSON);
}
