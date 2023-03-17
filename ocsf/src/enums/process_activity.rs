#[derive(Debug)]
pub enum ProcessActivity {
    /// 1 - Launch
    Launch,
    /// 2 - Terminate
    Terminate,
    /// 3 - Open
    Open,
    /// 4 - Inject
    Inject,
    /// 5 - Set User ID
    SetUserID,
}
impl From<u8> for ProcessActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => ProcessActivity::Launch,
            2 => ProcessActivity::Terminate,
            3 => ProcessActivity::Open,
            4 => ProcessActivity::Inject,
            5 => ProcessActivity::SetUserID,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for ProcessActivity {
    fn into(self) -> u8 {
        match &self {
            ProcessActivity::Launch => 1,
            ProcessActivity::Terminate => 2,
            ProcessActivity::Open => 3,
            ProcessActivity::Inject => 4,
            ProcessActivity::SetUserID => 5,
        }
    }
}
