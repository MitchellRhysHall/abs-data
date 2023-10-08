pub struct Frequency<'a> {
    value: &'a str,
}

impl<'a> Frequency<'a> {
    pub const QUARTERLY: &str = "Q";
    pub const MONTHLY: &str = "M";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for Frequency<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
