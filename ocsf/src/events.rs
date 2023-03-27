/// SSH Activity events report remote client connections to a server using the Secure Shell (SSH) Protocol.
///
/// Sourced from: `events/network/ssh.json`
#[derive(Deserialize, Serialize)]
pub struct SshActivity {
    client_hassh: Option<String>,
    server_hassh: Option<String>,
    /// The Secure Shell Protocol version.
    protocol_ver: Option<String>,
}

// kilroy was here ssh.json

/// DHCP Activity events report MAC to IP assignment via DHCP from a client or server.
///
/// Sourced from: `events/network/dhcp.json`
#[derive(Deserialize, Serialize)]
pub struct DhcpActivity {
    /// The unique identifier of the transaction. This is typically a random number generated from the client to associate a dhcp request/response pair.
    transaction_uid: Option<String>,
    relay: Option<String>,
    lease_dur: Option<String>,
    activity_id: String,
    /// The initiator (client) of the DHCP connection.
    src_endpoint: Option<String>,
    /// The responder (server) of the DHCP connection.
    dst_endpoint: Option<String>,
    is_renewal: Option<String>,
}

// kilroy was here dhcp.json

/// Server Message Block (SMB) Protocol Activity events report client/server connections sharing resources within the network.
///
/// Sourced from: `events/network/smb.json`
#[derive(Deserialize, Serialize)]
pub struct SmbActivity {
    share_type: Option<String>,
    /// The server response in an SMB network connection.
    response: Option<String>,
    /// Indicates how the file was opened (e.g. normal, delete on close).
    open_type: Option<String>,
    /// The command name (e.g. SMB2_COMMAND_CREATE, SMB1_COMMAND_WRITE_ANDX).
    command: Option<String>,
    activity_id: Option<String>,
    client_dialects: Option<String>,
    dialect: Option<String>,
    /// The file that is the target of the SMB activity.
    file: Option<String>,
    share: Option<String>,
    tree_uid: Option<String>,
    dce_rpc: Option<String>,
}

// kilroy was here smb.json

/// Remote Desktop Protocol (RDP) Activity events report remote client connections to a server as seen on the network.
///
/// Sourced from: `events/network/rdp.json`
#[derive(Deserialize, Serialize)]
pub struct RdpActivity {
    /// The client request in an RDP network connection.
    request: Option<String>,
    /// The list of observed certificates in an RDP TLS connection.
    certificate_chain: Option<String>,
    capabilities: Option<String>,
    /// The server response in an RDP network connection.
    response: Option<String>,
    /// The device instigating the RDP connection.
    device: Option<String>,
    activity_id: Option<String>,
    /// The Remote Desktop Protocol version.
    protocol_ver: Option<String>,
    identifier_cookie: Option<String>,
    remote_display: Option<String>,
}

// kilroy was here rdp.json

/// DNS Activity events report DNS queries and answers as seen on the network.
///
/// Sourced from: `events/network/dns.json`
#[derive(Deserialize, Serialize)]
pub struct DnsActivity {
    response_time: Option<String>,
    traffic: Option<String>,
    query_time: Option<String>,
    answers: Option<String>,
    proxy: Option<String>,
    activity_id: Option<String>,
    query: Option<String>,
    /// The DNS server response code, as defined by the event source.
    rcode: Option<String>,
    connection_info: Option<String>,
    /// The normalized identifier of the DNS server response code. See <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc6895'>RFC-6895</a>.
    rcode_id: Option<String>,
}

// kilroy was here dns.json

/// HTTP Activity events report HTTP connection and traffic information.
///
/// Sourced from: `events/network/http.json`
#[derive(Deserialize, Serialize)]
pub struct HttpActivity;

// kilroy was here http.json

/// Network Activity events report network connection and traffic activity.
///
/// Sourced from: `events/network/network.json`
#[derive(Deserialize, Serialize)]
pub struct NetworkActivity {
    activity_id: Option<String>,
}

// kilroy was here network.json

/// Email events report activities of emails.
///
/// Sourced from: `events/network/email.json`
#[derive(Deserialize, Serialize)]
pub struct EmailActivity {
    activity_id: Option<String>,
    banner: Option<String>,
    /// The value of the SMTP HELO or EHLO command sent by the initiator (client).
    smtp_hello: Option<String>,
    /// The initiator (client) sending the email.
    src_endpoint: Option<String>,
    /// The direction of the email relative to the scanning host or organization.
    /// ///Email scanned at an internet gateway might be characterized as inbound to the organization from the Internet, outbound from the organization to the Internet, or internal within the organization. Email scanned at a workstation might be characterized as inbound to, or outbound from the workstation.
    direction_id: String,
    /// The direction of the email, as defined by the `direction_id` value.
    direction: Option<String>,
    /// The attempt number for attempting to deliver the email.
    attempt: Option<String>,
    email: String,
    email_auth: Option<String>,
    /// The responder (server) receiving the email.
    dst_endpoint: Option<String>,
}

// kilroy was here email.json

/// File Transfer Protocol (FTP) Activity events report file transfers between a server and a client as seen on the network.
///
/// Sourced from: `events/network/ftp.json`
#[derive(Deserialize, Serialize)]
pub struct FtpActivity {
    activity_id: Option<String>,
    /// The list of responses to the FTP command.
    command_responses: Option<String>,
    /// The list of return codes to the FTP command.
    codes: Option<String>,
    /// The FTP command.
    command: Option<String>,
    /// The type of FTP network connection (e.g. active, passive).
    #[serde(alias="type")]
    type_field: Option<String>,
    /// The name of the data affiliated with the command.
    name: Option<String>,
    /// The dynamic port established for impending data transfers.
    port: Option<String>,
}

// kilroy was here ftp.json

/// Kernel Activity events report when an process creates, reads, or deletes a kernel resource.
///
/// Sourced from: `events/system/kernel.json`
#[derive(Deserialize, Serialize)]
pub struct KernelActivity {
    activity_id: Option<String>,
    /// The target kernel resource.
    kernel: String,
}

// kilroy was here kernel.json

/// Memory Activity events report when a process has memory allocated, read/modified, or other manipulation activities - such as a buffer overflow or turning off data execution protection (DEP).
///
/// Sourced from: `events/system/memory.json`
#[derive(Deserialize, Serialize)]
pub struct MemoryActivity {
    /// The memory address that was access or requested.
    base_address: Option<String>,
    actual_permissions: Option<String>,
    /// The process that had memory allocated, read/written, or had other manipulation activities performed on it.
    process: String,
    /// The memory size that was access or requested.
    size: Option<String>,
    requested_permissions: Option<String>,
    activity_id: Option<String>,
}

// kilroy was here memory.json

/// Kernel Extension events report when a driver/extension is loaded or unloaded into the kernel
///
/// Sourced from: `events/system/kernel_extension.json`
#[derive(Deserialize, Serialize)]
pub struct KernelExtension {
    activity_id: Option<String>,
    driver: String,
    /// The actor process that loaded or unloaded the driver/extension.
    actor: String,
}

// kilroy was here kernel_extension.json

/// Module  Activity events report when a process loads or unloads the <code>module</code>.
///
/// Sourced from: `events/system/module.json`
#[derive(Deserialize, Serialize)]
pub struct ModuleActivity {
    /// The module that was loaded or unloaded.
    module: String,
    /// The actor that loaded or unloaded the `module`.
    actor: String,
    activity_id: Option<String>,
}

// kilroy was here module.json

/// Scheduled Job Activity events report activities related to scheduled jobs or tasks.
///
/// Sourced from: `events/system/scheduled_job.json`
#[derive(Deserialize, Serialize)]
pub struct ScheduledJobActivity {
    activity_id: Option<String>,
    job: String,
    /// The actor that performed the activity on the `job` object.
    actor: Option<String>,
}

// kilroy was here scheduled_job.json

/// Process Activity events report when a process launches, injects, opens or terminates another process, successful or otherwise.
///
/// Sourced from: `events/system/process.json`
#[derive(Deserialize, Serialize)]
pub struct ProcessActivity {
    actual_permissions: Option<String>,
    injection_type: Option<String>,
    activity_id: Option<String>,
    /// The actor that performed the activity on the target `process`. For example, the process that started a new process or injected code into another process.
    actor: Option<String>,
    /// The module that was injected by the actor process.
    module: Option<String>,
    /// The process that was launched, injected into, opened, or terminated.
    process: String,
    injection_type_id: Option<String>,
    requested_permissions: Option<String>,
    exit_code: Option<String>,
}

// kilroy was here process.json

// kilroy was here registry_key.json

// kilroy was here registry_value.json

// kilroy was here resource.json

/// File System Activity events report when a process performs an action on a file or folder.
///
/// Sourced from: `events/system/filesystem.json`
#[derive(Deserialize, Serialize)]
pub struct FileActivity {
    connection_uid: Option<String>,
    access_mask: Option<String>,
    /// The file that is the target of the activity.
    file: String,
    /// The actor that performed the activity on the `file` object
    actor: String,
    file_diff: Option<String>,
    create_mask: Option<String>,
    /// The activity ID of the event.
    activity_id: Option<String>,
    component: Option<String>,
    /// The resulting file object when the activity was allowed and successful.
    file_result: Option<String>,
}

// kilroy was here filesystem.json

/// The System Activity event is a generic event that defines a set of attributes available in the system activity events. As a generic event, it could be used to log events that are not otherwise defined by the System Acivity category.
///
/// Sourced from: `events/system/system.json`
#[derive(Deserialize, Serialize)]
pub struct System {
    actor: String,
    device: String,
}

// kilroy was here system.json

/// The Audit Activity event is a generic event that defines a set of attributes available in the audit events. As a generic event, it could be used to log events that are not otherwise defined by the Audit Activity category.
///
/// Sourced from: `events/audit/audit.json`
#[derive(Deserialize, Serialize)]
pub struct Audit;

// kilroy was here audit.json

/// Access activity events describe successful/failed attempts to access an application.
///
/// Sourced from: `events/audit/access_activity.json`
#[derive(Deserialize, Serialize)]
pub struct AccessActivity {
    /// Details about the proxy if available.
    proxy: Option<String>,
    tls: Option<String>,
    actor: Option<String>,
    http_request: String,
    activity_id: Option<String>,
    http_response: String,
    /// Details about the source endpoint of the connection.
    src_endpoint: Option<String>,
}

// kilroy was here access_activity.json

/// Entity Management events report activity by a managed client, a micro service, or a user at a management console. The activity can be a create, read, update, and delete operation on a managed entity.
///
/// Sourced from: `events/audit/entity.json`
#[derive(Deserialize, Serialize)]
pub struct EntityManagement {
    activity_id: Option<String>,
    /// Use for when the entity acting upon another entity is a process or user.
    actor: Option<String>,
    entity_result: Option<String>,
    /// The user provided comment about why the entity was changed.
    comment: Option<String>,
    entity: String,
}

// kilroy was here entity.json

/// API events describe general CRUD (Create, Read, Update, Delete) API activities, e.g. (AWS Cloudtrail)
///
/// Sourced from: `events/audit/api.json`
#[derive(Deserialize, Serialize)]
pub struct ApiActivity {
    /// Details about the underlying http request.
    http_request: Option<String>,
    api: String,
    dst_endpoint: Option<String>,
    /// Details about the source of the activity.
    src_endpoint: String,
    actor: String,
    /// Details about resources that were affected by the activity/event.
    resources: Option<String>,
    activity_id: Option<String>,
}

// kilroy was here api.json

/// Authorization events report special privileges or groups assigned to a session.
///
/// Sourced from: `events/audit/authorization.json`
#[derive(Deserialize, Serialize)]
pub struct Authorization {
    /// The Endpoint for which the authentication was targeted.
    dst_endpoint: String,
    /// The list of sensitive privileges, assigned to the new user session.
    privileges: String,
    /// The modified user session.
    session: Option<String>,
    /// The user to which new privileges were assigned.
    user: String,
    activity_id: Option<String>,
}

// kilroy was here authorization.json

/// Authentication events report authentication session activities such as user attempts a logon or logoff, successfully or otherwise.
///
/// Sourced from: `events/audit/authentication.json`
#[derive(Deserialize, Serialize)]
pub struct Authentication {
    auth_protocol_id: Option<String>,
    /// The details about the authentication request. For example, possible details for Windows logon or logoff events are:<ul><li>Success</li><ul><li>LOGOFF_USER_INITIATED</li><li>LOGOFF_OTHER</li></ul><li>Failure</li><ul><li>USER_DOES_NOT_EXIST</li><li>INVALID_CREDENTIALS</li><li>ACCOUNT_DISABLED</li><li>ACCOUNT_LOCKED_OUT</li><li>PASSWORD_EXPIRED</li></ul></ul>
    status_detail: Option<String>,
    auth_protocol: Option<String>,
    /// Details about the underlying http request.
    http_request: Option<String>,
    /// The actor that requested the authentication.
    actor: Option<String>,
    /// The Endpoint for which the authentication was targeted.
    dst_endpoint: String,
    /// The Endpoint from which the authentication was requested.
    src_endpoint: Option<String>,
    activity_id: Option<String>,
    /// The target identity (user/role) to authenticate.
    user: String,
    is_cleartext: Option<String>,
    logon_type: Option<String>,
    logon_process: Option<String>,
    logon_type_id: Option<String>,
    is_remote: Option<String>,
    /// The new session of the authenticated user.
    session: Option<String>,
    mfa: Option<String>,
}

// kilroy was here authentication.json

/// Account Change events report when specific user account management tasks are performed, such as a user/role being created, changed, deleted, renamed, disabled, enabled, locked out or unlocked.
///
/// Sourced from: `events/audit/account.json`
#[derive(Deserialize, Serialize)]
pub struct AccountChange {
    activity_id: Option<String>,
    /// The user that was a target of an activity.
    user: Option<String>,
    /// Details about the source of the activity.
    src_endpoint: Option<String>,
    actor: String,
    /// Details about the underlying http request.
    http_request: Option<String>,
    user_result: Option<String>,
}

use serde::{Deserialize, Serialize};

// pub mod system;

// This file was automatically generated by ocsf-codegen at 2023-03-27T13:41:16+00:00 branch: "dev" link: <https://github.com/yaleman/ocsf-rs/commit/9c7ec90049b7915775ce2ef2ae8666a38b762418>