#[derive(Debug)]
pub enum ModuleActivity {
    /// 1 - Load
    Load,
    /// 2 - Unload
    Unload,
}
impl From<u8> for ModuleActivity {
    fn from(input: u8) -> Self {
        match input {
            1 => ModuleActivity::Load,
            2 => ModuleActivity::Unload,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for ModuleActivity {
    fn into(self) -> u8 {
        match &self {
            ModuleActivity::Load => 1,
            ModuleActivity::Unload => 2,
        }
    }
}
