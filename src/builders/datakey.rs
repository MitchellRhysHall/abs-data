use crate::{
    error_code::Result,
    models::typed::{
        datakey::DataKey, datakey_dimensions::DataKeyDimensions, frequency::Frequency,
        measure::Measure, region::Region,
    },
};

pub struct DataKeyBuilder {
    measure: Vec<Measure>,
    region: Vec<Region>,
    frequency: Vec<Frequency>,
    no_filter: bool,
    max_length: usize,
}

impl DataKeyBuilder {
    pub fn new() -> Self {
        Self {
            measure: Vec::new(),
            region: Vec::new(),
            frequency: Vec::new(),
            no_filter: false,
            max_length: 260,
        }
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn measure(mut self, measure: Measure) -> Self {
        self.measure.push(measure);
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region.push(region);
        self
    }

    pub fn frequency(mut self, frequency: Frequency) -> Self {
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
