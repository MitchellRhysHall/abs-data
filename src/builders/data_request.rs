use log::info;
use url::Url;

use crate::{
    error_code::{ErrorCode, Result},
    models::typed::enums::{DateGranularity, Detail, DimensionAtObservation},
};

use super::{client::SdmxClient, request::SdmxRequest};

pub struct SdmxDataRequestBuilder<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    path: &'a str,
    dataflow_id: &'a str,
    data_key: &'a str,
    start_period: Option<DateGranularity<'a>>,
    end_period: Option<DateGranularity<'a>>,
    detail: Option<Detail>,
    dimension_at_observation: Option<DimensionAtObservation<'a>>,
    key: Option<&'a str>,
}

impl<'a> SdmxDataRequestBuilder<'a> {
    pub fn new(
        client: &'a SdmxClient,
        base_url: &'a str,
        path: &'a str,
        dataflow_id: &'a str,
        data_key: &'a str,
        key: Option<&'a str>,
    ) -> Self {
        Self {
            client,
            base_url,
            path,
            dataflow_id,
            data_key,
            start_period: None,
            end_period: None,
            detail: None,
            dimension_at_observation: None,
            key,
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

    fn build_url(&self) -> Result<Url> {
        let mut url = Url::parse(self.base_url)?;

        url.path_segments_mut()
            .map_err(|_| ErrorCode::UrlCannotBeABase)?
            .extend(&[self.path, self.dataflow_id, self.data_key]);

        let mut query_string = Vec::new();

        if let Some(start_period) = &self.start_period {
            query_string.push(format!("start_period={}", start_period.to_string()));
        }
        if let Some(end_period) = &self.end_period {
            query_string.push(format!("end_period={}", end_period.to_string()));
        }
        if let Some(detail) = &self.detail {
            query_string.push(format!("detail={}", detail.to_string()));
        }
        if let Some(dimension_at_observation) = &self.dimension_at_observation {
            query_string.push(format!(
                "dimensionAtObservation={}",
                dimension_at_observation.to_string()
            ));
        }

        if !query_string.is_empty() {
            let query_string = query_string.join("&");
            url.set_query(Some(&query_string));
        }

        info!("{}", url);

        Ok(url)
    }

    pub fn build(&self) -> SdmxRequest {
        let url = self
            .build_url()
            .expect("Failed to build the URL; this should never happen");
        SdmxRequest::new(self.client, url, self.key)
    }
}
