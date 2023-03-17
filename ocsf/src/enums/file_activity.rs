#[derive(Debug)]
pub enum FileActivity {
    /// 1 - Create
    Create,
    /// 10 - Encrypt
    Encrypt,
    /// 11 - Decrypt
    Decrypt,
    /// 12 - Mount
    Mount,
    /// 13 - Unmount
    Unmount,
    /// 14 - Open
    Open,
    /// 2 - Read
    Read,
    /// 3 - Update
    Update,
    /// 4 - Delete
    Delete,
    /// 5 - Rename
    Rename,
    /// 6 - Set Attributes
    SetAttributes,
    /// 7 - Set Security
    SetSecurity,
    /// 8 - Get Attributes
    GetAttributes,
    /// 9 - Get Security
    GetSecurity,
}
impl From<u8> for FileActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => FileActivity::Create,
            10 => FileActivity::Encrypt,
            11 => FileActivity::Decrypt,
            12 => FileActivity::Mount,
            13 => FileActivity::Unmount,
            14 => FileActivity::Open,
            2 => FileActivity::Read,
            3 => FileActivity::Update,
            4 => FileActivity::Delete,
            5 => FileActivity::Rename,
            6 => FileActivity::SetAttributes,
            7 => FileActivity::SetSecurity,
            8 => FileActivity::GetAttributes,
            9 => FileActivity::GetSecurity,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for FileActivity {
    fn into(self) -> u8 {
        match &self {
            FileActivity::Create => 1,
            FileActivity::Encrypt => 10,
            FileActivity::Decrypt => 11,
            FileActivity::Mount => 12,
            FileActivity::Unmount => 13,
            FileActivity::Open => 14,
            FileActivity::Read => 2,
            FileActivity::Update => 3,
            FileActivity::Delete => 4,
            FileActivity::Rename => 5,
            FileActivity::SetAttributes => 6,
            FileActivity::SetSecurity => 7,
            FileActivity::GetAttributes => 8,
            FileActivity::GetSecurity => 9,
        }
    }
}
