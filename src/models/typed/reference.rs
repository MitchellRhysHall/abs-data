use std::fmt::{self, Display, Formatter};

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
