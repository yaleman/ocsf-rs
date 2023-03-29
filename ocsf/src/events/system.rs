/// Scheduled Job Activity events report activities related to scheduled jobs or tasks.
///
/// Sourced from: `events/system/scheduled_job.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ScheduledJobActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub job: String,
}

impl ScheduledJobActivity {
    pub const UID: u16 = 6;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self {
            actor: Some(actor),
            ..self
        }
    }

    pub fn new(job: String) -> Self {
        Self {
            activity_id: None,
            actor: None,
            job,
        }
    }
}

/// Kernel Extension events report when a driver/extension is loaded or unloaded into the kernel
///
/// Sourced from: `events/system/kernel_extension.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct KernelExtension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::KernelExtensionActivity>,
    pub actor: String,
    pub driver: String,
}

impl KernelExtension {
    pub const UID: u16 = 2;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::KernelExtensionActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn new(actor: String, driver: String) -> Self {
        Self {
            activity_id: None,
            actor,
            driver,
        }
    }
}

/// Memory Activity events report when a process has memory allocated, read/modified, or other manipulation activities - such as a buffer overflow or turning off data execution protection (DEP).
///
/// Sourced from: `events/system/memory.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct MemoryActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::MemoryActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_address: Option<String>,
    pub process: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl MemoryActivity {
    pub const UID: u16 = 4;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::MemoryActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of actual_permissions
    pub fn with_actual_permissions(self, actual_permissions: String) -> Self {
        Self {
            actual_permissions: Some(actual_permissions),
            ..self
        }
    }

    /// Set the value of base_address
    pub fn with_base_address(self, base_address: String) -> Self {
        Self {
            base_address: Some(base_address),
            ..self
        }
    }

    /// Set the value of requested_permissions
    pub fn with_requested_permissions(self, requested_permissions: String) -> Self {
        Self {
            requested_permissions: Some(requested_permissions),
            ..self
        }
    }

    /// Set the value of size
    pub fn with_size(self, size: String) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn new(process: String) -> Self {
        Self {
            activity_id: None,
            actual_permissions: None,
            base_address: None,
            process,
            requested_permissions: None,
            size: None,
        }
    }
}

/// The System Activity event is a generic event that defines a set of attributes available in the system activity events. As a generic event, it could be used to log events that are not otherwise defined by the System Acivity category.
///
/// Sourced from: `events/system/system.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct System {
    pub actor: String,
    pub device: String,
}

impl System {
    pub const UID: u16 = 1000;
    pub fn new(actor: String, device: String) -> Self {
        Self { actor, device }
    }
}

/// File System Activity events report when a process performs an action on a file or folder.
///
/// Sourced from: `events/system/filesystem.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct FileActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::FileActivity>,
    pub actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_mask: Option<String>,
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_result: Option<String>,
}

impl FileActivity {
    pub const UID: u16 = 1;
    /// Set the value of access_mask
    pub fn with_access_mask(self, access_mask: String) -> Self {
        Self {
            access_mask: Some(access_mask),
            ..self
        }
    }

    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::FileActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of component
    pub fn with_component(self, component: String) -> Self {
        Self {
            component: Some(component),
            ..self
        }
    }

    /// Set the value of connection_uid
    pub fn with_connection_uid(self, connection_uid: String) -> Self {
        Self {
            connection_uid: Some(connection_uid),
            ..self
        }
    }

    /// Set the value of create_mask
    pub fn with_create_mask(self, create_mask: String) -> Self {
        Self {
            create_mask: Some(create_mask),
            ..self
        }
    }

    /// Set the value of file_diff
    pub fn with_file_diff(self, file_diff: String) -> Self {
        Self {
            file_diff: Some(file_diff),
            ..self
        }
    }

    /// Set the value of file_result
    pub fn with_file_result(self, file_result: String) -> Self {
        Self {
            file_result: Some(file_result),
            ..self
        }
    }

    pub fn new(actor: String, file: String) -> Self {
        Self {
            access_mask: None,
            activity_id: None,
            actor,
            component: None,
            connection_uid: None,
            create_mask: None,
            file,
            file_diff: None,
            file_result: None,
        }
    }
}

/// Kernel Activity events report when an process creates, reads, or deletes a kernel resource.
///
/// Sourced from: `events/system/kernel.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct KernelActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::KernelActivity>,
    pub kernel: String,
}

impl KernelActivity {
    pub const UID: u16 = 3;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::KernelActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn new(kernel: String) -> Self {
        Self {
            activity_id: None,
            kernel,
        }
    }
}

/// Process Activity events report when a process launches, injects, opens or terminates another process, successful or otherwise.
///
/// Sourced from: `events/system/process.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ProcessActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::ProcessActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_permissions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injection_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injection_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub process: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_permissions: Option<String>,
}

impl ProcessActivity {
    pub const UID: u16 = 7;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::ProcessActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self {
            actor: Some(actor),
            ..self
        }
    }

    /// Set the value of actual_permissions
    pub fn with_actual_permissions(self, actual_permissions: String) -> Self {
        Self {
            actual_permissions: Some(actual_permissions),
            ..self
        }
    }

    /// Set the value of exit_code
    pub fn with_exit_code(self, exit_code: String) -> Self {
        Self {
            exit_code: Some(exit_code),
            ..self
        }
    }

    /// Set the value of injection_type
    pub fn with_injection_type(self, injection_type: String) -> Self {
        Self {
            injection_type: Some(injection_type),
            ..self
        }
    }

    /// Set the value of injection_type_id
    pub fn with_injection_type_id(self, injection_type_id: String) -> Self {
        Self {
            injection_type_id: Some(injection_type_id),
            ..self
        }
    }

    /// Set the value of module
    pub fn with_module(self, module: String) -> Self {
        Self {
            module: Some(module),
            ..self
        }
    }

    /// Set the value of requested_permissions
    pub fn with_requested_permissions(self, requested_permissions: String) -> Self {
        Self {
            requested_permissions: Some(requested_permissions),
            ..self
        }
    }

    pub fn new(process: String) -> Self {
        Self {
            activity_id: None,
            actor: None,
            actual_permissions: None,
            exit_code: None,
            injection_type: None,
            injection_type_id: None,
            module: None,
            process,
            requested_permissions: None,
        }
    }
}

/// Module  Activity events report when a process loads or unloads the <code>module</code>.
///
/// Sourced from: `events/system/module.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ModuleActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::ModuleActivity>,
    pub actor: String,
    pub module: String,
}

impl ModuleActivity {
    pub const UID: u16 = 5;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::ModuleActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn new(actor: String, module: String) -> Self {
        Self {
            activity_id: None,
            actor,
            module,
        }
    }
}

pub mod windows;

// This file was automatically generated by ocsf-codegen at 2023-03-29T12:27:22+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/60c657dc0c6103bdda104ca3be2ccbb778bd1c1a>
