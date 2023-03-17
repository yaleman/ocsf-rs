#[derive(Debug)]
pub enum Defaults {
    /// 0 - Unknown
    Unknown,
    /// 99 - Other
    Other,
}
impl From<u8> for Defaults {
    fn from(input: u8) -> Self {
        match input {
            0 => Defaults::Unknown,
            99 => Defaults::Other,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for Defaults {
    fn into(self) -> u8 {
        match &self {
            Defaults::Unknown => 0,
            Defaults::Other => 99,
        }
    }
}
