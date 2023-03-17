#[derive(Debug)]
pub enum KernelActivity {
    /// 1 - Create
    Create,
    /// 2 - Read
    Read,
    /// 3 - Delete
    Delete,
    /// 4 - Invoke
    Invoke,
}
impl From<u8> for KernelActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => KernelActivity::Create,
            2 => KernelActivity::Read,
            3 => KernelActivity::Delete,
            4 => KernelActivity::Invoke,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for KernelActivity {
    fn into(self) -> u8 {
        match &self {
            KernelActivity::Create => 1,
            KernelActivity::Read => 2,
            KernelActivity::Delete => 3,
            KernelActivity::Invoke => 4,
        }
    }
}
