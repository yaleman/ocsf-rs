/// Account Change events report when specific user account management tasks are performed, such as a user/role being created, changed, deleted, renamed, disabled, enabled, locked out or unlocked.
///
/// Sourced from: `events/iam/account_change.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct AccountChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_result: Option<String>,
}

impl AccountChange {
    pub const UID: u16 = 1;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self { http_request: Some(http_request),
        ..self  
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self { src_endpoint: Some(src_endpoint),
        ..self  
        }
    }

    /// Set the value of user_result
    pub fn with_user_result(self, user_result: String) -> Self {
        Self { user_result: Some(user_result),
        ..self  
        }
    }

    /// No description available. - optional
    pub fn new(user: String) -> Self {
        Self {
        activity_id: None,
        actor: None,
        http_request: None,
        src_endpoint: None,
        user,
        user_result: None,
        }
    }
}

/// Authentication events report authentication session activities such as user attempts a logon or logoff, successfully or otherwise.
///
/// Sourced from: `events/iam/authentication.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Authentication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_protocol_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cleartext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_logon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    pub user: String,
}

impl Authentication {
    pub const UID: u16 = 2;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of auth_protocol
    pub fn with_auth_protocol(self, auth_protocol: String) -> Self {
        Self { auth_protocol: Some(auth_protocol),
        ..self  
        }
    }

    /// Set the value of auth_protocol_id
    pub fn with_auth_protocol_id(self, auth_protocol_id: String) -> Self {
        Self { auth_protocol_id: Some(auth_protocol_id),
        ..self  
        }
    }

    /// Set the value of certificate
    pub fn with_certificate(self, certificate: String) -> Self {
        Self { certificate: Some(certificate),
        ..self  
        }
    }

    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self { dst_endpoint: Some(dst_endpoint),
        ..self  
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self { http_request: Some(http_request),
        ..self  
        }
    }

    /// Set the value of is_cleartext
    pub fn with_is_cleartext(self, is_cleartext: String) -> Self {
        Self { is_cleartext: Some(is_cleartext),
        ..self  
        }
    }

    /// Set the value of is_mfa
    pub fn with_is_mfa(self, is_mfa: String) -> Self {
        Self { is_mfa: Some(is_mfa),
        ..self  
        }
    }

    /// Set the value of is_new_logon
    pub fn with_is_new_logon(self, is_new_logon: String) -> Self {
        Self { is_new_logon: Some(is_new_logon),
        ..self  
        }
    }

    /// Set the value of is_remote
    pub fn with_is_remote(self, is_remote: String) -> Self {
        Self { is_remote: Some(is_remote),
        ..self  
        }
    }

    /// Set the value of logon_process
    pub fn with_logon_process(self, logon_process: String) -> Self {
        Self { logon_process: Some(logon_process),
        ..self  
        }
    }

    /// Set the value of logon_type
    pub fn with_logon_type(self, logon_type: String) -> Self {
        Self { logon_type: Some(logon_type),
        ..self  
        }
    }

    /// Set the value of logon_type_id
    pub fn with_logon_type_id(self, logon_type_id: String) -> Self {
        Self { logon_type_id: Some(logon_type_id),
        ..self  
        }
    }

    /// Set the value of service
    pub fn with_service(self, service: String) -> Self {
        Self { service: Some(service),
        ..self  
        }
    }

    /// Set the value of session
    pub fn with_session(self, session: String) -> Self {
        Self { session: Some(session),
        ..self  
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self { src_endpoint: Some(src_endpoint),
        ..self  
        }
    }

    /// Set the value of status_detail
    pub fn with_status_detail(self, status_detail: String) -> Self {
        Self { status_detail: Some(status_detail),
        ..self  
        }
    }

    /// The subject (user/role or account) to authenticate. - required
    pub fn new(user: String) -> Self {
        Self {
        activity_id: None,
        actor: None,
        auth_protocol: None,
        auth_protocol_id: None,
        certificate: None,
        dst_endpoint: None,
        http_request: None,
        is_cleartext: None,
        is_mfa: None,
        is_new_logon: None,
        is_remote: None,
        logon_process: None,
        logon_type: None,
        logon_type_id: None,
        service: None,
        session: None,
        src_endpoint: None,
        status_detail: None,
        user,
        }
    }
}

/// Authorize Session events report privileges or groups assigned to a new user session, usually at login time.
///
/// Sourced from: `events/iam/authorize_session.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct AuthorizeSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::Authorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    pub user: String,
}

impl AuthorizeSession {
    pub const UID: u16 = 3;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::Authorization) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self { dst_endpoint: Some(dst_endpoint),
        ..self  
        }
    }

    /// Set the value of group
    pub fn with_group(self, group: String) -> Self {
        Self { group: Some(group),
        ..self  
        }
    }

    /// Set the value of privileges
    pub fn with_privileges(self, privileges: String) -> Self {
        Self { privileges: Some(privileges),
        ..self  
        }
    }

    /// Set the value of session
    pub fn with_session(self, session: String) -> Self {
        Self { session: Some(session),
        ..self  
        }
    }

    /// The user to which new privileges were assigned. - required
    pub fn new(user: String) -> Self {
        Self {
        activity_id: None,
        dst_endpoint: None,
        group: None,
        privileges: None,
        session: None,
        user,
        }
    }
}

/// Entity Management events report activity by a managed client, a micro service, or a user at a management console. The activity can be a create, read, update, and delete operation on a managed entity.
///
/// Sourced from: `events/iam/entity_management.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct EntityManagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub entity: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_result: Option<String>,
}

impl EntityManagement {
    pub const UID: u16 = 4;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of comment
    pub fn with_comment(self, comment: String) -> Self {
        Self { comment: Some(comment),
        ..self  
        }
    }

    /// Set the value of entity_result
    pub fn with_entity_result(self, entity_result: String) -> Self {
        Self { entity_result: Some(entity_result),
        ..self  
        }
    }

    /// No description available. - optional
    pub fn new(entity: String) -> Self {
        Self {
        activity_id: None,
        actor: None,
        comment: None,
        entity,
        entity_result: None,
        }
    }
}

/// Group Management events report management updates to a group, including updates to membership and permissions.
///
/// Sourced from: `events/iam/group_management.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct GroupManagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl GroupManagement {
    pub const UID: u16 = 6;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of privileges
    pub fn with_privileges(self, privileges: String) -> Self {
        Self { privileges: Some(privileges),
        ..self  
        }
    }

    /// Set the value of resource
    pub fn with_resource(self, resource: String) -> Self {
        Self { resource: Some(resource),
        ..self  
        }
    }

    /// Set the value of user
    pub fn with_user(self, user: String) -> Self {
        Self { user: Some(user),
        ..self  
        }
    }

    /// A user that was added to or removed from the group. - recommended
    pub fn new(group: String) -> Self {
        Self {
        activity_id: None,
        group,
        privileges: None,
        resource: None,
        user: None,
        }
    }
}

/// The Identity & Access Management event is a generic event that defines a set of attributes available in the access control events. As a generic event, it could be used to log events that are not otherwise defined by the IAM category.
///
/// Sourced from: `events/iam/iam.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Iam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl Iam {
    pub const UID: u16 = 3000;
    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self { device: Some(device),
        ..self  
        }
    }

    /// No description available. - recommended
    pub fn new() -> Self {
        Self {
        actor: None,
        device: None,
        }
    }
}

/// User Access Management events report management updates to a user's privileges.
///
/// Sourced from: `events/iam/user_access.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct UserAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub privileges: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    pub user: String,
}

impl UserAccess {
    pub const UID: u16 = 5;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of resource
    pub fn with_resource(self, resource: String) -> Self {
        Self { resource: Some(resource),
        ..self  
        }
    }

    /// User to which privileges were assigned. - required
    pub fn new(privileges: String, user: String) -> Self {
        Self {
        activity_id: None,
        privileges,
        resource: None,
        user,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-06-06T22:56:49+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/7c25539db4b4823979b3d4ee870fad2ab8f805cf>