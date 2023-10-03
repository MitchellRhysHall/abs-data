use semver::Version;

use crate::{
    builders::url::UrlBuilder,
    config::Config,
    error_code::{ErrorCode, Result},
    models::{
        derived::{
            meta_data_sets::{MetaDataSet, MetaDataSets},
            sdmx_response::SdmxResponse,
        },
        typed::{
            agency_id::AgencyId, meta_detail::MetaDetail, reference::Reference,
            sdmx_meta_request::SdmxMetaRequest, sdmx_request::SdmxRequest,
            structure_id::StructureId, structure_type::StructureType,
        },
    },
};

pub struct SdmxMetaRequestBuilder<'a> {
    base_url: &'a str,
    structure_type: &'a StructureType,
    agency_id: Option<&'a AgencyId>,
    detail: Option<&'a MetaDetail>,
    structure_id: Option<&'a StructureId>,
    structure_version: Option<&'a Version>,
    references: Option<&'a Reference>,
    key: Option<&'a str>,
    headers: &'a [(&'a str, &'a str)],
}

impl<'a> SdmxMetaRequestBuilder<'a> {
    pub fn new(structure_type: &'a StructureType) -> Self {
        Self {
            base_url: Config::BASE_URL,
            structure_type,
            agency_id: None,
            detail: None,
            structure_id: None,
            structure_version: None,
            references: None,
            key: None,
            headers: &[Config::USER_AGENT_ANONYMOUS, Config::ACCEPT_STRUCTURE_JSON],
        }
    }

    pub fn detail(mut self, detail: &'a MetaDetail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn structure_id(mut self, structure_id: &'a StructureId) -> Self {
        self.structure_id = Some(structure_id);
        self
    }

    pub fn structure_version(mut self, structure_version: &'a Version) -> Self {
        self.structure_version = Some(structure_version);
        self
    }

    pub fn references(mut self, references: &'a Reference) -> Self {
        self.references = Some(references);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(&self) -> SdmxMetaRequest {
        let mut url_builder =
            UrlBuilder::new(self.base_url).add_path_segment(self.structure_type.to_string());

        if let Some(agency_id) = self.agency_id {
            url_builder = url_builder.add_path_segment(agency_id.to_string());
        } else {
            url_builder = url_builder.add_path_segment(AgencyId::default().to_string());
        }

        if let Some(structure_id) = self.structure_id {
            url_builder = url_builder.add_path_segment(structure_id.to_string());

            if let Some(structure_version) = self.structure_version {
                url_builder = url_builder.add_path_segment(structure_version.to_string());
            }
        }

        if let Some(references) = self.references {
            url_builder =
                url_builder.add_query_param(Config::QUERY_REFERENCES, references.to_string());
        }

        if let Some(detail) = self.detail {
            url_builder = url_builder.add_query_param(Config::QUERY_DETAIL, detail.to_string());
        }

        let url = url_builder.build().expect("Failed to build url");

        let request = SdmxRequest::new(url, self.key, self.headers);

        SdmxMetaRequest::from(request)
    }
}
