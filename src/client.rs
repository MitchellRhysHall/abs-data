use std::{
    cell::OnceCell,
    fmt::{Display, Formatter},
    sync::OnceLock,
};

use log::info;
use reqwest::header;
use url::Url;

use crate::{
    error_code::{ErrorCode, Result},
    models::SdmxResponse,
};

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub struct SdmxClient {
    inner: OnceCell<&'static reqwest::Client>,
}

impl SdmxClient {
    pub fn new() -> Self {
        let inner = CLIENT.get_or_init(|| reqwest::Client::new());
        Self {
            inner: inner.into(),
        }
    }

    pub fn get(&self) -> SdmxRequestBuilder {
        SdmxRequestBuilder::new(&self)
    }
}

pub struct SdmxRequestBuilder<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    key: Option<&'a str>,
}

impl<'a> SdmxRequestBuilder<'a> {
    pub fn new(client: &'a SdmxClient) -> Self {
        SdmxRequestBuilder {
            client,
            base_url: "https://api.data.abs.gov.au",
            key: None,
        }
    }

    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn data(&self, dataflow_id: &'a str, data_key: &'a str) -> SdmxDataRequestBuilder<'a> {
        SdmxDataRequestBuilder::new(
            self.client,
            self.base_url,
            "data",
            dataflow_id,
            data_key,
            self.key,
        )
    }

    pub fn meta(
        &self,
        structure_type: &'a StructureType,
        agency_id: &'a AgencyId,
    ) -> SdmxMetaRequestBuilder<'a> {
        SdmxMetaRequestBuilder::new(
            self.client,
            self.base_url,
            structure_type,
            agency_id,
            self.key,
        )
    }
}

pub struct SdmxRequest<'a> {
    client: &'a SdmxClient,
    url: Url,
    key: Option<&'a str>,
}

impl<'a> SdmxRequest<'a> {
    pub fn new(client: &'a SdmxClient, url: Url, key: Option<&'a str>) -> Self {
        Self { client, url, key }
    }

    pub async fn send<T>(&self) -> Result<SdmxResponse<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        info!("{:?}", self.url.as_ref());

        let mut request = self
            .client
            .inner
            .get()
            .expect("client not initialized")
            .get(self.url.as_ref())
            .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .header(header::ACCEPT, "application/vnd.sdmx.structure+json");

        if let Some(key) = &self.key {
            request = request.header("x-api-key", *key);
        }

        let response = request.send().await?;

        let status = response.status();

        if !status.is_success() {
            return Err(ErrorCode::Http(status));
        }

        let body_bytes = response.bytes().await?;

        if body_bytes.is_empty() {
            return Err(ErrorCode::HttpEmptyResponse);
        }

        info!("{}", std::str::from_utf8(&body_bytes.clone())?);

        let data: SdmxResponse<T> = serde_json::from_slice(&body_bytes)?;

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

    fn build_url(&self) -> Result<Url> {
        let mut url = Url::parse(self.base_url)?;

        url.path_segments_mut()
            .map_err(|_| ErrorCode::UrlCannotBeABase)?
            .extend(&[self.path, self.dataflow_id, self.data_key]);

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(start_period) = &self.start_period {
                query_pairs.append_pair("start_period", &start_period.to_string());
            }
            if let Some(end_period) = &self.end_period {
                query_pairs.append_pair("end_period", &end_period.to_string());
            }
            if let Some(detail) = &self.detail {
                query_pairs.append_pair("detail", &detail.to_string());
            }
            if let Some(dimension_at_observation) = &self.dimension_at_observation {
                query_pairs.append_pair(
                    "dimensionAtObservation",
                    &dimension_at_observation.to_string(),
                );
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

pub struct SdmxMetaRequestBuilder<'a> {
    client: &'a SdmxClient,
    base_url: &'a str,
    structure_type: &'a StructureType,
    agency_id: &'a AgencyId,
    stub: Option<Stub>,
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
            stub: None,
            structure_id: None,
            structure_version: None,
            references: None,
            key,
        }
    }

    pub fn stub(mut self, stub: Stub) -> Self {
        self.stub = Some(stub);
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

            if let Some(stub) = &self.stub {
                query_pairs.append_pair("detail", &stub.to_string());
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

pub enum Stub {
    All,
    Reference,
    ReferencePartial,
    AllComplete,
    ReferenceComplete,
    Full,
}

impl Display for Stub {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "allstubs"),
            Self::Reference => write!(f, "referencestubs"),
            Self::ReferencePartial => write!(f, "referencepartial"),
            Self::AllComplete => write!(f, "allcompletestubs"),
            Self::ReferenceComplete => write!(f, "referencecompletestubs"),
            Self::Full => write!(f, "full"),
        }
    }
}

pub enum Reference<'a> {
    None,
    Parents,
    ParentsAndSimplings,
    Children,
    Descendants,
    All,
    Specific(&'a str),
}

impl<'a> Display for Reference<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Parents => write!(f, "parents"),
            Self::ParentsAndSimplings => write!(f, "parentsandsiblings"),
            Self::Children => write!(f, "children"),
            Self::Descendants => write!(f, "descendants"),
            Self::All => write!(f, "all"),
            Self::Specific(structure_type) => write!(f, "{}", structure_type),
        }
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
            Self::Year(year) => write!(f, "{:04}", year),
            Self::YearSemester(year, semester) => write!(f, "{:04}-{}", year, semester),
            Self::YearQuarter(year, quarter) => write!(f, "{:04}-{}", year, quarter),
            Self::YearMonth(year, month) => write!(f, "{:04}-{:02}", year, month),
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
            Self::Full => write!(f, "full"),
            Self::DataOnly => write!(f, "dataonly"),
            Self::SeriesKeysOnly => write!(f, "serieskeysonly"),
            Self::NoData => write!(f, "nodata"),
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
            Self::TimePeriod => write!(f, "TIME_PERIOD"),
            Self::All => write!(f, "AllDimensions"),
            Self::Id(id) => write!(f, "{}", id),
        }
    }
}

pub enum AgencyId {
    ABS,
}

impl Display for AgencyId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            AgencyId::ABS => write!(f, "ABS"),
        }
    }
}

pub enum StructureType {
    DataFlow,
    DataStructure,
    CodeList,
    ConceptScheme,
    CategoryScheme,
    ContentConstraint,
    ActualConstraint,
    AgencyScheme,
    Categorisation,
    HierarchicalCodelist,
}

impl Display for StructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataFlow => write!(f, "dataflow"),
            Self::DataStructure => write!(f, "datastructure"),
            Self::CodeList => write!(f, "codelist"),
            Self::ConceptScheme => write!(f, "conceptscheme"),
            Self::CategoryScheme => write!(f, "categoryscheme"),
            Self::ContentConstraint => write!(f, "contentconstraint"),
            Self::ActualConstraint => write!(f, "actualconstraint"),
            Self::AgencyScheme => write!(f, "agencyscheme"),
            Self::Categorisation => write!(f, "categorisation"),
            Self::HierarchicalCodelist => write!(f, "hierarchicalcodelist"),
        }
    }
}
