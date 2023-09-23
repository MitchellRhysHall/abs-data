use semver::Version;

use super::{agency_id::AgencyId, dataflow_id::DataflowId};

pub struct DataflowIdentifierDimensions<'a> {
    pub agency_id: Option<&'a AgencyId>,
    pub dataflow_id: &'a DataflowId,
    pub version: Option<&'a Version>,
}
