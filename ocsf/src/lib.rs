//! OCSF crate, does Open Cyber Security Framework things.

//!

//! <h1><span color="red"> THIS IS VERY VERY VERY EARLY ALPHA</span></h1>

//!

//! The base schema is available at <https://ocsf.io>.

// This file was automatically generated by ocsf-codegen at 2023-03-28T11:36:12+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/c035bcbabbea474b72d2dfc1b4a316ad45549a19>

/// This was the schema version that the code was generated from (1.0.0-rc.2).

pub static OCSF_SCHEMA_VERSION: &str = "1.0.0-rc.2";

pub mod profiles;

pub mod categories;

pub mod events;

pub mod dictionary;

pub mod objects;

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Defaults {
    Unknown,
    Other,
}

impl From<Defaults> for u8 {
    fn from(input: Defaults) -> u8 {
        match input {
            Defaults::Unknown => 0,
            Defaults::Other => 99,
        }
    }
}

impl TryFrom<u8> for Defaults {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => Defaults::Unknown,
            99 => Defaults::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
pub enum KernelExtensionActivity {
    Unknown,
    /// A driver/extension was loaded into the kernel
    Load,
    /// A driver/extension was unloaded (removed) from the kernel
    Unload,
    Other,
}

impl From<KernelExtensionActivity> for u8 {
    fn from(input: KernelExtensionActivity) -> u8 {
        match input {
            KernelExtensionActivity::Unknown => 0,
            KernelExtensionActivity::Load => 1,
            KernelExtensionActivity::Unload => 2,
            KernelExtensionActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for KernelExtensionActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => KernelExtensionActivity::Unknown,
            1 => KernelExtensionActivity::Load,
            2 => KernelExtensionActivity::Unload,
            99 => KernelExtensionActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DnsActivity {
    Unknown,
    /// The DNS query request.
    Query,
    /// The DNS query response.
    Response,
    Other,
}

impl From<DnsActivity> for u8 {
    fn from(input: DnsActivity) -> u8 {
        match input {
            DnsActivity::Unknown => 0,
            DnsActivity::Query => 1,
            DnsActivity::Response => 2,
            DnsActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for DnsActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => DnsActivity::Unknown,
            1 => DnsActivity::Query,
            2 => DnsActivity::Response,
            99 => DnsActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum MemoryActivity {
    Unknown,
    AllocatePage,
    ModifyPage,
    DeletePage,
    BufferOverflow,
    /// Data Execution Permission
    DisableDEP,
    /// Data Execution Permission
    EnableDEP,
    /// Read (Example: <code>ReadProcessMemory</code>)
    Read,
    /// Write (Example: <code>WriteProcessMemory</code>)
    Write,
    Other,
}

impl From<MemoryActivity> for u8 {
    fn from(input: MemoryActivity) -> u8 {
        match input {
            MemoryActivity::Unknown => 0,
            MemoryActivity::AllocatePage => 1,
            MemoryActivity::ModifyPage => 2,
            MemoryActivity::DeletePage => 3,
            MemoryActivity::BufferOverflow => 4,
            MemoryActivity::DisableDEP => 5,
            MemoryActivity::EnableDEP => 6,
            MemoryActivity::Read => 7,
            MemoryActivity::Write => 8,
            MemoryActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for MemoryActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => MemoryActivity::Unknown,
            1 => MemoryActivity::AllocatePage,
            2 => MemoryActivity::ModifyPage,
            3 => MemoryActivity::DeletePage,
            4 => MemoryActivity::BufferOverflow,
            5 => MemoryActivity::DisableDEP,
            6 => MemoryActivity::EnableDEP,
            7 => MemoryActivity::Read,
            8 => MemoryActivity::Write,
            99 => MemoryActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum NetworkActivity {
    Unknown,
    /// A new network connection was opened.
    Open,
    /// The network connection was closed.
    Close,
    /// The network connection was abnormally terminated or closed by a middle device like firewalls.
    Reset,
    /// The network connection failed. For example a connection timeout or no route to host.
    Fail,
    /// The network connection was refused. For example an attempt to connect to a server port which is not open.
    Refuse,
    /// Network traffic report.
    Traffic,
    Other,
}

impl From<NetworkActivity> for u8 {
    fn from(input: NetworkActivity) -> u8 {
        match input {
            NetworkActivity::Unknown => 0,
            NetworkActivity::Open => 1,
            NetworkActivity::Close => 2,
            NetworkActivity::Reset => 3,
            NetworkActivity::Fail => 4,
            NetworkActivity::Refuse => 5,
            NetworkActivity::Traffic => 6,
            NetworkActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for NetworkActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => NetworkActivity::Unknown,
            1 => NetworkActivity::Open,
            2 => NetworkActivity::Close,
            3 => NetworkActivity::Reset,
            4 => NetworkActivity::Fail,
            5 => NetworkActivity::Refuse,
            6 => NetworkActivity::Traffic,
            99 => NetworkActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Authorization {
    Unknown,
    /// Assign special privileges to a new logon.
    AssignPrivileges,
    /// Assign special groups to a new logon.
    AssignGroups,
    Other,
}

impl From<Authorization> for u8 {
    fn from(input: Authorization) -> u8 {
        match input {
            Authorization::Unknown => 0,
            Authorization::AssignPrivileges => 1,
            Authorization::AssignGroups => 2,
            Authorization::Other => 99,
        }
    }
}

impl TryFrom<u8> for Authorization {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => Authorization::Unknown,
            1 => Authorization::AssignPrivileges,
            2 => Authorization::AssignGroups,
            99 => Authorization::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileActivity {
    Unknown,
    Create,
    Read,
    Update,
    Delete,
    Rename,
    SetAttributes,
    SetSecurity,
    GetAttributes,
    GetSecurity,
    Encrypt,
    Decrypt,
    Mount,
    Unmount,
    Open,
    Other,
}

impl From<FileActivity> for u8 {
    fn from(input: FileActivity) -> u8 {
        match input {
            FileActivity::Unknown => 0,
            FileActivity::Create => 1,
            FileActivity::Read => 2,
            FileActivity::Update => 3,
            FileActivity::Delete => 4,
            FileActivity::Rename => 5,
            FileActivity::SetAttributes => 6,
            FileActivity::SetSecurity => 7,
            FileActivity::GetAttributes => 8,
            FileActivity::GetSecurity => 9,
            FileActivity::Encrypt => 10,
            FileActivity::Decrypt => 11,
            FileActivity::Mount => 12,
            FileActivity::Unmount => 13,
            FileActivity::Open => 14,
            FileActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for FileActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => FileActivity::Unknown,
            1 => FileActivity::Create,
            2 => FileActivity::Read,
            3 => FileActivity::Update,
            4 => FileActivity::Delete,
            5 => FileActivity::Rename,
            6 => FileActivity::SetAttributes,
            7 => FileActivity::SetSecurity,
            8 => FileActivity::GetAttributes,
            9 => FileActivity::GetSecurity,
            10 => FileActivity::Encrypt,
            11 => FileActivity::Decrypt,
            12 => FileActivity::Mount,
            13 => FileActivity::Unmount,
            14 => FileActivity::Open,
            99 => FileActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ModuleActivity {
    Unknown,
    Load,
    Unload,
    Other,
}

impl From<ModuleActivity> for u8 {
    fn from(input: ModuleActivity) -> u8 {
        match input {
            ModuleActivity::Unknown => 0,
            ModuleActivity::Load => 1,
            ModuleActivity::Unload => 2,
            ModuleActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for ModuleActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => ModuleActivity::Unknown,
            1 => ModuleActivity::Load,
            2 => ModuleActivity::Unload,
            99 => ModuleActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ProcessActivity {
    Unknown,
    Launch,
    Terminate,
    Open,
    Inject,
    SetUserID,
    Other,
}

impl From<ProcessActivity> for u8 {
    fn from(input: ProcessActivity) -> u8 {
        match input {
            ProcessActivity::Unknown => 0,
            ProcessActivity::Launch => 1,
            ProcessActivity::Terminate => 2,
            ProcessActivity::Open => 3,
            ProcessActivity::Inject => 4,
            ProcessActivity::SetUserID => 5,
            ProcessActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for ProcessActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => ProcessActivity::Unknown,
            1 => ProcessActivity::Launch,
            2 => ProcessActivity::Terminate,
            3 => ProcessActivity::Open,
            4 => ProcessActivity::Inject,
            5 => ProcessActivity::SetUserID,
            99 => ProcessActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum KernelActivity {
    Unknown,
    Create,
    Read,
    Delete,
    Invoke,
    Other,
}

impl From<KernelActivity> for u8 {
    fn from(input: KernelActivity) -> u8 {
        match input {
            KernelActivity::Unknown => 0,
            KernelActivity::Create => 1,
            KernelActivity::Read => 2,
            KernelActivity::Delete => 3,
            KernelActivity::Invoke => 4,
            KernelActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for KernelActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => KernelActivity::Unknown,
            1 => KernelActivity::Create,
            2 => KernelActivity::Read,
            3 => KernelActivity::Delete,
            4 => KernelActivity::Invoke,
            99 => KernelActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}
