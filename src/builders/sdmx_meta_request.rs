use crate::{
    builders::url::UrlBuilder,
    models::typed::{
        agency_id::AgencyId, meta_detail::MetaDetail, reference::Reference,
        sdmx_client::SdmxClient, sdmx_request::SdmxRequest, structure_type::StructureType,
    },
};

pub struct SdmxMetaRequestBuilder<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    structure_type: &'a StructureType,
    agency_id: &'a AgencyId,
    detail: Option<MetaDetail>,
    structure_id: Option<&'a str>,
    structure_version: Option<&'a str>,
    references: Option<Reference<'a>>,
    key: Option<&'a str>,
}

impl<'a> SdmxMetaRequestBuilder<'a> {
    pub fn new(
        client: &'a SdmxClient,
        base_url: &'a str,
        structure_type: &'a StructureType,
        agency_id: &'a AgencyId,
        key: Option<&'a str>,
    ) -> Self {
        Self {
            client,
            base_url,
            structure_type,
            agency_id,
            detail: None,
            structure_id: None,
            structure_version: None,
            references: None,
            key,
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

    pub fn build(&self) -> SdmxRequest {
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

        SdmxRequest::new(self.client, url, self.key)
    }
}
