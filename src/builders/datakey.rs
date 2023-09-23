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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_measure() {
        let data_key = DataKeyBuilder::new().measure(Measure::M1).build().unwrap();
        assert_eq!(format!("{}", data_key), "M1..");
    }

    #[test]
    fn test_multiple_measures() {
        let data_key = DataKeyBuilder::new()
            .measure(Measure::M1)
            .measure(Measure::M2)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1+M2..");
    }

    #[test]
    fn test_single_measure_single_region() {
        let data_key = DataKeyBuilder::new()
            .measure(Measure::M1)
            .region(Region::Aus)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1.AUS.");
    }

    #[test]
    fn test_single_measure_multiple_region() {
        let data_key = DataKeyBuilder::new()
            .measure(Measure::M1)
            .region(Region::Aus)
            .region(Region::Usa)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1.AUS+USA.");
    }

    #[test]
    fn test_single_measure_single_frequency() {
        let data_key = DataKeyBuilder::new()
            .measure(Measure::M1)
            .frequency(Frequency::Monthly)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1..M");
    }

    #[test]
    fn test_max_length_exceeded() {
        let mut datakey = DataKeyBuilder::new();

        for _ in 0..=datakey.max_length() {
            datakey = datakey.frequency(Frequency::Quarterly);
        }

        assert!(datakey.build().is_err())
    }
}
