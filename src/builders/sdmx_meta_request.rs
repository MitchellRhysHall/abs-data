use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::{
    builders::url::UrlBuilder,
    config::Config,
    error_code::Result,
    models::{
        derived::sdmx_response::SdmxResponse,
        typed::{
            agency_id::AgencyId, meta_detail::MetaDetail, reference::Reference,
            sdmx_request::SdmxRequest,
        },
    },
    traits::url_path_segment::UrlPathSegment,
};

pub struct SdmxMetaRequestBuilder<'a, T>
where
    T: UrlPathSegment + DeserializeOwned,
{
    base_url: &'a str,
    agency_id: Option<&'a AgencyId>,
    detail: Option<&'a MetaDetail>,
    structure_id: Option<&'a str>,
    structure_version: Option<&'a str>,
    references: Option<Reference<'a>>,
    key: Option<&'a str>,
    headers: &'a [(&'a str, &'a str)],
    phantom: PhantomData<T>,
}

impl<'a, T> SdmxMetaRequestBuilder<'a, T>
where
    T: UrlPathSegment + DeserializeOwned,
{
    pub fn new() -> Self {
        Self {
            base_url: Config::BASE_URL,
            agency_id: None,
            detail: None,
            structure_id: None,
            structure_version: None,
            references: None,
            key: None,
            headers: &[Config::USER_AGENT_ANONYMOUS, Config::ACCEPT_STRUCTURE_JSON],
            phantom: PhantomData,
        }
    }

    pub fn detail(mut self, detail: &'a MetaDetail) -> Self {
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
        let mut url_builder =
            UrlBuilder::new(self.base_url).add_path_segment(T::url_path_segment());

        if let Some(agency_id) = &self.agency_id {
            url_builder = url_builder.add_path_segment(agency_id.to_string());
        } else {
            url_builder = url_builder.add_path_segment(AgencyId::default().to_string());
        }

        if let Some(structure_id) = &self.structure_id {
            url_builder = url_builder.add_path_segment(structure_id.to_string());

            if let Some(structure_version) = &self.structure_version {
                url_builder = url_builder.add_path_segment(structure_version.to_string());
            }
        }

        if let Some(detail) = &self.detail {
            url_builder = url_builder.add_query_param(Config::QUERY_DETAIL, detail.to_string());
        }

        if let Some(references) = &self.references {
            url_builder =
                url_builder.add_query_param(Config::QUERY_REFERENCES, references.to_string());
        }

        let url = url_builder.build().expect("Failed to build url");

        SdmxRequest::new(url, self.key, self.headers)
    }

    pub async fn send(&self) -> Result<SdmxResponse<T>> {
        self.build().send::<T>().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::derived::{
        codelist::Codelists, concept_scheme::ConceptSchemes, data_structures::DataStructures,
        dataflows::Dataflows,
    };

    #[tokio::test]
    async fn send_request_for_dataflows() -> Result<()> {
        let _response = SdmxMetaRequestBuilder::<Dataflows>::new().send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_for_data_structures() -> Result<()> {
        let _response = SdmxMetaRequestBuilder::<DataStructures>::new()
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_for_codelists() -> Result<()> {
        let _response = SdmxMetaRequestBuilder::<Codelists>::new()
            .detail(&MetaDetail::AllStubs)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_for_concept_schemes() -> Result<()> {
        let _response = SdmxMetaRequestBuilder::<ConceptSchemes>::new()
            .send()
            .await?;

        Ok(())
    }
}
