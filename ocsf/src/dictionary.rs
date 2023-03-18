//* "The Attribute Dictionary defines attributes and includes references to the events and objects in which they are used."
use serde::{Serialize, Deserialize};


#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DictAttribute {
    caption: Option<String>,
    default: Option<i32>,
    description: Option<String>,
    attr_enum: Option<String>,
    is_array: Option<bool>,
    sibling: Option<String>,
    attr_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DictType {
    caption: Option<String>,
    description: Option<String>,
    max_len: Option<String>,
    observable: Option<String>,
    range: Option<String>,
    regex: Option<String>,
    value_type: Option<String>,
    type_name: Option<String>,
    values: Option<String>,
}
const access_list: DictAttribute = DictAttribute {
    caption: Some(
        "Access List",
    ),
    default: None,
    description: Some(
        "The list of requested access rights.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const access_mask: DictAttribute = DictAttribute {
    caption: Some(
        "Access Mask",
    ),
    default: None,
    description: Some(
        "The access mask in a platform-native format.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const access_result: DictAttribute = DictAttribute {
    caption: Some(
        "Access Check Result",
    ),
    default: None,
    description: Some(
        "The list of access check results.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const accessed_time: DictAttribute = DictAttribute {
    caption: Some(
        "Accessed Time",
    ),
    default: None,
    description: Some(
        "The time when the file was last accessed.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const accessor: DictAttribute = DictAttribute {
    caption: Some(
        "Accessor",
    ),
    default: None,
    description: Some(
        "The name of the user who last accessed the object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const account_name: DictAttribute = DictAttribute {
    caption: Some(
        "Account Name",
    ),
    default: None,
    description: Some(
        "The name of the account (e.g. AWS Account Name).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const account_type: DictAttribute = DictAttribute {
    caption: Some(
        "Account Type",
    ),
    default: None,
    description: Some(
        "The user account type, normalized to the caption of 'account_type_id'. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const account_type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Account Type ID",
    ),
    default: None,
    description: Some(
        "The normalized user account type identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "account_type",
    ),
    attr_type: None,
};

const account_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Account UID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the account (e.g. AWS Account ID).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const activity_id: DictAttribute = DictAttribute {
    caption: Some(
        "Activity ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the activity that triggered the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "activity_name",
    ),
    attr_type: None,
};

const activity_name: DictAttribute = DictAttribute {
    caption: Some(
        "Activity",
    ),
    default: None,
    description: Some(
        "The event activity name, as defined by the activity_id.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const actor: DictAttribute = DictAttribute {
    caption: Some(
        "Actor",
    ),
    default: None,
    description: Some(
        "The actor object describes details about the user/role/process that was the source of the activity.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const actual_permissions: DictAttribute = DictAttribute {
    caption: Some(
        "Actual Permissions",
    ),
    default: None,
    description: Some(
        "The permissions that were granted to the in a platform-native format.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const alert: DictAttribute = DictAttribute {
    caption: Some(
        "Client TLS Alert",
    ),
    default: None,
    description: Some(
        "The integer value of TLS alert if present. The alerts are defined in the TLS specification in <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc2246'>RFC-2246</a>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const algorithm: DictAttribute = DictAttribute {
    caption: Some(
        "Algorithm",
    ),
    default: None,
    description: Some(
        "The hash algorithm used to create the digital fingerprint, normalized to the caption of 'algorithm_id'. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const algorithm_id: DictAttribute = DictAttribute {
    caption: Some(
        "Algorithm ID",
    ),
    default: None,
    description: Some(
        "The identifier of the normalized hash algorithm, which was used to create the digital fingerprint.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "algorithm",
    ),
    attr_type: None,
};

const answers: DictAttribute = DictAttribute {
    caption: Some(
        "DNS Answer",
    ),
    default: None,
    description: Some(
        "The Domain Name System (DNS) answers.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const api: DictAttribute = DictAttribute {
    caption: Some(
        "API details",
    ),
    default: None,
    description: Some(
        "API object details information pertaining to the API calls",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const app_name: DictAttribute = DictAttribute {
    caption: Some(
        "Application Name",
    ),
    default: None,
    description: Some(
        "The name of the application that is associated with the event or object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const architecture: DictAttribute = DictAttribute {
    caption: Some(
        "Architecture",
    ),
    default: None,
    description: Some(
        "Architecture is a shorthand name describing the type of computer hardware the packaged software is meant to run on.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const args: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Arguments",
    ),
    default: None,
    description: Some(
        "The arguments sent along with the HTTP request.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const attacks: DictAttribute = DictAttribute {
    caption: Some(
        "Attacks",
    ),
    default: None,
    description: Some(
        "An array of attacks associated with an event.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const attempt: DictAttribute = DictAttribute {
    caption: Some(
        "Attempt",
    ),
    default: None,
    description: Some(
        "The delivery attempt.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const attributes: DictAttribute = DictAttribute {
    caption: Some(
        "Attributes",
    ),
    default: None,
    description: Some(
        "The bitmask value that represents the file attributes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const auth_protocol: DictAttribute = DictAttribute {
    caption: Some(
        "Auth Protocol",
    ),
    default: None,
    description: Some(
        "The authentication protocol as defined by the caption of 'auth_protocol_id'. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const auth_protocol_id: DictAttribute = DictAttribute {
    caption: Some(
        "Auth Protocol ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the authentication protocol used to create the user session.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "auth_protocol",
    ),
    attr_type: None,
};

const auth_type: DictAttribute = DictAttribute {
    caption: Some(
        "Authentication Type",
    ),
    default: None,
    description: Some(
        "The agreed upon authentication type, normalized to the caption of 'auth_type_id'. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const auth_type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Authentication Type ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the agreed upon authentication type. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "auth_type",
    ),
    attr_type: None,
};

const authorizations: DictAttribute = DictAttribute {
    caption: Some(
        "Authorization Information",
    ),
    default: None,
    description: Some(
        "This object provides details such as authorization outcome, associated policies related to activity/event.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const autoscale_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Autoscale UID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the cloud autoscale configuration.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const banner: DictAttribute = DictAttribute {
    caption: Some(
        "SMTP Banner",
    ),
    default: None,
    description: Some(
        "The initial SMTP connection response that a messaging server receives after it connects to a email server.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const base_address: DictAttribute = DictAttribute {
    caption: Some(
        "Base Address",
    ),
    default: None,
    description: Some(
        "The memory address where the module was loaded.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const base_score: DictAttribute = DictAttribute {
    caption: Some(
        "Base Score",
    ),
    default: None,
    description: Some(
        "The base score as reported by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const big_endian: DictAttribute = DictAttribute {
    caption: Some(
        "Big Endian",
    ),
    default: None,
    description: Some(
        "A boolean indicating whether the most significant byte is stored/transmitted first.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bios_date: DictAttribute = DictAttribute {
    caption: Some(
        "BIOS Date",
    ),
    default: None,
    description: Some(
        "The BIOS date. For example: <code>03/31/16</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bios_manufacturer: DictAttribute = DictAttribute {
    caption: Some(
        "BIOS Manufacturer",
    ),
    default: None,
    description: Some(
        "The BIOS manufacturer. For example: <code>LENOVO</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bios_ver: DictAttribute = DictAttribute {
    caption: Some(
        "BIOS Version",
    ),
    default: None,
    description: Some(
        "The BIOS version. For example: <code>LENOVO G5ETA2WW (2.62)</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const boundary: DictAttribute = DictAttribute {
    caption: Some(
        "Boundary",
    ),
    default: None,
    description: Some(
        "The boundary of the connection, normalized to the caption of 'boundary_id'. In the case of 'Other', it is defined by the event source. <p> For cloud connections, this translates to the traffic-boundary(same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const boundary_id: DictAttribute = DictAttribute {
    caption: Some(
        "Boundary ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the boundary of the connection. </p> For cloud connections, this translates to the traffic-boundary (same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "boundary",
    ),
    attr_type: None,
};

const build: DictAttribute = DictAttribute {
    caption: Some(
        "OS Build",
    ),
    default: None,
    description: Some(
        "The operating system build number.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bytes: DictAttribute = DictAttribute {
    caption: Some(
        "Total Bytes",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The total number of bytes (in and out).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bytes_in: DictAttribute = DictAttribute {
    caption: Some(
        "Bytes In",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The number of bytes sent from the destination to the source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const bytes_out: DictAttribute = DictAttribute {
    caption: Some(
        "Bytes Out",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The number of bytes sent from the source to the destination.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const capabilities: DictAttribute = DictAttribute {
    caption: Some(
        "Capabilities",
    ),
    default: None,
    description: Some(
        "A list of RDP capabilities.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const caption: DictAttribute = DictAttribute {
    caption: Some(
        "Caption",
    ),
    default: None,
    description: Some(
        "A short description or caption of the device. For example: <code>Scanner 1</code> or <code>Database Manager</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const categories: DictAttribute = DictAttribute {
    caption: Some(
        "Website Categorization",
    ),
    default: None,
    description: Some(
        "The Website categorization names, as defined by <code>category_ids</code> enum values.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const category: DictAttribute = DictAttribute {
    caption: Some(
        "Category",
    ),
    default: None,
    description: Some(
        "The object category. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const category_ids: DictAttribute = DictAttribute {
    caption: Some(
        "Website Categorization IDs",
    ),
    default: None,
    description: Some(
        "The Website categorization identifies.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: Some(
        "categories",
    ),
    attr_type: None,
};

const category_name: DictAttribute = DictAttribute {
    caption: Some(
        "Category",
    ),
    default: None,
    description: Some(
        "The event category name, as defined by category_uid value.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const category_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Category ID",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The category unique identifier of the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "category_name",
    ),
    attr_type: None,
};

const cc: DictAttribute = DictAttribute {
    caption: Some(
        "Cc",
    ),
    default: None,
    description: Some(
        "The email header Cc values, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const certificate: DictAttribute = DictAttribute {
    caption: Some(
        "Certificate",
    ),
    default: None,
    description: Some(
        "The certificate object containing information about the digital certificate.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const certificate_chain: DictAttribute = DictAttribute {
    caption: Some(
        "Certificate Chain",
    ),
    default: None,
    description: Some(
        "The Chain of Certificate Serial Numbers field provides a chain of Certificate Issuer Serial Numbers leading to the Root Certificate Issuer.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const challenge: DictAttribute = DictAttribute {
    caption: Some(
        "Challenge",
    ),
    default: None,
    description: Some(
        "The VNC challenge",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const challenge_response: DictAttribute = DictAttribute {
    caption: Some(
        "Challenge Response",
    ),
    default: None,
    description: Some(
        "The VNC challenge response",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const chassis: DictAttribute = DictAttribute {
    caption: Some(
        "Chassis",
    ),
    default: None,
    description: Some(
        "The chassis type describes the system enclosure or physical form factor. Such as the following examples for Windows <a target='_blank' href='https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure'>Windows Chassis Types</a>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cipher: DictAttribute = DictAttribute {
    caption: Some(
        "Cipher Suite",
    ),
    default: None,
    description: Some(
        "The negotiated cipher suite.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cis_benchmark_result: DictAttribute = DictAttribute {
    caption: Some(
        "CIS Benchmark Result",
    ),
    default: None,
    description: Some(
        "The CIS benchmark result.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const city: DictAttribute = DictAttribute {
    caption: Some(
        "City",
    ),
    default: None,
    description: Some(
        "The name of the city.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const class: DictAttribute = DictAttribute {
    caption: Some(
        "Class",
    ),
    default: None,
    description: Some(
        "The class name of the object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const class_name: DictAttribute = DictAttribute {
    caption: Some(
        "Class",
    ),
    default: None,
    description: Some(
        "The event class name, as defined by class_uid value.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const class_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Class ID",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The unique identifier of a class. A Class describes the attributes available in an event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "class_name",
    ),
    attr_type: None,
};

const classification_ids: DictAttribute = DictAttribute {
    caption: Some(
        "Classification IDs",
    ),
    default: None,
    description: Some(
        "The list of normalzied identifiers of the malware classifications. Reference: <a target='_blank' href='https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_oxlc4df65spl'>STIX Malware Types</a> ",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: Some(
        "classifications",
    ),
    attr_type: None,
};

const classifications: DictAttribute = DictAttribute {
    caption: Some(
        "Classifications",
    ),
    default: None,
    description: Some(
        "The list of malware classifications, normalized to the captions of the classifcation_id values. In the case of 'Other', they are defined by the event source.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const client_ciphers: DictAttribute = DictAttribute {
    caption: Some(
        "Client Cipher Suites",
    ),
    default: None,
    description: Some(
        "The client cipher suites that were exchanged during the TLS handshake negotiation.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const client_dialects: DictAttribute = DictAttribute {
    caption: Some(
        "Client Dialects",
    ),
    default: None,
    description: Some(
        "The list of SMB dialects that the client speaks.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const client_hassh: DictAttribute = DictAttribute {
    caption: Some(
        "Client HASSH",
    ),
    default: None,
    description: Some(
        "The Client HASSH fingerprinting object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cloud: DictAttribute = DictAttribute {
    caption: Some(
        "Cloud",
    ),
    default: None,
    description: Some(
        "Describes details about the Cloud enviroment where the event was originally created or logged.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cloud_partition: DictAttribute = DictAttribute {
    caption: Some(
        "Cloud Partition",
    ),
    default: None,
    description: Some(
        "The canonical cloud partition name to which the region is assigned (e.g. AWS Partitions: aws, aws-cn, aws-us-gov).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cmd_line: DictAttribute = DictAttribute {
    caption: Some(
        "Command Line",
    ),
    default: None,
    description: Some(
        "The full command line used to launch an application, service, process, or job. For example: <code>ssh user@10.0.0.10</code>. If the command line is unavailable or missing, the empty string <code>''</code> is to be used",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const code: DictAttribute = DictAttribute {
    caption: Some(
        "Response Code",
    ),
    default: None,
    description: Some(
        "The numeric response sent to a request.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const codes: DictAttribute = DictAttribute {
    caption: Some(
        "Response Codes",
    ),
    default: None,
    description: Some(
        "The list of numeric responses sent to a request.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const color_depth: DictAttribute = DictAttribute {
    caption: Some(
        "Color Depth",
    ),
    default: None,
    description: Some(
        "The numeric color depth.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const color_max: DictAttribute = DictAttribute {
    caption: Some(
        "Color Maximum",
    ),
    default: None,
    description: Some(
        "The maximum color value (with 'n' bits this would result in a 2^n-1 maximum value).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const color_shift: DictAttribute = DictAttribute {
    caption: Some(
        "Color Shift",
    ),
    default: None,
    description: Some(
        "The color shift value represents the number of shifts needed in order to get the color value in a pixel to the least significant bit.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const command: DictAttribute = DictAttribute {
    caption: Some(
        "Command",
    ),
    default: None,
    description: Some(
        "The command name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const command_response: DictAttribute = DictAttribute {
    caption: Some(
        "Command Response",
    ),
    default: None,
    description: Some(
        "The response to the command.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const command_responses: DictAttribute = DictAttribute {
    caption: Some(
        "Command Responses",
    ),
    default: None,
    description: Some(
        "The responses to the command.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const comment: DictAttribute = DictAttribute {
    caption: Some(
        "Comment",
    ),
    default: None,
    description: Some(
        "The user-provided comment.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const company_name: DictAttribute = DictAttribute {
    caption: Some(
        "Company Name",
    ),
    default: None,
    description: Some(
        "The name of the company that published the file. For example: <code>Microsoft Corporation</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const compliance: DictAttribute = DictAttribute {
    caption: Some(
        "Compliance",
    ),
    default: None,
    description: Some(
        "The complaince object provides context to compliance findings (e.g., a check against a specific regulatory or best practice framework such as CIS or NIST) and contains compliance related details.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const component: DictAttribute = DictAttribute {
    caption: Some(
        "Component",
    ),
    default: None,
    description: Some(
        "The name or relative pathname of a sub-component of the data object, if applicable. </p>For example: <code>attachment.doc</code>, <code>attachment.zip/bad.doc</code>, or <code>part.mime/part.cab/part.uue/part.doc</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const confidence: DictAttribute = DictAttribute {
    caption: Some(
        "Confidence",
    ),
    default: None,
    description: Some(
        "The confidence of the reported event severity as a percentage: 0%-100%.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const confidentiality: DictAttribute = DictAttribute {
    caption: Some(
        "Confidentiality",
    ),
    default: None,
    description: Some(
        "The file content confidentiality, normalized to the confidentiality_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const confidentiality_id: DictAttribute = DictAttribute {
    caption: Some(
        "Confidentiality ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the file content confidentiality indicator.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "confidentiality",
    ),
    attr_type: None,
};

const connection_info: DictAttribute = DictAttribute {
    caption: Some(
        "Connection Info",
    ),
    default: None,
    description: Some(
        "The network connection information.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const connection_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Connection Identifier",
    ),
    default: None,
    description: Some(
        "The network connection identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const container: DictAttribute = DictAttribute {
    caption: Some(
        "Container",
    ),
    default: None,
    description: Some(
        "The container information.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const content_type: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Content Type",
    ),
    default: None,
    description: Some(
        "The request header that identifies the original <a target='_blank' href='https://www.iana.org/assignments/media-types/media-types.xhtml'>media type </a> of the resource (prior to any content encoding applied for sending).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const continent: DictAttribute = DictAttribute {
    caption: Some(
        "Continent",
    ),
    default: None,
    description: Some(
        "The name of the continent.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const coordinates: DictAttribute = DictAttribute {
    caption: Some(
        "Coordinates",
    ),
    default: None,
    description: Some(
        "A two-element array, containing a longitude/latitude pair. The format conforms with <a target='_blank' href='https://geojson.org'>GeoJSON</a>. For example: <code>[-73.983, 40.719]</code>.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const correlation_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Correlation UID",
    ),
    default: None,
    description: Some(
        "The unique identifier used to correlate events.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const count: DictAttribute = DictAttribute {
    caption: Some(
        "Count",
    ),
    default: Some(
        1,
    ),
    description: Some(
        "The number of times that events in the same logical group occurred during the event <strong>Start Time</strong> to <strong>End Time</strong> period.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const country: DictAttribute = DictAttribute {
    caption: Some(
        "Country",
    ),
    default: None,
    description: Some(
        "The ISO 3166-1 Alpha-2 country code. For the complete list of country codes see <a target='_blank' href='https://www.iso.org/obp/ui/#iso:pub:PUB500001:en' >ISO 3166-1 alpha-2 codes</a>.<p><b>Note:</b> The two letter country code should be capitalized. For example: <code>US</code> or <code>CA</code>.</p>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cpu_bits: DictAttribute = DictAttribute {
    caption: Some(
        "CPU Bits",
    ),
    default: None,
    description: Some(
        "The cpu architecture, the number of bits used for addressing in memory. For example: <code>32</code> or <code>64</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cpu_cores: DictAttribute = DictAttribute {
    caption: Some(
        "CPU Cores",
    ),
    default: None,
    description: Some(
        "The number of processor cores in all installed processors. For Example: <code>42</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cpu_count: DictAttribute = DictAttribute {
    caption: Some(
        "CPU Count",
    ),
    default: None,
    description: Some(
        "The number of physical processors on a system. For example: <code>1</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cpu_speed: DictAttribute = DictAttribute {
    caption: Some(
        "Processor Type",
    ),
    default: None,
    description: Some(
        "The speed of the processor in Mhz. For Example: <code>4200</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cpu_type: DictAttribute = DictAttribute {
    caption: Some(
        "Processor Type",
    ),
    default: None,
    description: Some(
        "The processor type. For example: <code>x86 Family 6 Model 37 Stepping 5</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const create_mask: DictAttribute = DictAttribute {
    caption: Some(
        "Create Mask",
    ),
    default: None,
    description: Some(
        "The original Windows mask that is required to create the object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const created_time: DictAttribute = DictAttribute {
    caption: Some(
        "Created Time",
    ),
    default: None,
    description: Some(
        "The time when the object was created. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const creator: DictAttribute = DictAttribute {
    caption: Some(
        "Creator",
    ),
    default: None,
    description: Some(
        "The user that created the object associated with event. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const credential_uid: DictAttribute = DictAttribute {
    caption: Some(
        "User Credential ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the user's credential. For example, AWS Access Key ID.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const criticality: DictAttribute = DictAttribute {
    caption: Some(
        "Criticality",
    ),
    default: None,
    description: Some(
        "Criticality of a resource/object in question",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const customer_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Customer UID",
    ),
    default: None,
    description: Some(
        "The unique customer identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cve: DictAttribute = DictAttribute {
    caption: Some(
        "CVE",
    ),
    default: None,
    description: Some(
        "The Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cves: DictAttribute = DictAttribute {
    caption: Some(
        "CVE List",
    ),
    default: None,
    description: Some(
        "List of Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const cvss: DictAttribute = DictAttribute {
    caption: Some(
        "CVSS Score",
    ),
    default: None,
    description: Some(
        "The CVSS object details Common Vulnerability Scoring System (<a target='_blank' href='https://www.first.org/cvss/'>CVSS</a>) scores from the advisory that are related to the vulnerability.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cwe_uid: DictAttribute = DictAttribute {
    caption: Some(
        "CWE UID",
    ),
    default: None,
    description: Some(
        "The <a target='_blank' href='https://cwe.mitre.org/'>Common Weakness Enumeration (CWE)</a> unique identifier. For example: <code>CWE-787</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const cwe_url: DictAttribute = DictAttribute {
    caption: Some(
        "CWE URL",
    ),
    default: None,
    description: Some(
        "Common Weakness Enumiration (CWE) definition URL. For example: <code>https://cwe.mitre.org/data/definitions/787.html</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const data: DictAttribute = DictAttribute {
    caption: Some(
        "Data",
    ),
    default: None,
    description: Some(
        "The additional data that is associated with the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const data_bucket: DictAttribute = DictAttribute {
    caption: Some(
        "Data Bucket",
    ),
    default: None,
    description: Some(
        "The name of the data bucket (e.g. bucket name for AWS/GCP and blob name for Azure).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dce_rpc: DictAttribute = DictAttribute {
    caption: Some(
        "Distributed Computing Environment/Remote Procedure Call (DCE/RPC)",
    ),
    default: None,
    description: Some(
        "The DCE/RPC object describes the remote procedure call system for distributed computing environments.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const decision: DictAttribute = DictAttribute {
    caption: Some(
        "Authorization Decision/Outcome",
    ),
    default: None,
    description: Some(
        "Decision/outcome of the authorization mechanism (e.g. Approved, Denied)",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const delivered_to: DictAttribute = DictAttribute {
    caption: Some(
        "Delivered To",
    ),
    default: None,
    description: Some(
        "The <strong>Delivered-To</strong> email header field.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const depth: DictAttribute = DictAttribute {
    caption: Some(
        "CVSS Depth",
    ),
    default: None,
    description: Some(
        "The CVSS depth represents a depth of the equation used to calculate CVSS score.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const desc: DictAttribute = DictAttribute {
    caption: Some(
        "Description",
    ),
    default: None,
    description: Some(
        "The description that pertains to the object or event. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const desktop_display: DictAttribute = DictAttribute {
    caption: Some(
        "Desktop Display",
    ),
    default: None,
    description: Some(
        "The desktop display affiliated with the event",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const details: DictAttribute = DictAttribute {
    caption: Some(
        "Details",
    ),
    default: None,
    description: Some(
        "Details of an entity. See specific usage",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const detection_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Detection UID",
    ),
    default: None,
    description: Some(
        "The associated unique detection event identifier. For example: detection response events include the <b>Detection ID</b> of the original event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const developer_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Developer UID",
    ),
    default: None,
    description: Some(
        "The developer ID on the certificate that signed the file.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const device: DictAttribute = DictAttribute {
    caption: Some(
        "Device",
    ),
    default: None,
    description: Some(
        "The device that reported the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const devices: DictAttribute = DictAttribute {
    caption: Some(
        "Devices",
    ),
    default: None,
    description: Some(
        "The object describes details related to the list of devices.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const dialect: DictAttribute = DictAttribute {
    caption: Some(
        "Dialect",
    ),
    default: None,
    description: Some(
        "The negotiated protocol dialect.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const direction: DictAttribute = DictAttribute {
    caption: Some(
        "Direction",
    ),
    default: None,
    description: Some(
        "The direction of the initiated connection, traffic, or email, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const direction_id: DictAttribute = DictAttribute {
    caption: Some(
        "Direction ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the direction of the initiated connection, traffic, or email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "direction",
    ),
    attr_type: None,
};

const disposition: DictAttribute = DictAttribute {
    caption: Some(
        "Disposition",
    ),
    default: None,
    description: Some(
        "The event disposition name, normalized to the caption of the disposition_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const disposition_id: DictAttribute = DictAttribute {
    caption: Some(
        "Disposition ID",
    ),
    default: None,
    description: Some(
        "When security issues, such as malware or policy violations, are detected and possibly corrected, then <code>disposition_id</code> describes the action taken by the security product.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "disposition",
    ),
    attr_type: None,
};

const dkim: DictAttribute = DictAttribute {
    caption: Some(
        "DKIM Status",
    ),
    default: None,
    description: Some(
        "The DomainKeys Identified Mail (DKIM) status of the email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dkim_domain: DictAttribute = DictAttribute {
    caption: Some(
        "DKIM Domain",
    ),
    default: None,
    description: Some(
        "The DomainKeys Identified Mail (DKIM) signing domain of the email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dkim_signature: DictAttribute = DictAttribute {
    caption: Some(
        "DKIM Signature",
    ),
    default: None,
    description: Some(
        "The DomainKeys Identified Mail (DKIM) signature used by the sending/receiving system.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dmarc: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Status",
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) status of the email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dmarc_override: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Override",
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) override action.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dmarc_policy: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Policy",
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) policy status.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const domain: DictAttribute = DictAttribute {
    caption: Some(
        "Domain",
    ),
    default: None,
    description: Some(
        "The name of the domain.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const domain_info: DictAttribute = DictAttribute {
    caption: Some(
        "Domain Information",
    ),
    default: None,
    description: Some(
        "The registration information pertaining to a domain.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const driver: DictAttribute = DictAttribute {
    caption: Some(
        "Kernel Driver",
    ),
    default: None,
    description: Some(
        "The driver that was loaded/unloaded into the kernel",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const dst_endpoint: DictAttribute = DictAttribute {
    caption: Some(
        "Destination Endpoint",
    ),
    default: None,
    description: Some(
        "The network destination endpoint.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const duration: DictAttribute = DictAttribute {
    caption: Some(
        "Duration",
    ),
    default: None,
    description: Some(
        "The event duration or aggregate time, the amount of time the event covers from <code>start_time</code> to <code>end_time</code> in milliseconds.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const edition: DictAttribute = DictAttribute {
    caption: Some(
        "OS Edition",
    ),
    default: None,
    description: Some(
        "The operating system edition. For example: <code>Professional</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const email: DictAttribute = DictAttribute {
    caption: Some(
        "Email",
    ),
    default: None,
    description: Some(
        "The email object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const email_addr: DictAttribute = DictAttribute {
    caption: Some(
        "Email Address",
    ),
    default: None,
    description: Some(
        "The user's email address.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const email_auth: DictAttribute = DictAttribute {
    caption: Some(
        "Email Authentication",
    ),
    default: None,
    description: Some(
        "The SPF, DKIM and DMARC attributes of an email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const end_time: DictAttribute = DictAttribute {
    caption: Some(
        "End Time",
    ),
    default: None,
    description: Some(
        "The end time of a time period. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const enrichments: DictAttribute = DictAttribute {
    caption: Some(
        "Enrichments",
    ),
    default: None,
    description: Some(
        "The additional information from an external data source, which is associated with the event. For example add location information for the IP address in the DNS answers:</p><code>[{\"name\": \"answers.ip\", \"value\": \"92.24.47.250\", \"type\": \"location\", \"data\": {\"city\": \"Socotra\", \"continent\": \"Asia\", \"coordinates\": [-25.4153, 17.0743], \"country\": \"YE\", \"desc\": \"Yemen\"}}]</code>",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const entity: DictAttribute = DictAttribute {
    caption: Some(
        "Entity",
    ),
    default: None,
    description: Some(
        "The managed entity that is being acted upon.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const entity_result: DictAttribute = DictAttribute {
    caption: Some(
        "Entity Result",
    ),
    default: None,
    description: Some(
        "The updated managed entity.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const epoch: DictAttribute = DictAttribute {
    caption: Some(
        "Epoch",
    ),
    default: None,
    description: Some(
        "The software package epoch. Epoch is a way to define weighted dependencies based on version numbers.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const error: DictAttribute = DictAttribute {
    caption: Some(
        "Error Code",
    ),
    default: None,
    description: Some(
        "Error Code",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const error_message: DictAttribute = DictAttribute {
    caption: Some(
        "Error Message",
    ),
    default: None,
    description: Some(
        "Error Message",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const exit_code: DictAttribute = DictAttribute {
    caption: Some(
        "Exit Code",
    ),
    default: None,
    description: Some(
        "The exit code reported by a process when it terminates. The convention is that zero indicates success and any non-zero exit code indicates that some error occurred.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const expiration_time: DictAttribute = DictAttribute {
    caption: Some(
        "Expiration Time",
    ),
    default: None,
    description: Some(
        "The expiration time. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const exposed_port: DictAttribute = DictAttribute {
    caption: Some(
        "Port",
    ),
    default: None,
    description: Some(
        "The IP port number exposed by container. For example 0.0.0.0:49155-> <<exposed_port>>/tcp",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const extension_list: DictAttribute = DictAttribute {
    caption: Some(
        "Extension List",
    ),
    default: None,
    description: Some(
        "The list of TLS extensions",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const feature: DictAttribute = DictAttribute {
    caption: Some(
        "Feature",
    ),
    default: None,
    description: Some(
        "The feature that reported the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const file: DictAttribute = DictAttribute {
    caption: Some(
        "File",
    ),
    default: None,
    description: Some(
        "The file that pertains to the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const file_diff: DictAttribute = DictAttribute {
    caption: Some(
        "File Diff",
    ),
    default: None,
    description: Some(
        "File content differences used for change detection. For example, a common use case is to identify itemized changes within INI or configuration/property setting values.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const file_result: DictAttribute = DictAttribute {
    caption: Some(
        "File Result",
    ),
    default: None,
    description: Some(
        "The result of the file change. It should contain the new values of the changed attributes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const finding: DictAttribute = DictAttribute {
    caption: Some(
        "Finding",
    ),
    default: None,
    description: Some(
        "Finding object provides details related to a finding generated by security tool",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const fingerprint: DictAttribute = DictAttribute {
    caption: Some(
        "Fingerprint",
    ),
    default: None,
    description: Some(
        "The digital fingerprint associated with an object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const fingerprints: DictAttribute = DictAttribute {
    caption: Some(
        "Fingerprints",
    ),
    default: None,
    description: Some(
        "An array of digital fingerprint objects.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const first_seen_time: DictAttribute = DictAttribute {
    caption: Some(
        "First Seen",
    ),
    default: None,
    description: Some(
        "The initial detection time of the activity or object. See specific usage",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const flag_ids: DictAttribute = DictAttribute {
    caption: Some(
        "Communication Flag IDs",
    ),
    default: None,
    description: Some(
        "The list of normalized identifiers of the communication flag IDs.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: Some(
        "flags",
    ),
    attr_type: None,
};

const flags: DictAttribute = DictAttribute {
    caption: Some(
        "Flags",
    ),
    default: None,
    description: Some(
        "The list of communication flags, normalized to the captions of the flag_ids values. In the case of 'Other', they are defined by the event source.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const folder: DictAttribute = DictAttribute {
    caption: Some(
        "Folder",
    ),
    default: None,
    description: Some(
        "The folder that pertains to the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const from: DictAttribute = DictAttribute {
    caption: Some(
        "From",
    ),
    default: None,
    description: Some(
        "The email header From values, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const function_keys: DictAttribute = DictAttribute {
    caption: Some(
        "Function Keys",
    ),
    default: None,
    description: Some(
        "The number of function keys on client keyboard.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const function_name: DictAttribute = DictAttribute {
    caption: Some(
        "Function Name",
    ),
    default: None,
    description: Some(
        "The entry-point function of the module. The system calls the entry-point function whenever a process or thread loads or unloads the module.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const group: DictAttribute = DictAttribute {
    caption: Some(
        "Group",
    ),
    default: None,
    description: Some(
        "The group object associated with an entity such as user, policy, or rule.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const group_name: DictAttribute = DictAttribute {
    caption: Some(
        "Group Name",
    ),
    default: None,
    description: Some(
        "The name of the group that the resource belongs to.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const groups: DictAttribute = DictAttribute {
    caption: Some(
        "Groups",
    ),
    default: None,
    description: Some(
        "The groups to which an entity belongs. See specific usage.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const handshake_dur: DictAttribute = DictAttribute {
    caption: Some(
        "Handshake Duration",
    ),
    default: None,
    description: Some(
        "The amount of total time for the TLS handshake to complete after the TCP connection is established, including client-side delays, in milliseconds.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const hostname: DictAttribute = DictAttribute {
    caption: Some(
        "Hostname",
    ),
    default: None,
    description: Some(
        "The hostname of an endpoint or a device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const http_cookies: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Cookies",
    ),
    default: None,
    description: Some(
        "The cookies object describes details about HTTP cookies",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const http_headers: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Headers",
    ),
    default: None,
    description: Some(
        "Additional HTTP headers of an HTTP request or response.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const http_method: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Method",
    ),
    default: None,
    description: Some(
        "The HTTP request method indicates the desired action to be performed for a given resource. Expected values: <ul> <li>TRACE</li> <li>CONNECT</li> <li>OPTIONS</li> <li>HEAD</li> <li>DELETE</li> <li>POST</li> <li>PUT</li> <li>GET</li></ul>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const http_only: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Only",
    ),
    default: None,
    description: Some(
        "A cookie attribute to make it inaccessible via JavaScript",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const http_request: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Request",
    ),
    default: None,
    description: Some(
        "The HTTP Request made to a web server.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const http_response: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Response",
    ),
    default: None,
    description: Some(
        "The HTTP Response from a web server to a requester.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const http_status: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Status",
    ),
    default: None,
    description: Some(
        "The Hypertext Transfer Protocol (HTTP) <a target='_blank' href='https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml'>status code</a> returned to the client.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const hw_info: DictAttribute = DictAttribute {
    caption: Some(
        "Hardware Info",
    ),
    default: None,
    description: Some(
        "The device hardware information.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const hypervisor: DictAttribute = DictAttribute {
    caption: Some(
        "Hypervisor",
    ),
    default: None,
    description: Some(
        "The name of the hypervisor running on the device. For example, <code>Xen</code>, <code>VMware</code>, <code>Hyper-V</code>, <code>VirtualBox</code>, etc.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const identifier_cookie: DictAttribute = DictAttribute {
    caption: Some(
        "Identifier Cookie",
    ),
    default: None,
    description: Some(
        "The client identifier cookie during client/server exchange.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const idp: DictAttribute = DictAttribute {
    caption: Some(
        "Identity Provider",
    ),
    default: None,
    description: Some(
        "This object describes details about the Identity Provider used.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const image: DictAttribute = DictAttribute {
    caption: Some(
        "Image",
    ),
    default: None,
    description: Some(
        "The image used as a template to run a container or virtual machine.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ime: DictAttribute = DictAttribute {
    caption: Some(
        "IME",
    ),
    default: None,
    description: Some(
        "The Input Method Editor (IME) file name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const imei: DictAttribute = DictAttribute {
    caption: Some(
        "IMEI",
    ),
    default: None,
    description: Some(
        "The International Mobile Station Equipment Identifier that is associated with the device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const injection_type: DictAttribute = DictAttribute {
    caption: Some(
        "Injection Type",
    ),
    default: None,
    description: Some(
        "The process injection method, normalized to the caption of the injection_type_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const injection_type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Injection Type ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the process injection method.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "injection_type",
    ),
    attr_type: None,
};

const instance_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Instance ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of a VM instance.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const integrity: DictAttribute = DictAttribute {
    caption: Some(
        "Integrity",
    ),
    default: None,
    description: Some(
        "The process integrity level, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source (Windows only).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const integrity_id: DictAttribute = DictAttribute {
    caption: Some(
        "Integrity Level",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the process integrity level (Windows only).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "integrity",
    ),
    attr_type: None,
};

const interface_name: DictAttribute = DictAttribute {
    caption: Some(
        "Network Interface Name",
    ),
    default: None,
    description: Some(
        "The name of the network interface (e.g. eth2).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const interface_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Network Interface ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the network interface.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const intermediate_ips: DictAttribute = DictAttribute {
    caption: Some(
        "Intermediate IP Addresses",
    ),
    default: None,
    description: Some(
        "The intermediate IP Addresses. For example, the IP addresses in the HTTP X-Forwarded-For header.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const invoked_by: DictAttribute = DictAttribute {
    caption: Some(
        "Invoked by",
    ),
    default: None,
    description: Some(
        "The name of the service that invoked the activity as described in the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ip: DictAttribute = DictAttribute {
    caption: Some(
        "IP Address",
    ),
    default: None,
    description: Some(
        "The IP address, in either IPv4 or IPv6 format.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_cleartext: DictAttribute = DictAttribute {
    caption: Some(
        "Cleartext Credentials",
    ),
    default: None,
    description: Some(
        "Indicates whether the credentials were passed in clear text.<p><b>Note:</b> True if the credentials were passed in a clear text protocol such as FTP or TELNET, or if Windows detected that a user's logon password was passed to the authentication package in clear text.</p>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_compliant: DictAttribute = DictAttribute {
    caption: Some(
        "Compliant Device",
    ),
    default: None,
    description: Some(
        "The event occurred on a compliant device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_default: DictAttribute = DictAttribute {
    caption: Some(
        "Default Value",
    ),
    default: None,
    description: Some(
        "The indication of whether the value is from a default value name. For example, the value name could be missing.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_managed: DictAttribute = DictAttribute {
    caption: Some(
        "Managed Device",
    ),
    default: None,
    description: Some(
        "The event occurred on a managed device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_on_premises: DictAttribute = DictAttribute {
    caption: Some(
        "On Premises",
    ),
    default: None,
    description: Some(
        "The indication of whether the location is on premises.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_personal: DictAttribute = DictAttribute {
    caption: Some(
        "Personal Device",
    ),
    default: None,
    description: Some(
        "The event occurred on a personal device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_remote: DictAttribute = DictAttribute {
    caption: Some(
        "Remote",
    ),
    default: None,
    description: Some(
        "The indication of whether the session is remote.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_renewal: DictAttribute = DictAttribute {
    caption: Some(
        "Renewal",
    ),
    default: None,
    description: Some(
        "The indication of whether this is a lease/session renewal event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_system: DictAttribute = DictAttribute {
    caption: Some(
        "System",
    ),
    default: None,
    description: Some(
        "The indication of whether the object is part of the operating system.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const is_trusted: DictAttribute = DictAttribute {
    caption: Some(
        "Trusted Device",
    ),
    default: None,
    description: Some(
        "The event occurred on a trusted device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const isp: DictAttribute = DictAttribute {
    caption: Some(
        "ISP",
    ),
    default: None,
    description: Some(
        "The name of the Internet Service Provider (ISP).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const issuer: DictAttribute = DictAttribute {
    caption: Some(
        "Issuer Details",
    ),
    default: None,
    description: Some(
        "The identifier of the session issuer.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const issuer_dn: DictAttribute = DictAttribute {
    caption: Some(
        "Issuer Distinguished Name",
    ),
    default: None,
    description: Some(
        "The certificate issuer distinguished name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const issuer_name: DictAttribute = DictAttribute {
    caption: Some(
        "Issuer Name",
    ),
    default: None,
    description: Some(
        "The certificate issuer name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ja3_fingerprint: DictAttribute = DictAttribute {
    caption: Some(
        "JA3 Fingerprint",
    ),
    default: None,
    description: Some(
        "The fingerprint of JA3 string.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ja3_string: DictAttribute = DictAttribute {
    caption: Some(
        "JA3 String",
    ),
    default: None,
    description: Some(
        "The JA3 string.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ja3s_fingerprint: DictAttribute = DictAttribute {
    caption: Some(
        "JAS3 Fingerprint",
    ),
    default: None,
    description: Some(
        "The fingerprint of JAS3 string.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ja3s_string: DictAttribute = DictAttribute {
    caption: Some(
        "JAS3 String",
    ),
    default: None,
    description: Some(
        "The JAS3 string.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const job: DictAttribute = DictAttribute {
    caption: Some(
        "Job",
    ),
    default: None,
    description: Some(
        "The job object that pertains to the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const kb_articles: DictAttribute = DictAttribute {
    caption: Some(
        "Knowledgebase Articles",
    ),
    default: None,
    description: Some(
        "The KB article/s related to the entity",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const kernel: DictAttribute = DictAttribute {
    caption: Some(
        "Kernel",
    ),
    default: None,
    description: Some(
        "The kernel resource object that pertains to the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const key_length: DictAttribute = DictAttribute {
    caption: Some(
        "Key Length",
    ),
    default: None,
    description: Some(
        "The length of the encryption key.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const keyboard_info: DictAttribute = DictAttribute {
    caption: Some(
        "Keyboard Information",
    ),
    default: None,
    description: Some(
        "The keyboard detailed information.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const keyboard_layout: DictAttribute = DictAttribute {
    caption: Some(
        "Keyboard Layout",
    ),
    default: None,
    description: Some(
        "The keyboard locale identifier name (e.g., en-US).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const keyboard_subtype: DictAttribute = DictAttribute {
    caption: Some(
        "Keyboard Subtype",
    ),
    default: None,
    description: Some(
        "The keyboard numeric code.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const keyboard_type: DictAttribute = DictAttribute {
    caption: Some(
        "Keyboard Type",
    ),
    default: None,
    description: Some(
        "The keyboard type (e.g., xt, ico).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const labels: DictAttribute = DictAttribute {
    caption: Some(
        "Labels",
    ),
    default: None,
    description: Some(
        "The list of labels attached to an event, object, or attribute.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const lang: DictAttribute = DictAttribute {
    caption: Some(
        "Language",
    ),
    default: None,
    description: Some(
        "The two letter lower case language codes, as defined by <a target='_blank' href='https://en.wikipedia.org/wiki/ISO_639-1'>ISO 639-1</a>. For example: <code>en</code> (English), <code>de</code> (German), or <code>fr</code> (French).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const last_run_time: DictAttribute = DictAttribute {
    caption: Some(
        "Last Run",
    ),
    default: None,
    description: Some(
        "The last run time of application or service. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const last_seen_time: DictAttribute = DictAttribute {
    caption: Some(
        "Last Seen",
    ),
    default: None,
    description: Some(
        "The most recent detection time of the activity or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const latency: DictAttribute = DictAttribute {
    caption: Some(
        "Latency",
    ),
    default: None,
    description: Some(
        "TODO: The HTTP response latency. In seconds, milliseconds, etc.?",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const lease_dur: DictAttribute = DictAttribute {
    caption: Some(
        "Lease Duration",
    ),
    default: None,
    description: Some(
        "This represents the length of the DHCP lease in seconds. This is present in DHCP Ack events. (activity_id = 1)",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const length: DictAttribute = DictAttribute {
    caption: Some(
        "Response Length",
    ),
    default: None,
    description: Some(
        "The HTTP response length, in number of bytes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const license: DictAttribute = DictAttribute {
    caption: Some(
        "Software License",
    ),
    default: None,
    description: Some(
        "The name or identifier of the license applied on package or software. See <a target='_blank' href='https://spdx.org/licenses/'>SPDX License List</a>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const lineage: DictAttribute = DictAttribute {
    caption: Some(
        "Lineage",
    ),
    default: None,
    description: Some(
        "The lineage of the process, represented by a list of paths for each ancestor process. For example: <code>['/usr/sbin/sshd', '/usr/bin/bash', '/usr/bin/whoami']</code>",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const load_type: DictAttribute = DictAttribute {
    caption: Some(
        "Load Type",
    ),
    default: None,
    description: Some(
        "The load type, normalized to the caption of the load_type_id value. In the case of 'Other', it is defined by the event source. It describes how the module was loaded in memory.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const load_type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Load Type ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the load type. It identifies how the module was loaded in memory.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "load_type",
    ),
    attr_type: None,
};

const loaded_modules: DictAttribute = DictAttribute {
    caption: Some(
        "Loaded Modules",
    ),
    default: None,
    description: Some(
        "The list of loaded module names.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const location: DictAttribute = DictAttribute {
    caption: Some(
        "Detailed Geo Location",
    ),
    default: None,
    description: Some(
        "The detailed geographical location usually associated with an IP address.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const logged_time: DictAttribute = DictAttribute {
    caption: Some(
        "Logged Time",
    ),
    default: None,
    description: Some(
        "The time when the logging system collected and logged the event.</p>This attribute is distinct from the event time in that event time typically contain the time extracted from the original event. Most of the time, these two times will be different.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const logon_process: DictAttribute = DictAttribute {
    caption: Some(
        "Logon Process",
    ),
    default: None,
    description: Some(
        "The trusted process that validated the authentication credentials.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const logon_type: DictAttribute = DictAttribute {
    caption: Some(
        "Logon Type",
    ),
    default: None,
    description: Some(
        "The logon type, normalized to the caption of the logon_type_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const logon_type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Logon Type ID",
    ),
    default: None,
    description: Some(
        "The normalized logon type identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "logon_type",
    ),
    attr_type: None,
};

const mac: DictAttribute = DictAttribute {
    caption: Some(
        "MAC Address",
    ),
    default: None,
    description: Some(
        "The Media Access Control (MAC) address that is associated with the network interface.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const malware: DictAttribute = DictAttribute {
    caption: Some(
        "Malware",
    ),
    default: None,
    description: Some(
        "The list of malware identified by a finding.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const message: DictAttribute = DictAttribute {
    caption: Some(
        "Message",
    ),
    default: None,
    description: Some(
        "The description of the event, as defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const message_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Message UID",
    ),
    default: None,
    description: Some(
        "The email header Message-Id value, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const metadata: DictAttribute = DictAttribute {
    caption: Some(
        "Metadata",
    ),
    default: None,
    description: Some(
        "The metadata associated with the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const metrics: DictAttribute = DictAttribute {
    caption: Some(
        "Metrics",
    ),
    default: None,
    description: Some(
        "The general purpose metrics associated with the event. See specific usage.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const mfa: DictAttribute = DictAttribute {
    caption: Some(
        "Multi Factor Authentication",
    ),
    default: None,
    description: Some(
        "Indicates whether Multi Factor Authentication was used during authentication.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const mime_type: DictAttribute = DictAttribute {
    caption: Some(
        "MIME type",
    ),
    default: None,
    description: Some(
        "The Multipurpose Internet Mail Extensions (MIME) type of the file, if applicable.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const model: DictAttribute = DictAttribute {
    caption: Some(
        "Model",
    ),
    default: None,
    description: Some(
        "The peripheral device model.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const modified_time: DictAttribute = DictAttribute {
    caption: Some(
        "Modified Time",
    ),
    default: None,
    description: Some(
        "The time when the object was last modified. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const modifier: DictAttribute = DictAttribute {
    caption: Some(
        "Modifier",
    ),
    default: None,
    description: Some(
        "The user that last modified the object associated with the event. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const module: DictAttribute = DictAttribute {
    caption: Some(
        "Module",
    ),
    default: None,
    description: Some(
        "The module that pertains to the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const name: DictAttribute = DictAttribute {
    caption: Some(
        "Name",
    ),
    default: None,
    description: Some(
        "The name of the entity. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const namespace: DictAttribute = DictAttribute {
    caption: Some(
        "Namespace",
    ),
    default: None,
    description: Some(
        "The namespace is useful in merger or acquisition situations. For example, when similar entities exists that you need to keep separate.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const network_driver: DictAttribute = DictAttribute {
    caption: Some(
        "Network Driver",
    ),
    default: None,
    description: Some(
        "The network driver used by the container. For example, bridge, overlay, host, none, etc.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const network_interface: DictAttribute = DictAttribute {
    caption: Some(
        "Network Interface",
    ),
    default: None,
    description: Some(
        "The network interface that is associated with the device.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const network_interfaces: DictAttribute = DictAttribute {
    caption: Some(
        "Network Interfaces",
    ),
    default: None,
    description: Some(
        "The network interfaces that are associated with the device, one for each MAC address/IP address combination.<p><b>Note:</b> The first element of the array is the network information that pertains to the event.</p>",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const next_run_time: DictAttribute = DictAttribute {
    caption: Some(
        "Next Run",
    ),
    default: None,
    description: Some(
        "The next run time. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const observables: DictAttribute = DictAttribute {
    caption: Some(
        "Observables",
    ),
    default: None,
    description: Some(
        "The observables associated with the event.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const opcode: DictAttribute = DictAttribute {
    caption: Some(
        "DNS Opcode",
    ),
    default: None,
    description: Some(
        "The DNS opcode specifies the type of the query message.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const opcode_id: DictAttribute = DictAttribute {
    caption: Some(
        "DNS Opcode ID",
    ),
    default: None,
    description: Some(
        "The DNS opcode ID specifies the normalized query message type.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const open_mask: DictAttribute = DictAttribute {
    caption: Some(
        "Open Mask",
    ),
    default: None,
    description: Some(
        "The Windows options needed to open a registry key.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const open_type: DictAttribute = DictAttribute {
    caption: Some(
        "Open Type",
    ),
    default: None,
    description: Some(
        "The file open type.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const operation: DictAttribute = DictAttribute {
    caption: Some(
        "Operation",
    ),
    default: None,
    description: Some(
        "Verb/Operation associated with the request",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const opnum: DictAttribute = DictAttribute {
    caption: Some(
        "Opnum",
    ),
    default: None,
    description: Some(
        "An operation number used to identify a specific remote procedure call (RPC) method or a method in an interface.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const orchestrator: DictAttribute = DictAttribute {
    caption: Some(
        "Orchestrator",
    ),
    default: None,
    description: Some(
        "The orchestrator managing the container, such as ECS, EKS, K8s, OpenShift, None.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const org_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Org ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the organization to which the user belongs. For example, Active Directory or AWS Org ID.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const org_unit: DictAttribute = DictAttribute {
    caption: Some(
        "Org Unit",
    ),
    default: None,
    description: Some(
        "The name of the organization to which the user belongs.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const original_time: DictAttribute = DictAttribute {
    caption: Some(
        "Original Time",
    ),
    default: None,
    description: Some(
        "The original event time as reported by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const os: DictAttribute = DictAttribute {
    caption: Some(
        "OS",
    ),
    default: None,
    description: Some(
        "The device operation system.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const overall_score: DictAttribute = DictAttribute {
    caption: Some(
        "Overall Score",
    ),
    default: None,
    description: Some(
        "The overall score as reported by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const owner: DictAttribute = DictAttribute {
    caption: Some(
        "Owner",
    ),
    default: None,
    description: Some(
        "The user that owns the file/object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const packages: DictAttribute = DictAttribute {
    caption: Some(
        "Software Packages",
    ),
    default: None,
    description: Some(
        "List of vulnerable packages as identified by the security product",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const packet_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Packet UID",
    ),
    default: None,
    description: Some(
        "The packet identifier assigned by the protocol.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const packets: DictAttribute = DictAttribute {
    caption: Some(
        "Total Packets",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The total number of packets (in and out).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const packets_in: DictAttribute = DictAttribute {
    caption: Some(
        "Packets In",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The number of packets sent from the destination to the source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const packets_out: DictAttribute = DictAttribute {
    caption: Some(
        "Packets Out",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The number of packets sent from the source to the destination.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const parent_folder: DictAttribute = DictAttribute {
    caption: Some(
        "Parent Folder",
    ),
    default: None,
    description: Some(
        "The parent folder in which the file resides. For example: <code>c:\\windows\\system32</code>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const parent_process: DictAttribute = DictAttribute {
    caption: Some(
        "Parent Process",
    ),
    default: None,
    description: Some(
        "The parent process of this process object. It is recommended to only populate this field for the first process object, to prevent deep nesting.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const path: DictAttribute = DictAttribute {
    caption: Some(
        "Path",
    ),
    default: None,
    description: Some(
        "The path that pertains to the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const peripheral_device: DictAttribute = DictAttribute {
    caption: Some(
        "Peripheral Device",
    ),
    default: None,
    description: Some(
        "The peripheral device that triggered the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const permission: DictAttribute = DictAttribute {
    caption: Some(
        "Permission",
    ),
    default: None,
    description: Some(
        "The IAM permission related to an event",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const physical_height: DictAttribute = DictAttribute {
    caption: Some(
        "Physical Height",
    ),
    default: None,
    description: Some(
        "The numeric physical height of display.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const physical_orientation: DictAttribute = DictAttribute {
    caption: Some(
        "Physical Orientation",
    ),
    default: None,
    description: Some(
        "The numeric physical orientation of display.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const physical_width: DictAttribute = DictAttribute {
    caption: Some(
        "Physical Width",
    ),
    default: None,
    description: Some(
        "The numeric physical width of display.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const pid: DictAttribute = DictAttribute {
    caption: Some(
        "Process ID",
    ),
    default: None,
    description: Some(
        "The process identifier, as reported by the operating system. Process ID (PID) is a number used by the operating system to uniquely identify an active process.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const pixel_bits: DictAttribute = DictAttribute {
    caption: Some(
        "Pixel Bits",
    ),
    default: None,
    description: Some(
        "The number of bits per pixel.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const policy: DictAttribute = DictAttribute {
    caption: Some(
        "Policy",
    ),
    default: None,
    description: Some(
        "Describes details of a policy. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const port: DictAttribute = DictAttribute {
    caption: Some(
        "Port",
    ),
    default: None,
    description: Some(
        "The TCP/UDP port number associated with a connection. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const postal_code: DictAttribute = DictAttribute {
    caption: Some(
        "Postal Code",
    ),
    default: None,
    description: Some(
        "The postal code of the location.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const prefix: DictAttribute = DictAttribute {
    caption: Some(
        "Prefix",
    ),
    default: None,
    description: Some(
        "The domain prefix.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const prev_reg_key: DictAttribute = DictAttribute {
    caption: Some(
        "Previous Registry Key",
    ),
    default: None,
    description: Some(
        "The registry key before the mutation",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const prev_reg_value: DictAttribute = DictAttribute {
    caption: Some(
        "Previous Registry Value",
    ),
    default: None,
    description: Some(
        "The registry value before the mutation",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const priority: DictAttribute = DictAttribute {
    caption: Some(
        "Priority",
    ),
    default: None,
    description: Some(
        "The priority, normalized to the caption of the priority_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const priority_id: DictAttribute = DictAttribute {
    caption: Some(
        "Priority ID",
    ),
    default: None,
    description: Some(
        "The normalized priority. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "priority",
    ),
    attr_type: None,
};

const privileges: DictAttribute = DictAttribute {
    caption: Some(
        "Privileges",
    ),
    default: None,
    description: Some(
        "The user or group privileges.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const process: DictAttribute = DictAttribute {
    caption: Some(
        "Process",
    ),
    default: None,
    description: Some(
        "The process object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const processed_time: DictAttribute = DictAttribute {
    caption: Some(
        "Processed Time",
    ),
    default: None,
    description: Some(
        "The event processed time, such as an ETL operation.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const product: DictAttribute = DictAttribute {
    caption: Some(
        "Product",
    ),
    default: None,
    description: Some(
        "The product that reported the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const product_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Product Identifier",
    ),
    default: None,
    description: Some(
        "Unique Identifier of a product.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const profiles: DictAttribute = DictAttribute {
    caption: Some(
        "Profiles",
    ),
    default: None,
    description: Some(
        "The list of profiles used to create the event.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const project_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Project ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of a Cloud project.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const protocol_name: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Name",
    ),
    default: None,
    description: Some(
        "The TCP/IP protocol name in lowercase, as defined by the Internet Assigned Numbers Authority (IANA). See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>tcp</code> or <code>udp</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const protocol_num: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Number",
    ),
    default: None,
    description: Some(
        "The TCP/IP protocol number, as defined by the Internet Assigned Numbers Authority (IANA). Use -1 if the protocol is not defined by IANA. See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>6</code> for TCP and <code>17</code> for UDP.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const protocol_ver: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Version",
    ),
    default: None,
    description: Some(
        "The Protocol version, normalized to the caption of the protocol_ver_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const protocol_ver_id: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Version ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the Protocol version.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "protocol_ver",
    ),
    attr_type: None,
};

const provider: DictAttribute = DictAttribute {
    caption: Some(
        "Provider",
    ),
    default: None,
    description: Some(
        "The origin of information associated with the event. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const proxy: DictAttribute = DictAttribute {
    caption: Some(
        "Proxy",
    ),
    default: None,
    description: Some(
        "If a proxy connection is present, the connection from the client to the proxy server.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const query: DictAttribute = DictAttribute {
    caption: Some(
        "DNS Query",
    ),
    default: None,
    description: Some(
        "The Domain Name System (DNS) query.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const query_string: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Query String",
    ),
    default: None,
    description: Some(
        "The query portion of the URL. For example: the query portion of the URL <code>http://www.example.com/search?q=bad&sort=date</code> is <code>q=bad&sort=date</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const query_time: DictAttribute = DictAttribute {
    caption: Some(
        "Query Time",
    ),
    default: None,
    description: Some(
        "The Domain Name System (DNS) query time.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ram_size: DictAttribute = DictAttribute {
    caption: Some(
        "RAM Size",
    ),
    default: None,
    description: Some(
        "The ctotal amount of installed RAM, in Megabytes. For example: <code>2048</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const raw_data: DictAttribute = DictAttribute {
    caption: Some(
        "Raw Data",
    ),
    default: None,
    description: Some(
        "The event data as received from the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const raw_header: DictAttribute = DictAttribute {
    caption: Some(
        "Raw Header",
    ),
    default: None,
    description: Some(
        "The email authentication header.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const rcode: DictAttribute = DictAttribute {
    caption: Some(
        "Response Code",
    ),
    default: None,
    description: Some(
        "The server response code, normalized to the caption of the rcode_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const rcode_id: DictAttribute = DictAttribute {
    caption: Some(
        "Response Code ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the server response code. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "rcode",
    ),
    attr_type: None,
};

const rdata: DictAttribute = DictAttribute {
    caption: Some(
        "DNS RData",
    ),
    default: None,
    description: Some(
        "The data describing the DNS resource. The meaning of this data depends on the type and class of the resource record.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const references: DictAttribute = DictAttribute {
    caption: Some(
        "References",
    ),
    default: None,
    description: Some(
        "Supporting reference URLs",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const referrer: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Referrer",
    ),
    default: None,
    description: Some(
        "The request header that identifies the address of the previous web page, which is linked to the current web page or resource being requested.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const reg_key: DictAttribute = DictAttribute {
    caption: Some(
        "Registry Key",
    ),
    default: None,
    description: Some(
        "The registry key.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const reg_value: DictAttribute = DictAttribute {
    caption: Some(
        "Registry Value",
    ),
    default: None,
    description: Some(
        "The registry value.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const region: DictAttribute = DictAttribute {
    caption: Some(
        "Region",
    ),
    default: None,
    description: Some(
        "The name or the code of a region. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const registrar: DictAttribute = DictAttribute {
    caption: Some(
        "Domain Registrar",
    ),
    default: None,
    description: Some(
        "The domain registrar.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const related_events: DictAttribute = DictAttribute {
    caption: Some(
        "Related Events",
    ),
    default: None,
    description: Some(
        "Describes events related to a finding or detection as identified by the security product.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const related_vulnerabilities: DictAttribute = DictAttribute {
    caption: Some(
        "Related Vulnerabilities",
    ),
    default: None,
    description: Some(
        "List of vulnerabilities that are related to this vulnerability.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const relay: DictAttribute = DictAttribute {
    caption: Some(
        "Relay",
    ),
    default: None,
    description: Some(
        "The network relay that is associated with the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const release: DictAttribute = DictAttribute {
    caption: Some(
        "Software Release Details",
    ),
    default: None,
    description: Some(
        "Release is the number of times a version of the software has been packaged.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const remediation: DictAttribute = DictAttribute {
    caption: Some(
        "Remediation",
    ),
    default: None,
    description: Some(
        "The remediation recommendations on how to fix the identified issue(s).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const remote_display: DictAttribute = DictAttribute {
    caption: Some(
        "Remote Display",
    ),
    default: None,
    description: Some(
        "The remote display affiliated with the event",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const reply_to: DictAttribute = DictAttribute {
    caption: Some(
        "Reply To",
    ),
    default: None,
    description: Some(
        "The email header Reply-To values, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const reputation: DictAttribute = DictAttribute {
    caption: Some(
        "Reputation Scores",
    ),
    default: None,
    description: Some(
        "Contains the original and normalized reputation scores.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const request: DictAttribute = DictAttribute {
    caption: Some(
        "API Request Details",
    ),
    default: None,
    description: Some(
        "General Purpose API Request Object. See specific usage",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const requested_permissions: DictAttribute = DictAttribute {
    caption: Some(
        "Requested Permissions",
    ),
    default: None,
    description: Some(
        "The permissions mask that were requested by the process.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const requirements: DictAttribute = DictAttribute {
    caption: Some(
        "Requirements",
    ),
    default: None,
    description: Some(
        "A list of applicable compliance requirements for which this finding is related to.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const resource: DictAttribute = DictAttribute {
    caption: Some(
        "Resource",
    ),
    default: None,
    description: Some(
        "The target resource.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const resource_type: DictAttribute = DictAttribute {
    caption: Some(
        "Resource Type",
    ),
    default: None,
    description: Some(
        "The context in which a resource was retrieved in a web request.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const resource_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Resource ID",
    ),
    default: None,
    description: Some(
        "The unique identifier of a cloud resource. For example, S3 Bucket name, EC2 Instance Id.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const resources: DictAttribute = DictAttribute {
    caption: Some(
        "Resources Array",
    ),
    default: None,
    description: Some(
        "Describes details about resources that were affected by the activity/event.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const response: DictAttribute = DictAttribute {
    caption: Some(
        "API Response Details",
    ),
    default: None,
    description: Some(
        "General Purpose API Response Object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const response_time: DictAttribute = DictAttribute {
    caption: Some(
        "Response Time",
    ),
    default: None,
    description: Some(
        "The Domain Name System (DNS) response time.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const risk_level: DictAttribute = DictAttribute {
    caption: Some(
        "Risk Level",
    ),
    default: None,
    description: Some(
        "The risk level, normalized to the caption of the risk_level_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const risk_level_id: DictAttribute = DictAttribute {
    caption: Some(
        "Risk Level ID",
    ),
    default: None,
    description: Some(
        "The normalized risk level id.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "risk_level",
    ),
    attr_type: None,
};

const risk_score: DictAttribute = DictAttribute {
    caption: Some(
        "Risk Score",
    ),
    default: None,
    description: Some(
        "The risk score as reported by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const rule: DictAttribute = DictAttribute {
    caption: Some(
        "Rule",
    ),
    default: None,
    description: Some(
        "The rules that reported the events.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const run_state: DictAttribute = DictAttribute {
    caption: Some(
        "Run State",
    ),
    default: None,
    description: Some(
        "The state of the job or service, normalized to the caption of the run_state_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const run_state_id: DictAttribute = DictAttribute {
    caption: Some(
        "Run State ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the state of the job or service. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "run_state",
    ),
    attr_type: None,
};

const runtime: DictAttribute = DictAttribute {
    caption: Some(
        "Runtime",
    ),
    default: None,
    description: Some(
        "The runtime managing this container.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const samesite: DictAttribute = DictAttribute {
    caption: Some(
        "SameSite",
    ),
    default: None,
    description: Some(
        "The cookie attribute that lets servers specify whether/when cookies are sent with cross-site requests. Values are: Strict, Lax or None",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const sandbox: DictAttribute = DictAttribute {
    caption: Some(
        "Sandbox",
    ),
    default: None,
    description: Some(
        "The name of the containment jail (i.e., sandbox). For example, hardened_ps, high_security_ps, oracle_ps, netsvcs_ps, or default_ps.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const sans: DictAttribute = DictAttribute {
    caption: Some(
        "Subject Alternative Names",
    ),
    default: None,
    description: Some(
        "The list of subject alternative names that are secured by a specific certificate.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const scale_factor: DictAttribute = DictAttribute {
    caption: Some(
        "Scale Factor",
    ),
    default: None,
    description: Some(
        "The numeric scale factor of display.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const scheme: DictAttribute = DictAttribute {
    caption: Some(
        "Scheme",
    ),
    default: None,
    description: Some(
        "The scheme portion of the URL. For example: <code>http</code>, <code>https</code>, <code>ftp</code>, or <code>sftp</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const score: DictAttribute = DictAttribute {
    caption: Some(
        "Reputation Score",
    ),
    default: None,
    description: Some(
        "The reputation score, normalized to the caption of the score_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const score_id: DictAttribute = DictAttribute {
    caption: Some(
        "Reputation Score ID",
    ),
    default: None,
    description: Some(
        "The normalized reputation score identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "score",
    ),
    attr_type: None,
};

const secure: DictAttribute = DictAttribute {
    caption: Some(
        "Secure",
    ),
    default: None,
    description: Some(
        "The cookie attribute to only send cookies to the server with an encrypted request over the HTTPS protocol.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const security_descriptor: DictAttribute = DictAttribute {
    caption: Some(
        "Security Descriptor",
    ),
    default: None,
    description: Some(
        "The object security descriptor.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const sequence: DictAttribute = DictAttribute {
    caption: Some(
        "Sequence Number",
    ),
    default: None,
    description: Some(
        "Sequence number of the event. The sequence number is a value available in some events, to make the exact ordering of events unambiguous, regardless of the event time precision.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const serial_number: DictAttribute = DictAttribute {
    caption: Some(
        "Serial Number",
    ),
    default: None,
    description: Some(
        "The serial number that pertains to the object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const server_ciphers: DictAttribute = DictAttribute {
    caption: Some(
        "Server Cipher Suites",
    ),
    default: None,
    description: Some(
        "The server cipher suites that were exchanged during the TLS handshake negotiation.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const server_hassh: DictAttribute = DictAttribute {
    caption: Some(
        "Server HASSH",
    ),
    default: None,
    description: Some(
        "The Server HASSH fingerprinting object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const service: DictAttribute = DictAttribute {
    caption: Some(
        "Service",
    ),
    default: None,
    description: Some(
        "The service that pertains to the event.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const session: DictAttribute = DictAttribute {
    caption: Some(
        "User Session",
    ),
    default: None,
    description: Some(
        "The authenticated user session.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const severity: DictAttribute = DictAttribute {
    caption: Some(
        "Severity",
    ),
    default: None,
    description: Some(
        "The event severity, normalized to the caption of the severity_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const severity_id: DictAttribute = DictAttribute {
    caption: Some(
        "Severity ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the event severity.</p>The normalized severity is a measurement the effort and expense required to manage and resolve an event or incident. Smaller numerical values represent lower impact events, and larger numerical values represent higher impact events.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "severity",
    ),
    attr_type: None,
};

const share: DictAttribute = DictAttribute {
    caption: Some(
        "Share",
    ),
    default: None,
    description: Some(
        "The SMB share name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const share_type: DictAttribute = DictAttribute {
    caption: Some(
        "Share Type",
    ),
    default: None,
    description: Some(
        "The SMB share type.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const signature: DictAttribute = DictAttribute {
    caption: Some(
        "Digital Signature",
    ),
    default: None,
    description: Some(
        "The digital signature of the file.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const size: DictAttribute = DictAttribute {
    caption: Some(
        "Size",
    ),
    default: None,
    description: Some(
        "The size of data, in bytes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const smtp_from: DictAttribute = DictAttribute {
    caption: Some(
        "SMTP From",
    ),
    default: None,
    description: Some(
        "The value of the SMTP MAIL FROM command.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const smtp_hello: DictAttribute = DictAttribute {
    caption: Some(
        "SMTP Hello",
    ),
    default: None,
    description: Some(
        "The value of the SMTP HELO or EHLO command.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const smtp_to: DictAttribute = DictAttribute {
    caption: Some(
        "SMTP To",
    ),
    default: None,
    description: Some(
        "The value of the SMTP envelope RCPT TO command.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const sni: DictAttribute = DictAttribute {
    caption: Some(
        "Server Name Indication",
    ),
    default: None,
    description: Some(
        " The Server Name Indication (SNI) extension sent by the client.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const sp_name: DictAttribute = DictAttribute {
    caption: Some(
        "OS Service Pack",
    ),
    default: None,
    description: Some(
        "The name of the latest Service Pack.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const sp_ver: DictAttribute = DictAttribute {
    caption: Some(
        "OS Service Pack Version",
    ),
    default: None,
    description: Some(
        "The version number of the latest Service Pack.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const spf: DictAttribute = DictAttribute {
    caption: Some(
        "SPF Status",
    ),
    default: None,
    description: Some(
        "The Sender Policy Framework (SPF) status of the email.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const src_endpoint: DictAttribute = DictAttribute {
    caption: Some(
        "Source Endpoint",
    ),
    default: None,
    description: Some(
        "The network source endpoint.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const src_url: DictAttribute = DictAttribute {
    caption: Some(
        "Source URL",
    ),
    default: None,
    description: Some(
        "The URL pointing towards the source of an entity. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const start_address: DictAttribute = DictAttribute {
    caption: Some(
        "Start Address",
    ),
    default: None,
    description: Some(
        "The start address of the execution.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const start_time: DictAttribute = DictAttribute {
    caption: Some(
        "Start Time",
    ),
    default: None,
    description: Some(
        "The start time of a time period. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const state: DictAttribute = DictAttribute {
    caption: Some(
        "State",
    ),
    default: None,
    description: Some(
        "The state of the event or object, normalized to the caption of the state_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const state_id: DictAttribute = DictAttribute {
    caption: Some(
        "State ID",
    ),
    default: None,
    description: Some(
        "The normalized state ID of the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "state",
    ),
    attr_type: None,
};

const status: DictAttribute = DictAttribute {
    caption: Some(
        "Status",
    ),
    default: None,
    description: Some(
        "The event status, normalized to the caption of the status_id value. In the case of 'Other', it is defined by the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const status_code: DictAttribute = DictAttribute {
    caption: Some(
        "Status Code",
    ),
    default: None,
    description: Some(
        "The event status code, as reported by the event source.<br /><br />For example, in a Windows Failed Authentication event, this would be the value of 'Failure Code', e.g. 0x18.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const status_detail: DictAttribute = DictAttribute {
    caption: Some(
        "Status Details",
    ),
    default: None,
    description: Some(
        "The status details contains additional information about the event outcome.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const status_id: DictAttribute = DictAttribute {
    caption: Some(
        "Status ID",
    ),
    default: None,
    description: Some(
        "The normalized identifier of the event status.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "status",
    ),
    attr_type: None,
};

const subject: DictAttribute = DictAttribute {
    caption: Some(
        "Subject",
    ),
    default: None,
    description: Some(
        "The email header Subject value, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const subject_dn: DictAttribute = DictAttribute {
    caption: Some(
        "Subject Distinguished Name",
    ),
    default: None,
    description: Some(
        "The certificate subject distinguished name.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const subnet: DictAttribute = DictAttribute {
    caption: Some(
        "Subnet",
    ),
    default: None,
    description: Some(
        "The subnet mask.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const subnet_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Subnet UID",
    ),
    default: None,
    description: Some(
        "The unique identifier of a virtual subnet.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const supporting_data: DictAttribute = DictAttribute {
    caption: Some(
        "Supporting Data",
    ),
    default: None,
    description: Some(
        "Additional data supporting a finding as provided by security tool",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const svc_name: DictAttribute = DictAttribute {
    caption: Some(
        "Service Name",
    ),
    default: None,
    description: Some(
        "The service name in service-to-service connections. For example, AWS VPC logs the pkt-src-aws-service and pkt-dst-aws-service fields identify the connection is coming from or going to an AWS service.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const system_call: DictAttribute = DictAttribute {
    caption: Some(
        "System Call",
    ),
    default: None,
    description: Some(
        "The system call that was invoked.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const tactics: DictAttribute = DictAttribute {
    caption: Some(
        "Tactics",
    ),
    default: None,
    description: Some(
        "The a list of tactic ID's/names that are associated with the attack technique, as defined by <a target='_blank' href='https://attack.mitre.org/wiki/ATT&CK_Matrix'>ATT&CK Matrix<sup>TM</sup></a>.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const tag: DictAttribute = DictAttribute {
    caption: Some(
        "Image Tag",
    ),
    default: None,
    description: Some(
        "The image tag. For example: <code>1.11-alpine</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const tcp_flags: DictAttribute = DictAttribute {
    caption: Some(
        "TCP Flags",
    ),
    default: None,
    description: Some(
        "The network connection TCP header flags (i.e., control bits).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const technique: DictAttribute = DictAttribute {
    caption: Some(
        "Technique",
    ),
    default: None,
    description: Some(
        "The attack technique.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const terminated_time: DictAttribute = DictAttribute {
    caption: Some(
        "Terminated Time",
    ),
    default: None,
    description: Some(
        "The time when the entity was terminated. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const text: DictAttribute = DictAttribute {
    caption: Some(
        "URL Text",
    ),
    default: None,
    description: Some(
        "The URL. For example: <code>http://www.example.com/download/trouble.exe</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const tid: DictAttribute = DictAttribute {
    caption: Some(
        "Thread ID",
    ),
    default: None,
    description: Some(
        "The Identifier of the thread associated with the event, as returned by the operating system.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const time: DictAttribute = DictAttribute {
    caption: Some(
        "Event Time",
    ),
    default: None,
    description: Some(
        "The normalized event occurrence time.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const timezone_offset: DictAttribute = DictAttribute {
    caption: Some(
        "Timezone Offset",
    ),
    default: None,
    description: Some(
        "The number of minutes that the reported event <code>time</code> is ahead or behind UTC, in the range -1,080 to +1,080.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const title: DictAttribute = DictAttribute {
    caption: Some(
        "Title",
    ),
    default: None,
    description: Some(
        "The title of an entity. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const tls: DictAttribute = DictAttribute {
    caption: Some(
        "TLS",
    ),
    default: None,
    description: Some(
        "The Transport Layer Security (TLS) attributes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const to: DictAttribute = DictAttribute {
    caption: Some(
        "To",
    ),
    default: None,
    description: Some(
        "The email header To values, as defined by RFC 5322.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const traffic: DictAttribute = DictAttribute {
    caption: Some(
        "Traffic",
    ),
    default: None,
    description: Some(
        "The network traffic refers to the amount of data moving across a network at a given point of time. Intended to be used alongside Network Connection.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const transaction_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Transaction UID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the transaction.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const tree_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Tree UID",
    ),
    default: None,
    description: Some(
        "The tree id is a unique SMB identifier which represents an open connection to a share.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const true_color: DictAttribute = DictAttribute {
    caption: Some(
        "True Color",
    ),
    default: None,
    description: Some(
        "A boolean indicating whether to extract pixel values through red/green/blue intensities.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const ttl: DictAttribute = DictAttribute {
    caption: Some(
        "TTL",
    ),
    default: None,
    description: Some(
        "The time interval that the resource record may be cached. Zero value means that the resource record can only be used for the transaction in progress, and should not be cached.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const type: DictAttribute = DictAttribute {
    caption: Some(
        "Type",
    ),
    default: None,
    description: Some(
        "The type of an object or value, normalized to the caption of the type_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const type_id: DictAttribute = DictAttribute {
    caption: Some(
        "Type ID",
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The normalized type identifier of an object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "type",
    ),
    attr_type: None,
};

const type_name: DictAttribute = DictAttribute {
    caption: Some(
        "Type Name",
    ),
    default: None,
    description: Some(
        "The event type name, as defined by the type_uid.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const type_uid: DictAttribute = DictAttribute {
    caption: Some(
        "Type ID",
    ),
    default: None,
    description: Some(
        "The event type ID. It identifies the event's semantics and structure. The value is calculated by the logging system as: <code>class_uid * 100 + activity_id</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "type_name",
    ),
    attr_type: None,
};

const types: DictAttribute = DictAttribute {
    caption: Some(
        "Types",
    ),
    default: None,
    description: Some(
        "The type/s of an entity. See specific usage.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const uid: DictAttribute = DictAttribute {
    caption: Some(
        "Unique ID",
    ),
    default: None,
    description: Some(
        "The unique identifier. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const unmapped: DictAttribute = DictAttribute {
    caption: Some(
        "Unmapped Data",
    ),
    default: None,
    description: Some(
        "The attributes that are not mapped to the event schema. The names and values of those attributes are specific to the event source.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const url: DictAttribute = DictAttribute {
    caption: Some(
        "URL",
    ),
    default: None,
    description: Some(
        "The URL object that pertains to the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const user: DictAttribute = DictAttribute {
    caption: Some(
        "User",
    ),
    default: None,
    description: Some(
        "The user that pertains to the event or object.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const user_agent: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP User-Agent",
    ),
    default: None,
    description: Some(
        "The request header that identifies the operating system and web browser.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const user_result: DictAttribute = DictAttribute {
    caption: Some(
        "User Result",
    ),
    default: None,
    description: Some(
        "The result of the user account change. It should contain the new values of the changed attributes.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const uuid: DictAttribute = DictAttribute {
    caption: Some(
        "UUID",
    ),
    default: None,
    description: Some(
        "The universally unique identifier. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const value: DictAttribute = DictAttribute {
    caption: Some(
        "Value",
    ),
    default: None,
    description: Some(
        "The value that pertains to the object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vector_string: DictAttribute = DictAttribute {
    caption: Some(
        "Vector String",
    ),
    default: None,
    description: Some(
        "The CVSS vector string is a text representation of a set of CVSS metrics. It is commonly used to record or transfer CVSS metric information in a concise form. For example: <code>3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:N/A:H</code>.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vendor_name: DictAttribute = DictAttribute {
    caption: Some(
        "Vendor Name",
    ),
    default: None,
    description: Some(
        "The name of the vendor. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const version: DictAttribute = DictAttribute {
    caption: Some(
        "Version",
    ),
    default: None,
    description: Some(
        "The version that pertains to the event or object. See specific usage.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vlan_uid: DictAttribute = DictAttribute {
    caption: Some(
        "VLAN",
    ),
    default: None,
    description: Some(
        "The Virtual LAN identifier.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vnc_auth: DictAttribute = DictAttribute {
    caption: Some(
        "VNC Authentication",
    ),
    default: None,
    description: Some(
        "The Virtual Network Computing (VNC) authentication object describes the VNC authentication values.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vpc_uid: DictAttribute = DictAttribute {
    caption: Some(
        "VPC UID",
    ),
    default: None,
    description: Some(
        "The unique identifier of the Virtual Private Cloud (VPC).",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const vulnerabilities: DictAttribute = DictAttribute {
    caption: Some(
        "Vulnerabilities",
    ),
    default: None,
    description: Some(
        "This object describes vulnerabilities reported in a security finding.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const vulnerability: DictAttribute = DictAttribute {
    caption: Some(
        "Vulnerability",
    ),
    default: None,
    description: Some(
        "The vulnerability object describes details related to the observed vulnerability",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const win_resource: DictAttribute = DictAttribute {
    caption: Some(
        "Windows Resource",
    ),
    default: None,
    description: Some(
        "The Windows resource object that was accessed, such as a mutant or timer.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const x_forwarded_for: DictAttribute = DictAttribute {
    caption: Some(
        "X-Forwarded-For",
    ),
    default: None,
    description: Some(
        "The X-Forwarded-For header identifying the originating IP address(es) of a client connecting to a web server through an HTTP proxy or a load balancer.",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const x_originating_ip: DictAttribute = DictAttribute {
    caption: Some(
        "X-Originating-IP",
    ),
    default: None,
    description: Some(
        "The X-Originating-IP header identifying the emails originating IP address(es).",
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};

const xattributes: DictAttribute = DictAttribute {
    caption: Some(
        "Extended Attributes",
    ),
    default: None,
    description: Some(
        "An unordered collection of zero or more name/value pairs where each pair represents a file or folder extended attribute.</p>For example: Windows alternate data stream attributes (ADS stream name, ADS size, etc.), user-defined or application-defined attributes, ACL, owner, primary group, etc. Examples from DCS: </p><ul><li><strong>ads_name</strong></li><li><strong>ads_size</strong></li><li><strong>dacl</strong></li><li><strong>owner</strong></li><li><strong>primary_group</strong></li><li><strong>link_name</strong> - name of the link associated to the file.</li><li><strong>hard_link_count</strong> - the number of links that are associated to the file.</li></ul>",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};

const zone: DictAttribute = DictAttribute {
    caption: Some(
        "Network Zone",
    ),
    default: None,
    description: Some(
        "The network zone or LAN segment.",
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
