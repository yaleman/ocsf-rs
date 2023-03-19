// This file was automatically generated by ocsf-codegen at 2023-03-19T13:22:12+00:00

pub enum RegistryKeyActivity {
    Unknown,
    Create,
    Read,
    Modify,
    Delete,
    Rename,
    SetSecurity,
    Restore,
    Import,
    Export,
    Other,
}

impl From<RegistryKeyActivity> for u8 {
    fn from(input: RegistryKeyActivity) -> u8 {
        match input {
            RegistryKeyActivity::Unknown => 0,
            RegistryKeyActivity::Create => 1,
            RegistryKeyActivity::Read => 2,
            RegistryKeyActivity::Modify => 3,
            RegistryKeyActivity::Delete => 4,
            RegistryKeyActivity::Rename => 5,
            RegistryKeyActivity::SetSecurity => 6,
            RegistryKeyActivity::Restore => 7,
            RegistryKeyActivity::Import => 8,
            RegistryKeyActivity::Export => 9,
            RegistryKeyActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for RegistryKeyActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => RegistryKeyActivity::Unknown,
            1 => RegistryKeyActivity::Create,
            2 => RegistryKeyActivity::Read,
            3 => RegistryKeyActivity::Modify,
            4 => RegistryKeyActivity::Delete,
            5 => RegistryKeyActivity::Rename,
            6 => RegistryKeyActivity::SetSecurity,
            7 => RegistryKeyActivity::Restore,
            8 => RegistryKeyActivity::Import,
            9 => RegistryKeyActivity::Export,
            99 => RegistryKeyActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

// kilroy was here registry_key.json

pub enum RegistryValueActivity {
    Unknown,
    Get,
    Set,
    Modify,
    Delete,
    Other,
}

impl From<RegistryValueActivity> for u8 {
    fn from(input: RegistryValueActivity) -> u8 {
        match input {
            RegistryValueActivity::Unknown => 0,
            RegistryValueActivity::Get => 1,
            RegistryValueActivity::Set => 2,
            RegistryValueActivity::Modify => 3,
            RegistryValueActivity::Delete => 4,
            RegistryValueActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for RegistryValueActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => RegistryValueActivity::Unknown,
            1 => RegistryValueActivity::Get,
            2 => RegistryValueActivity::Set,
            3 => RegistryValueActivity::Modify,
            4 => RegistryValueActivity::Delete,
            99 => RegistryValueActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

// kilroy was here registry_value.json

// kilroy was here resource.json
