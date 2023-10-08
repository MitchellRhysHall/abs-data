use super::version::Version;

pub struct DataflowIdentifierDimensions<'a> {
    pub agency_id: Option<&'a str>,
    pub dataflow_id: &'a str,
    pub version: Option<&'a Version>,
}
