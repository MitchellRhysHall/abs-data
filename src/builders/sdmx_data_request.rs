use log::info;

use crate::{
    builders::url::UrlBuilder,
    error_code::Result,
    factories::{request_header::RequestHeaderFactory, url::UrlFactory},
    models::{
        derived::sdmx_response::SdmxResponse,
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
    dataflow_identifier: DataflowIdentifier,
    data_key: DataKey,
    start_period: Option<DateGranularity<'a>>,
    end_period: Option<DateGranularity<'a>>,
    detail: Option<Detail>,
    dimension_at_observation: Option<DimensionAtObservation<'a>>,
    key: Option<&'a str>,
    headers: &'a [(&'a str, &'a str)],
}

impl<'a> SdmxDataRequestBuilder<'a> {
    pub fn new(dataflow_identifier: DataflowIdentifier, data_key: DataKey) -> Self {
        Self {
            base_url: UrlFactory::BASE,
            path: "data",
            dataflow_identifier,
            data_key,
            start_period: None,
            end_period: None,
            detail: None,
            dimension_at_observation: None,
            key: None,
            headers: &[
                RequestHeaderFactory::USER_AGENT_ANONYMOUS,
                RequestHeaderFactory::ACCEPT_DATA_JSON,
            ],
        }
    }

    pub fn start_period(mut self, start_period: DateGranularity<'a>) -> Self {
        self.start_period = Some(start_period);
        self
    }

    pub fn end_period(mut self, end_period: DateGranularity<'a>) -> Self {
        self.end_period = Some(end_period);
        self
    }

    pub fn detail(mut self, detail: Detail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn dimension_at_observation(
        mut self,
        dimension_at_observation: DimensionAtObservation<'a>,
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
            .add_path_segment(format!("{}", self.dataflow_identifier))
            .add_path_segment(format!("{}", self.data_key));

        if let Some(start_period) = &self.start_period {
            url_builder = url_builder.add_query_param("startPeriod", start_period.to_string());
        }
        if let Some(end_period) = &self.end_period {
            url_builder = url_builder.add_query_param("endPeriod", end_period.to_string());
        }
        if let Some(detail) = &self.detail {
            url_builder = url_builder.add_query_param("detail", detail.to_string());
        }
        if let Some(dimension_at_observation) = &self.dimension_at_observation {
            url_builder = url_builder.add_query_param(
                "dimensionAtObservation",
                dimension_at_observation.to_string(),
            );
        }

        let url = url_builder.build().expect("Failed to build url");
        info!("{}", url);

        SdmxRequest::new(url, self.key, self.headers)
    }

    pub async fn send<T>(&self) -> Result<SdmxResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.build().send::<T>().await
    }
}
