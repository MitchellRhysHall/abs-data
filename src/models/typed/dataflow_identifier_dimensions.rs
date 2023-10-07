use super::{agency_id::AgencyId, dataflow_id::DataflowId, version::Version};

pub struct DataflowIdentifierDimensions<'a> {
    pub agency_id: Option<&'a AgencyId>,
    pub dataflow_id: &'a DataflowId,
    pub version: Option<&'a Version>,
}
