use std::fmt;

/// Represents a dimension within a data key.
///
/// The enum variants capture the value at a specified position (index) within the data key.
/// The first `u8` value in each variant denotes the position (index) of the value in the data key,
/// while the second value represents the actual value at that position.
///
/// Different variants hold values of different sizes to optimize memory usage, given that there are
/// thousands of const enums of this type defined.
pub enum DataKeyDimension {
    /// Holds an 8-bit dimension.
    U8(u8, u8),
    /// Holds a 16-bit dimension.
    U16(u8, u16),
    /// Holds a 32-bit dimension.
    U32(u8, u32),
    /// Holds a 64-bit dimension.
    U64(u8, u64),
    /// Holds a string slice dimension.
    Str(u8, &'static str),
}

impl fmt::Display for DataKeyDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::U8(_, v) => write!(f, "{}", v),
            Self::U16(_, v) => write!(f, "{}", v),
            Self::U32(_, v) => write!(f, "{}", v),
            Self::U64(_, v) => write!(f, "{}", v),
            Self::Str(_, v) => write!(f, "{}", v),
        }
    }
}
