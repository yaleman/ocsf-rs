/// Kernel Activity events report when an process creates, reads, or deletes a kernel resource.
///
/// Sourced from: `events/system/kernel.json`
#[derive(Deserialize, Serialize)]
pub struct KernelActivity {
    activity_id: Option<String>,
    /// The target kernel resource.
    kernel: String,
}

/// Memory Activity events report when a process has memory allocated, read/modified, or other manipulation activities - such as a buffer overflow or turning off data execution protection (DEP).
///
/// Sourced from: `events/system/memory.json`
#[derive(Deserialize, Serialize)]
pub struct MemoryActivity {
    /// The memory size that was access or requested.
    size: Option<String>,
    activity_id: Option<String>,
    /// The process that had memory allocated, read/written, or had other manipulation activities performed on it.
    process: String,
    requested_permissions: Option<String>,
    actual_permissions: Option<String>,
    /// The memory address that was access or requested.
    base_address: Option<String>,
}

/// Kernel Extension events report when a driver/extension is loaded or unloaded into the kernel
///
/// Sourced from: `events/system/kernel_extension.json`
#[derive(Deserialize, Serialize)]
pub struct KernelExtension {
    /// The actor process that loaded or unloaded the driver/extension.
    actor: String,
    activity_id: Option<String>,
    driver: String,
}

/// Module  Activity events report when a process loads or unloads the <code>module</code>.
///
/// Sourced from: `events/system/module.json`
#[derive(Deserialize, Serialize)]
pub struct ModuleActivity {
    /// The module that was loaded or unloaded.
    module: String,
    activity_id: Option<String>,
    /// The actor that loaded or unloaded the `module`.
    actor: String,
}

/// Scheduled Job Activity events report activities related to scheduled jobs or tasks.
///
/// Sourced from: `events/system/scheduled_job.json`
#[derive(Deserialize, Serialize)]
pub struct ScheduledJobActivity {
    activity_id: Option<String>,
    /// The actor that performed the activity on the `job` object.
    actor: Option<String>,
    job: String,
}

/// Process Activity events report when a process launches, injects, opens or terminates another process, successful or otherwise.
///
/// Sourced from: `events/system/process.json`
#[derive(Deserialize, Serialize)]
pub struct ProcessActivity {
    /// The process that was launched, injected into, opened, or terminated.
    process: String,
    injection_type: Option<String>,
    injection_type_id: Option<String>,
    /// The module that was injected by the actor process.
    module: Option<String>,
    activity_id: Option<String>,
    exit_code: Option<String>,
    requested_permissions: Option<String>,
    actual_permissions: Option<String>,
    /// The actor that performed the activity on the target `process`. For example, the process that started a new process or injected code into another process.
    actor: Option<String>,
}

/// Registry Key Activity events report when a process performs an action on a Windows registry key.
///
/// Sourced from: `events/system/windows/registry_key.json`
#[derive(Deserialize, Serialize)]
pub struct RegistryKeyActivity {
    prev_reg_key: Option<String>,
    reg_key: String,
    create_mask: Option<String>,
    activity_id: Option<String>,
    /// The actor that performed the activity on the `reg_key` object.
    actor: String,
    access_mask: Option<String>,
    open_mask: Option<String>,
}

/// Registry Value Activity events reports when a process performs an action on a Windows registry value.
///
/// Sourced from: `events/system/windows/registry_value.json`
#[derive(Deserialize, Serialize)]
pub struct RegistryValueActivity {
    prev_reg_value: Option<String>,
    reg_value: String,
    /// The actor that performed the activity on the `reg_value` object.
    actor: String,
    activity_id: Option<String>,
}

/// Windows Resource Activity events report when a process accesses a Windows managed resource object, successful or otherwise.
///
/// Sourced from: `events/system/windows/resource.json`
#[derive(Deserialize, Serialize)]
pub struct ResourceActivity {
    activity_id: Option<String>,
    win_resource: String,
}

/// File System Activity events report when a process performs an action on a file or folder.
///
/// Sourced from: `events/system/filesystem.json`
#[derive(Deserialize, Serialize)]
pub struct FileActivity {
    /// The resulting file object when the activity was allowed and successful.
    file_result: Option<String>,
    create_mask: Option<String>,
    file_diff: Option<String>,
    access_mask: Option<String>,
    /// The actor that performed the activity on the `file` object
    actor: String,
    component: Option<String>,
    /// The activity ID of the event.
    activity_id: Option<String>,
    connection_uid: Option<String>,
    /// The file that is the target of the activity.
    file: String,
}

/// The System Activity event is a generic event that defines a set of attributes available in the system activity events. As a generic event, it could be used to log events that are not otherwise defined by the System Acivity category.
///
/// Sourced from: `events/system/system.json`
#[derive(Deserialize, Serialize)]
pub struct System {
    device: String,
    actor: String,
}

use serde::{Deserialize, Serialize};

// This file was automatically generated by ocsf-codegen at 2023-03-27T21:46:59+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/4e69c4f97b90710c53906ab4e63de0c80aa8f60a>
