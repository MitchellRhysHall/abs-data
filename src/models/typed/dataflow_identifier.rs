use std::fmt;

use crate::{
    builders::dataflow_identifier::DataflowIdentifierBuilder, error_code::ErrorCode,
    error_code::Result,
};

use super::{
    dataflow_id::DataflowId, dataflow_identifier_dimensions::DataflowIdentifierDimensions,
};

/// The dataflow identifier in {agencyId},{dataflowId},{version} format
/// (eg. "ABS,CPI,1.0.0"). A list of all available dataflows can be returned
/// using the GET /{structureType}/{agencyId} operation.
pub struct DataflowIdentifier {
    inner: Box<str>,
}

impl DataflowIdentifier {
    // Can create from invalid state
    pub fn new(str: &str) -> Self {
        Self { inner: str.into() }
    }

    pub fn builder(dataflow_id: DataflowId) -> DataflowIdentifierBuilder {
        DataflowIdentifierBuilder::new(dataflow_id)
    }
}

impl AsRef<str> for DataflowIdentifier {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl fmt::Display for DataflowIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl TryFrom<DataflowIdentifierDimensions<'_>> for DataflowIdentifier {
    type Error = ErrorCode;

    fn try_from(dimensions: DataflowIdentifierDimensions) -> Result<Self> {
        let mut dimensions_vec = Vec::new();

        if let Some(agency_id) = dimensions.agency_id {
            dimensions_vec.push(agency_id.to_string());
        }

        dimensions_vec.push(dimensions.dataflow_id.to_string());

        if let Some(version) = dimensions.version {
            dimensions_vec.push(version.to_string());
        }

        let inner = dimensions_vec.join(",");

        let dataflow_identifier = DataflowIdentifier {
            inner: inner.into(),
        };

        Ok(dataflow_identifier)
    }
}