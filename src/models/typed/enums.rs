use core::fmt;
use std::fmt::{Display, Formatter};

pub enum MetaDetail {
    All,
    Reference,
    ReferencePartial,
    AllComplete,
    ReferenceComplete,
    Full,
}

impl Display for MetaDetail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Debug, Clone)]
pub enum Measure {
    M1,
    M2,
}

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Measure::M1 => write!(f, "M1"),
            Measure::M2 => write!(f, "M2"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Region {
    Aus,
    Usa,
    Eu,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Region::Aus => write!(f, "AUS"),
            Region::Usa => write!(f, "USA"),
            Region::Eu => write!(f, "EU"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Frequency {
    Quarterly,
    Monthly,
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Frequency::Quarterly => write!(f, "Q"),
            Frequency::Monthly => write!(f, "M"),
        }
    }
}
