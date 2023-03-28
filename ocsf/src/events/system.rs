/// Kernel Extension events report when a driver/extension is loaded or unloaded into the kernel
///
/// Sourced from: `events/events/system/kernel_extension.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct KernelExtension {
    pub activity_id: Option<crate::KernelExtensionActivity>,
    /// The actor process that loaded or unloaded the driver/extension.
    pub actor: String,
    pub driver: String,
}

impl KernelExtension {
    /// Returns the UID of the event type (2)
    pub fn uid() -> u16 {
        2 // UID value of event 2
    }
}

/// Scheduled Job Activity events report activities related to scheduled jobs or tasks.
///
/// Sourced from: `events/events/system/scheduled_job.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScheduledJobActivity {
    pub activity_id: Option<String>,
    /// The actor that performed the activity on the `job` object.
    pub actor: Option<String>,
    pub job: String,
}

impl ScheduledJobActivity {
    /// Returns the UID of the event type (6)
    pub fn uid() -> u16 {
        6 // UID value of event 6
    }
}

/// Kernel Activity events report when an process creates, reads, or deletes a kernel resource.
///
/// Sourced from: `events/events/system/kernel.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct KernelActivity {
    pub activity_id: Option<crate::KernelActivity>,
    /// The target kernel resource.
    pub kernel: String,
}

impl KernelActivity {
    /// Returns the UID of the event type (3)
    pub fn uid() -> u16 {
        3 // UID value of event 3
    }
}

/// Memory Activity events report when a process has memory allocated, read/modified, or other manipulation activities - such as a buffer overflow or turning off data execution protection (DEP).
///
/// Sourced from: `events/events/system/memory.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct MemoryActivity {
    pub activity_id: Option<crate::MemoryActivity>,
    pub actual_permissions: Option<String>,
    /// The memory address that was access or requested.
    pub base_address: Option<String>,
    /// The process that had memory allocated, read/written, or had other manipulation activities performed on it.
    pub process: String,
    pub requested_permissions: Option<String>,
    /// The memory size that was access or requested.
    pub size: Option<String>,
}

impl MemoryActivity {
    /// Returns the UID of the event type (4)
    pub fn uid() -> u16 {
        4 // UID value of event 4
    }
}

/// The System Activity event is a generic event that defines a set of attributes available in the system activity events. As a generic event, it could be used to log events that are not otherwise defined by the System Acivity category.
///
/// Sourced from: `events/events/system/system.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct System {
    pub actor: String,
    pub device: String,
}

impl System {
    /// Returns the UID of the event type (1000)
    pub fn uid() -> u16 {
        1000 // UID value of event 1000
    }
}

/// Process Activity events report when a process launches, injects, opens or terminates another process, successful or otherwise.
///
/// Sourced from: `events/events/system/process.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProcessActivity {
    pub activity_id: Option<crate::ProcessActivity>,
    /// The actor that performed the activity on the target `process`. For example, the process that started a new process or injected code into another process.
    pub actor: Option<String>,
    pub actual_permissions: Option<String>,
    pub exit_code: Option<String>,
    pub injection_type: Option<String>,
    pub injection_type_id: Option<String>,
    /// The module that was injected by the actor process.
    pub module: Option<String>,
    /// The process that was launched, injected into, opened, or terminated.
    pub process: String,
    pub requested_permissions: Option<String>,
}

impl ProcessActivity {
    /// Returns the UID of the event type (7)
    pub fn uid() -> u16 {
        7 // UID value of event 7
    }
}

/// Module  Activity events report when a process loads or unloads the <code>module</code>.
///
/// Sourced from: `events/events/system/module.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ModuleActivity {
    pub activity_id: Option<crate::ModuleActivity>,
    /// The actor that loaded or unloaded the `module`.
    pub actor: String,
    /// The module that was loaded or unloaded.
    pub module: String,
}

impl ModuleActivity {
    /// Returns the UID of the event type (5)
    pub fn uid() -> u16 {
        5 // UID value of event 5
    }
}

/// File System Activity events report when a process performs an action on a file or folder.
///
/// Sourced from: `events/events/system/filesystem.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileActivity {
    pub access_mask: Option<String>,
    /// The activity ID of the event.
    pub activity_id: Option<crate::FileActivity>,
    /// The actor that performed the activity on the `file` object
    pub actor: String,
    pub component: Option<String>,
    pub connection_uid: Option<String>,
    pub create_mask: Option<String>,
    /// The file that is the target of the activity.
    pub file: String,
    pub file_diff: Option<String>,
    /// The resulting file object when the activity was allowed and successful.
    pub file_result: Option<String>,
}

impl FileActivity {
    /// Returns the UID of the event type (1)
    pub fn uid() -> u16 {
        1 // UID value of event 1
    }
}

pub mod windows;

// This file was automatically generated by ocsf-codegen at 2023-03-28T11:14:40+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/9d78df7466f9ac2dd533f1a5efdb53c8ce479741>
