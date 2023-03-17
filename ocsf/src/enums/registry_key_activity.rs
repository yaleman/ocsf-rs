#[derive(Debug)]
pub enum RegistryKeyActivity {
    /// 1 - Create
    Create,
    /// 2 - Read
    Read,
    /// 3 - Modify
    Modify,
    /// 4 - Delete
    Delete,
    /// 5 - Rename
    Rename,
    /// 6 - Set Security
    SetSecurity,
    /// 7 - Restore
    Restore,
    /// 8 - Import
    Import,
    /// 9 - Export
    Export,
}
impl From<u8> for RegistryKeyActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => RegistryKeyActivity::Create,
            2 => RegistryKeyActivity::Read,
            3 => RegistryKeyActivity::Modify,
            4 => RegistryKeyActivity::Delete,
            5 => RegistryKeyActivity::Rename,
            6 => RegistryKeyActivity::SetSecurity,
            7 => RegistryKeyActivity::Restore,
            8 => RegistryKeyActivity::Import,
            9 => RegistryKeyActivity::Export,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for RegistryKeyActivity {
    fn into(self) -> u8 {
        match &self {
            RegistryKeyActivity::Create => 1,
            RegistryKeyActivity::Read => 2,
            RegistryKeyActivity::Modify => 3,
            RegistryKeyActivity::Delete => 4,
            RegistryKeyActivity::Rename => 5,
            RegistryKeyActivity::SetSecurity => 6,
            RegistryKeyActivity::Restore => 7,
            RegistryKeyActivity::Import => 8,
            RegistryKeyActivity::Export => 9,
        }
    }
}
