pub struct Region<'a> {
    value: &'a str,
}

impl<'a> Region<'a> {
    pub const SYDNEY: &str = "1";
    pub const MELBOURNE: &str = "2";
    pub const BRISBANE: &str = "3";
    pub const ADELAIDE: &str = "4";
    pub const PERTH: &str = "5";
    pub const HOBART: &str = "6";
    pub const DARWIN: &str = "7";
    pub const CANBERRA: &str = "8";
    pub const WEIGHTED_AVG_CAPITAL_CITIES: &str = "50";

    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl<'a> AsRef<str> for Region<'a> {
    fn as_ref(&self) -> &'a str {
        self.value
    }
}
