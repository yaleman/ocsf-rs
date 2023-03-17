#[derive(Debug)]
pub enum RegistryValueActivity {
    /// 1 - Get
    Get,
    /// 2 - Set
    Set,
    /// 3 - Modify
    Modify,
    /// 4 - Delete
    Delete,
}
impl From<u8> for RegistryValueActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => RegistryValueActivity::Get,
            2 => RegistryValueActivity::Set,
            3 => RegistryValueActivity::Modify,
            4 => RegistryValueActivity::Delete,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for RegistryValueActivity {
    fn into(self) -> u8 {
        match &self {
            RegistryValueActivity::Get => 1,
            RegistryValueActivity::Set => 2,
            RegistryValueActivity::Modify => 3,
            RegistryValueActivity::Delete => 4,
        }
    }
}
