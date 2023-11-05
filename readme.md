# ABS Data API Rust Client

This Rust library provides a convenient way to interact with the Australian Bureau of Statistics (ABS) Data API [https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html](https://api.gov.au/assets/APIs/abs/DataAPI.openapi.html). It Utilizes builder types for fluent requests to the API.

## Disclaimer

- **Unofficial:** This library has not been endorsed, sponsored, or officially recognized by the Australian Government in any capacity.
- **Alpha:** This library is currently alpha stage and incomplete. Additionally, the API its built on is in beta.

## Features

- **Builder Pattern for Requests:** Constructing requests is fluent with builder types.
- **Strongly Typed Models:** Strongly typed models for the ABS interface, minimizing runtime errors.
  
## Examples

Add this to your `Cargo.toml`:

```toml
[dependencies]
abs_data = "0.1"
```

Example requests:

```rust
use abs_data::{
    builders::{
        dataflow_identifier_builder::DataflowIdentifierBuilder,
        datakey_builder::DataKeyBuilder, sdmx_data_request_builder::SdmxDataRequestBuilder,
        sdmx_meta_request_builder::SdmxMetaRequestBuilder,
    },
    models::typed::{
        datakey::DataKey, datakey_dimension::DataKeyDimension, detail::Detail, period::Period,
        structure_type::StructureType,
    },
    result::Result,
};

async fn get_dynamic_meta_and_use_for_request() -> Result<()> {
    let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
        .build()
        .send()
        .await?;

    let dataflow = &meta_response.data.dataflows.unwrap()[10]; // Select desired dataflow

    let dataflow_identifier = DataflowIdentifierBuilder::new(&dataflow.id)
        .agency_id(&dataflow.agency_id)
        .version(&dataflow.version)
        .build();

    let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .detail(&Detail::DataOnly)
        .start_period(&Period::Year(2012))
        .end_period(&Period::Year(2022))
        .build()
        .send()
        .await?;

    Ok(())
}

async fn get_all_data_for_structure_id_without_filter() -> Result<()> {
    let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

    let _response = SdmxDataRequestBuilder::new(&dataflow_identifier) // Avoid 500 response with data only detail (issue with beta api)
        .build()
        .send()
        .await?;

    Ok(())
}

async fn get_datakeys_for_structure_id() -> Result<()> {
    let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

    let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .detail(&Detail::SeriesKeysOnly)
        .build()
        .send()
        .await?;

    Ok(())
}

async fn get_data_with_custom_datakey() -> Result<()> {
    let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

    let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .data_key(&DataKey::parse("1.40066.10.8.Q")?)
        .detail(&Detail::DataOnly)
        .build()
        .send()
        .await?;

    Ok(())
}

async fn get_data_with_datakey_builder_validation() -> Result<()> {
    let dataflow_identifier = DataflowIdentifierBuilder::new("CPI").build();

    let key = DataKeyBuilder::new(&dataflow_identifier)
        .add(&DataKeyDimension::new("MEASURE", "1"))
        .add(&DataKeyDimension::new("INDEX", "40066"))
        .add(&DataKeyDimension::new("REGION", "8"))
        .add(&DataKeyDimension::new("FREQ", "Q"))
        .add(&DataKeyDimension::new("TSEST", "10"))
        .build()
        .await?;

    assert_eq!(key, DataKey::parse("1.40066.10.8.Q")?);

    let _response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .data_key(&key)
        .detail(&Detail::DataOnly)
        .build()
        .send()
        .await?;

    Ok(())
}
```