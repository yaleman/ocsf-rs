/// Device Config State Change events report state changes that impact the security of the device.
///
/// Sourced from: `events/events/discovery/device_config_state_change.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DeviceConfigStateChange {
    pub actor: Option<String>,
    /// The device that is impacted by the state change.
    pub device: String,
    pub prev_security_level: Option<String>,
    pub prev_security_level_id: Option<String>,
    /// The previous security states of the device.
    pub prev_security_states: Option<String>,
    pub security_level: Option<String>,
    pub security_level_id: Option<String>,
    /// The current security states of the device.
    pub security_states: Option<String>,
}

impl DeviceConfigStateChange {
    pub const UID: u16 = 19;
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

/// Operating System Patch State reports the installation of an OS patch to a device and any associated knowledgebase articles.
///
/// Sourced from: `events/events/discovery/patch_state.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PatchState {
    pub device: String,
    pub kb_article_list: Option<String>,
}

impl PatchState {
    pub const UID: u16 = 4;
}

/// Device Config State events report device configuration data and CIS Benchmark results.
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

/// Device Inventory Info events report device inventory data that is either logged or proactively collected. For example, when collecting device information from a CMDB or running a network sweep of connected devices.
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

/// Discovery Result events report the results of a discovery request.
///
/// Sourced from: `events/events/discovery/discovery_result.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DiscoveryResult {
    pub activity_id: Option<String>,
}

impl DiscoveryResult {
    pub const UID: u16 = 5000;
}

/// User Inventory Info events report user inventory data that is either logged or proactively collected. For example, when collecting user information from Active Directory entries.
///
/// Sourced from: `events/events/discovery/user_inventory.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserInventory {
    /// The actor describes the process that was the source of the inventory activity. In the case of user inventory data, that could be a particular process or script that is run to scrape the user data. For example, it could be a powershell process that runs to pull data from the Azure AD graph API.
    pub actor: Option<String>,
    /// The user that is being discovered by an inventory process.
    pub user: String,
}

impl UserInventory {
    pub const UID: u16 = 3;
}

// This file was automatically generated by ocsf-codegen at 2024-02-26T03:11:12+00:00 branch: "main" link: <https://github.com/yaleman/ocsf-rs/commit/cee9b6fcdc93b8937747d894e9586cbc355c3490> OCSF Schema version: 1.1.0
