use std::fmt::{self, Display, Formatter};

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
