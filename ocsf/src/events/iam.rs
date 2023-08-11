/// The Identity & Access Management event is a generic event that defines a set of attributes available in the access control events. As a generic event, it could be used to log events that are not otherwise defined by the IAM category.
///
/// Sourced from: `events/events/iam/iam.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Iam;

impl Iam {
    pub const UID: u16 = 3000;
}

/// Authorize Session events report privileges or groups assigned to a new user session, usually at login time.
///
/// Sourced from: `events/events/iam/authorize_session.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthorizeSession {
    pub activity_id: Option<crate::Authorization>,
    /// The Endpoint for which the user session was targeted.
    pub dst_endpoint: Option<String>,
    /// Group that was assigned to the new user session.
    pub group: Option<String>,
    /// The list of sensitive privileges, assigned to the new user session.
    pub privileges: Option<String>,
    /// The user session with the assigned privileges.
    pub session: Option<String>,
    /// The user to which new privileges were assigned.
    pub user: String,
}

impl AuthorizeSession {
    pub const UID: u16 = 3;
}

/// Group Management events report management updates to a group, including updates to membership and permissions.
///
/// Sourced from: `events/events/iam/group_management.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GroupManagement {
    pub activity_id: Option<String>,
    /// Group that was the target of the event.
    pub group: String,
    /// A list of privileges assigned to the group.
    pub privileges: Option<String>,
    /// Resource that the privileges give access to.
    pub resource: Option<String>,
    /// A user that was added to or removed from the group.
    pub user: Option<String>,
}

impl GroupManagement {
    pub const UID: u16 = 6;
}

/// User Access Management events report management updates to a user's privileges.
///
/// Sourced from: `events/events/iam/user_access.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserAccess {
    pub activity_id: Option<String>,
    /// List of privileges assigned to a user.
    pub privileges: String,
    /// Resource that the privileges give access to.
    pub resource: Option<String>,
    /// User to which privileges were assigned.
    pub user: String,
}

impl UserAccess {
    pub const UID: u16 = 5;
}

/// Account Change events report when specific user account management tasks are performed, such as a user/role being created, changed, deleted, renamed, disabled, enabled, locked out or unlocked.
///
/// Sourced from: `events/events/iam/account_change.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AccountChange {
    pub activity_id: Option<String>,
    pub actor: Option<String>,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: Option<String>,
    /// The user that was a target of an activity.
    pub user: String,
    pub user_result: Option<String>,
}

impl AccountChange {
    pub const UID: u16 = 1;
}

/// Entity Management events report activity by a managed client, a micro service, or a user at a management console. The activity can be a create, read, update, and delete operation on a managed entity.
///
/// Sourced from: `events/events/iam/entity_management.json`
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

/// Authentication events report authentication session activities such as user attempts a logon or logoff, successfully or otherwise.
///
/// Sourced from: `events/events/iam/authentication.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Authentication {
    pub activity_id: Option<String>,
    /// The actor that requested the authentication.
    pub actor: Option<String>,
    pub auth_protocol: Option<String>,
    pub auth_protocol_id: Option<String>,
    /// The certificate associated with the authentication or pre-authentication (Kerberos).
    pub certificate: Option<String>,
    /// The endpoint to which the authentication was targeted.
    pub dst_endpoint: Option<String>,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    pub is_cleartext: Option<String>,
    pub is_mfa: Option<String>,
    pub is_new_logon: Option<String>,
    /// The attempted authentication is over a remote connection.
    pub is_remote: Option<String>,
    pub logon_process: Option<String>,
    pub logon_type: Option<String>,
    pub logon_type_id: Option<String>,
    /// The service or gateway to which the user or process is being authenticated
    pub service: Option<String>,
    pub session: Option<String>,
    /// The Endpoint from which the authentication was requested.
    pub src_endpoint: Option<String>,
    /// The details about the authentication request. For example, possible details for Windows logon or logoff events are:<ul><li>Success</li><ul><li>LOGOFF_USER_INITIATED</li><li>LOGOFF_OTHER</li></ul><li>Failure</li><ul><li>USER_DOES_NOT_EXIST</li><li>INVALID_CREDENTIALS</li><li>ACCOUNT_DISABLED</li><li>ACCOUNT_LOCKED_OUT</li><li>PASSWORD_EXPIRED</li></ul></ul>
    pub status_detail: Option<String>,
    /// The subject (user/role or account) to authenticate.
    pub user: String,
}

impl Authentication {
    pub const UID: u16 = 2;
}

// This file was automatically generated by ocsf-codegen at 2023-08-11T23:04:15+00:00 branch: "fixing-build-updating-schema" link: <https://github.com/yaleman/ocsf-rs/commit/c9ee180815729dcae5c937f32da30495d8aecf1a> OCSF Schema version: 1.1.0-dev
