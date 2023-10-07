# ABS Data API Rust Client

This Rust library provides a convenient way to interact with the Australian Bureau of Statistics (ABS) Data API [https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html](https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html). Utilizing builder types, this library allows fluent, type-safe requests to the API.

## Features

- **Builder Pattern for Requests:** Constructing requests is streamlined using builder types, allowing for a fluent and intuitive API.
- **Strongly Typed Models:** Benefit from Rust's type system with strongly typed models for the ABS data, minimizing runtime errors.
- **Ease of Use:** Designed to be straightforward and user-friendly, this library abstracts away much of the boilerplate associated with interacting with the ABS Data API in rust.
  
## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
abs_data = "0.1"
```

Request and select by dataflow:

```rust
use abs_data::builders::{
    dataflow_identifier::DataflowIdentifierBuilder, sdmx_data_request::SdmxDataRequestBuilder,
};
use abs_data::models::typed::{dataflow_id::DataflowId, detail::Detail};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
        .build()
        .send()
        .await?;

    let dataflow = &meta_response.data[10]; // Select desired dataflow

    println!("{dataflow:?}");

    let dataflow_identifier =
        DataflowIdentifierBuilder::new(&DataflowId::Specific(dataflow.id.clone()))
            .agency_id(&dataflow.agency_id)
            .version(&dataflow.version)
            .build()?;

    let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .detail(&Detail::DataOnly)
        .start_period(&DateGranularity::Year(2012))
        .end_period(&DateGranularity::Year(2022))
        .build()
        .send()
        .await?;

    println!("{response:?}");

    Ok(())
}

```

