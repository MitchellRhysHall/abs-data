use crate::{
    builders::{
        sdmx_data_request::SdmxDataRequestBuilder, sdmx_meta_request::SdmxMetaRequestBuilder,
    },
    models::typed::{
        agency_id::AgencyId, dataflow_identifier::DataflowIdentifier, datakey::DataKey,
        sdmx_client::SdmxClient, structure_type::StructureType,
    },
};

use super::request_header::RequestHeaderFactory;

pub struct SdmxRequestBuilderFactory<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    key: Option<&'a str>,
}

impl<'a> SdmxRequestBuilderFactory<'a> {
    pub fn new(client: &'a SdmxClient) -> Self {
        Self {
            client,
            base_url: "https://api.data.abs.gov.au",
            key: None,
        }
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn data(
        &self,
        dataflow_identifier: DataflowIdentifier,
        data_key: DataKey,
    ) -> SdmxDataRequestBuilder<'a> {
        SdmxDataRequestBuilder::new(
            self.client,
            self.base_url,
            "data",
            dataflow_identifier,
            data_key,
            self.key,
            &[
                RequestHeaderFactory::USER_AGENT_ANONYMOUS,
                RequestHeaderFactory::ACCEPT_DATA_JSON,
            ],
        )
    }

    pub fn meta(
        &self,
        structure_type: &'a StructureType,
        agency_id: &'a AgencyId,
    ) -> SdmxMetaRequestBuilder<'a> {
        SdmxMetaRequestBuilder::new(
            self.client,
            self.base_url,
            structure_type,
            agency_id,
            self.key,
            &[
                RequestHeaderFactory::USER_AGENT_ANONYMOUS,
                RequestHeaderFactory::ACCEPT_STRUCTURE_JSON,
            ],
        )
    }
}
