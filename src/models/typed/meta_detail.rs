use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum MetaDetail {
    AllStubs,
    Reference,
    ReferencePartial,
    AllComplete,
    ReferenceComplete,
    Full,
}

impl Display for MetaDetail {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::AllStubs => write!(f, "allstubs"),
            Self::Reference => write!(f, "referencestubs"),
            Self::ReferencePartial => write!(f, "referencepartial"),
            Self::AllComplete => write!(f, "allcompletestubs"),
            Self::ReferenceComplete => write!(f, "referencecompletestubs"),
            Self::Full => write!(f, "full"),
        }
    }
}
