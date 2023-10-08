pub struct Region<'a> {
    value: &'a str,
}

impl<'a> Region<'a> {
    pub const AUS: &str = "AUS";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for Region<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
