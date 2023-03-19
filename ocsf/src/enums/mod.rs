pub enum RegistryKeyActivity {
    Export,
    Other,
    Unknown,
    Rename,
    Restore,
    Create,
    Modify,
    Delete,
    Import,
    Read,
    SetSecurity,
}

impl From<RegistryKeyActivity> for u8 {
    fn from(input: RegistryKeyActivity) -> u8 {
        match input {
            RegistryKeyActivity::Export => 9,
            RegistryKeyActivity::Other => 99,
            RegistryKeyActivity::Unknown => 0,
            RegistryKeyActivity::Rename => 5,
            RegistryKeyActivity::Restore => 7,
            RegistryKeyActivity::Create => 1,
            RegistryKeyActivity::Modify => 3,
            RegistryKeyActivity::Delete => 4,
            RegistryKeyActivity::Import => 8,
            RegistryKeyActivity::Read => 2,
            RegistryKeyActivity::SetSecurity => 6,
        }
    }
}

impl TryFrom<u8> for RegistryKeyActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            9 => RegistryKeyActivity::Export,
            99 => RegistryKeyActivity::Other,
            0 => RegistryKeyActivity::Unknown,
            5 => RegistryKeyActivity::Rename,
            7 => RegistryKeyActivity::Restore,
            1 => RegistryKeyActivity::Create,
            3 => RegistryKeyActivity::Modify,
            4 => RegistryKeyActivity::Delete,
            8 => RegistryKeyActivity::Import,
            2 => RegistryKeyActivity::Read,
            6 => RegistryKeyActivity::SetSecurity,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

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

pub enum RegistryValueActivity {
    Unknown,
    Delete,
    Get,
    Set,
    Modify,
    Other,
}

impl From<RegistryValueActivity> for u8 {
    fn from(input: RegistryValueActivity) -> u8 {
        match input {
            RegistryValueActivity::Unknown => 0,
            RegistryValueActivity::Delete => 4,
            RegistryValueActivity::Get => 1,
            RegistryValueActivity::Set => 2,
            RegistryValueActivity::Modify => 3,
            RegistryValueActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for RegistryValueActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => RegistryValueActivity::Unknown,
            4 => RegistryValueActivity::Delete,
            1 => RegistryValueActivity::Get,
            2 => RegistryValueActivity::Set,
            3 => RegistryValueActivity::Modify,
            99 => RegistryValueActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

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

pub enum MemoryActivity {
    /// Data Execution Permission
    DisableDEP,
    Other,
    Unknown,
    /// Read (Example: <code>ReadProcessMemory</code>)
    Read,
    AllocatePage,
    DeletePage,
    BufferOverflow,
    /// Write (Example: <code>WriteProcessMemory</code>)
    Write,
    ModifyPage,
    /// Data Execution Permission
    EnableDEP,
}

impl From<MemoryActivity> for u8 {
    fn from(input: MemoryActivity) -> u8 {
        match input {
            MemoryActivity::DisableDEP => 5,
            MemoryActivity::Other => 99,
            MemoryActivity::Unknown => 0,
            MemoryActivity::Read => 7,
            MemoryActivity::AllocatePage => 1,
            MemoryActivity::DeletePage => 3,
            MemoryActivity::BufferOverflow => 4,
            MemoryActivity::Write => 8,
            MemoryActivity::ModifyPage => 2,
            MemoryActivity::EnableDEP => 6,
        }
    }
}

impl TryFrom<u8> for MemoryActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            5 => MemoryActivity::DisableDEP,
            99 => MemoryActivity::Other,
            0 => MemoryActivity::Unknown,
            7 => MemoryActivity::Read,
            1 => MemoryActivity::AllocatePage,
            3 => MemoryActivity::DeletePage,
            4 => MemoryActivity::BufferOverflow,
            8 => MemoryActivity::Write,
            2 => MemoryActivity::ModifyPage,
            6 => MemoryActivity::EnableDEP,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

pub enum NetworkActivity {
    /// Network traffic report.
    Traffic,
    Other,
    Unknown,
    /// A new network connection was opened.
    Open,
    /// The network connection was abnormally terminated or closed by a middle device like firewalls.
    Reset,
    /// The network connection failed. For example a connection timeout or no route to host.
    Fail,
    /// The network connection was closed.
    Close,
    /// The network connection was refused. For example an attempt to connect to a server port which is not open.
    Refuse,
}

impl From<NetworkActivity> for u8 {
    fn from(input: NetworkActivity) -> u8 {
        match input {
            NetworkActivity::Traffic => 6,
            NetworkActivity::Other => 99,
            NetworkActivity::Unknown => 0,
            NetworkActivity::Open => 1,
            NetworkActivity::Reset => 3,
            NetworkActivity::Fail => 4,
            NetworkActivity::Close => 2,
            NetworkActivity::Refuse => 5,
        }
    }
}

impl TryFrom<u8> for NetworkActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            6 => NetworkActivity::Traffic,
            99 => NetworkActivity::Other,
            0 => NetworkActivity::Unknown,
            1 => NetworkActivity::Open,
            3 => NetworkActivity::Reset,
            4 => NetworkActivity::Fail,
            2 => NetworkActivity::Close,
            5 => NetworkActivity::Refuse,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

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

pub enum FileActivity {
    Encrypt,
    Unknown,
    Decrypt,
    GetAttributes,
    Other,
    SetAttributes,
    GetSecurity,
    SetSecurity,
    Create,
    Unmount,
    Update,
    Mount,
    Delete,
    Read,
    Open,
    Rename,
}

impl From<FileActivity> for u8 {
    fn from(input: FileActivity) -> u8 {
        match input {
            FileActivity::Encrypt => 10,
            FileActivity::Unknown => 0,
            FileActivity::Decrypt => 11,
            FileActivity::GetAttributes => 8,
            FileActivity::Other => 99,
            FileActivity::SetAttributes => 6,
            FileActivity::GetSecurity => 9,
            FileActivity::SetSecurity => 7,
            FileActivity::Create => 1,
            FileActivity::Unmount => 13,
            FileActivity::Update => 3,
            FileActivity::Mount => 12,
            FileActivity::Delete => 4,
            FileActivity::Read => 2,
            FileActivity::Open => 14,
            FileActivity::Rename => 5,
        }
    }
}

impl TryFrom<u8> for FileActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            10 => FileActivity::Encrypt,
            0 => FileActivity::Unknown,
            11 => FileActivity::Decrypt,
            8 => FileActivity::GetAttributes,
            99 => FileActivity::Other,
            6 => FileActivity::SetAttributes,
            9 => FileActivity::GetSecurity,
            7 => FileActivity::SetSecurity,
            1 => FileActivity::Create,
            13 => FileActivity::Unmount,
            3 => FileActivity::Update,
            12 => FileActivity::Mount,
            4 => FileActivity::Delete,
            2 => FileActivity::Read,
            14 => FileActivity::Open,
            5 => FileActivity::Rename,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

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

pub enum ProcessActivity {
    SetUserID,
    Inject,
    Unknown,
    Launch,
    Terminate,
    Open,
    Other,
}

impl From<ProcessActivity> for u8 {
    fn from(input: ProcessActivity) -> u8 {
        match input {
            ProcessActivity::SetUserID => 5,
            ProcessActivity::Inject => 4,
            ProcessActivity::Unknown => 0,
            ProcessActivity::Launch => 1,
            ProcessActivity::Terminate => 2,
            ProcessActivity::Open => 3,
            ProcessActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for ProcessActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            5 => ProcessActivity::SetUserID,
            4 => ProcessActivity::Inject,
            0 => ProcessActivity::Unknown,
            1 => ProcessActivity::Launch,
            2 => ProcessActivity::Terminate,
            3 => ProcessActivity::Open,
            99 => ProcessActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

pub enum KernelActivity {
    Unknown,
    Invoke,
    Create,
    Read,
    Delete,
    Other,
}

impl From<KernelActivity> for u8 {
    fn from(input: KernelActivity) -> u8 {
        match input {
            KernelActivity::Unknown => 0,
            KernelActivity::Invoke => 4,
            KernelActivity::Create => 1,
            KernelActivity::Read => 2,
            KernelActivity::Delete => 3,
            KernelActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for KernelActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => KernelActivity::Unknown,
            4 => KernelActivity::Invoke,
            1 => KernelActivity::Create,
            2 => KernelActivity::Read,
            3 => KernelActivity::Delete,
            99 => KernelActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}
