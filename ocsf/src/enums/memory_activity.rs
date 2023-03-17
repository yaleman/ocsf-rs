#[derive(Debug)]
pub enum MemoryActivity {
    /// 1 - Allocate Page
    AllocatePage,
    /// 2 - Modify Page
    ModifyPage,
    /// 3 - Delete Page
    DeletePage,
    /// 4 - Buffer Overflow
    BufferOverflow,
    /// 5 - Disable DEP
    DisableDEP,
    /// 6 - Enable DEP
    EnableDEP,
    /// 7 - Read
    Read,
    /// 8 - Write
    Write,
}
impl From<u8> for MemoryActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => MemoryActivity::AllocatePage,
            2 => MemoryActivity::ModifyPage,
            3 => MemoryActivity::DeletePage,
            4 => MemoryActivity::BufferOverflow,
            5 => MemoryActivity::DisableDEP,
            6 => MemoryActivity::EnableDEP,
            7 => MemoryActivity::Read,
            8 => MemoryActivity::Write,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for MemoryActivity {
    fn into(self) -> u8 {
        match &self {
            MemoryActivity::AllocatePage => 1,
            MemoryActivity::ModifyPage => 2,
            MemoryActivity::DeletePage => 3,
            MemoryActivity::BufferOverflow => 4,
            MemoryActivity::DisableDEP => 5,
            MemoryActivity::EnableDEP => 6,
            MemoryActivity::Read => 7,
            MemoryActivity::Write => 8,
        }
    }
}
