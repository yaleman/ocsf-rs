// This file was automatically generated by ocsf-codegen at 2023-03-19T13:22:12+00:00

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

// kilroy was here kernel.json

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

// kilroy was here memory.json

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

// kilroy was here kernel_extension.json

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

// kilroy was here module.json

// kilroy was here scheduled_job.json

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

// kilroy was here process.json

pub mod windows;

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

// kilroy was here filesystem.json

// kilroy was here system.json