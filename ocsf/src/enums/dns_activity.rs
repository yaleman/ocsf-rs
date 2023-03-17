#[derive(Debug)]
pub enum DnsActivity {
    /// 1 - Query
    Query,
    /// 2 - Response
    Response,
}
impl From<u8> for DnsActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => DnsActivity::Query,
            2 => DnsActivity::Response,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for DnsActivity {
    fn into(self) -> u8 {
        match &self {
            DnsActivity::Query => 1,
            DnsActivity::Response => 2,
        }
    }
}
