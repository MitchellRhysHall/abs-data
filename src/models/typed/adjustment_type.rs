pub struct AdjustmentType<'a> {
    value: &'a str,
}

impl<'a> AdjustmentType<'a> {
    pub const ORIGINAL: &str = "10";
    pub const SEASONALLY_ADJUSTED: &str = "20";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for AdjustmentType<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
