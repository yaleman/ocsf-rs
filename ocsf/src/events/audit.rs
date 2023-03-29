/// Account Change events report when specific user account management tasks are performed, such as a user/role being created, changed, deleted, renamed, disabled, enabled, locked out or unlocked.
///
/// Sourced from: `events/audit/account.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct AccountChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub actor: String,
    /// Details about the underlying http request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    /// Details about the source of the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    /// The user that was a target of an activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_result: Option<String>,
}

impl AccountChange {
    pub const UID: u16 = 1;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self {
            http_request: Some(http_request),
            ..self
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self {
            src_endpoint: Some(src_endpoint),
            ..self
        }
    }

    /// Set the value of user
    pub fn with_user(self, user: String) -> Self {
        Self {
            user: Some(user),
            ..self
        }
    }

    /// Set the value of user_result
    pub fn with_user_result(self, user_result: String) -> Self {
        Self {
            user_result: Some(user_result),
            ..self
        }
    }

    pub fn new(actor: String) -> Self {
        Self {
            activity_id: None,
            actor,
            http_request: None,
            src_endpoint: None,
            user: None,
            user_result: None,
        }
    }
}

/// Access activity events describe successful/failed attempts to access an application.
///
/// Sourced from: `events/audit/access_activity.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct AccessActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub http_request: String,
    pub http_response: String,
    /// Details about the proxy if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    /// Details about the source endpoint of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
}

impl AccessActivity {
    pub const UID: u16 = 3006;
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

    /// Set the value of proxy
    pub fn with_proxy(self, proxy: String) -> Self {
        Self {
            proxy: Some(proxy),
            ..self
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self {
            src_endpoint: Some(src_endpoint),
            ..self
        }
    }

    /// Set the value of tls
    pub fn with_tls(self, tls: String) -> Self {
        Self {
            tls: Some(tls),
            ..self
        }
    }

    pub fn new(http_request: String, http_response: String) -> Self {
        Self {
            activity_id: None,
            actor: None,
            http_request,
            http_response,
            proxy: None,
            src_endpoint: None,
            tls: None,
        }
    }
}

/// Authentication events report authentication session activities such as user attempts a logon or logoff, successfully or otherwise.
///
/// Sourced from: `events/audit/authentication.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Authentication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The actor that requested the authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_protocol_id: Option<String>,
    /// The Endpoint for which the authentication was targeted.
    pub dst_endpoint: String,
    /// Details about the underlying http request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cleartext: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<String>,
    /// The new session of the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// The Endpoint from which the authentication was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    /// The details about the authentication request. For example, possible details for Windows logon or logoff events are:<ul><li>Success</li><ul><li>LOGOFF_USER_INITIATED</li><li>LOGOFF_OTHER</li></ul><li>Failure</li><ul><li>USER_DOES_NOT_EXIST</li><li>INVALID_CREDENTIALS</li><li>ACCOUNT_DISABLED</li><li>ACCOUNT_LOCKED_OUT</li><li>PASSWORD_EXPIRED</li></ul></ul>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// The target identity (user/role) to authenticate.
    pub user: String,
}

impl Authentication {
    pub const UID: u16 = 2;
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

    /// Set the value of auth_protocol
    pub fn with_auth_protocol(self, auth_protocol: String) -> Self {
        Self {
            auth_protocol: Some(auth_protocol),
            ..self
        }
    }

    /// Set the value of auth_protocol_id
    pub fn with_auth_protocol_id(self, auth_protocol_id: String) -> Self {
        Self {
            auth_protocol_id: Some(auth_protocol_id),
            ..self
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self {
            http_request: Some(http_request),
            ..self
        }
    }

    /// Set the value of is_cleartext
    pub fn with_is_cleartext(self, is_cleartext: String) -> Self {
        Self {
            is_cleartext: Some(is_cleartext),
            ..self
        }
    }

    /// Set the value of is_remote
    pub fn with_is_remote(self, is_remote: String) -> Self {
        Self {
            is_remote: Some(is_remote),
            ..self
        }
    }

    /// Set the value of logon_process
    pub fn with_logon_process(self, logon_process: String) -> Self {
        Self {
            logon_process: Some(logon_process),
            ..self
        }
    }

    /// Set the value of logon_type
    pub fn with_logon_type(self, logon_type: String) -> Self {
        Self {
            logon_type: Some(logon_type),
            ..self
        }
    }

    /// Set the value of logon_type_id
    pub fn with_logon_type_id(self, logon_type_id: String) -> Self {
        Self {
            logon_type_id: Some(logon_type_id),
            ..self
        }
    }

    /// Set the value of mfa
    pub fn with_mfa(self, mfa: String) -> Self {
        Self {
            mfa: Some(mfa),
            ..self
        }
    }

    /// Set the value of session
    pub fn with_session(self, session: String) -> Self {
        Self {
            session: Some(session),
            ..self
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self {
            src_endpoint: Some(src_endpoint),
            ..self
        }
    }

    /// Set the value of status_detail
    pub fn with_status_detail(self, status_detail: String) -> Self {
        Self {
            status_detail: Some(status_detail),
            ..self
        }
    }

    pub fn new(dst_endpoint: String, user: String) -> Self {
        Self {
            activity_id: None,
            actor: None,
            auth_protocol: None,
            auth_protocol_id: None,
            dst_endpoint,
            http_request: None,
            is_cleartext: None,
            is_remote: None,
            logon_process: None,
            logon_type: None,
            logon_type_id: None,
            mfa: None,
            session: None,
            src_endpoint: None,
            status_detail: None,
            user,
        }
    }
}

/// The Audit Activity event is a generic event that defines a set of attributes available in the audit events. As a generic event, it could be used to log events that are not otherwise defined by the Audit Activity category.
///
/// Sourced from: `events/audit/audit.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Audit;

impl Audit {
    pub const UID: u16 = 3000;
    pub fn new() -> Self {
        Self {}
    }
}

/// API events describe general CRUD (Create, Read, Update, Delete) API activities, e.g. (AWS Cloudtrail)
///
/// Sourced from: `events/audit/api.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ApiActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub actor: String,
    pub api: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    /// Details about the underlying http request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    /// Details about resources that were affected by the activity/event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: String,
}

impl ApiActivity {
    pub const UID: u16 = 3005;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self {
            dst_endpoint: Some(dst_endpoint),
            ..self
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self {
            http_request: Some(http_request),
            ..self
        }
    }

    /// Set the value of resources
    pub fn with_resources(self, resources: String) -> Self {
        Self {
            resources: Some(resources),
            ..self
        }
    }

    pub fn new(actor: String, api: String, src_endpoint: String) -> Self {
        Self {
            activity_id: None,
            actor,
            api,
            dst_endpoint: None,
            http_request: None,
            resources: None,
            src_endpoint,
        }
    }
}

/// Authorization events report special privileges or groups assigned to a session.
///
/// Sourced from: `events/audit/authorization.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Authorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::Authorization>,
    /// The Endpoint for which the authentication was targeted.
    pub dst_endpoint: String,
    /// The list of sensitive privileges, assigned to the new user session.
    pub privileges: String,
    /// The modified user session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    /// The user to which new privileges were assigned.
    pub user: String,
}

impl Authorization {
    pub const UID: u16 = 3;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::Authorization) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of session
    pub fn with_session(self, session: String) -> Self {
        Self {
            session: Some(session),
            ..self
        }
    }

    pub fn new(dst_endpoint: String, privileges: String, user: String) -> Self {
        Self {
            activity_id: None,
            dst_endpoint,
            privileges,
            session: None,
            user,
        }
    }
}

/// Entity Management events report activity by a managed client, a micro service, or a user at a management console. The activity can be a create, read, update, and delete operation on a managed entity.
///
/// Sourced from: `events/audit/entity.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct EntityManagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// Use for when the entity acting upon another entity is a process or user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// The user provided comment about why the entity was changed.
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

    /// Set the value of comment
    pub fn with_comment(self, comment: String) -> Self {
        Self {
            comment: Some(comment),
            ..self
        }
    }

    /// Set the value of entity_result
    pub fn with_entity_result(self, entity_result: String) -> Self {
        Self {
            entity_result: Some(entity_result),
            ..self
        }
    }

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

// This file was automatically generated by ocsf-codegen at 2023-03-29T11:20:42+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/e3cab7448ca160ea1e324c46692c5096b4c5b3fa>
