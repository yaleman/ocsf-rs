enum RegistryKeyActivity {
    Other,
    Read,
    Restore,
    Rename,
    Modify,
    Delete,
    Unknown,
    Create,
    Import,
    Export,
    SetSecurity,
}

impl From<RegistryKeyActivity> for u8 {
    fn from(input: RegistryKeyActivity) -> u8 {
        match input {
            RegistryKeyActivity::Other => 99,
            RegistryKeyActivity::Read => 2,
            RegistryKeyActivity::Restore => 7,
            RegistryKeyActivity::Rename => 5,
            RegistryKeyActivity::Modify => 3,
            RegistryKeyActivity::Delete => 4,
            RegistryKeyActivity::Unknown => 0,
            RegistryKeyActivity::Create => 1,
            RegistryKeyActivity::Import => 8,
            RegistryKeyActivity::Export => 9,
            RegistryKeyActivity::SetSecurity => 6,
        }
    }
}

impl TryFrom<u8> for RegistryKeyActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => RegistryKeyActivity::Other,
            2 => RegistryKeyActivity::Read,
            7 => RegistryKeyActivity::Restore,
            5 => RegistryKeyActivity::Rename,
            3 => RegistryKeyActivity::Modify,
            4 => RegistryKeyActivity::Delete,
            0 => RegistryKeyActivity::Unknown,
            1 => RegistryKeyActivity::Create,
            8 => RegistryKeyActivity::Import,
            9 => RegistryKeyActivity::Export,
            6 => RegistryKeyActivity::SetSecurity,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum Defaults {
    Other,
    Unknown,
}

impl From<Defaults> for u8 {
    fn from(input: Defaults) -> u8 {
        match input {
            Defaults::Other => 99,
            Defaults::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for Defaults {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => Defaults::Other,
            0 => Defaults::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum RegistryValueActivity {
    Modify,
    Delete,
    Other,
    Get,
    Set,
    Unknown,
}

impl From<RegistryValueActivity> for u8 {
    fn from(input: RegistryValueActivity) -> u8 {
        match input {
            RegistryValueActivity::Modify => 3,
            RegistryValueActivity::Delete => 4,
            RegistryValueActivity::Other => 99,
            RegistryValueActivity::Get => 1,
            RegistryValueActivity::Set => 2,
            RegistryValueActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for RegistryValueActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            3 => RegistryValueActivity::Modify,
            4 => RegistryValueActivity::Delete,
            99 => RegistryValueActivity::Other,
            1 => RegistryValueActivity::Get,
            2 => RegistryValueActivity::Set,
            0 => RegistryValueActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum KernelExtensionActivity {
    Other,
    Load,
    Unload,
    Unknown,
}

impl From<KernelExtensionActivity> for u8 {
    fn from(input: KernelExtensionActivity) -> u8 {
        match input {
            KernelExtensionActivity::Other => 99,
            KernelExtensionActivity::Load => 1,
            KernelExtensionActivity::Unload => 2,
            KernelExtensionActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for KernelExtensionActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => KernelExtensionActivity::Other,
            1 => KernelExtensionActivity::Load,
            2 => KernelExtensionActivity::Unload,
            0 => KernelExtensionActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum DnsActivity {
    Other,
    Response,
    Query,
    Unknown,
}

impl From<DnsActivity> for u8 {
    fn from(input: DnsActivity) -> u8 {
        match input {
            DnsActivity::Other => 99,
            DnsActivity::Response => 2,
            DnsActivity::Query => 1,
            DnsActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for DnsActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => DnsActivity::Other,
            2 => DnsActivity::Response,
            1 => DnsActivity::Query,
            0 => DnsActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum MemoryActivity {
    Other,
    ModifyPage,
    DisableDEP,
    Read,
    DeletePage,
    BufferOverflow,
    Unknown,
    AllocatePage,
    EnableDEP,
    Write,
}

impl From<MemoryActivity> for u8 {
    fn from(input: MemoryActivity) -> u8 {
        match input {
            MemoryActivity::Other => 99,
            MemoryActivity::ModifyPage => 2,
            MemoryActivity::DisableDEP => 5,
            MemoryActivity::Read => 7,
            MemoryActivity::DeletePage => 3,
            MemoryActivity::BufferOverflow => 4,
            MemoryActivity::Unknown => 0,
            MemoryActivity::AllocatePage => 1,
            MemoryActivity::EnableDEP => 6,
            MemoryActivity::Write => 8,
        }
    }
}

impl TryFrom<u8> for MemoryActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => MemoryActivity::Other,
            2 => MemoryActivity::ModifyPage,
            5 => MemoryActivity::DisableDEP,
            7 => MemoryActivity::Read,
            3 => MemoryActivity::DeletePage,
            4 => MemoryActivity::BufferOverflow,
            0 => MemoryActivity::Unknown,
            1 => MemoryActivity::AllocatePage,
            6 => MemoryActivity::EnableDEP,
            8 => MemoryActivity::Write,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum NetworkActivity {
    Other,
    Close,
    Refuse,
    Unknown,
    Reset,
    Fail,
    Open,
    Traffic,
}

impl From<NetworkActivity> for u8 {
    fn from(input: NetworkActivity) -> u8 {
        match input {
            NetworkActivity::Other => 99,
            NetworkActivity::Close => 2,
            NetworkActivity::Refuse => 5,
            NetworkActivity::Unknown => 0,
            NetworkActivity::Reset => 3,
            NetworkActivity::Fail => 4,
            NetworkActivity::Open => 1,
            NetworkActivity::Traffic => 6,
        }
    }
}

impl TryFrom<u8> for NetworkActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => NetworkActivity::Other,
            2 => NetworkActivity::Close,
            5 => NetworkActivity::Refuse,
            0 => NetworkActivity::Unknown,
            3 => NetworkActivity::Reset,
            4 => NetworkActivity::Fail,
            1 => NetworkActivity::Open,
            6 => NetworkActivity::Traffic,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum Authorization {
    Other,
    AssignGroups,
    AssignPrivileges,
    Unknown,
}

impl From<Authorization> for u8 {
    fn from(input: Authorization) -> u8 {
        match input {
            Authorization::Other => 99,
            Authorization::AssignGroups => 2,
            Authorization::AssignPrivileges => 1,
            Authorization::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for Authorization {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => Authorization::Other,
            2 => Authorization::AssignGroups,
            1 => Authorization::AssignPrivileges,
            0 => Authorization::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum FileActivity {
    Encrypt,
    Read,
    Decrypt,
    Unmount,
    Open,
    Update,
    Other,
    Rename,
    SetSecurity,
    Unknown,
    Delete,
    Mount,
    Create,
    SetAttributes,
    GetAttributes,
    GetSecurity,
}

impl From<FileActivity> for u8 {
    fn from(input: FileActivity) -> u8 {
        match input {
            FileActivity::Encrypt => 10,
            FileActivity::Read => 2,
            FileActivity::Decrypt => 11,
            FileActivity::Unmount => 13,
            FileActivity::Open => 14,
            FileActivity::Update => 3,
            FileActivity::Other => 99,
            FileActivity::Rename => 5,
            FileActivity::SetSecurity => 7,
            FileActivity::Unknown => 0,
            FileActivity::Delete => 4,
            FileActivity::Mount => 12,
            FileActivity::Create => 1,
            FileActivity::SetAttributes => 6,
            FileActivity::GetAttributes => 8,
            FileActivity::GetSecurity => 9,
        }
    }
}

impl TryFrom<u8> for FileActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            10 => FileActivity::Encrypt,
            2 => FileActivity::Read,
            11 => FileActivity::Decrypt,
            13 => FileActivity::Unmount,
            14 => FileActivity::Open,
            3 => FileActivity::Update,
            99 => FileActivity::Other,
            5 => FileActivity::Rename,
            7 => FileActivity::SetSecurity,
            0 => FileActivity::Unknown,
            4 => FileActivity::Delete,
            12 => FileActivity::Mount,
            1 => FileActivity::Create,
            6 => FileActivity::SetAttributes,
            8 => FileActivity::GetAttributes,
            9 => FileActivity::GetSecurity,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum ModuleActivity {
    Other,
    Unload,
    Load,
    Unknown,
}

impl From<ModuleActivity> for u8 {
    fn from(input: ModuleActivity) -> u8 {
        match input {
            ModuleActivity::Other => 99,
            ModuleActivity::Unload => 2,
            ModuleActivity::Load => 1,
            ModuleActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for ModuleActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            99 => ModuleActivity::Other,
            2 => ModuleActivity::Unload,
            1 => ModuleActivity::Load,
            0 => ModuleActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum ProcessActivity {
    Open,
    Inject,
    Other,
    Terminate,
    Launch,
    SetUserID,
    Unknown,
}

impl From<ProcessActivity> for u8 {
    fn from(input: ProcessActivity) -> u8 {
        match input {
            ProcessActivity::Open => 3,
            ProcessActivity::Inject => 4,
            ProcessActivity::Other => 99,
            ProcessActivity::Terminate => 2,
            ProcessActivity::Launch => 1,
            ProcessActivity::SetUserID => 5,
            ProcessActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for ProcessActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            3 => ProcessActivity::Open,
            4 => ProcessActivity::Inject,
            99 => ProcessActivity::Other,
            2 => ProcessActivity::Terminate,
            1 => ProcessActivity::Launch,
            5 => ProcessActivity::SetUserID,
            0 => ProcessActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

enum KernelActivity {
    Delete,
    Invoke,
    Other,
    Read,
    Create,
    Unknown,
}

impl From<KernelActivity> for u8 {
    fn from(input: KernelActivity) -> u8 {
        match input {
            KernelActivity::Delete => 3,
            KernelActivity::Invoke => 4,
            KernelActivity::Other => 99,
            KernelActivity::Read => 2,
            KernelActivity::Create => 1,
            KernelActivity::Unknown => 0,
        }
    }
}

impl TryFrom<u8> for KernelActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            3 => KernelActivity::Delete,
            4 => KernelActivity::Invoke,
            99 => KernelActivity::Other,
            2 => KernelActivity::Read,
            1 => KernelActivity::Create,
            0 => KernelActivity::Unknown,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}
