use super::version::Version;

/// The dataflow identifier in {agencyId},{dataflowId},{version} format
/// (eg. "ABS,CPI,1.0.0"). A list of all available dataflows can be returned
/// using the GET /{structureType}/{agencyId} operation.
pub struct DataflowIdentifier<'a> {
    agency_id: Option<&'a str>,
    structure_id: &'a str,
    version: Option<&'a Version>,
    key: Box<str>,
}

impl<'a> DataflowIdentifier<'a> {
    fn format_key(
        agency_id: Option<&'a str>,
        structure_id: &'a str,
        version: Option<&'a Version>,
    ) -> Box<str> {
        [
            agency_id.as_ref().unwrap_or(&"ABS"),
            structure_id,
            version.unwrap_or(&Version::default()).as_ref(),
        ]
        .join(",")
        .into()
    }

    pub fn new(
        agency_id: Option<&'a str>,
        structure_id: &'a str,
        version: Option<&'a Version>,
    ) -> Self {
        Self {
            agency_id,
            structure_id,
            version,
            key: Self::format_key(agency_id, structure_id, version),
        }
    }

    pub fn agency_id(&self) -> Option<&str> {
        self.agency_id
    }

    pub fn structure_id(&self) -> &str {
        self.structure_id
    }

    pub fn version(&self) -> Option<&Version> {
        self.version
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}
