/// Folder Query events report information about folders that are present on the system.
///
/// Sourced from: `events/events/discovery/folder_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FolderQuery {
    /// The folder that is the target of the query.
    pub folder: String,
}

impl FolderQuery {
    pub const UID: u16 = 8;
}

/// Network Connection Query events report information about active network connections.
///
/// Sourced from: `events/events/discovery/network_connection_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworkConnectionQuery {
    pub connection_info: String,
    /// The process that owns the socket.
    pub process: String,
    /// The state of the socket, normalized to the caption of the state_id value. In the case of 'Other', it is defined by the event source.
    pub state: Option<String>,
    /// The state of the socket.
    pub state_id: String,
}

impl NetworkConnectionQuery {
    pub const UID: u16 = 12;
}

/// Job Query events report information about scheduled jobs.
///
/// Sourced from: `events/events/discovery/job_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct JobQuery {
    pub job: String,
}

impl JobQuery {
    pub const UID: u16 = 10;
}

/// Process Query events report information about running processes.
///
/// Sourced from: `events/events/discovery/process_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProcessQuery {
    pub process: String,
}

impl ProcessQuery {
    pub const UID: u16 = 15;
}

/// Admin Group Query events report information about administrative groups.
///
/// Sourced from: `events/events/discovery/group_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AdminGroupQuery {
    /// The administrative group.
    pub group: String,
    /// The users that belong to the administrative group.
    pub users: Option<String>,
}

impl AdminGroupQuery {
    pub const UID: u16 = 9;
}

/// Kernel Object Query events report information about discovered kernel resources.
///
/// Sourced from: `events/events/discovery/kernel_object_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct KernelObjectQuery {
    /// The kernel object that pertains to the event.
    pub kernel: String,
}

impl KernelObjectQuery {
    pub const UID: u16 = 6;
}

/// Service Query events report information about running services.
///
/// Sourced from: `events/events/discovery/service_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ServiceQuery {
    pub service: String,
}

impl ServiceQuery {
    pub const UID: u16 = 16;
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

/// File Query events report information about files that are present on the system.
///
/// Sourced from: `events/events/discovery/file_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileQuery {
    /// The file that is the target of the query.
    pub file: String,
}

impl FileQuery {
    pub const UID: u16 = 7;
}

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

/// Networks Query events report information about network adapters.
///
/// Sourced from: `events/events/discovery/networks_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworksQuery {
    pub network_interfaces: String,
}

impl NetworksQuery {
    pub const UID: u16 = 13;
}

/// Discovery Result events report the results of a discovery request.
///
/// Sourced from: `events/events/discovery/discovery_result.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DiscoveryResult {
    pub activity_id: Option<String>,
    /// The search details associated with the query request.
    pub query_info: Option<String>,
    pub query_result: Option<String>,
    pub query_result_id: String,
}

impl DiscoveryResult {
    pub const UID: u16 = 5000;
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

/// Peripheral Device Query events report information about peripheral devices.
///
/// Sourced from: `events/events/discovery/peripheral_device_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PeripheralDeviceQuery {
    pub peripheral_device: String,
}

impl PeripheralDeviceQuery {
    pub const UID: u16 = 14;
}

/// User Session Query events report information about existing user sessions.
///
/// Sourced from: `events/events/discovery/session_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SessionQuery {
    pub session: String,
}

impl SessionQuery {
    pub const UID: u16 = 17;
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

/// User Query events report user data that have been discovered, queried, polled or searched. This event differs from User Inventory as it describes the result of a targeted search by filtering a subset of user attributes.
///
/// Sourced from: `events/events/discovery/user_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserQuery {
    pub user: String,
}

impl UserQuery {
    pub const UID: u16 = 18;
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

/// Module Query events report information about loaded modules.
///
/// Sourced from: `events/events/discovery/module_query.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ModuleQuery {
    pub module: String,
    /// The process that loaded the module.
    pub process: String,
}

impl ModuleQuery {
    pub const UID: u16 = 11;
}

// This file was automatically generated by ocsf-codegen at 2024-05-08T22:25:02+00:00 branch: "main" link: <https://github.com/yaleman/ocsf-rs/commit/e1a82e9b5299743a0c62bf0756f2eee94b7238a8> OCSF Schema version: 1.2.0
