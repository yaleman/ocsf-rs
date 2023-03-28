//! OCSF Category data

//!

//! Initial working list of categories (work in progress).

// This file was automatically generated by ocsf-codegen at 2023-03-28T12:22:45+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/f3541d74751677f4bb36aff1c977d9dc0b9ef4d6>

/// Initial working list of categories (work in progress).
pub enum Category {
    /// Audit events relate to the supervision of the system's access control model. Examples of such events are the success or failure of authentication, granting of authority, password change, entity change, privileged use, system state change, and resource access.
    ///
    /// `uid=3000`
    Audit,
    /// Findings events report findings, detections, and possible resolutions of malware, anomalies, or other actions performed by security products.
    ///
    /// `uid=2000`
    Findings,
    /// Configuration and Inventory Activity events.
    ///
    /// `uid=5000`
    Inventory,
    /// Network Activity events.
    ///
    /// `uid=4000`
    Network,
    /// System Activity events.
    ///
    /// `uid=1000`
    System,
}

impl From<Category> for u8 {
    fn from(input: Category) -> u8 {
        match input {
            Category::Audit => 3,
            Category::Findings => 2,
            Category::Inventory => 5,
            Category::Network => 4,
            Category::System => 1,
        }
    }
}

impl TryFrom<u8> for Category {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            3 => Category::Audit,
            2 => Category::Findings,
            5 => Category::Inventory,
            4 => Category::Network,
            1 => Category::System,
            _ => return Err(format!("Invalid value specified: {input}")),
        };
        Ok(res)
    }
}
