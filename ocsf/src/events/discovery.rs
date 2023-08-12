/// Device Inventory Info events report device inventory data.
///
/// Sourced from: `events/events/discovery/inventory_info.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct InventoryInfo {
    pub actor: Option<String>,
    /// The device that is being discovered by an inventory process.
    pub device: String,
}

impl InventoryInfo {
    pub const UID: u16 = 1;
}

/// User Inventory Info events report user inventory data. For example, this can be used to collect information about users by dumping Active Directory data. This event class is meant to be used in conjunction with the <code>person</code> profile to allow capturing extended information about the user.
///
/// Sourced from: `events/events/discovery/user_inventory.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserInventory {
    /// The actor describes the process that was the source of the inventory activity. In the case of user inventory data, that could be a particular procss or script that is run to scrape the user data. For example, it could be a powershell process that runs to pull data from the Azure AD graph API.
    pub actor: Option<String>,
    /// The user that is being discovered by an inventory process.
    pub user: String,
}

impl UserInventory {
    pub const UID: u16 = 3;
}

/// Device Config State events report device configuration data.
///
/// Sourced from: `events/events/discovery/config_state.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ConfigState {
    pub actor: Option<String>,
    pub cis_benchmark_result: Option<String>,
    /// The device that is being discovered by an inventory process.
    pub device: String,
}

impl ConfigState {
    pub const UID: u16 = 2;
}

/// The Discovery event is a generic event that defines a set of attributes available in Discovery category events. As a generic event, it could be used to log events that are not otherwise defined by the Discovery specific event classes.
///
/// Sourced from: `events/events/discovery/discovery.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Discovery {
    pub activity_id: Option<String>,
}

impl Discovery {
    pub const UID: u16 = 5000;
}

// This file was automatically generated by ocsf-codegen at 2023-08-11T23:13:26+00:00 branch: "fixing-build-updating-schema" link: <https://github.com/yaleman/ocsf-rs/commit/8eed3271e56a81cc15bffb5bdd8540651315c06b> OCSF Schema version: 1.1.0-dev