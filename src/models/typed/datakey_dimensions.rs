use super::{frequency::Frequency, measure::Measure, region::Region};

pub struct DataKeyDimensions<'a> {
    pub measure: &'a [Measure<'a>],
    pub region: &'a [Region<'a>],
    pub frequency: &'a [Frequency<'a>],
}
