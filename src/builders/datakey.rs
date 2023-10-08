use crate::{
    config::Config,
    models::typed::{
        datakey::DataKey, datakey_dimensions::DataKeyDimensions, frequency::Frequency,
        measure::Measure, region::Region,
    },
};

use crate::result::Result;

pub struct DataKeyBuilder<'a> {
    measure: Vec<Measure<'a>>,
    region: Vec<Region<'a>>,
    frequency: Vec<Frequency<'a>>,
    max_length: usize,
}

impl<'a> DataKeyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            measure: Vec::new(),
            region: Vec::new(),
            frequency: Vec::new(),
            max_length: Config::DATA_KEY_MAX_LENGTH,
        }
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn measure(mut self, measure: Measure<'a>) -> Self {
        self.measure.push(measure);
        self
    }

    pub fn region(mut self, region: Region<'a>) -> Self {
        self.region.push(region);
        self
    }

    pub fn frequency(mut self, frequency: Frequency<'a>) -> Self {
        self.frequency.push(frequency);
        self
    }

    pub fn build(self) -> Result<DataKey> {
        let dimensions = DataKeyDimensions {
            measure: &self.measure,
            region: &self.region,
            frequency: &self.frequency,
        };

        let key = DataKey::try_from(dimensions)?;

        Ok(key)
    }
}
