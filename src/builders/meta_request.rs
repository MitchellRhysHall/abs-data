use url::Url;

use crate::{
    error_code::{ErrorCode, Result},
    models::typed::enums::{AgencyId, MetaDetail, Reference, StructureType},
};

use super::{client::SdmxClient, request::SdmxRequest};

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

    fn build_url(&self) -> Result<Url> {
        let mut url = Url::parse(self.base_url)?;

        {
            let mut path_segments = url
                .path_segments_mut()
                .map_err(|_| ErrorCode::UrlCannotBeABase)?;
            path_segments.extend(&[
                &self.structure_type.to_string(),
                &self.agency_id.to_string(),
            ]);

            if let Some(structure_id) = self.structure_id {
                path_segments.push(structure_id);

                if let Some(structure_version) = self.structure_version {
                    path_segments.push(structure_version);
                }
            }
        }

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(detail) = &self.detail {
                query_pairs.append_pair("detail", &detail.to_string());
            }
        }

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(references) = &self.references {
                query_pairs.append_pair("references", &references.to_string());
            }
        }

        Ok(url)
    }

    pub fn build(&self) -> SdmxRequest {
        let url = self
            .build_url()
            .expect("Failed to build the URL; this should never happen");

        SdmxRequest::new(self.client, url, self.key)
    }
}
