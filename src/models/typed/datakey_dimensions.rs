use super::{frequency::Frequency, measure::Measure, region::Region};

pub struct DataKeyDimensions<'a> {
    pub measure: &'a [Measure],
    pub region: &'a [Region],
    pub frequency: &'a [Frequency],
}
