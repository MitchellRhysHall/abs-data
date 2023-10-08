use super::{agency_id::AgencyId, version::Version};

pub struct DataflowIdentifierDimensions<'a> {
    pub agency_id: Option<&'a AgencyId>,
    pub dataflow_id: &'a str,
    pub version: Option<&'a Version>,
}
