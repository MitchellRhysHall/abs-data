use std::collections::HashMap;

use crate::{
    error_code::ErrorCode,
    error_code::Result,
    models::derived::{meta_data_sets::MetaDataSets, sdmx_response::SdmxResponse},
};

use super::sdmx_request::SdmxRequest;

pub struct SdmxMetaRequest<'a> {
    request: SdmxRequest<'a>,
}

impl<'a> SdmxMetaRequest<'a> {
    pub async fn send(&self) -> Result<SdmxResponse<MetaDataSets>> {
        let mut raw = self
            .request
            .send::<HashMap<Box<str>, MetaDataSets>>()
            .await?;

        let meta_data_sets = raw
            .data
            .drain()
            .next()
            .ok_or(ErrorCode::HashMapNoKeyValuesFound)?
            .1;

        let response: SdmxResponse<MetaDataSets> = SdmxResponse {
            data: meta_data_sets,
            meta: raw.meta,
        };

        Ok(response)
    }
}

impl<'a> From<SdmxRequest<'a>> for SdmxMetaRequest<'a> {
    fn from(request: SdmxRequest<'a>) -> Self {
        SdmxMetaRequest { request }
    }
}

// Prop testing instead? poptest crate
#[cfg(test)]
mod tests {
    use crate::{
        builders::sdmx_meta_request::SdmxMetaRequestBuilder,
        models::typed::{
            meta_detail::MetaDetail, reference::Reference, structure_id::StructureId,
            structure_type::StructureType,
        },
    };

    use super::*;
    use futures::future::join_all;
    use semver::Version;
    use strum::IntoEnumIterator;

    #[tokio::test]
    async fn send_request_all_structure_types() -> Result<()> {
        let futures: Vec<_> = StructureType::iter()
            .map(|structure_type| async move {
                let result = SdmxMetaRequestBuilder::new(&structure_type)
                    .detail(&MetaDetail::AllStubs)
                    .build()
                    .send()
                    .await;
                (structure_type, result)
            })
            .collect();

        let results: Vec<_> = join_all(futures).await;

        results.iter().for_each(|(structure_type, result)| {
            assert!(
                result.is_ok(),
                "Failed for StructureType::{:?} with error: {:?}",
                structure_type,
                result.as_ref().err().unwrap()
            )
        });

        Ok(())
    }

    #[tokio::test]
    async fn send_request_all_structure_ids() -> Result<()> {
        let futures: Vec<_> = StructureId::iter()
            .map(|structure_id| async move {
                let result = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
                    .detail(&MetaDetail::AllStubs)
                    .structure_id(&StructureId::All)
                    .build()
                    .send()
                    .await;
                (structure_id, result)
            })
            .collect();

        let results: Vec<_> = join_all(futures).await;

        results.iter().for_each(|(structure_id, result)| {
            assert!(
                result.is_ok(),
                "Failed for StructureId::{:?} with error: {:?}",
                structure_id,
                result.as_ref().err().unwrap()
            )
        });

        Ok(())
    }

    #[tokio::test]
    async fn send_request_all_references() -> Result<()> {
        let futures: Vec<_> = Reference::iter()
            .map(|reference| async move {
                let result = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
                    .detail(&MetaDetail::AllStubs)
                    .references(&reference)
                    .build()
                    .send()
                    .await;
                (reference, result)
            })
            .collect();

        let results: Vec<_> = join_all(futures).await;

        results.iter().for_each(|(reference, result)| {
            assert!(
                result.is_ok(),
                "Failed for Reference::{:?} with error: {:?}",
                reference,
                result.as_ref().err().unwrap()
            )
        });

        Ok(())
    }

    #[tokio::test]
    async fn send_request_default_version() -> Result<()> {
        let version = &Version::new(1, 0, 0);
        let result = SdmxMetaRequestBuilder::new(&StructureType::DataFlow)
            .detail(&MetaDetail::AllStubs)
            .structure_version(&version)
            .build()
            .send()
            .await;

        assert!(
            result.is_ok(),
            "Failed for Version::{:?} with error: {:?}",
            version,
            result.as_ref().err().unwrap()
        );

        Ok(())
    }
}
