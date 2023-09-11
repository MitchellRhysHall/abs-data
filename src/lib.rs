mod client;
mod error_code;
mod models;

static BASE_URL: &str = "https://api.data.abs.gov.au";

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{client::Client, models::SdmxResponse};

    use super::*;

    // Mock responses for time being.
    #[tokio::test]
    async fn base() {
        let url = Url::parse(BASE_URL).expect("Failed to parse URL");
        let response = Client::new().fetch(url).await;
        if response.is_err() {
            eprintln!("Fetch JSON failed: {:?}", response);
        }
        println!("{}", response.unwrap());
    }

    #[tokio::test]
    async fn endpoint() {
        let endpoint = "/dataflow/ABS?detail=full";
        let path = format!("{}{}", BASE_URL, endpoint);
        let url = Url::parse(&path).expect("Failed to parse URL");
        let response = Client::new().fetch_json::<SdmxResponse>(url).await;
        if response.is_err() {
            panic!("Fetch JSON failed: {:?}", response);
        }
        println!("{:?}", response.unwrap());
    }
}
