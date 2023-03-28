/// Entity Management events report activity by a managed client, a micro service, or a user at a management console. The activity can be a create, read, update, and delete operation on a managed entity.
///
/// Sourced from: `events/events/audit/entity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EntityManagement {
    pub activity_id: Option<String>,
    /// Use for when the entity acting upon another entity is a process or user.
    pub actor: Option<String>,
    /// The user provided comment about why the entity was changed.
    pub comment: Option<String>,
    pub entity: String,
    pub entity_result: Option<String>,
}

impl EntityManagement {
    pub const UID: u16 = 4;
}

/// Access activity events describe successful/failed attempts to access an application.
///
/// Sourced from: `events/events/audit/access_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AccessActivity {
    pub activity_id: Option<String>,
    pub actor: Option<String>,
    pub http_request: String,
    pub http_response: String,
    /// Details about the proxy if available.
    pub proxy: Option<String>,
    /// Details about the source endpoint of the connection.
    pub src_endpoint: Option<String>,
    pub tls: Option<String>,
}

impl AccessActivity {
    pub const UID: u16 = 3006;
}

/// Account Change events report when specific user account management tasks are performed, such as a user/role being created, changed, deleted, renamed, disabled, enabled, locked out or unlocked.
///
/// Sourced from: `events/events/audit/account.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AccountChange {
    pub activity_id: Option<String>,
    pub actor: String,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: Option<String>,
    /// The user that was a target of an activity.
    pub user: Option<String>,
    pub user_result: Option<String>,
}

impl AccountChange {
    pub const UID: u16 = 1;
}

/// Authentication events report authentication session activities such as user attempts a logon or logoff, successfully or otherwise.
///
/// Sourced from: `events/events/audit/authentication.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Authentication {
    pub activity_id: Option<String>,
    /// The actor that requested the authentication.
    pub actor: Option<String>,
    pub auth_protocol: Option<String>,
    pub auth_protocol_id: Option<String>,
    /// The Endpoint for which the authentication was targeted.
    pub dst_endpoint: String,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    pub is_cleartext: Option<String>,
    pub is_remote: Option<String>,
    pub logon_process: Option<String>,
    pub logon_type: Option<String>,
    pub logon_type_id: Option<String>,
    pub mfa: Option<String>,
    /// The new session of the authenticated user.
    pub session: Option<String>,
    /// The Endpoint from which the authentication was requested.
    pub src_endpoint: Option<String>,
    /// The details about the authentication request. For example, possible details for Windows logon or logoff events are:<ul><li>Success</li><ul><li>LOGOFF_USER_INITIATED</li><li>LOGOFF_OTHER</li></ul><li>Failure</li><ul><li>USER_DOES_NOT_EXIST</li><li>INVALID_CREDENTIALS</li><li>ACCOUNT_DISABLED</li><li>ACCOUNT_LOCKED_OUT</li><li>PASSWORD_EXPIRED</li></ul></ul>
    pub status_detail: Option<String>,
    /// The target identity (user/role) to authenticate.
    pub user: String,
}

impl Authentication {
    pub const UID: u16 = 2;
}

/// The Audit Activity event is a generic event that defines a set of attributes available in the audit events. As a generic event, it could be used to log events that are not otherwise defined by the Audit Activity category.
///
/// Sourced from: `events/events/audit/audit.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Audit;

impl Audit {
    pub const UID: u16 = 3000;
}

/// Authorization events report special privileges or groups assigned to a session.
///
/// Sourced from: `events/events/audit/authorization.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Authorization {
    pub activity_id: Option<crate::Authorization>,
    /// The Endpoint for which the authentication was targeted.
    pub dst_endpoint: String,
    /// The list of sensitive privileges, assigned to the new user session.
    pub privileges: String,
    /// The modified user session.
    pub session: Option<String>,
    /// The user to which new privileges were assigned.
    pub user: String,
}

impl Authorization {
    pub const UID: u16 = 3;
}

/// API events describe general CRUD (Create, Read, Update, Delete) API activities, e.g. (AWS Cloudtrail)
///
/// Sourced from: `events/events/audit/api.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApiActivity {
    pub activity_id: Option<String>,
    pub actor: String,
    pub api: String,
    pub dst_endpoint: Option<String>,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    /// Details about resources that were affected by the activity/event.
    pub resources: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: String,
}

impl ApiActivity {
    pub const UID: u16 = 3005;
}

// This file was automatically generated by ocsf-codegen at 2023-03-28T11:36:12+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/c035bcbabbea474b72d2dfc1b4a316ad45549a19>
