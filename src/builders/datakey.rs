use std::fmt::Display;

use crate::models::typed::{datakey::DataKey, datakey_dimension::DataKeyDimension};

pub struct DataKeyBuilder {
    dimensions: [Box<str>; 5],
}

impl DataKeyBuilder {
    pub fn new() -> Self {
        Self {
            dimensions: ["".into(), "".into(), "".into(), "".into(), "".into()],
        }
    }

    pub fn add(mut self, dimension: DataKeyDimension) -> Self {
        match dimension {
            DataKeyDimension::U8(i, v) => self.dimensions[i as usize] = v.to_string().into(),
            DataKeyDimension::U16(i, v) => self.dimensions[i as usize] = v.to_string().into(),
            DataKeyDimension::U32(i, v) => self.dimensions[i as usize] = v.to_string().into(),
            DataKeyDimension::U64(i, v) => self.dimensions[i as usize] = v.to_string().into(),
            DataKeyDimension::Str(i, v) => self.dimensions[i as usize] = v.to_string().into(),
        }

        println!("{:?}", self.dimensions);
        self
    }

    pub fn build(self) -> DataKey {
        let str = self.dimensions.join(".");
        println!("{str}");
        let key = DataKey::parse(&str).expect("should always be valid");

        key
    }
}
