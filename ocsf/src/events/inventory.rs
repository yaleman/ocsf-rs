/// Device Config State events report device inventory data.
///
/// Sourced from: `events/events/inventory/config_state.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConfigState {
    pub cis_benchmark_result: Option<String>,
}

impl ConfigState {
    pub const UID: u16 = 2;
}

/// Device Inventory Info events report device inventory data.
///
/// Sourced from: `events/events/inventory/inventory_info.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct InventoryInfo {
    pub device: Option<String>,
}

impl InventoryInfo {
    pub const UID: u16 = 1;
}

/// The Device Inventory/Configuration event is a generic event that defines a set of attributes available in the inventory/configuration events. As a generic event, it could be used to log events that are not otherwise defined by the Device Inventory/Configuration category.
///
/// Sourced from: `events/events/inventory/inventory.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Inventory {
    pub activity_id: Option<String>,
    pub device: String,
}

impl Inventory {
    pub const UID: u16 = 5000;
}

// This file was automatically generated by ocsf-codegen at 2023-03-28T12:51:15+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/a19241f10710e7d5f5a476bfa2484030090763e4>
