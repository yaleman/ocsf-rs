/// Remote Desktop Protocol (RDP) Activity events report remote client connections to a server as seen on the network.
///
/// Sourced from: `events/network/rdp.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct RdpActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    /// The list of observed certificates in an RDP TLS connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// The device instigating the RDP connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_cookie: Option<String>,
    /// The Remote Desktop Protocol version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_display: Option<String>,
    /// The client request in an RDP network connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    /// The server response in an RDP network connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
}

impl RdpActivity {
    pub const UID: u16 = 5;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of capabilities
    pub fn with_capabilities(self, capabilities: String) -> Self {
        Self {
            capabilities: Some(capabilities),
            ..self
        }
    }

    /// Set the value of certificate_chain
    pub fn with_certificate_chain(self, certificate_chain: String) -> Self {
        Self {
            certificate_chain: Some(certificate_chain),
            ..self
        }
    }

    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self {
            device: Some(device),
            ..self
        }
    }

    /// Set the value of identifier_cookie
    pub fn with_identifier_cookie(self, identifier_cookie: String) -> Self {
        Self {
            identifier_cookie: Some(identifier_cookie),
            ..self
        }
    }

    /// Set the value of protocol_ver
    pub fn with_protocol_ver(self, protocol_ver: String) -> Self {
        Self {
            protocol_ver: Some(protocol_ver),
            ..self
        }
    }

    /// Set the value of remote_display
    pub fn with_remote_display(self, remote_display: String) -> Self {
        Self {
            remote_display: Some(remote_display),
            ..self
        }
    }

    /// Set the value of request
    pub fn with_request(self, request: String) -> Self {
        Self {
            request: Some(request),
            ..self
        }
    }

    /// Set the value of response
    pub fn with_response(self, response: String) -> Self {
        Self {
            response: Some(response),
            ..self
        }
    }

    pub fn new() -> Self {
        Self {
            activity_id: None,
            capabilities: None,
            certificate_chain: None,
            device: None,
            identifier_cookie: None,
            protocol_ver: None,
            remote_display: None,
            request: None,
            response: None,
        }
    }
}

/// DHCP Activity events report MAC to IP assignment via DHCP from a client or server.
///
/// Sourced from: `events/network/dhcp.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct DhcpActivity {
    pub activity_id: String,
    /// The responder (server) of the DHCP connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_renewal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_dur: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    /// The initiator (client) of the DHCP connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    /// The unique identifier of the transaction. This is typically a random number generated from the client to associate a dhcp request/response pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_uid: Option<String>,
}

impl DhcpActivity {
    pub const UID: u16 = 4004;
    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self {
            dst_endpoint: Some(dst_endpoint),
            ..self
        }
    }

    /// Set the value of is_renewal
    pub fn with_is_renewal(self, is_renewal: String) -> Self {
        Self {
            is_renewal: Some(is_renewal),
            ..self
        }
    }

    /// Set the value of lease_dur
    pub fn with_lease_dur(self, lease_dur: String) -> Self {
        Self {
            lease_dur: Some(lease_dur),
            ..self
        }
    }

    /// Set the value of relay
    pub fn with_relay(self, relay: String) -> Self {
        Self {
            relay: Some(relay),
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

    /// Set the value of transaction_uid
    pub fn with_transaction_uid(self, transaction_uid: String) -> Self {
        Self {
            transaction_uid: Some(transaction_uid),
            ..self
        }
    }

    pub fn new(activity_id: String) -> Self {
        Self {
            activity_id,
            dst_endpoint: None,
            is_renewal: None,
            lease_dur: None,
            relay: None,
            src_endpoint: None,
            transaction_uid: None,
        }
    }
}

/// HTTP Activity events report HTTP connection and traffic information.
///
/// Sourced from: `events/network/http.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct HttpActivity;

impl HttpActivity {
    pub const UID: u16 = 2;
    pub fn new() -> Self {
        Self {}
    }
}

/// DNS Activity events report DNS queries and answers as seen on the network.
///
/// Sourced from: `events/network/dns.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct DnsActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::DnsActivity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_time: Option<String>,
    /// The DNS server response code, as defined by the event source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcode: Option<String>,
    /// The normalized identifier of the DNS server response code. See <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc6895'>RFC-6895</a>.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcode_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic: Option<String>,
}

impl DnsActivity {
    pub const UID: u16 = 3;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::DnsActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of answers
    pub fn with_answers(self, answers: String) -> Self {
        Self {
            answers: Some(answers),
            ..self
        }
    }

    /// Set the value of connection_info
    pub fn with_connection_info(self, connection_info: String) -> Self {
        Self {
            connection_info: Some(connection_info),
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

    /// Set the value of query
    pub fn with_query(self, query: String) -> Self {
        Self {
            query: Some(query),
            ..self
        }
    }

    /// Set the value of query_time
    pub fn with_query_time(self, query_time: String) -> Self {
        Self {
            query_time: Some(query_time),
            ..self
        }
    }

    /// Set the value of rcode
    pub fn with_rcode(self, rcode: String) -> Self {
        Self {
            rcode: Some(rcode),
            ..self
        }
    }

    /// Set the value of rcode_id
    pub fn with_rcode_id(self, rcode_id: String) -> Self {
        Self {
            rcode_id: Some(rcode_id),
            ..self
        }
    }

    /// Set the value of response_time
    pub fn with_response_time(self, response_time: String) -> Self {
        Self {
            response_time: Some(response_time),
            ..self
        }
    }

    /// Set the value of traffic
    pub fn with_traffic(self, traffic: String) -> Self {
        Self {
            traffic: Some(traffic),
            ..self
        }
    }

    pub fn new() -> Self {
        Self {
            activity_id: None,
            answers: None,
            connection_info: None,
            proxy: None,
            query: None,
            query_time: None,
            rcode: None,
            rcode_id: None,
            response_time: None,
            traffic: None,
        }
    }
}

/// SSH Activity events report remote client connections to a server using the Secure Shell (SSH) Protocol.
///
/// Sourced from: `events/network/ssh.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct SshActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_hassh: Option<String>,
    /// The Secure Shell Protocol version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_hassh: Option<String>,
}

impl SshActivity {
    pub const UID: u16 = 7;
    /// Set the value of client_hassh
    pub fn with_client_hassh(self, client_hassh: String) -> Self {
        Self {
            client_hassh: Some(client_hassh),
            ..self
        }
    }

    /// Set the value of protocol_ver
    pub fn with_protocol_ver(self, protocol_ver: String) -> Self {
        Self {
            protocol_ver: Some(protocol_ver),
            ..self
        }
    }

    /// Set the value of server_hassh
    pub fn with_server_hassh(self, server_hassh: String) -> Self {
        Self {
            server_hassh: Some(server_hassh),
            ..self
        }
    }

    pub fn new() -> Self {
        Self {
            client_hassh: None,
            protocol_ver: None,
            server_hassh: None,
        }
    }
}

/// File Transfer Protocol (FTP) Activity events report file transfers between a server and a client as seen on the network.
///
/// Sourced from: `events/network/ftp.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct FtpActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The list of return codes to the FTP command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codes: Option<String>,
    /// The FTP command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// The list of responses to the FTP command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_responses: Option<String>,
    /// The name of the data affiliated with the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The dynamic port established for impending data transfers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// The type of FTP network connection (e.g. active, passive).
    #[serde(alias = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

impl FtpActivity {
    pub const UID: u16 = 8;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of codes
    pub fn with_codes(self, codes: String) -> Self {
        Self {
            codes: Some(codes),
            ..self
        }
    }

    /// Set the value of command
    pub fn with_command(self, command: String) -> Self {
        Self {
            command: Some(command),
            ..self
        }
    }

    /// Set the value of command_responses
    pub fn with_command_responses(self, command_responses: String) -> Self {
        Self {
            command_responses: Some(command_responses),
            ..self
        }
    }

    /// Set the value of name
    pub fn with_name(self, name: String) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }

    /// Set the value of port
    pub fn with_port(self, port: String) -> Self {
        Self {
            port: Some(port),
            ..self
        }
    }

    /// Set the value of type_name
    pub fn with_type_name(self, type_name: String) -> Self {
        Self {
            type_name: Some(type_name),
            ..self
        }
    }

    pub fn new() -> Self {
        Self {
            activity_id: None,
            codes: None,
            command: None,
            command_responses: None,
            name: None,
            port: None,
            type_name: None,
        }
    }
}

/// Network Activity events report network connection and traffic activity.
///
/// Sourced from: `events/network/network.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct NetworkActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<crate::NetworkActivity>,
}

impl NetworkActivity {
    pub const UID: u16 = 4001;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: crate::NetworkActivity) -> Self {
        Self {
            activity_id: Some(activity_id),
        }
    }

    pub fn new() -> Self {
        Self { activity_id: None }
    }
}

/// Server Message Block (SMB) Protocol Activity events report client/server connections sharing resources within the network.
///
/// Sourced from: `events/network/smb.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct SmbActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_dialects: Option<String>,
    /// The command name (e.g. SMB2_COMMAND_CREATE, SMB1_COMMAND_WRITE_ANDX).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dce_rpc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    /// The file that is the target of the SMB activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Indicates how the file was opened (e.g. normal, delete on close).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_type: Option<String>,
    /// The server response in an SMB network connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_uid: Option<String>,
}

impl SmbActivity {
    pub const UID: u16 = 6;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of client_dialects
    pub fn with_client_dialects(self, client_dialects: String) -> Self {
        Self {
            client_dialects: Some(client_dialects),
            ..self
        }
    }

    /// Set the value of command
    pub fn with_command(self, command: String) -> Self {
        Self {
            command: Some(command),
            ..self
        }
    }

    /// Set the value of dce_rpc
    pub fn with_dce_rpc(self, dce_rpc: String) -> Self {
        Self {
            dce_rpc: Some(dce_rpc),
            ..self
        }
    }

    /// Set the value of dialect
    pub fn with_dialect(self, dialect: String) -> Self {
        Self {
            dialect: Some(dialect),
            ..self
        }
    }

    /// Set the value of file
    pub fn with_file(self, file: String) -> Self {
        Self {
            file: Some(file),
            ..self
        }
    }

    /// Set the value of open_type
    pub fn with_open_type(self, open_type: String) -> Self {
        Self {
            open_type: Some(open_type),
            ..self
        }
    }

    /// Set the value of response
    pub fn with_response(self, response: String) -> Self {
        Self {
            response: Some(response),
            ..self
        }
    }

    /// Set the value of share
    pub fn with_share(self, share: String) -> Self {
        Self {
            share: Some(share),
            ..self
        }
    }

    /// Set the value of share_type
    pub fn with_share_type(self, share_type: String) -> Self {
        Self {
            share_type: Some(share_type),
            ..self
        }
    }

    /// Set the value of tree_uid
    pub fn with_tree_uid(self, tree_uid: String) -> Self {
        Self {
            tree_uid: Some(tree_uid),
            ..self
        }
    }

    pub fn new() -> Self {
        Self {
            activity_id: None,
            client_dialects: None,
            command: None,
            dce_rpc: None,
            dialect: None,
            file: None,
            open_type: None,
            response: None,
            share: None,
            share_type: None,
            tree_uid: None,
        }
    }
}

/// Email events report activities of emails.
///
/// Sourced from: `events/network/email.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct EmailActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    /// The attempt number for attempting to deliver the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    /// The direction of the email, as defined by the `direction_id` value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// The direction of the email relative to the scanning host or organization.
    /// ///Email scanned at an internet gateway might be characterized as inbound to the organization from the Internet, outbound from the organization to the Internet, or internal within the organization. Email scanned at a workstation might be characterized as inbound to, or outbound from the workstation.
    pub direction_id: String,
    /// The responder (server) receiving the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_auth: Option<String>,
    /// The value of the SMTP HELO or EHLO command sent by the initiator (client).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_hello: Option<String>,
    /// The initiator (client) sending the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
}

impl EmailActivity {
    pub const UID: u16 = 4009;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    /// Set the value of attempt
    pub fn with_attempt(self, attempt: String) -> Self {
        Self {
            attempt: Some(attempt),
            ..self
        }
    }

    /// Set the value of banner
    pub fn with_banner(self, banner: String) -> Self {
        Self {
            banner: Some(banner),
            ..self
        }
    }

    /// Set the value of direction
    pub fn with_direction(self, direction: String) -> Self {
        Self {
            direction: Some(direction),
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

    /// Set the value of email_auth
    pub fn with_email_auth(self, email_auth: String) -> Self {
        Self {
            email_auth: Some(email_auth),
            ..self
        }
    }

    /// Set the value of smtp_hello
    pub fn with_smtp_hello(self, smtp_hello: String) -> Self {
        Self {
            smtp_hello: Some(smtp_hello),
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

    pub fn new(direction_id: String, email: String) -> Self {
        Self {
            activity_id: None,
            attempt: None,
            banner: None,
            direction: None,
            direction_id,
            dst_endpoint: None,
            email,
            email_auth: None,
            smtp_hello: None,
            src_endpoint: None,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-03-29T11:20:42+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/e3cab7448ca160ea1e324c46692c5096b4c5b3fa>
