use abs_data::{
    builders::{
        dataflow_identifier::DataflowIdentifierBuilder, sdmx_data_request::SdmxDataRequestBuilder,
        sdmx_meta_request::SdmxMetaRequestBuilder,
    },
    models::{
        derived::{data_sets::DataSets, meta_data_sets::MetaDataSets, sdmx_response::SdmxResponse},
        typed::{
            dataflow_id::DataflowId, detail::Detail, meta_detail::MetaDetail,
            structure_type::StructureType,
        },
    },
};
use std::{collections::HashSet, fs};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if std::env::var("CARGO_FEATURE_DYN_CONST_GEN").is_err() {
    //     println!("cargo:warning=The build script is being skipped because the dyn-const-gen feature is not enabled.");
    //     return Ok(());
    // }

    let filepath = "src/models/generated/constants.rs";

    let rt = Runtime::new().unwrap();

    let future = async {
        let metadata = get_all_dataflows().await.unwrap();

        let mut contents = String::new();
        contents.push_str("use super::super::typed::datakey_dimension::DataKeyDimension;");

        for data in metadata.data.iter() {
            let values = get_structure_types(&data.id).await.unwrap();

            let new_contents = generate_rust_code(values);

            contents.push_str(&new_contents);
        }

        let _ = fs::write(filepath, contents);
    };

    rt.block_on(future);

    Ok(())
}

async fn get_all_dataflows() -> Result<SdmxResponse<MetaDataSets>, Box<dyn std::error::Error>> {
    let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
        .build()
        .send()
        .await?;

    Ok(meta_response)
}

async fn get_structure_types(
    dataflowid: &str,
) -> Result<SdmxResponse<DataSets>, Box<dyn std::error::Error>> {
    let dataflow_identifier = DataflowIdentifierBuilder::new(dataflowid).build()?;

    let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
        .detail(&Detail::SeriesKeysOnly)
        .build()
        .send()
        .await?;

    Ok(response)
}

fn generate_rust_code(data: SdmxResponse<DataSets>) -> String {
    let mut output = String::new();
    for series in data.structure.unwrap().dimensions.series.iter() {
        let struct_name = to_pascalcase_name(&series.name);
        output.push_str(&format!("\n\npub struct {};\n\n", struct_name));
        let mut prev_names = HashSet::new();
        output.push_str(&format!("impl {} {{\n", struct_name));
        for value in series.values.iter() {
            if !prev_names.contains(&value.name) {
                let uppercase_name = to_uppercase_name(&value.name);
                let const_representation =
                    determine_best_representation(series.key_position.unwrap(), &value.id);
                output.push_str(&format!(
                    "    pub const {}: {};\n",
                    uppercase_name, const_representation
                ));

                prev_names.insert(value.name.clone());
            }
        }
        output.push_str("}\n\n");
    }

    output
}

fn to_uppercase_name(s: &str) -> String {
    let output = s
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();

    let re = regex::Regex::new(r"__+").unwrap();
    re.replace_all(&output, "_").to_string()
}

fn to_pascalcase_name(s: &str) -> String {
    s.replace(" ", "")
}

fn determine_best_representation(key_position: u8, value: &str) -> String {
    if let Ok(val) = value.parse::<u8>() {
        return format!(
            "DataKeyDimension = DataKeyDimension::U8({}, {})",
            key_position, val
        );
    } else if let Ok(val) = value.parse::<u16>() {
        return format!(
            "DataKeyDimension = DataKeyDimension::U16({}, {})",
            key_position, val
        );
    } else if let Ok(val) = value.parse::<u32>() {
        return format!(
            "DataKeyDimension = DataKeyDimension::U32({}, {})",
            key_position, val
        );
    } else if let Ok(val) = value.parse::<u64>() {
        return format!(
            "DataKeyDimension = DataKeyDimension::U64({}, {})",
            key_position, val
        );
    } else {
        return format!(
            "DataKeyDimension = DataKeyDimension::Str({},\"{}\")",
            key_position, value
        );
    }
}
