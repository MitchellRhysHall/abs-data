use crate::{
    builders::url_builder::UrlBuilder,
    config::Config,
    models::typed::{
        dataflow_identifier::DataflowIdentifier, datakey::DataKey, detail::Detail,
        dimension_at_observation::DimensionAtObservation, period::Period,
        sdmx_data_request::SdmxDataRequest, sdmx_request::SdmxRequest,
    },
};

pub struct SdmxDataRequestBuilder<'a> {
    base_url: &'a str,
    path: &'a str,
    dataflow_identifier: &'a DataflowIdentifier,
    data_key: Option<&'a DataKey>,
    start_period: Option<&'a Period>,
    end_period: Option<&'a Period>,
    detail: Option<&'a Detail>,
    dimension_at_observation: Option<&'a DimensionAtObservation>,
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

    pub fn start_period(mut self, start_period: &'a Period) -> Self {
        self.start_period = Some(start_period);
        self
    }

    pub fn end_period(mut self, end_period: &'a Period) -> Self {
        self.end_period = Some(end_period);
        self
    }

    pub fn detail(mut self, detail: &'a Detail) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn dimension_at_observation(
        mut self,
        dimension_at_observation: &'a DimensionAtObservation,
    ) -> Self {
        self.dimension_at_observation = Some(dimension_at_observation);
        self
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(&self) -> SdmxDataRequest {
        let mut url_builder = UrlBuilder::new(self.base_url)
            .add_path_segment(self.path)
            .add_path_segment(self.dataflow_identifier.key());

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

        let request = SdmxRequest::new(url, self.key, self.headers);

        SdmxDataRequest::from(request)
    }
}
