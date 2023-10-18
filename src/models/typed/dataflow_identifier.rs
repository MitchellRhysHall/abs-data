use super::version::Version;

/// The dataflow identifier in {agencyId},{dataflowId},{version} format
/// (eg. "ABS,CPI,1.0.0"). A list of all available dataflows can be returned
/// using the GET /{structureType}/{agencyId} operation.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataflowIdentifier {
    agency_id: Option<Box<str>>,
    structure_id: Box<str>,
    version: Option<Version>,
    key: Box<str>,
}

impl DataflowIdentifier {
    fn format_key(
        agency_id: Option<&str>,
        structure_id: &str,
        version: Option<&Version>,
    ) -> Box<str> {
        [
            agency_id.as_ref().unwrap_or(&"ABS".into()),
            &structure_id,
            version.unwrap_or(&Version::default()).as_ref(),
        ]
        .join(",")
        .into()
    }

    pub fn new(
        agency_id: Option<Box<str>>,
        structure_id: Box<str>,
        version: Option<Version>,
    ) -> Self {
        Self {
            key: Self::format_key(agency_id.as_deref(), &structure_id, version.as_ref()),
            agency_id,
            structure_id,
            version,
        }
    }

    pub fn agency_id(&self) -> Option<&str> {
        self.agency_id.as_deref()
    }

    pub fn structure_id(&self) -> &str {
        &self.structure_id
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}
