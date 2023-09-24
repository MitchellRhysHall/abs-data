use crate::{
    builders::url::UrlBuilder,
    error_code::Result,
    factories::{request_header::RequestHeaderFactory, url::UrlFactory},
    models::{
        derived::sdmx_response::SdmxResponse,
        typed::{
            agency_id::AgencyId, meta_detail::MetaDetail, reference::Reference,
            sdmx_request::SdmxRequest, structure_type::StructureType,
        },
    },
};

pub struct SdmxMetaRequestBuilder<'a> {
    base_url: &'a str,
    structure_type: &'a StructureType,
    agency_id: &'a AgencyId,
    detail: Option<MetaDetail>,
    structure_id: Option<&'a str>,
    structure_version: Option<&'a str>,
    references: Option<Reference<'a>>,
    key: Option<&'a str>,
    headers: &'a [(&'a str, &'a str)],
}

impl<'a> SdmxMetaRequestBuilder<'a> {
    pub fn new(structure_type: &'a StructureType, agency_id: &'a AgencyId) -> Self {
        Self {
            base_url: UrlFactory::BASE,
            structure_type,
            agency_id,
            detail: None,
            structure_id: None,
            structure_version: None,
            references: None,
            key: None,
            headers: &[
                RequestHeaderFactory::USER_AGENT_ANONYMOUS,
                RequestHeaderFactory::ACCEPT_STRUCTURE_JSON,
            ],
        }
    }

    pub fn detail(mut self, detail: MetaDetail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn structure_id(mut self, structure_id: &'a str) -> Self {
        self.structure_id = Some(structure_id);
        self
    }

    pub fn structure_version(mut self, structure_version: &'a str) -> Self {
        self.structure_version = Some(structure_version);
        self
    }

    pub fn references(mut self, references: Reference<'a>) -> Self {
        self.references = Some(references);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    fn build(&self) -> SdmxRequest {
        let mut url_builder = UrlBuilder::new(self.base_url)
            .add_path_segment(self.structure_type.to_string())
            .add_path_segment(self.agency_id.to_string());

        if let Some(structure_id) = &self.structure_id {
            url_builder = url_builder.add_path_segment(structure_id.to_string());

            if let Some(structure_version) = &self.structure_version {
                url_builder = url_builder.add_path_segment(structure_version.to_string());
            }
        }

        if let Some(detail) = &self.detail {
            url_builder = url_builder.add_query_param("detail", detail.to_string());
        }

        if let Some(references) = &self.references {
            url_builder = url_builder.add_query_param("references", references.to_string());
        }

        let url = url_builder.build().expect("Failed to build url");

        SdmxRequest::new(url, self.key, self.headers)
    }

    pub async fn send<T>(&self) -> Result<SdmxResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.build().send::<T>().await
    }
}
