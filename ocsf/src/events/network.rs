// This file was automatically generated by ocsf-codegen at 2023-03-19T13:22:12+00:00

// kilroy was here ssh.json

// kilroy was here dhcp.json

// kilroy was here smb.json

// kilroy was here rdp.json

pub enum DnsActivity {
    Unknown,
    /// The DNS query request.
    Query,
    /// The DNS query response.
    Response,
    Other,
}

impl From<DnsActivity> for u8 {
    fn from(input: DnsActivity) -> u8 {
        match input {
            DnsActivity::Unknown => 0,
            DnsActivity::Query => 1,
            DnsActivity::Response => 2,
            DnsActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for DnsActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => DnsActivity::Unknown,
            1 => DnsActivity::Query,
            2 => DnsActivity::Response,
            99 => DnsActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

// kilroy was here dns.json

// kilroy was here http.json

pub enum NetworkActivity {
    Unknown,
    /// A new network connection was opened.
    Open,
    /// The network connection was closed.
    Close,
    /// The network connection was abnormally terminated or closed by a middle device like firewalls.
    Reset,
    /// The network connection failed. For example a connection timeout or no route to host.
    Fail,
    /// The network connection was refused. For example an attempt to connect to a server port which is not open.
    Refuse,
    /// Network traffic report.
    Traffic,
    Other,
}

impl From<NetworkActivity> for u8 {
    fn from(input: NetworkActivity) -> u8 {
        match input {
            NetworkActivity::Unknown => 0,
            NetworkActivity::Open => 1,
            NetworkActivity::Close => 2,
            NetworkActivity::Reset => 3,
            NetworkActivity::Fail => 4,
            NetworkActivity::Refuse => 5,
            NetworkActivity::Traffic => 6,
            NetworkActivity::Other => 99,
        }
    }
}

impl TryFrom<u8> for NetworkActivity {
    type Error = String;

    fn try_from(input: u8) -> Result<Self, String> {
        let res = match input {
            0 => NetworkActivity::Unknown,
            1 => NetworkActivity::Open,
            2 => NetworkActivity::Close,
            3 => NetworkActivity::Reset,
            4 => NetworkActivity::Fail,
            5 => NetworkActivity::Refuse,
            6 => NetworkActivity::Traffic,
            99 => NetworkActivity::Other,
            _ => return Err("invalid value".to_string()),
        };
        Ok(res)
    }
}

// kilroy was here network.json

// kilroy was here email.json

// kilroy was here ftp.json
