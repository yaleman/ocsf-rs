/// DHCP Activity events report MAC to IP assignment via DHCP from a client or server.
///
/// Sourced from: `events/events/network/dhcp.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DhcpActivity {
    pub activity_id: String,
    /// The responder (server) of the DHCP connection.
    pub dst_endpoint: Option<String>,
    pub is_renewal: Option<String>,
    pub lease_dur: Option<String>,
    pub relay: Option<String>,
    /// The initiator (client) of the DHCP connection.
    pub src_endpoint: Option<String>,
    /// The unique identifier of the transaction. This is typically a random number generated from the client to associate a dhcp request/response pair.
    pub transaction_uid: Option<String>,
}

impl DhcpActivity {
    pub const UID: u16 = 4004;
}

/// DNS Activity events report DNS queries and answers as seen on the network.
///
/// Sourced from: `events/events/network/dns.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DnsActivity {
    pub activity_id: Option<crate::DnsActivity>,
    pub answers: Option<String>,
    pub connection_info: Option<String>,
    pub proxy: Option<String>,
    pub query: Option<String>,
    pub query_time: Option<String>,
    /// The DNS server response code, normalized to the caption of the rcode_id value. In the case of 'Other', it is defined by the event source.
    pub rcode: Option<String>,
    /// The normalized identifier of the DNS server response code. See <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc6895'>RFC-6895</a>.
    pub rcode_id: Option<String>,
    pub response_time: Option<String>,
    pub traffic: Option<String>,
}

impl DnsActivity {
    pub const UID: u16 = 3;
}

/// Email events report activities of emails.
///
/// Sourced from: `events/events/network/email.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EmailActivity {
    pub activity_id: Option<String>,
    /// The attempt number for attempting to deliver the email.
    pub attempt: Option<String>,
    pub banner: Option<String>,
    /// The direction of the email, as defined by the `direction_id` value.
    pub direction: Option<String>,
    /// The direction of the email relative to the scanning host or organization.
    /// ///Email scanned at an internet gateway might be characterized as inbound to the organization from the Internet, outbound from the organization to the Internet, or internal within the organization. Email scanned at a workstation might be characterized as inbound to, or outbound from the workstation.
    pub direction_id: String,
    /// The responder (server) receiving the email.
    pub dst_endpoint: Option<String>,
    pub email: String,
    pub email_auth: Option<String>,
    /// The value of the SMTP HELO or EHLO command sent by the initiator (client).
    pub smtp_hello: Option<String>,
    /// The initiator (client) sending the email.
    pub src_endpoint: Option<String>,
}

impl EmailActivity {
    pub const UID: u16 = 4009;
}

/// Remote Desktop Protocol (RDP) Activity events report remote client connections to a server as seen on the network.
///
/// Sourced from: `events/events/network/rdp.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RdpActivity {
    pub activity_id: Option<String>,
    pub capabilities: Option<String>,
    /// The list of observed certificates in an RDP TLS connection.
    pub certificate_chain: Option<String>,
    /// The device instigating the RDP connection.
    pub device: Option<String>,
    pub identifier_cookie: Option<String>,
    /// The Remote Desktop Protocol version.
    pub protocol_ver: Option<String>,
    pub remote_display: Option<String>,
    /// The client request in an RDP network connection.
    pub request: Option<String>,
    /// The server response in an RDP network connection.
    pub response: Option<String>,
}

impl RdpActivity {
    pub const UID: u16 = 5;
}

/// HTTP Activity events report HTTP connection and traffic information.
///
/// Sourced from: `events/events/network/http.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct HttpActivity {
    pub activity_id: Option<crate::HttpActivity>,
    pub http_request: String,
    pub http_response: String,
    pub http_status: Option<String>,
}

impl HttpActivity {
    pub const UID: u16 = 4002;
}

/// Server Message Block (SMB) Protocol Activity events report client/server connections sharing resources within the network.
///
/// Sourced from: `events/events/network/smb.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SmbActivity {
    pub activity_id: Option<String>,
    pub client_dialects: Option<String>,
    /// The command name (e.g. SMB2_COMMAND_CREATE, SMB1_COMMAND_WRITE_ANDX).
    pub command: Option<String>,
    pub dce_rpc: Option<String>,
    pub dialect: Option<String>,
    /// The file that is the target of the SMB activity.
    pub file: Option<String>,
    /// Indicates how the file was opened (e.g. normal, delete on close).
    pub open_type: Option<String>,
    /// The server response in an SMB network connection.
    pub response: Option<String>,
    pub share: Option<String>,
    pub share_type: Option<String>,
    pub share_type_id: Option<String>,
    pub tree_uid: Option<String>,
}

impl SmbActivity {
    pub const UID: u16 = 6;
}

/// Network Activity events report network connection and traffic activity.
///
/// Sourced from: `events/events/network/network.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworkActivity {
    pub activity_id: Option<crate::NetworkActivity>,
}

impl NetworkActivity {
    pub const UID: u16 = 4001;
}

/// Network File Activity events report file activities traversing the network, including file storage services such as Box, MS OneDrive, or Google Drive.
///
/// Sourced from: `events/events/network/file_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct NetworkFileActivity {
    pub activity_id: Option<String>,
    /// The actor that performed the activity on the target file.
    pub actor: String,
    pub connection_info: Option<String>,
    /// The endpoint that received the activity on the target file.
    pub dst_endpoint: Option<String>,
    /// The share expiration time.
    pub expiration_time: Option<String>,
    /// The file that is the target of the activity.
    pub file: String,
    /// The endpoint that performed the activity on the target file.
    pub src_endpoint: String,
}

impl NetworkFileActivity {
    pub const UID: u16 = 4010;
}

/// Email URL Activity events report URLs within an email.
///
/// Sourced from: `events/events/network/email_url.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EmailUrlActivity {
    pub activity_id: Option<String>,
    pub email_uid: String,
    /// The URL included in the email content.
    pub url: String,
}

impl EmailUrlActivity {
    pub const UID: u16 = 4012;
}

/// File Transfer Protocol (FTP) Activity events report file transfers between a server and a client as seen on the network.
///
/// Sourced from: `events/events/network/ftp.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FtpActivity {
    pub activity_id: Option<String>,
    /// The list of return codes to the FTP command.
    pub codes: Option<String>,
    /// The FTP command.
    pub command: Option<String>,
    /// The list of responses to the FTP command.
    pub command_responses: Option<String>,
    /// The name of the data affiliated with the command.
    pub name: Option<String>,
    /// The dynamic port established for impending data transfers.
    pub port: Option<String>,
    /// The type of FTP network connection (e.g. active, passive).
    pub type_name: Option<String>,
}

impl FtpActivity {
    pub const UID: u16 = 8;
}

/// SSH Activity events report remote client connections to a server using the Secure Shell (SSH) Protocol.
///
/// Sourced from: `events/events/network/ssh.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SshActivity {
    pub client_hassh: Option<String>,
    /// The Secure Shell Protocol version.
    pub protocol_ver: Option<String>,
    pub server_hassh: Option<String>,
}

impl SshActivity {
    pub const UID: u16 = 7;
}

/// Email File Activity events report files within emails.
///
/// Sourced from: `events/events/network/email_file.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct EmailFileActivity {
    pub activity_id: Option<String>,
    pub email_uid: String,
    /// The email file attachment.
    pub file: String,
}

impl EmailFileActivity {
    pub const UID: u16 = 4011;
}

// This file was automatically generated by ocsf-codegen at 2023-08-11T23:13:26+00:00 branch: "fixing-build-updating-schema" link: <https://github.com/yaleman/ocsf-rs/commit/8eed3271e56a81cc15bffb5bdd8540651315c06b> OCSF Schema version: 1.1.0-dev
