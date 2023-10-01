use serde::de::DeserializeOwned;

pub trait StructureType: DeserializeOwned {
    fn url_path_segment() -> &'static str;
}
