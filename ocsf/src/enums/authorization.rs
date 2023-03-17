#[derive(Debug)]
pub enum Authorization {
    /// 1 - Assign Privileges
    AssignPrivileges,
    /// 2 - Assign Groups
    AssignGroups,
}
impl From<u8> for Authorization {
    fn from(input: u8) -> Self {
        match input {
            1 => Authorization::AssignPrivileges,
            2 => Authorization::AssignGroups,
            _ => panic!("Invalid value!"),
        }
    }
}

impl Into<u8> for Authorization {
    fn into(self) -> u8 {
        match &self {
            Authorization::AssignPrivileges => 1,
            Authorization::AssignGroups => 2,
        }
    }
}
