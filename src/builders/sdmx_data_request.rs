use log::info;

use crate::{
    builders::url::UrlBuilder,
    config::Config,
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
            base_url: Config::BASE,
            path: "data",
            dataflow_identifier,
            data_key,
            start_period: None,
            end_period: None,
            detail: None,
            dimension_at_observation: None,
            key: None,
            headers: &[Config::USER_AGENT_ANONYMOUS, Config::ACCEPT_DATA_JSON],
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

    pub async fn send(&self) -> Result<SdmxResponse<DataSets>> {
        self.build().send::<DataSets>().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        builders::dataflow_identifier::DataflowIdentifierBuilder,
        models::typed::dataflow_id::DataflowId,
    };

    #[tokio::test]
    async fn send_request_with_default_detail() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi).build()?;
        let datakey = DataKey::default();
        let _response = SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_with_full_detail() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi).build()?;
        let datakey = DataKey::default();
        let _response = SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
            .detail(Detail::Full)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_with_data_only_detail() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi).build()?;
        let datakey = DataKey::default();
        let _response = SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
            .detail(Detail::DataOnly)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_with_no_data_detail() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi).build()?;
        let datakey = DataKey::default();
        let _response = SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
            .detail(Detail::NoData)
            .send()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn send_request_with_series_keys_only_detail() -> Result<()> {
        let dataflow_identifier = DataflowIdentifierBuilder::new(DataflowId::Cpi).build()?;
        let datakey = DataKey::default();
        let _response = SdmxDataRequestBuilder::new(dataflow_identifier, datakey)
            .detail(Detail::SeriesKeysOnly)
            .send()
            .await?;

        Ok(())
    }
}
