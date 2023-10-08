# ABS Data API Rust Client

This Rust library provides a convenient way to interact with the Australian Bureau of Statistics (ABS) Data API [https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html](https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html). Utilizing builder types, this library allows fluent, type-safe requests to the API.

## Features

- **Builder Pattern for Requests:** Constructing requests is streamlined using builder types, allowing for a fluent and intuitive API.
- **Strongly Typed Models:** Benefit from Rust's type system with strongly typed models for the ABS data, minimizing runtime errors.
- **Ease of Use:** This library simplifies interacting in the SDMX format, making it as convenient as common HTTP JSON APIs.
  
## Examples

Add this to your `Cargo.toml`:

```toml
[dependencies]
abs_data = "0.1"
```

Getting all cpi datasets:

```rust
use abs_data::{
    builders::{
        dataflow_identifier::DataflowIdentifierBuilder, sdmx_data_request::SdmxDataRequestBuilder
    },
    models::derived::{data_sets::DataSets, sdmx_response::SdmxResponse},
    models::typed::dataflow_id::DataflowId,
};

async fn get_cpi_datasets() -> Result<SdmxResponse<DataSets>, Box<dyn std::error::Error>> {
    let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::CPI).build()?;

    let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .build()
        .send()
        .await?;

    Ok(response)
}
```

Getting all metadata dataflows and then a dataset:

```rust
use abs_data::{
    builders::{
        dataflow_identifier::DataflowIdentifierBuilder, sdmx_data_request::SdmxDataRequestBuilder,
        sdmx_meta_request::SdmxMetaRequestBuilder,
    },
    models::derived::{data_sets::DataSets, sdmx_response::SdmxResponse},
    models::typed::{
        dataflow_id::DataflowId, detail::Detail, period::Period, structure_type::StructureType,
    },
};

async fn get_dataflows_then_dataset() -> Result<SdmxResponse<DataSets>, Box<dyn std::error::Error>>
{
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
}

```
