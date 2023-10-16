use crate::models::typed::dataflow_identifier::DataflowIdentifier;
use crate::models::typed::version::Version;

pub struct DataflowIdentifierBuilder<'a> {
    agency_id: Option<&'a str>,
    structure_id: &'a str,
    version: Option<&'a Version>,
}

impl<'a> DataflowIdentifierBuilder<'a> {
    pub fn new(structure_id: &'a str) -> Self {
        DataflowIdentifierBuilder {
            agency_id: None,
            structure_id,
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

    pub fn build(self) -> DataflowIdentifier<'a> {
        DataflowIdentifier::new(self.agency_id, self.structure_id, self.version)
    }
}
