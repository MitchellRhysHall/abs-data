use crate::models::typed::agency_id::AgencyId;
use crate::models::typed::dataflow_identifier::DataflowIdentifier;
use crate::models::typed::dataflow_identifier_dimensions::DataflowIdentifierDimensions;
use crate::models::typed::version::Version;
use crate::result::Result;

pub struct DataflowIdentifierBuilder<'a> {
    agency_id: Option<&'a str>,
    dataflow_id: &'a str,
    version: Option<&'a Version>,
}

impl<'a> DataflowIdentifierBuilder<'a> {
    pub fn new(dataflow_id: &'a str) -> Self {
        DataflowIdentifierBuilder {
            agency_id: None,
            dataflow_id,
            version: None,
        }
    }

    pub fn agency_id(mut self, agency_id: &'a str) -> Self {
        self.agency_id = Some(agency_id);
        self
    }

    pub fn version(mut self, version: &'a Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn build(self) -> Result<DataflowIdentifier> {
        let dimensions = DataflowIdentifierDimensions {
            agency_id: self.agency_id,
            dataflow_id: self.dataflow_id,
            version: self.version,
        };

        let dataflow_identifier = DataflowIdentifier::try_from(dimensions)?;

        Ok(dataflow_identifier)
    }
}
