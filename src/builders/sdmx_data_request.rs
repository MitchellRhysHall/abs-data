use crate::{
    builders::url::UrlBuilder,
    config::{self, Config},
    error_code::Result,
    models::{
        derived::{data_sets::DataSets, sdmx_response::SdmxResponse},
        typed::{
            dataflow_identifier::DataflowIdentifier, datakey::DataKey,
            date_granularity::DateGranularity, detail::Detail,
            dimension_at_observation::DimensionAtObservation, sdmx_request::SdmxRequest,
        },
    },
};

pub struct SdmxDataRequestBuilder<'a> {
    base_url: &'a str,
    path: &'a str,
    dataflow_identifier: &'a DataflowIdentifier,
    data_key: Option<&'a DataKey>,
    start_period: Option<&'a DateGranularity<'a>>,
    end_period: Option<&'a DateGranularity<'a>>,
    detail: Option<&'a Detail>,
    dimension_at_observation: Option<&'a DimensionAtObservation<'a>>,
    key: Option<&'a str>,
    headers: &'a [(&'a str, &'a str)],
}

impl<'a> SdmxDataRequestBuilder<'a> {
    pub fn new(dataflow_identifier: &'a DataflowIdentifier) -> Self {
        Self {
            base_url: Config::BASE_URL,
            path: Config::DATA_PATH,
            dataflow_identifier,
            data_key: None,
            start_period: None,
            end_period: None,
            detail: None,
            dimension_at_observation: None,
            key: None,
            headers: &[Config::USER_AGENT_ANONYMOUS, Config::ACCEPT_DATA_JSON],
        }
    }

    pub fn data_key(mut self, data_key: &'a DataKey) -> Self {
        self.data_key = Some(data_key);
        self
    }

    pub fn start_period(mut self, start_period: &'a DateGranularity<'a>) -> Self {
        self.start_period = Some(start_period);
        self
    }

    pub fn end_period(mut self, end_period: &'a DateGranularity<'a>) -> Self {
        self.end_period = Some(end_period);
        self
    }

    pub fn detail(mut self, detail: &'a Detail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn dimension_at_observation(
        mut self,
        dimension_at_observation: &'a DimensionAtObservation<'a>,
    ) -> Self {
        self.dimension_at_observation = Some(dimension_at_observation);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    fn build(&self) -> SdmxRequest {
        let mut url_builder = UrlBuilder::new(self.base_url)
            .add_path_segment(self.path)
            .add_path_segment(self.dataflow_identifier.to_string());

        if let Some(data_key) = self.data_key {
            url_builder = url_builder.add_path_segment(data_key.to_string());
        } else {
            url_builder = url_builder.add_path_segment(DataKey::default().to_string());
        }

        if let Some(start_period) = &self.start_period {
            url_builder =
                url_builder.add_query_param(Config::QUERY_START_PERIOD, start_period.to_string());
        }
        if let Some(end_period) = &self.end_period {
            url_builder =
                url_builder.add_query_param(Config::QUERY_END_PERIOD, end_period.to_string());
        }
        if let Some(detail) = &self.detail {
            url_builder = url_builder.add_query_param(Config::QUERY_DETAIL, detail.to_string());
        }
        if let Some(dimension_at_observation) = &self.dimension_at_observation {
            url_builder = url_builder.add_query_param(
                Config::QUERY_DIMENSION_AT_OBSERVATION,
                dimension_at_observation.to_string(),
            );
        }

        let url = url_builder.build().expect("Failed to build url");

        SdmxRequest::new(url, self.key, self.headers)
    }

    pub async fn send(&self) -> Result<SdmxResponse<DataSets>> {
        self.build().send::<DataSets>().await
    }
}

#[cfg(test)]
mod tests {
    use futures::future::join_all;
    use strum::IntoEnumIterator;

    use super::*;
    use crate::{
        builders::dataflow_identifier::DataflowIdentifierBuilder,
        models::typed::dataflow_id::DataflowId,
    };

    #[tokio::test]
    async fn send_request_with_all_details() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(&DataflowId::Cpi).build()?;

        let futures: Vec<_> = Detail::iter()
            .map(|detail| async {
                let result = SdmxDataRequestBuilder::new(&dataflow_identifier)
                    .detail(&detail)
                    .send()
                    .await;
                (detail, result)
            })
            .collect();

        let results: Vec<_> = join_all(futures).await;

        results.iter().for_each(|(detail, result)| {
            assert!(
                result.is_ok(),
                "Failed for Detail::{:?} with error: {:?}",
                detail,
                result.as_ref().err().unwrap()
            )
        });

        Ok(())
    }
}
