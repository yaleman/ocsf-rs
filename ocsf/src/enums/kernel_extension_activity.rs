#[derive(Debug)]
pub enum KernelExtensionActivity {
    /// 1 - Load
    Load,
    /// 2 - Unload
    Unload,
}
impl From<u8> for KernelExtensionActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => KernelExtensionActivity::Load,
            2 => KernelExtensionActivity::Unload,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for KernelExtensionActivity {
    fn into(self) -> u8 {
        match &self {
            KernelExtensionActivity::Load => 1,
            KernelExtensionActivity::Unload => 2,
        }
    }
}
