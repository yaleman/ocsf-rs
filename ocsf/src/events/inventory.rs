/// Device Inventory Info events report device inventory data.
///
/// Sourced from: `events/inventory/inventory_info.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct InventoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl InventoryInfo {
    pub const UID: u16 = 1;
    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self {
            device: Some(device),
        }
    }

    pub fn new() -> Self {
        Self { device: None }
    }
}

/// Device Config State events report device inventory data.
///
/// Sourced from: `events/inventory/config_state.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ConfigState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_benchmark_result: Option<String>,
}

impl ConfigState {
    pub const UID: u16 = 2;
    /// Set the value of cis_benchmark_result
    pub fn with_cis_benchmark_result(self, cis_benchmark_result: String) -> Self {
        Self {
            cis_benchmark_result: Some(cis_benchmark_result),
        }
    }

    pub fn new() -> Self {
        Self {
            cis_benchmark_result: None,
        }
    }
}

/// The Device Inventory/Configuration event is a generic event that defines a set of attributes available in the inventory/configuration events. As a generic event, it could be used to log events that are not otherwise defined by the Device Inventory/Configuration category.
///
/// Sourced from: `events/inventory/inventory.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Inventory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub device: String,
}

impl Inventory {
    pub const UID: u16 = 5000;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn new(device: String) -> Self {
        Self {
            activity_id: None,
            device,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-03-29T12:27:22+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/60c657dc0c6103bdda104ca3be2ccbb778bd1c1a>
