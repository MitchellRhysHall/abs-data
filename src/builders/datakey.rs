use crate::{
    error_code::{ErrorCode, Result},
    models::typed::enums::{Frequency, Measure, Region},
};
use core::fmt;
use std::fmt::{Display, Formatter};

pub struct DataKey {
    measure: Vec<Measure>,
    region: Vec<Region>,
    frequency: Vec<Frequency>,
    as_str: Box<str>,
}

impl DataKey {
    fn format_dimension<T: Display>(items: &[T]) -> String {
        items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join("+")
    }
}

impl Display for DataKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str)
    }
}

impl AsRef<str> for DataKey {
    fn as_ref(&self) -> &str {
        &self.as_str
    }
}

pub struct DataKeyBuilder {
    measure: Vec<Measure>,
    region: Vec<Region>,
    frequency: Vec<Frequency>,
    max_length: usize,
}

impl DataKeyBuilder {
    pub fn new() -> Self {
        Self {
            measure: Vec::new(),
            region: Vec::new(),
            frequency: Vec::new(),
            max_length: 260,
        }
    }

    pub fn max_length(&self) -> usize {
        self.max_length
    }

    pub fn add_measure(mut self, measure: Measure) -> Self {
        self.measure.push(measure);
        self
    }

    pub fn add_region(mut self, region: Region) -> Self {
        self.region.push(region);
        self
    }

    pub fn add_frequency(mut self, frequency: Frequency) -> Self {
        self.frequency.push(frequency);
        self
    }

    pub fn build(self) -> Result<DataKey> {
        let measure_str = DataKey::format_dimension(&self.measure);
        let region_str = DataKey::format_dimension(&self.region);
        let frequency_str = DataKey::format_dimension(&self.frequency);

        let dimensions = vec![measure_str, region_str, frequency_str].join(".");

        let key = DataKey {
            measure: self.measure,
            region: self.region,
            frequency: self.frequency,
            as_str: dimensions.into(),
        };

        if format!("{}", key).len() > self.max_length {
            return Err(ErrorCode::DataKeyLengthExceeded(self.max_length));
        }

        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_measure() {
        let data_key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1..");
    }

    #[test]
    fn test_multiple_measures() {
        let data_key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .add_measure(Measure::M2)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1+M2..");
    }

    #[test]
    fn test_single_measure_single_region() {
        let data_key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .add_region(Region::Aus)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1.AUS.");
    }

    #[test]
    fn test_single_measure_multiple_region() {
        let data_key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .add_region(Region::Aus)
            .add_region(Region::Usa)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1.AUS+USA.");
    }

    #[test]
    fn test_single_measure_single_frequency() {
        let data_key = DataKeyBuilder::new()
            .add_measure(Measure::M1)
            .add_frequency(Frequency::Monthly)
            .build()
            .unwrap();
        assert_eq!(format!("{}", data_key), "M1..M");
    }

    #[test]
    fn test_max_length_exceeded() {
        let mut datakey = DataKeyBuilder::new();

        for _ in 0..=datakey.max_length() {
            datakey = datakey.add_frequency(Frequency::Quarterly);
        }

        assert!(datakey.build().is_err())
    }
}
