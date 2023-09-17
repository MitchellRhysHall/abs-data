use std::fmt::{Display, Formatter};

use reqwest::header::{ACCEPT, USER_AGENT};

use crate::{
    error_code::{ErrorCode, Result},
    models::SdmxResponse,
};

const BASE_URL: &str = "https://api.data.abs.gov.au";
const USER: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
const FORMAT: &str = "application/json";
const KEY: &str = "x-api-key";

pub struct SdmxClient {
    inner: reqwest::Client,
}

impl SdmxClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    pub fn get(&self) -> SdmxRequestBuilder {
        SdmxRequestBuilder::new(&self)
    }
}

pub struct SdmxRequestBuilder<'a> {
    client: &'a SdmxClient,
    key: Option<&'a str>,
}

impl<'a> SdmxRequestBuilder<'a> {
    pub fn new(client: &'a SdmxClient) -> Self {
        SdmxRequestBuilder { client, key: None }
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn data(&self, dataflow_id: &'a str, data_key: &'a str) -> SdmxDataRequestBuilder<'a> {
        SdmxDataRequestBuilder::new(
            self.client,
            BASE_URL,
            "data",
            dataflow_id,
            data_key,
            self.key,
        )
    }
}

pub struct SdmxRequest<'a> {
    client: &'a SdmxClient,
    url: Box<str>,
    key: Option<&'a str>,
}

impl<'a> SdmxRequest<'a> {
    pub fn new(client: &'a SdmxClient, url: Box<str>, key: Option<&'a str>) -> Self {
        Self { client, url, key }
    }

    pub async fn send(&self) -> Result<SdmxResponse> {
        let mut request = self
            .client
            .inner
            .get(&*self.url)
            .header(USER_AGENT, USER)
            .header(ACCEPT, FORMAT);

        if let Some(key) = &self.key {
            request = request.header(KEY, *key);
        }
        let response = request.send().await?;

        let status = response.status();

        if !status.is_success() {
            return Err(ErrorCode::Http(status));
        }

        let data = response.json::<SdmxResponse>().await?;
        Ok(data)
    }
}

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

    pub fn build(&self) -> SdmxRequest {
        let base_url = [self.base_url, self.path, self.dataflow_id, self.data_key].join("/");

        let mut query_params = Vec::new();

        if let Some(start_period) = &self.start_period {
            query_params.push(format!("start_period={}", start_period));
        }
        if let Some(end_period) = &self.end_period {
            query_params.push(format!("end_period={}", end_period));
        }
        if let Some(detail) = &self.detail {
            query_params.push(format!("detail={}", detail));
        }
        if let Some(dimension_at_observation) = &self.dimension_at_observation {
            query_params.push(format!(
                "dimensionAtObservation={}",
                dimension_at_observation
            ));
        }

        let full_url = if query_params.is_empty() {
            base_url
        } else {
            format!("{}?{}", base_url, query_params.join("&"))
        };

        SdmxRequest::new(self.client, full_url.into(), self.key)
    }
}

pub enum DateGranularity<'a> {
    Year(u16),
    YearSemester(u16, &'a str),
    YearQuarter(u16, &'a str),
    YearMonth(u16, u8),
}

impl<'a> Display for DateGranularity<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            DateGranularity::Year(year) => write!(f, "{:04}", year),
            DateGranularity::YearSemester(year, semester) => write!(f, "{:04}-{}", year, semester),
            DateGranularity::YearQuarter(year, quarter) => write!(f, "{:04}-{}", year, quarter),
            DateGranularity::YearMonth(year, month) => write!(f, "{:04}-{:02}", year, month),
        }
    }
}

pub enum Detail {
    Full,
    DataOnly,
    SeriesKeysOnly,
    NoData,
}

impl<'a> Display for Detail {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Detail::Full => write!(f, "full"),
            Detail::DataOnly => write!(f, "dataonly"),
            Detail::SeriesKeysOnly => write!(f, "serieskeysonly"),
            Detail::NoData => write!(f, "nodata"),
        }
    }
}

pub enum DimensionAtObservation<'a> {
    TimePeriod,
    All,
    Id(&'a str),
}

impl<'a> Display for DimensionAtObservation<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            DimensionAtObservation::TimePeriod => write!(f, "TIME_PERIOD"),
            DimensionAtObservation::All => write!(f, "AllDimensions"),
            DimensionAtObservation::Id(id) => write!(f, "{}", id),
        }
    }
}
