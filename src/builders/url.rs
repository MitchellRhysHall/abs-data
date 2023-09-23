use crate::{error_code::ErrorCode, Result};
use url::Url;

pub struct UrlBuilder {
    base_url: Box<str>,
    path_segments: Vec<Box<str>>,
    query_params: Vec<(Box<str>, Box<str>)>,
}

impl UrlBuilder {
    pub fn new<U>(base_url: U) -> Self
    where
        U: Into<Box<str>>,
    {
        UrlBuilder {
            base_url: base_url.into(),
            path_segments: Vec::new(),
            query_params: Vec::new(),
        }
    }

    pub fn add_path_segment<U>(mut self, segment: U) -> Self
    where
        U: Into<Box<str>>,
    {
        self.path_segments.push(segment.into());
        self
    }

    pub fn add_query_param<U, V>(mut self, key: U, value: V) -> Self
    where
        U: Into<Box<str>>,
        V: Into<Box<str>>,
    {
        self.query_params.push((key.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Url> {
        let mut url = Url::parse(&*self.base_url)?;

        url.path_segments_mut()
            .map_err(|_| ErrorCode::UrlCannotBeABase)?
            .extend(self.path_segments.into_iter().map(|s| s));

        if !self.query_params.is_empty() {
            let query_string = self
                .query_params
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&");
            url.set_query(Some(&query_string));
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::UrlBuilder;
    use url::Url;

    #[test]
    fn test_valid_url_building() {
        let url = UrlBuilder::new("http://example.com")
            .add_path_segment("path")
            .add_query_param("key", "value")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/path?key=value");
    }

    #[test]
    fn test_invalid_base_url() {
        let result = UrlBuilder::new("invalid_url")
            .add_path_segment("path")
            .add_query_param("key", "value")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_no_path_segments() {
        let url = UrlBuilder::new("http://example.com")
            .add_query_param("key", "value")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/?key=value");
    }

    #[test]
    fn test_multiple_path_segments() {
        let url = UrlBuilder::new("http://example.com")
            .add_path_segment("path1")
            .add_path_segment("path2")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/path1/path2");
    }

    #[test]
    fn test_no_query_params() {
        let url = UrlBuilder::new("http://example.com")
            .add_path_segment("path")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/path");
    }

    #[test]
    fn test_multiple_query_params() {
        let url = UrlBuilder::new("http://example.com")
            .add_query_param("key1", "value1")
            .add_query_param("key2", "value2")
            .build()
            .unwrap();

        let expected_url = Url::parse("http://example.com/?key1=value1&key2=value2").unwrap();
        assert_eq!(url, expected_url);
    }

    #[test]
    fn test_empty_path_segment() {
        let url = UrlBuilder::new("http://example.com")
            .add_path_segment("")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/");
    }

    #[test]
    fn test_empty_query_param() {
        let url = UrlBuilder::new("http://example.com")
            .add_query_param("", "")
            .build()
            .unwrap();

        assert_eq!(url.as_str(), "http://example.com/?=");
    }
}
