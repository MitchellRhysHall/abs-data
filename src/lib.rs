mod client;
mod error_code;
mod models;

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::client::SdmxClient;

    use super::*;

    #[tokio::test]
    async fn endpoint() {
        let client = SdmxClient::new();

        let response = client
            .get()
            .data("ABS,CPI,1.0.0", "1.1.1.1.1")
            .build()
            .send()
            .await;

        if response.is_err() {
            eprintln!("Fetch JSON failed: {:?}", response);
        } else {
            println!("{:?}", response.unwrap());
        }
    }
}
