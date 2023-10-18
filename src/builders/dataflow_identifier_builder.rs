use crate::models::typed::dataflow_identifier::DataflowIdentifier;
use crate::models::typed::version::Version;

pub struct DataflowIdentifierBuilder {
    agency_id: Option<Box<str>>,
    structure_id: Box<str>,
    version: Option<Version>,
}

impl DataflowIdentifierBuilder {
    pub fn new(structure_id: &str) -> Self {
        DataflowIdentifierBuilder {
            agency_id: None,
            structure_id: structure_id.into(),
            version: None,
        }
    }

    pub fn agency_id(mut self, agency_id: &str) -> Self {
        self.agency_id = Some(agency_id.into());
        self
    }

    pub fn version(mut self, version: &Version) -> Self {
        self.version = Some(version.clone());
        self
    }

    pub fn build(self) -> DataflowIdentifier {
        DataflowIdentifier::new(self.agency_id, self.structure_id, self.version)
    }
}
