/// SSH Activity events report remote client connections to a server using the Secure Shell (SSH) Protocol.
///
/// Sourced from: `events/network/ssh.json`
#[derive(Deserialize, Serialize)]
pub struct SshActivity {
    server_hassh: Option<String>,
    /// The Secure Shell Protocol version.
    protocol_ver: Option<String>,
    client_hassh: Option<String>,
}

/// DHCP Activity events report MAC to IP assignment via DHCP from a client or server.
///
/// Sourced from: `events/network/dhcp.json`
#[derive(Deserialize, Serialize)]
pub struct DhcpActivity {
    activity_id: String,
    relay: Option<String>,
    /// The initiator (client) of the DHCP connection.
    src_endpoint: Option<String>,
    /// The unique identifier of the transaction. This is typically a random number generated from the client to associate a dhcp request/response pair.
    transaction_uid: Option<String>,
    /// The responder (server) of the DHCP connection.
    dst_endpoint: Option<String>,
    lease_dur: Option<String>,
    is_renewal: Option<String>,
}

/// Server Message Block (SMB) Protocol Activity events report client/server connections sharing resources within the network.
///
/// Sourced from: `events/network/smb.json`
#[derive(Deserialize, Serialize)]
pub struct SmbActivity {
    client_dialects: Option<String>,
    dce_rpc: Option<String>,
    dialect: Option<String>,
    /// The command name (e.g. SMB2_COMMAND_CREATE, SMB1_COMMAND_WRITE_ANDX).
    command: Option<String>,
    tree_uid: Option<String>,
    /// Indicates how the file was opened (e.g. normal, delete on close).
    open_type: Option<String>,
    /// The server response in an SMB network connection.
    response: Option<String>,
    /// The file that is the target of the SMB activity.
    file: Option<String>,
    share: Option<String>,
    activity_id: Option<String>,
    share_type: Option<String>,
}

/// Remote Desktop Protocol (RDP) Activity events report remote client connections to a server as seen on the network.
///
/// Sourced from: `events/network/rdp.json`
#[derive(Deserialize, Serialize)]
pub struct RdpActivity {
    /// The list of observed certificates in an RDP TLS connection.
    certificate_chain: Option<String>,
    /// The client request in an RDP network connection.
    request: Option<String>,
    activity_id: Option<String>,
    capabilities: Option<String>,
    identifier_cookie: Option<String>,
    /// The Remote Desktop Protocol version.
    protocol_ver: Option<String>,
    remote_display: Option<String>,
    /// The server response in an RDP network connection.
    response: Option<String>,
    /// The device instigating the RDP connection.
    device: Option<String>,
}

/// DNS Activity events report DNS queries and answers as seen on the network.
///
/// Sourced from: `events/network/dns.json`
#[derive(Deserialize, Serialize)]
pub struct DnsActivity {
    answers: Option<String>,
    activity_id: Option<String>,
    query_time: Option<String>,
    connection_info: Option<String>,
    proxy: Option<String>,
    traffic: Option<String>,
    response_time: Option<String>,
    query: Option<String>,
    /// The normalized identifier of the DNS server response code. See <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc6895'>RFC-6895</a>.
    rcode_id: Option<String>,
    /// The DNS server response code, as defined by the event source.
    rcode: Option<String>,
}

/// HTTP Activity events report HTTP connection and traffic information.
///
/// Sourced from: `events/network/http.json`
#[derive(Deserialize, Serialize)]
pub struct HttpActivity;

/// Network Activity events report network connection and traffic activity.
///
/// Sourced from: `events/network/network.json`
#[derive(Deserialize, Serialize)]
pub struct NetworkActivity {
    activity_id: Option<String>,
}

/// Email events report activities of emails.
///
/// Sourced from: `events/network/email.json`
#[derive(Deserialize, Serialize)]
pub struct EmailActivity {
    activity_id: Option<String>,
    email_auth: Option<String>,
    /// The attempt number for attempting to deliver the email.
    attempt: Option<String>,
    /// The direction of the email relative to the scanning host or organization.
    /// ///Email scanned at an internet gateway might be characterized as inbound to the organization from the Internet, outbound from the organization to the Internet, or internal within the organization. Email scanned at a workstation might be characterized as inbound to, or outbound from the workstation.
    direction_id: String,
    /// The initiator (client) sending the email.
    src_endpoint: Option<String>,
    /// The value of the SMTP HELO or EHLO command sent by the initiator (client).
    smtp_hello: Option<String>,
    banner: Option<String>,
    email: String,
    /// The direction of the email, as defined by the `direction_id` value.
    direction: Option<String>,
    /// The responder (server) receiving the email.
    dst_endpoint: Option<String>,
}

/// File Transfer Protocol (FTP) Activity events report file transfers between a server and a client as seen on the network.
///
/// Sourced from: `events/network/ftp.json`
#[derive(Deserialize, Serialize)]
pub struct FtpActivity {
    /// The list of responses to the FTP command.
    command_responses: Option<String>,
    /// The list of return codes to the FTP command.
    codes: Option<String>,
    /// The dynamic port established for impending data transfers.
    port: Option<String>,
    /// The FTP command.
    command: Option<String>,
    activity_id: Option<String>,
    /// The type of FTP network connection (e.g. active, passive).
    #[serde(alias = "type")]
    type_field: Option<String>,
    /// The name of the data affiliated with the command.
    name: Option<String>,
}

use serde::{Deserialize, Serialize};

// This file was automatically generated by ocsf-codegen at 2023-03-27T21:46:59+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/4e69c4f97b90710c53906ab4e63de0c80aa8f60a>
