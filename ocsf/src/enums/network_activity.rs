#[derive(Debug)]
pub enum NetworkActivity {
    /// 1 - Open
    Open,
    /// 2 - Close
    Close,
    /// 3 - Reset
    Reset,
    /// 4 - Fail
    Fail,
    /// 5 - Refuse
    Refuse,
    /// 6 - Traffic
    Traffic,
}
impl From<u8> for NetworkActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => NetworkActivity::Open,
            2 => NetworkActivity::Close,
            3 => NetworkActivity::Reset,
            4 => NetworkActivity::Fail,
            5 => NetworkActivity::Refuse,
            6 => NetworkActivity::Traffic,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for NetworkActivity {
    fn into(self) -> u8 {
        match &self {
            NetworkActivity::Open => 1,
            NetworkActivity::Close => 2,
            NetworkActivity::Reset => 3,
            NetworkActivity::Fail => 4,
            NetworkActivity::Refuse => 5,
            NetworkActivity::Traffic => 6,
        }
    }
}
