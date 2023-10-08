pub struct Measure<'a> {
    value: &'a str,
}

impl<'a> Measure<'a> {
    pub const M1: &str = "M1";
    pub const M2: &str = "M2";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for Measure<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
