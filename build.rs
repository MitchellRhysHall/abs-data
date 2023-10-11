// use abs_data::{
//     builders::{
//         dataflow_identifier::DataflowIdentifierBuilder, sdmx_data_request::SdmxDataRequestBuilder,
//         sdmx_meta_request::SdmxMetaRequestBuilder,
//     },
//     models::{
//         derived::{
//             data_sets::DataSets, meta_data_sets::MetaDataSets, sdmx_response::SdmxResponse,
//             series::Series, value::Value,
//         },
//         typed::{
//             dataflow_id::DataflowId, detail::Detail, meta_detail::MetaDetail,
//             structure_type::StructureType,
//         },
//     },
// };
// use regex::Regex;
// use std::{
//     collections::{HashMap, HashSet},
//     fs,
//     sync::Arc,
// };
// use tokio::{runtime::Runtime, sync::Mutex};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     if std::env::var("CARGO_FEATURE_DYN_CONST_GEN").is_err() {
//         println!("cargo:warning=The build script is being skipped because the dyn-const-gen feature is not enabled.");
//         return Ok(());
//     }

//     let filepath = "src/models/generated/constants.rs";

//     let rt = Runtime::new().unwrap();

//     let future = async {
//         let contents = Arc::new(Mutex::new(String::new()));
//         contents
//             .clone()
//             .lock()
//             .await
//             .push_str("use super::super::typed::datakey_dimension::DataKeyDimension;");

//         let metadata = get_all_dataflows().await.unwrap();

//         // Maybe collect into (group by) struct name in loop, then write to file conetnts
//         let futures: Vec<_> = metadata
//             .data
//             .iter()
//             .map(|data| {
//                 let data_id = data.id.clone();
//                 let contents_clone = contents.clone();

//                 async move {
//                     println!("{}", data_id);
//                     let values = get_structure_types(&data_id).await;

//                     if let Ok(values) = values {
//                         let grouped = group_by_series_name(values);
//                         let new_contents = generate_rust_code(grouped);
//                         contents_clone.lock().await.push_str(&new_contents);
//                     }
//                 }
//             })
//             .collect();

//         let _: Vec<_> = futures::future::join_all(futures).await;

//         let _ = fs::write(filepath, contents.lock().await.as_bytes());
//     };

//     rt.block_on(future);

//     Ok(())
// }

// async fn get_all_dataflows() -> Result<SdmxResponse<MetaDataSets>, Box<dyn std::error::Error>> {
//     let meta_response = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
//         .build()
//         .send()
//         .await?;

//     Ok(meta_response)
// }

// async fn get_structure_types(
//     dataflowid: &str,
// ) -> Result<SdmxResponse<DataSets>, Box<dyn std::error::Error>> {
//     let dataflow_identifier = DataflowIdentifierBuilder::new(dataflowid).build()?;

//     let response = SdmxDataRequestBuilder::new(&dataflow_identifier)
//         .detail(&Detail::SeriesKeysOnly)
//         .build()
//         .send()
//         .await?;

//     Ok(response)
// }

// fn group_by_series_name(
//     response: SdmxResponse<DataSets>,
// ) -> HashMap<(Box<str>, u8), HashMap<Box<str>, (Box<str>, Box<str>)>> {
//     let mut grouped: HashMap<(Box<str>, u8), HashMap<Box<str>, (Box<str>, Box<str>)>> =
//         HashMap::new();

//     for series in response.structure.unwrap().dimensions.series.iter() {
//         let inner_map = grouped
//             .entry((series.name.clone(), series.key_position.unwrap_or(0)))
//             .or_default();

//         for value in series.values.iter() {
//             inner_map
//                 .entry(value.name.clone())
//                 .or_insert((value.name.clone(), value.id.clone()));
//         }
//     }

//     println!("{:?}", grouped.keys());

//     grouped
// }

// fn generate_rust_code(
//     series: HashMap<(Box<str>, u8), HashMap<Box<str>, (Box<str>, Box<str>)>>,
// ) -> String {
//     let mut output = String::new();
//     for series in series.iter() {
//         let struct_name = to_pascalcase_name(&series.0 .0);
//         output.push_str(&format!("\n\npub struct {};\n\n", struct_name));
//         let mut prev_names = HashSet::new();
//         output.push_str(&format!("impl {} {{\n", struct_name));
//         for value in series.1.iter() {
//             if !prev_names.contains(value.0) {
//                 let uppercase_name = to_uppercase_name(&value.0);
//                 let const_representation = determine_best_representation(series.0 .1, &value.1 .1);
//                 output.push_str(&format!(
//                     "    pub const {}: {};\n",
//                     uppercase_name, const_representation
//                 ));

//                 prev_names.insert(value.0.clone());
//             }
//         }
//         output.push_str("}\n\n");
//     }

//     output
// }

// fn to_uppercase_name(s: &str) -> String {
//     let mut output = s
//         .chars()
//         .map(|c| {
//             if c.is_alphanumeric() {
//                 c.to_ascii_uppercase()
//             } else {
//                 '_'
//             }
//         })
//         .collect::<String>();

//     if let Some(first) = s.chars().next() {
//         if !first.is_ascii_alphabetic() {
//             output.insert(0, '_');
//         }
//     }

//     let re = Regex::new(r"__+").unwrap();
//     re.replace_all(&output, "_").to_string()
// }

// fn to_pascalcase_name(input: &str) -> String {
//     let regex = Regex::new(r"[^a-zA-Z0-9\s]").unwrap();
//     let cleaned = regex.replace_all(input, "");
//     cleaned
//         .split_whitespace()
//         .map(|word| {
//             let mut chars = word.chars();
//             match chars.next() {
//                 None => String::new(),
//                 Some(fst) => fst.to_uppercase().collect::<String>() + chars.as_str(),
//             }
//         })
//         .collect()
// }

// fn determine_best_representation(key_position: u8, value: &str) -> String {
//     if let Ok(val) = value.parse::<u8>() {
//         return format!(
//             "DataKeyDimension = DataKeyDimension::U8({}, {})",
//             key_position, val
//         );
//     } else if let Ok(val) = value.parse::<u16>() {
//         return format!(
//             "DataKeyDimension = DataKeyDimension::U16({}, {})",
//             key_position, val
//         );
//     } else if let Ok(val) = value.parse::<u32>() {
//         return format!(
//             "DataKeyDimension = DataKeyDimension::U32({}, {})",
//             key_position, val
//         );
//     } else if let Ok(val) = value.parse::<u64>() {
//         return format!(
//             "DataKeyDimension = DataKeyDimension::U64({}, {})",
//             key_position, val
//         );
//     } else {
//         return format!(
//             "DataKeyDimension = DataKeyDimension::Str({},\"{}\")",
//             key_position, value
//         );
//     }
// }
