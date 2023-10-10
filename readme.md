# ABS Data API Rust Client

This Rust library provides a convenient way to interact with the Australian Bureau of Statistics (ABS) Data API [https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html](https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html). Utilizing builder types, this library allows fluent, type-safe requests to the API.

## Disclaimer

- **Unofficial:** This library has not been endorsed, sponsored, or officially recognized by the Australian Government in any capacity.
- **Alpha:** This library is currently alpha stage and incomplete. Additionally, the API its built on is in beta.

## Features

- **Builder Pattern for Requests:** Constructing requests is streamlined using builder types, allowing for a fluent and intuitive API.
- **Strongly Typed Models:** Benefit from Rust's type system with strongly typed models for the ABS data, minimizing runtime errors.
- **Cached Metadata** Reduce network roundtrips by using many const definitions set at compile-time.
  
## Examples

Add this to your `Cargo.toml`:

```toml
[dependencies]
abs_data = "0.1"
```

Getting all cpi datasets:

```rust
use abs_data::*;

let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::CPI).build()?;

let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
    .build()
    .send()
    .await?;

Ok(response)
```

Getting all metadata dataflows and then a dataset:

```rust
use abs_data::*;

let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
    .build()
    .send()
    .await?;

let dataflow = &meta_response.data[10]; // Select desired dataflow

let dataflow_identifier = DataflowIdentifierBuilder::new(&dataflow.id)
    .agency_id(&dataflow.agency_id)
    .version(&dataflow.version)
    .build()?;

let data_response = SdmxDataRequestBuilder::new(&dataflow_identifier)
    .detail(&Detail::DataOnly)
    .start_period(&Period::Year(2012))
    .end_period(&Period::Year(2022))
    .build()
    .send()
    .await?;

Ok(data_response)
```
