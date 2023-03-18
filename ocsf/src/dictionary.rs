//* "The Attribute Dictionary defines attributes and includes references to the events and objects in which they are used."
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DictAttribute {
    pub caption: &'static str,
    pub default: Option<i32>,
    pub description: &'static str,
    pub attr_enum: Option<&'static str>,
    pub is_array: Option<bool>,
    pub sibling: Option<&'static str>,
    #[serde(alias = "type")]
    pub attr_type: TypeNames,
}

#[derive(Debug, Clone, Serialize)]
pub struct DictType {
    pub caption: Option<&'static str>,
    pub description: Option<&'static str>,
    pub max_len: Option<&'static str>,
    pub observable: Option<&'static str>,
    pub range: Option<&'static str>,
    pub regex: Option<&'static str>,
    pub value_type: Option<&'static str>,
    pub type_name: Option<&'static str>,
    pub values: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub enum TypeNames {
    Integer,
    Json,
    String,
    Timestamp,
    NotSupported { name: &'static str },
}

pub const ACCESS_LIST: DictAttribute = DictAttribute {
    caption: "Access List",
    default: None,
    description: "The list of requested access rights.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ACCESS_MASK: DictAttribute = DictAttribute {
    caption: "Access Mask",
    default: None,
    description: "The access mask in a platform-native format.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ACCESS_RESULT: DictAttribute = DictAttribute {
    caption: "Access Check Result",
    default: None,
    description: "The list of access check results.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Json,
};

pub const ACCESSED_TIME: DictAttribute = DictAttribute {
    caption: "Accessed Time",
    default: None,
    description: "The time when the file was last accessed.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const ACCESSOR: DictAttribute = DictAttribute {
    caption: "Accessor",
    default: None,
    description: "The name of the user who last accessed the object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "user" },
};

pub const ACCOUNT_NAME: DictAttribute = DictAttribute {
    caption: "Account Name",
    default: None,
    description: "The name of the account (e.g. AWS Account Name).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ACCOUNT_TYPE: DictAttribute = DictAttribute {
    caption: "Account Type",
    default: None,
    description: "The user account type, normalized to the caption of 'account_type_id'. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ACCOUNT_TYPE_ID: DictAttribute = DictAttribute {
    caption: "Account Type ID",
    default: None,
    description: "The normalized user account type identifier.",
    attr_enum: None,
    is_array: None,
    sibling: Some("account_type"),
    attr_type: TypeNames::Integer,
};

pub const ACCOUNT_UID: DictAttribute = DictAttribute {
    caption: "Account UID",
    default: None,
    description: "The unique identifier of the account (e.g. AWS Account ID).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ACTIVITY_ID: DictAttribute = DictAttribute {
    caption: "Activity ID",
    default: None,
    description: "The normalized identifier of the activity that triggered the event.",
    attr_enum: None,
    is_array: None,
    sibling: Some("activity_name"),
    attr_type: TypeNames::Integer,
};

pub const ACTIVITY_NAME: DictAttribute = DictAttribute {
    caption: "Activity",
    default: None,
    description: "The event activity name, as defined by the activity_id.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ACTOR: DictAttribute = DictAttribute {
    caption: "Actor",
    default: None,
    description: "The actor object describes details about the user/role/process that was the source of the activity.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "actor" },
};

pub const ACTUAL_PERMISSIONS: DictAttribute = DictAttribute {
    caption: "Actual Permissions",
    default: None,
    description: "The permissions that were granted to the in a platform-native format.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ALERT: DictAttribute = DictAttribute {
    caption: "Client TLS Alert",
    default: None,
    description: "The integer value of TLS alert if present. The alerts are defined in the TLS specification in <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc2246'>RFC-2246</a>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ALGORITHM: DictAttribute = DictAttribute {
    caption: "Algorithm",
    default: None,
    description: "The hash algorithm used to create the digital fingerprint, normalized to the caption of 'algorithm_id'. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ALGORITHM_ID: DictAttribute = DictAttribute {
    caption: "Algorithm ID",
    default: None,
    description: "The identifier of the normalized hash algorithm, which was used to create the digital fingerprint.",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "algorithm",
    ),
    attr_type: TypeNames::Integer,
};

pub const ANSWERS: DictAttribute = DictAttribute {
    caption: "DNS Answer",
    default: None,
    description: "The Domain Name System (DNS) answers.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "dns_answer" },
};

pub const API: DictAttribute = DictAttribute {
    caption: "API details",
    default: None,
    description: "API object details information pertaining to the API calls",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "api" },
};

pub const APP_NAME: DictAttribute = DictAttribute {
    caption: "Application Name",
    default: None,
    description: "The name of the application that is associated with the event or object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ARCHITECTURE: DictAttribute = DictAttribute {
    caption: "Architecture",
    default: None,
    description: "Architecture is a shorthand name describing the type of computer hardware the packaged software is meant to run on.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ARGS: DictAttribute = DictAttribute {
    caption: "HTTP Arguments",
    default: None,
    description: "The arguments sent along with the HTTP request.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ATTACKS: DictAttribute = DictAttribute {
    caption: "Attacks",
    default: None,
    description: "An array of attacks associated with an event.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "attack" },
};

pub const ATTEMPT: DictAttribute = DictAttribute {
    caption: "Attempt",
    default: None,
    description: "The delivery attempt.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ATTRIBUTES: DictAttribute = DictAttribute {
    caption: "Attributes",
    default: None,
    description: "The bitmask value that represents the file attributes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const AUTH_PROTOCOL: DictAttribute = DictAttribute {
    caption: "Auth Protocol",
    default: None,
    description: "The authentication protocol as defined by the caption of 'auth_protocol_id'. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const AUTH_PROTOCOL_ID: DictAttribute = DictAttribute {
    caption: "Auth Protocol ID",
    default: None,
    description:
        "The normalized identifier of the authentication protocol used to create the user session.",
    attr_enum: None,
    is_array: None,
    sibling: Some("auth_protocol"),
    attr_type: TypeNames::Integer,
};

pub const AUTH_TYPE: DictAttribute = DictAttribute {
    caption: "Authentication Type",
    default: None,
    description: "The agreed upon authentication type, normalized to the caption of 'auth_type_id'. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const AUTH_TYPE_ID: DictAttribute = DictAttribute {
    caption: "Authentication Type ID",
    default: None,
    description:
        "The normalized identifier of the agreed upon authentication type. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("auth_type"),
    attr_type: TypeNames::Integer,
};

pub const AUTHORIZATIONS: DictAttribute = DictAttribute {
    caption: "Authorization Information",
    default: None,
    description: "This object provides details such as authorization outcome, associated policies related to activity/event.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "authorization" },
};

pub const AUTOSCALE_UID: DictAttribute = DictAttribute {
    caption: "Autoscale UID",
    default: None,
    description: "The unique identifier of the cloud autoscale configuration.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BANNER: DictAttribute = DictAttribute {
    caption: "SMTP Banner",
    default: None,
    description: "The initial SMTP connection response that a messaging server receives after it connects to a email server.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BASE_ADDRESS: DictAttribute = DictAttribute {
    caption: "Base Address",
    default: None,
    description: "The memory address where the module was loaded.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BASE_SCORE: DictAttribute = DictAttribute {
    caption: "Base Score",
    default: None,
    description: "The base score as reported by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "float_t" },
};

pub const BIG_ENDIAN: DictAttribute = DictAttribute {
    caption: "Big Endian",
    default: None,
    description:
        "A boolean indicating whether the most significant byte is stored/transmitted first.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const BIOS_DATE: DictAttribute = DictAttribute {
    caption: "BIOS Date",
    default: None,
    description: "The BIOS date. For example: <code>03/31/16</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BIOS_MANUFACTURER: DictAttribute = DictAttribute {
    caption: "BIOS Manufacturer",
    default: None,
    description: "The BIOS manufacturer. For example: <code>LENOVO</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BIOS_VER: DictAttribute = DictAttribute {
    caption: "BIOS Version",
    default: None,
    description: "The BIOS version. For example: <code>LENOVO G5ETA2WW (2.62)</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BOUNDARY: DictAttribute = DictAttribute {
    caption: "Boundary",
    default: None,
    description: "The boundary of the connection, normalized to the caption of 'boundary_id'. In the case of 'Other', it is defined by the event source. <p> For cloud connections, this translates to the traffic-boundary(same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BOUNDARY_ID: DictAttribute = DictAttribute {
    caption: "Boundary ID",
    default: None,
    description: "The normalized identifier of the boundary of the connection. </p> For cloud connections, this translates to the traffic-boundary (same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "boundary",
    ),
    attr_type: TypeNames::Integer,
};

pub const BUILD: DictAttribute = DictAttribute {
    caption: "OS Build",
    default: None,
    description: "The operating system build number.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const BYTES: DictAttribute = DictAttribute {
    caption: "Total Bytes",
    default: Some(0),
    description: "The total number of bytes (in and out).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const BYTES_IN: DictAttribute = DictAttribute {
    caption: "Bytes In",
    default: Some(0),
    description: "The number of bytes sent from the destination to the source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const BYTES_OUT: DictAttribute = DictAttribute {
    caption: "Bytes Out",
    default: Some(0),
    description: "The number of bytes sent from the source to the destination.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const CAPABILITIES: DictAttribute = DictAttribute {
    caption: "Capabilities",
    default: None,
    description: "A list of RDP capabilities.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CAPTION: DictAttribute = DictAttribute {
    caption: "Caption",
    default: None,
    description: "A short description or caption of the device. For example: <code>Scanner 1</code> or <code>Database Manager</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CATEGORIES: DictAttribute = DictAttribute {
    caption: "Website Categorization",
    default: None,
    description:
        "The Website categorization names, as defined by <code>category_ids</code> enum values.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CATEGORY: DictAttribute = DictAttribute {
    caption: "Category",
    default: None,
    description: "The object category. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CATEGORY_IDS: DictAttribute = DictAttribute {
    caption: "Website Categorization IDs",
    default: None,
    description: "The Website categorization identifies.",
    attr_enum: None,
    is_array: Some(true),
    sibling: Some("categories"),
    attr_type: TypeNames::Integer,
};

pub const CATEGORY_NAME: DictAttribute = DictAttribute {
    caption: "Category",
    default: None,
    description: "The event category name, as defined by category_uid value.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CATEGORY_UID: DictAttribute = DictAttribute {
    caption: "Category ID",
    default: Some(0),
    description: "The category unique identifier of the event.",
    attr_enum: None,
    is_array: None,
    sibling: Some("category_name"),
    attr_type: TypeNames::Integer,
};

pub const CC: DictAttribute = DictAttribute {
    caption: "Cc",
    default: None,
    description: "The email header Cc values, as defined by RFC 5322.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const CERTIFICATE: DictAttribute = DictAttribute {
    caption: "Certificate",
    default: None,
    description: "The certificate object containing information about the digital certificate.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "certificate",
    },
};

pub const CERTIFICATE_CHAIN: DictAttribute = DictAttribute {
    caption: "Certificate Chain",
    default: None,
    description: "The Chain of Certificate Serial Numbers field provides a chain of Certificate Issuer Serial Numbers leading to the Root Certificate Issuer.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CHALLENGE: DictAttribute = DictAttribute {
    caption: "Challenge",
    default: None,
    description: "The VNC challenge",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CHALLENGE_RESPONSE: DictAttribute = DictAttribute {
    caption: "Challenge Response",
    default: None,
    description: "The VNC challenge response",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CHASSIS: DictAttribute = DictAttribute {
    caption: "Chassis",
    default: None,
    description: "The chassis type describes the system enclosure or physical form factor. Such as the following examples for Windows <a target='_blank' href='https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure'>Windows Chassis Types</a>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CIPHER: DictAttribute = DictAttribute {
    caption: "Cipher Suite",
    default: None,
    description: "The negotiated cipher suite.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CIS_BENCHMARK_RESULT: DictAttribute = DictAttribute {
    caption: "CIS Benchmark Result",
    default: None,
    description: "The CIS benchmark result.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "cis_benchmark_result",
    },
};

pub const CITY: DictAttribute = DictAttribute {
    caption: "City",
    default: None,
    description: "The name of the city.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLASS: DictAttribute = DictAttribute {
    caption: "Class",
    default: None,
    description: "The class name of the object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLASS_NAME: DictAttribute = DictAttribute {
    caption: "Class",
    default: None,
    description: "The event class name, as defined by class_uid value.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLASS_UID: DictAttribute = DictAttribute {
    caption: "Class ID",
    default: Some(0),
    description:
        "The unique identifier of a class. A Class describes the attributes available in an event.",
    attr_enum: None,
    is_array: None,
    sibling: Some("class_name"),
    attr_type: TypeNames::Integer,
};

pub const CLASSIFICATION_IDS: DictAttribute = DictAttribute {
    caption: "Classification IDs",
    default: None,
    description: "The list of normalzied identifiers of the malware classifications. Reference: <a target='_blank' href='https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_oxlc4df65spl'>STIX Malware Types</a> ",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: Some(
        "classifications",
    ),
    attr_type: TypeNames::Integer,
};

pub const CLASSIFICATIONS: DictAttribute = DictAttribute {
    caption: "Classifications",
    default: None,
    description: "The list of malware classifications, normalized to the captions of the classifcation_id values. In the case of 'Other', they are defined by the event source.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLIENT_CIPHERS: DictAttribute = DictAttribute {
    caption: "Client Cipher Suites",
    default: None,
    description:
        "The client cipher suites that were exchanged during the TLS handshake negotiation.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLIENT_DIALECTS: DictAttribute = DictAttribute {
    caption: "Client Dialects",
    default: None,
    description: "The list of SMB dialects that the client speaks.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CLIENT_HASSH: DictAttribute = DictAttribute {
    caption: "Client HASSH",
    default: None,
    description: "The Client HASSH fingerprinting object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "hassh" },
};

pub const CLOUD: DictAttribute = DictAttribute {
    caption: "Cloud",
    default: None,
    description: "Describes details about the Cloud enviroment where the event was originally created or logged.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "cloud" },
};

pub const CLOUD_PARTITION: DictAttribute = DictAttribute {
    caption: "Cloud Partition",
    default: None,
    description: "The canonical cloud partition name to which the region is assigned (e.g. AWS Partitions: aws, aws-cn, aws-us-gov).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CMD_LINE: DictAttribute = DictAttribute {
    caption: "Command Line",
    default: None,
    description: "The full command line used to launch an application, service, process, or job. For example: <code>ssh user@10.0.0.10</code>. If the command line is unavailable or missing, the empty string <code>''</code> is to be used",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CODE: DictAttribute = DictAttribute {
    caption: "Response Code",
    default: None,
    description: "The numeric response sent to a request.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CODES: DictAttribute = DictAttribute {
    caption: "Response Codes",
    default: None,
    description: "The list of numeric responses sent to a request.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const COLOR_DEPTH: DictAttribute = DictAttribute {
    caption: "Color Depth",
    default: None,
    description: "The numeric color depth.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const COLOR_MAX: DictAttribute = DictAttribute {
    caption: "Color Maximum",
    default: None,
    description:
        "The maximum color value (with 'n' bits this would result in a 2^n-1 maximum value).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const COLOR_SHIFT: DictAttribute = DictAttribute {
    caption: "Color Shift",
    default: None,
    description: "The color shift value represents the number of shifts needed in order to get the color value in a pixel to the least significant bit.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const COMMAND: DictAttribute = DictAttribute {
    caption: "Command",
    default: None,
    description: "The command name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COMMAND_RESPONSE: DictAttribute = DictAttribute {
    caption: "Command Response",
    default: None,
    description: "The response to the command.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COMMAND_RESPONSES: DictAttribute = DictAttribute {
    caption: "Command Responses",
    default: None,
    description: "The responses to the command.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COMMENT: DictAttribute = DictAttribute {
    caption: "Comment",
    default: None,
    description: "The user-provided comment.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COMPANY_NAME: DictAttribute = DictAttribute {
    caption: "Company Name",
    default: None,
    description: "The name of the company that published the file. For example: <code>Microsoft Corporation</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COMPLIANCE: DictAttribute = DictAttribute {
    caption: "Compliance",
    default: None,
    description: "The complaince object provides context to compliance findings (e.g., a check against a specific regulatory or best practice framework such as CIS or NIST) and contains compliance related details.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "compliance" },
};

pub const COMPONENT: DictAttribute = DictAttribute {
    caption: "Component",
    default: None,
    description: "The name or relative pathname of a sub-component of the data object, if applicable. </p>For example: <code>attachment.doc</code>, <code>attachment.zip/bad.doc</code>, or <code>part.mime/part.cab/part.uue/part.doc</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CONFIDENCE: DictAttribute = DictAttribute {
    caption: "Confidence",
    default: None,
    description: "The confidence of the reported event severity as a percentage: 0%-100%.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CONFIDENTIALITY: DictAttribute = DictAttribute {
    caption: "Confidentiality",
    default: None,
    description: "The file content confidentiality, normalized to the confidentiality_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CONFIDENTIALITY_ID: DictAttribute = DictAttribute {
    caption: "Confidentiality ID",
    default: None,
    description: "The normalized identifier of the file content confidentiality indicator.",
    attr_enum: None,
    is_array: None,
    sibling: Some("confidentiality"),
    attr_type: TypeNames::Integer,
};

pub const CONNECTION_INFO: DictAttribute = DictAttribute {
    caption: "Connection Info",
    default: None,
    description: "The network connection information.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_connection_info",
    },
};

pub const CONNECTION_UID: DictAttribute = DictAttribute {
    caption: "Connection Identifier",
    default: None,
    description: "The network connection identifier.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CONTAINER: DictAttribute = DictAttribute {
    caption: "Container",
    default: None,
    description: "The container information.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "container" },
};

pub const CONTENT_TYPE: DictAttribute = DictAttribute {
    caption: "HTTP Content Type",
    default: None,
    description: "The request header that identifies the original <a target='_blank' href='https://www.iana.org/assignments/media-types/media-types.xhtml'>media type </a> of the resource (prior to any content encoding applied for sending).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CONTINENT: DictAttribute = DictAttribute {
    caption: "Continent",
    default: None,
    description: "The name of the continent.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COORDINATES: DictAttribute = DictAttribute {
    caption: "Coordinates",
    default: None,
    description: "A two-element array, containing a longitude/latitude pair. The format conforms with <a target='_blank' href='https://geojson.org'>GeoJSON</a>. For example: <code>[-73.983, 40.719]</code>.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "float_t" },
};

pub const CORRELATION_UID: DictAttribute = DictAttribute {
    caption: "Correlation UID",
    default: None,
    description: "The unique identifier used to correlate events.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const COUNT: DictAttribute = DictAttribute {
    caption: "Count",
    default: Some(
        1,
    ),
    description: "The number of times that events in the same logical group occurred during the event <strong>Start Time</strong> to <strong>End Time</strong> period.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const COUNTRY: DictAttribute = DictAttribute {
    caption: "Country",
    default: None,
    description: "The ISO 3166-1 Alpha-2 country code. For the complete list of country codes see <a target='_blank' href='https://www.iso.org/obp/ui/#iso:pub:PUB500001:en' >ISO 3166-1 alpha-2 codes</a>.<p><b>Note:</b> The two letter country code should be capitalized. For example: <code>US</code> or <code>CA</code>.</p>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CPU_BITS: DictAttribute = DictAttribute {
    caption: "CPU Bits",
    default: None,
    description: "The cpu architecture, the number of bits used for addressing in memory. For example: <code>32</code> or <code>64</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CPU_CORES: DictAttribute = DictAttribute {
    caption: "CPU Cores",
    default: None,
    description:
        "The number of processor cores in all installed processors. For Example: <code>42</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CPU_COUNT: DictAttribute = DictAttribute {
    caption: "CPU Count",
    default: None,
    description: "The number of physical processors on a system. For example: <code>1</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CPU_SPEED: DictAttribute = DictAttribute {
    caption: "Processor Type",
    default: None,
    description: "The speed of the processor in Mhz. For Example: <code>4200</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const CPU_TYPE: DictAttribute = DictAttribute {
    caption: "Processor Type",
    default: None,
    description: "The processor type. For example: <code>x86 Family 6 Model 37 Stepping 5</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CREATE_MASK: DictAttribute = DictAttribute {
    caption: "Create Mask",
    default: None,
    description: "The original Windows mask that is required to create the object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CREATED_TIME: DictAttribute = DictAttribute {
    caption: "Created Time",
    default: None,
    description: "The time when the object was created. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const CREATOR: DictAttribute = DictAttribute {
    caption: "Creator",
    default: None,
    description: "The user that created the object associated with event. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "user" },
};

pub const CREDENTIAL_UID: DictAttribute = DictAttribute {
    caption: "User Credential ID",
    default: None,
    description: "The unique identifier of the user's credential. For example, AWS Access Key ID.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CRITICALITY: DictAttribute = DictAttribute {
    caption: "Criticality",
    default: None,
    description: "Criticality of a resource/object in question",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CUSTOMER_UID: DictAttribute = DictAttribute {
    caption: "Customer UID",
    default: None,
    description: "The unique customer identifier.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CVE: DictAttribute = DictAttribute {
    caption: "CVE",
    default: None,
    description: "The Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "cve" },
};

pub const CVES: DictAttribute = DictAttribute {
    caption: "CVE List",
    default: None,
    description: "List of Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "cve" },
};

pub const CVSS: DictAttribute = DictAttribute {
    caption: "CVSS Score",
    default: None,
    description: "The CVSS object details Common Vulnerability Scoring System (<a target='_blank' href='https://www.first.org/cvss/'>CVSS</a>) scores from the advisory that are related to the vulnerability.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "cvss" },
};

pub const CWE_UID: DictAttribute = DictAttribute {
    caption: "CWE UID",
    default: None,
    description: "The <a target='_blank' href='https://cwe.mitre.org/'>Common Weakness Enumeration (CWE)</a> unique identifier. For example: <code>CWE-787</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const CWE_URL: DictAttribute = DictAttribute {
    caption: "CWE URL",
    default: None,
    description: "Common Weakness Enumiration (CWE) definition URL. For example: <code>https://cwe.mitre.org/data/definitions/787.html</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "url_t" },
};

pub const DATA: DictAttribute = DictAttribute {
    caption: "Data",
    default: None,
    description:
        "The additional data that is associated with the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Json,
};

pub const DATA_BUCKET: DictAttribute = DictAttribute {
    caption: "Data Bucket",
    default: None,
    description:
        "The name of the data bucket (e.g. bucket name for AWS/GCP and blob name for Azure).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DCE_RPC: DictAttribute = DictAttribute {
    caption: "Distributed Computing Environment/Remote Procedure Call (DCE/RPC)",
    default: None,
    description: "The DCE/RPC object describes the remote procedure call system for distributed computing environments.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "dce_rpc" },
};

pub const DECISION: DictAttribute = DictAttribute {
    caption: "Authorization Decision/Outcome",
    default: None,
    description: "Decision/outcome of the authorization mechanism (e.g. Approved, Denied)",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DELIVERED_TO: DictAttribute = DictAttribute {
    caption: "Delivered To",
    default: None,
    description: "The <strong>Delivered-To</strong> email header field.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const DEPTH: DictAttribute = DictAttribute {
    caption: "CVSS Depth",
    default: None,
    description: "The CVSS depth represents a depth of the equation used to calculate CVSS score.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DESC: DictAttribute = DictAttribute {
    caption: "Description",
    default: None,
    description: "The description that pertains to the object or event. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DESKTOP_DISPLAY: DictAttribute = DictAttribute {
    caption: "Desktop Display",
    default: None,
    description: "The desktop display affiliated with the event",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "display" },
};

pub const DETAILS: DictAttribute = DictAttribute {
    caption: "Details",
    default: None,
    description: "Details of an entity. See specific usage",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DETECTION_UID: DictAttribute = DictAttribute {
    caption: "Detection UID",
    default: None,
    description: "The associated unique detection event identifier. For example: detection response events include the <b>Detection ID</b> of the original event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DEVELOPER_UID: DictAttribute = DictAttribute {
    caption: "Developer UID",
    default: None,
    description: "The developer ID on the certificate that signed the file.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DEVICE: DictAttribute = DictAttribute {
    caption: "Device",
    default: None,
    description: "The device that reported the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "device" },
};

pub const DEVICES: DictAttribute = DictAttribute {
    caption: "Devices",
    default: None,
    description: "The object describes details related to the list of devices.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "device" },
};

pub const DIALECT: DictAttribute = DictAttribute {
    caption: "Dialect",
    default: None,
    description: "The negotiated protocol dialect.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DIRECTION: DictAttribute = DictAttribute {
    caption: "Direction",
    default: None,
    description: "The direction of the initiated connection, traffic, or email, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DIRECTION_ID: DictAttribute = DictAttribute {
    caption: "Direction ID",
    default: None,
    description:
        "The normalized identifier of the direction of the initiated connection, traffic, or email.",
    attr_enum: None,
    is_array: None,
    sibling: Some("direction"),
    attr_type: TypeNames::Integer,
};

pub const DISPOSITION: DictAttribute = DictAttribute {
    caption: "Disposition",
    default: None,
    description: "The event disposition name, normalized to the caption of the disposition_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DISPOSITION_ID: DictAttribute = DictAttribute {
    caption: "Disposition ID",
    default: None,
    description: "When security issues, such as malware or policy violations, are detected and possibly corrected, then <code>disposition_id</code> describes the action taken by the security product.",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "disposition",
    ),
    attr_type: TypeNames::Integer,
};

pub const DKIM: DictAttribute = DictAttribute {
    caption: "DKIM Status",
    default: None,
    description: "The DomainKeys Identified Mail (DKIM) status of the email.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DKIM_DOMAIN: DictAttribute = DictAttribute {
    caption: "DKIM Domain",
    default: None,
    description: "The DomainKeys Identified Mail (DKIM) signing domain of the email.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DKIM_SIGNATURE: DictAttribute = DictAttribute {
    caption: "DKIM Signature",
    default: None,
    description:
        "The DomainKeys Identified Mail (DKIM) signature used by the sending/receiving system.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DMARC: DictAttribute = DictAttribute {
    caption: "DMARC Status",
    default: None,
    description: "The Domain-based Message Authentication, Reporting and Conformance (DMARC) status of the email.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DMARC_OVERRIDE: DictAttribute = DictAttribute {
    caption: "DMARC Override",
    default: None,
    description: "The Domain-based Message Authentication, Reporting and Conformance (DMARC) override action.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DMARC_POLICY: DictAttribute = DictAttribute {
    caption: "DMARC Policy",
    default: None,
    description:
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) policy status.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DOMAIN: DictAttribute = DictAttribute {
    caption: "Domain",
    default: None,
    description: "The name of the domain.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const DOMAIN_INFO: DictAttribute = DictAttribute {
    caption: "Domain Information",
    default: None,
    description: "The registration information pertaining to a domain.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "domain_info",
    },
};

pub const DRIVER: DictAttribute = DictAttribute {
    caption: "Kernel Driver",
    default: None,
    description: "The driver that was loaded/unloaded into the kernel",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "kernel_driver",
    },
};

pub const DST_ENDPOINT: DictAttribute = DictAttribute {
    caption: "Destination Endpoint",
    default: None,
    description: "The network destination endpoint.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_endpoint",
    },
};

pub const DURATION: DictAttribute = DictAttribute {
    caption: "Duration",
    default: None,
    description: "The event duration or aggregate time, the amount of time the event covers from <code>start_time</code> to <code>end_time</code> in milliseconds.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const EDITION: DictAttribute = DictAttribute {
    caption: "OS Edition",
    default: None,
    description: "The operating system edition. For example: <code>Professional</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const EMAIL: DictAttribute = DictAttribute {
    caption: "Email",
    default: None,
    description: "The email object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email" },
};

pub const EMAIL_ADDR: DictAttribute = DictAttribute {
    caption: "Email Address",
    default: None,
    description: "The user's email address.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const EMAIL_AUTH: DictAttribute = DictAttribute {
    caption: "Email Authentication",
    default: None,
    description: "The SPF, DKIM and DMARC attributes of an email.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_auth" },
};

pub const END_TIME: DictAttribute = DictAttribute {
    caption: "End Time",
    default: None,
    description: "The end time of a time period. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const ENRICHMENTS: DictAttribute = DictAttribute {
    caption: "Enrichments",
    default: None,
    description: "The additional information from an external data source, which is associated with the event. For example add location information for the IP address in the DNS answers:</p><code>[{\"name\": \"answers.ip\", \"value\": \"92.24.47.250\", \"type\": \"location\", \"data\": {\"city\": \"Socotra\", \"continent\": \"Asia\", \"coordinates\": [-25.4153, 17.0743], \"country\": \"YE\", \"desc\": \"Yemen\"}}]</code>",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "enrichment" },
};

pub const ENTITY: DictAttribute = DictAttribute {
    caption: "Entity",
    default: None,
    description: "The managed entity that is being acted upon.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "managed_entity",
    },
};

pub const ENTITY_RESULT: DictAttribute = DictAttribute {
    caption: "Entity Result",
    default: None,
    description: "The updated managed entity.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "managed_entity",
    },
};

pub const EPOCH: DictAttribute = DictAttribute {
    caption: "Epoch",
    default: None,
    description: "The software package epoch. Epoch is a way to define weighted dependencies based on version numbers.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ERROR: DictAttribute = DictAttribute {
    caption: "Error Code",
    default: None,
    description: "Error Code",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ERROR_MESSAGE: DictAttribute = DictAttribute {
    caption: "Error Message",
    default: None,
    description: "Error Message",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const EXIT_CODE: DictAttribute = DictAttribute {
    caption: "Exit Code",
    default: None,
    description: "The exit code reported by a process when it terminates. The convention is that zero indicates success and any non-zero exit code indicates that some error occurred.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const EXPIRATION_TIME: DictAttribute = DictAttribute {
    caption: "Expiration Time",
    default: None,
    description: "The expiration time. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const EXPOSED_PORT: DictAttribute = DictAttribute {
    caption: "Port",
    default: None,
    description:
        "The IP port number exposed by container. For example 0.0.0.0:49155-> <<exposed_port>>/tcp",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "port_t" },
};

pub const EXTENSION_LIST: DictAttribute = DictAttribute {
    caption: "Extension List",
    default: None,
    description: "The list of TLS extensions",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "tls_extension",
    },
};

pub const FEATURE: DictAttribute = DictAttribute {
    caption: "Feature",
    default: None,
    description: "The feature that reported the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "feature" },
};

pub const FILE: DictAttribute = DictAttribute {
    caption: "File",
    default: None,
    description: "The file that pertains to the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "file" },
};

pub const FILE_DIFF: DictAttribute = DictAttribute {
    caption: "File Diff",
    default: None,
    description: "File content differences used for change detection. For example, a common use case is to identify itemized changes within INI or configuration/property setting values.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const FILE_RESULT: DictAttribute = DictAttribute {
    caption: "File Result",
    default: None,
    description:
        "The result of the file change. It should contain the new values of the changed attributes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "file" },
};

pub const FINDING: DictAttribute = DictAttribute {
    caption: "Finding",
    default: None,
    description: "Finding object provides details related to a finding generated by security tool",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "finding" },
};

pub const FINGERPRINT: DictAttribute = DictAttribute {
    caption: "Fingerprint",
    default: None,
    description: "The digital fingerprint associated with an object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "fingerprint",
    },
};

pub const FINGERPRINTS: DictAttribute = DictAttribute {
    caption: "Fingerprints",
    default: None,
    description: "An array of digital fingerprint objects.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "fingerprint",
    },
};

pub const FIRST_SEEN_TIME: DictAttribute = DictAttribute {
    caption: "First Seen",
    default: None,
    description: "The initial detection time of the activity or object. See specific usage",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const FLAG_IDS: DictAttribute = DictAttribute {
    caption: "Communication Flag IDs",
    default: None,
    description: "The list of normalized identifiers of the communication flag IDs.",
    attr_enum: None,
    is_array: Some(true),
    sibling: Some("flags"),
    attr_type: TypeNames::Integer,
};

pub const FLAGS: DictAttribute = DictAttribute {
    caption: "Flags",
    default: None,
    description: "The list of communication flags, normalized to the captions of the flag_ids values. In the case of 'Other', they are defined by the event source.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const FOLDER: DictAttribute = DictAttribute {
    caption: "Folder",
    default: None,
    description: "The folder that pertains to the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "file" },
};

pub const FROM: DictAttribute = DictAttribute {
    caption: "From",
    default: None,
    description: "The email header From values, as defined by RFC 5322.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const FUNCTION_KEYS: DictAttribute = DictAttribute {
    caption: "Function Keys",
    default: None,
    description: "The number of function keys on client keyboard.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const FUNCTION_NAME: DictAttribute = DictAttribute {
    caption: "Function Name",
    default: None,
    description: "The entry-point function of the module. The system calls the entry-point function whenever a process or thread loads or unloads the module.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const GROUP: DictAttribute = DictAttribute {
    caption: "Group",
    default: None,
    description: "The group object associated with an entity such as user, policy, or rule.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "group" },
};

pub const GROUP_NAME: DictAttribute = DictAttribute {
    caption: "Group Name",
    default: None,
    description: "The name of the group that the resource belongs to.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const GROUPS: DictAttribute = DictAttribute {
    caption: "Groups",
    default: None,
    description: "The groups to which an entity belongs. See specific usage.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "group" },
};

pub const HANDSHAKE_DUR: DictAttribute = DictAttribute {
    caption: "Handshake Duration",
    default: None,
    description: "The amount of total time for the TLS handshake to complete after the TCP connection is established, including client-side delays, in milliseconds.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const HOSTNAME: DictAttribute = DictAttribute {
    caption: "Hostname",
    default: None,
    description: "The hostname of an endpoint or a device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "hostname_t" },
};

pub const HTTP_COOKIES: DictAttribute = DictAttribute {
    caption: "HTTP Cookies",
    default: None,
    description: "The cookies object describes details about HTTP cookies",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "http_cookie",
    },
};

pub const HTTP_HEADERS: DictAttribute = DictAttribute {
    caption: "HTTP Headers",
    default: None,
    description: "Additional HTTP headers of an HTTP request or response.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "http_header",
    },
};

pub const HTTP_METHOD: DictAttribute = DictAttribute {
    caption: "HTTP Method",
    default: None,
    description: "The HTTP request method indicates the desired action to be performed for a given resource. Expected values: <ul> <li>TRACE</li> <li>CONNECT</li> <li>OPTIONS</li> <li>HEAD</li> <li>DELETE</li> <li>POST</li> <li>PUT</li> <li>GET</li></ul>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const HTTP_ONLY: DictAttribute = DictAttribute {
    caption: "HTTP Only",
    default: None,
    description: "A cookie attribute to make it inaccessible via JavaScript",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const HTTP_REQUEST: DictAttribute = DictAttribute {
    caption: "HTTP Request",
    default: None,
    description: "The HTTP Request made to a web server.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "http_request",
    },
};

pub const HTTP_RESPONSE: DictAttribute = DictAttribute {
    caption: "HTTP Response",
    default: None,
    description: "The HTTP Response from a web server to a requester.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "http_response",
    },
};

pub const HTTP_STATUS: DictAttribute = DictAttribute {
    caption: "HTTP Status",
    default: None,
    description: "The Hypertext Transfer Protocol (HTTP) <a target='_blank' href='https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml'>status code</a> returned to the client.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const HW_INFO: DictAttribute = DictAttribute {
    caption: "Hardware Info",
    default: None,
    description: "The device hardware information.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "device_hw_info",
    },
};

pub const HYPERVISOR: DictAttribute = DictAttribute {
    caption: "Hypervisor",
    default: None,
    description: "The name of the hypervisor running on the device. For example, <code>Xen</code>, <code>VMware</code>, <code>Hyper-V</code>, <code>VirtualBox</code>, etc.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const IDENTIFIER_COOKIE: DictAttribute = DictAttribute {
    caption: "Identifier Cookie",
    default: None,
    description: "The client identifier cookie during client/server exchange.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const IDP: DictAttribute = DictAttribute {
    caption: "Identity Provider",
    default: None,
    description: "This object describes details about the Identity Provider used.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "idp" },
};

pub const IMAGE: DictAttribute = DictAttribute {
    caption: "Image",
    default: None,
    description: "The image used as a template to run a container or virtual machine.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "image" },
};

pub const IME: DictAttribute = DictAttribute {
    caption: "IME",
    default: None,
    description: "The Input Method Editor (IME) file name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const IMEI: DictAttribute = DictAttribute {
    caption: "IMEI",
    default: None,
    description:
        "The International Mobile Station Equipment Identifier that is associated with the device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INJECTION_TYPE: DictAttribute = DictAttribute {
    caption: "Injection Type",
    default: None,
    description: "The process injection method, normalized to the caption of the injection_type_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INJECTION_TYPE_ID: DictAttribute = DictAttribute {
    caption: "Injection Type ID",
    default: None,
    description: "The normalized identifier of the process injection method.",
    attr_enum: None,
    is_array: None,
    sibling: Some("injection_type"),
    attr_type: TypeNames::Integer,
};

pub const INSTANCE_UID: DictAttribute = DictAttribute {
    caption: "Instance ID",
    default: None,
    description: "The unique identifier of a VM instance.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INTEGRITY: DictAttribute = DictAttribute {
    caption: "Integrity",
    default: None,
    description: "The process integrity level, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source (Windows only).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INTEGRITY_ID: DictAttribute = DictAttribute {
    caption: "Integrity Level",
    default: None,
    description: "The normalized identifier of the process integrity level (Windows only).",
    attr_enum: None,
    is_array: None,
    sibling: Some("integrity"),
    attr_type: TypeNames::Integer,
};

pub const INTERFACE_NAME: DictAttribute = DictAttribute {
    caption: "Network Interface Name",
    default: None,
    description: "The name of the network interface (e.g. eth2).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INTERFACE_UID: DictAttribute = DictAttribute {
    caption: "Network Interface ID",
    default: None,
    description: "The unique identifier of the network interface.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const INTERMEDIATE_IPS: DictAttribute = DictAttribute {
    caption: "Intermediate IP Addresses",
    default: None,
    description: "The intermediate IP Addresses. For example, the IP addresses in the HTTP X-Forwarded-For header.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "ip_t" },
};

pub const INVOKED_BY: DictAttribute = DictAttribute {
    caption: "Invoked by",
    default: None,
    description: "The name of the service that invoked the activity as described in the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const IP: DictAttribute = DictAttribute {
    caption: "IP Address",
    default: None,
    description: "The IP address, in either IPv4 or IPv6 format.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "ip_t" },
};

pub const IS_CLEARTEXT: DictAttribute = DictAttribute {
    caption: "Cleartext Credentials",
    default: None,
    description: "Indicates whether the credentials were passed in clear text.<p><b>Note:</b> True if the credentials were passed in a clear text protocol such as FTP or TELNET, or if Windows detected that a user's logon password was passed to the authentication package in clear text.</p>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "boolean_t" },
};

pub const IS_COMPLIANT: DictAttribute = DictAttribute {
    caption: "Compliant Device",
    default: None,
    description: "The event occurred on a compliant device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_DEFAULT: DictAttribute = DictAttribute {
    caption: "Default Value",
    default: None,
    description: "The indication of whether the value is from a default value name. For example, the value name could be missing.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "boolean_t" },
};

pub const IS_MANAGED: DictAttribute = DictAttribute {
    caption: "Managed Device",
    default: None,
    description: "The event occurred on a managed device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_ON_PREMISES: DictAttribute = DictAttribute {
    caption: "On Premises",
    default: None,
    description: "The indication of whether the location is on premises.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_PERSONAL: DictAttribute = DictAttribute {
    caption: "Personal Device",
    default: None,
    description: "The event occurred on a personal device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_REMOTE: DictAttribute = DictAttribute {
    caption: "Remote",
    default: None,
    description: "The indication of whether the session is remote.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_RENEWAL: DictAttribute = DictAttribute {
    caption: "Renewal",
    default: None,
    description: "The indication of whether this is a lease/session renewal event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_SYSTEM: DictAttribute = DictAttribute {
    caption: "System",
    default: None,
    description: "The indication of whether the object is part of the operating system.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const IS_TRUSTED: DictAttribute = DictAttribute {
    caption: "Trusted Device",
    default: None,
    description: "The event occurred on a trusted device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const ISP: DictAttribute = DictAttribute {
    caption: "ISP",
    default: None,
    description: "The name of the Internet Service Provider (ISP).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ISSUER: DictAttribute = DictAttribute {
    caption: "Issuer Details",
    default: None,
    description: "The identifier of the session issuer.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ISSUER_DN: DictAttribute = DictAttribute {
    caption: "Issuer Distinguished Name",
    default: None,
    description: "The certificate issuer distinguished name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ISSUER_NAME: DictAttribute = DictAttribute {
    caption: "Issuer Name",
    default: None,
    description: "The certificate issuer name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const JA3_FINGERPRINT: DictAttribute = DictAttribute {
    caption: "JA3 Fingerprint",
    default: None,
    description: "The fingerprint of JA3 string.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "fingerprint",
    },
};

pub const JA3_STRING: DictAttribute = DictAttribute {
    caption: "JA3 String",
    default: None,
    description: "The JA3 string.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const JA3S_FINGERPRINT: DictAttribute = DictAttribute {
    caption: "JAS3 Fingerprint",
    default: None,
    description: "The fingerprint of JAS3 string.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "fingerprint",
    },
};

pub const JA3S_STRING: DictAttribute = DictAttribute {
    caption: "JAS3 String",
    default: None,
    description: "The JAS3 string.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const JOB: DictAttribute = DictAttribute {
    caption: "Job",
    default: None,
    description: "The job object that pertains to the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "job" },
};

pub const KB_ARTICLES: DictAttribute = DictAttribute {
    caption: "Knowledgebase Articles",
    default: None,
    description: "The KB article/s related to the entity",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const KERNEL: DictAttribute = DictAttribute {
    caption: "Kernel",
    default: None,
    description: "The kernel resource object that pertains to the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "kernel" },
};

pub const KEY_LENGTH: DictAttribute = DictAttribute {
    caption: "Key Length",
    default: None,
    description: "The length of the encryption key.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const KEYBOARD_INFO: DictAttribute = DictAttribute {
    caption: "Keyboard Information",
    default: None,
    description: "The keyboard detailed information.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "keyboard_info",
    },
};

pub const KEYBOARD_LAYOUT: DictAttribute = DictAttribute {
    caption: "Keyboard Layout",
    default: None,
    description: "The keyboard locale identifier name (e.g., en-US).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const KEYBOARD_SUBTYPE: DictAttribute = DictAttribute {
    caption: "Keyboard Subtype",
    default: None,
    description: "The keyboard numeric code.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const KEYBOARD_TYPE: DictAttribute = DictAttribute {
    caption: "Keyboard Type",
    default: None,
    description: "The keyboard type (e.g., xt, ico).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LABELS: DictAttribute = DictAttribute {
    caption: "Labels",
    default: None,
    description: "The list of labels attached to an event, object, or attribute.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LANG: DictAttribute = DictAttribute {
    caption: "Language",
    default: None,
    description: "The two letter lower case language codes, as defined by <a target='_blank' href='https://en.wikipedia.org/wiki/ISO_639-1'>ISO 639-1</a>. For example: <code>en</code> (English), <code>de</code> (German), or <code>fr</code> (French).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LAST_RUN_TIME: DictAttribute = DictAttribute {
    caption: "Last Run",
    default: None,
    description: "The last run time of application or service. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const LAST_SEEN_TIME: DictAttribute = DictAttribute {
    caption: "Last Seen",
    default: None,
    description: "The most recent detection time of the activity or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const LATENCY: DictAttribute = DictAttribute {
    caption: "Latency",
    default: None,
    description: "TODO: The HTTP response latency. In seconds, milliseconds, etc.?",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const LEASE_DUR: DictAttribute = DictAttribute {
    caption: "Lease Duration",
    default: None,
    description: "This represents the length of the DHCP lease in seconds. This is present in DHCP Ack events. (activity_id = 1)",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const LENGTH: DictAttribute = DictAttribute {
    caption: "Response Length",
    default: None,
    description: "The HTTP response length, in number of bytes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const LICENSE: DictAttribute = DictAttribute {
    caption: "Software License",
    default: None,
    description: "The name or identifier of the license applied on package or software. See <a target='_blank' href='https://spdx.org/licenses/'>SPDX License List</a>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LINEAGE: DictAttribute = DictAttribute {
    caption: "Lineage",
    default: None,
    description: "The lineage of the process, represented by a list of paths for each ancestor process. For example: <code>['/usr/sbin/sshd', '/usr/bin/bash', '/usr/bin/whoami']</code>",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LOAD_TYPE: DictAttribute = DictAttribute {
    caption: "Load Type",
    default: None,
    description: "The load type, normalized to the caption of the load_type_id value. In the case of 'Other', it is defined by the event source. It describes how the module was loaded in memory.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LOAD_TYPE_ID: DictAttribute = DictAttribute {
    caption: "Load Type ID",
    default: None,
    description: "The normalized identifier of the load type. It identifies how the module was loaded in memory.",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "load_type",
    ),
    attr_type: TypeNames::Integer,
};

pub const LOADED_MODULES: DictAttribute = DictAttribute {
    caption: "Loaded Modules",
    default: None,
    description: "The list of loaded module names.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LOCATION: DictAttribute = DictAttribute {
    caption: "Detailed Geo Location",
    default: None,
    description: "The detailed geographical location usually associated with an IP address.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "location" },
};

pub const LOGGED_TIME: DictAttribute = DictAttribute {
    caption: "Logged Time",
    default: None,
    description: "The time when the logging system collected and logged the event.</p>This attribute is distinct from the event time in that event time typically contain the time extracted from the original event. Most of the time, these two times will be different.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const LOGON_PROCESS: DictAttribute = DictAttribute {
    caption: "Logon Process",
    default: None,
    description: "The trusted process that validated the authentication credentials.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "process" },
};

pub const LOGON_TYPE: DictAttribute = DictAttribute {
    caption: "Logon Type",
    default: None,
    description: "The logon type, normalized to the caption of the logon_type_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const LOGON_TYPE_ID: DictAttribute = DictAttribute {
    caption: "Logon Type ID",
    default: None,
    description: "The normalized logon type identifier.",
    attr_enum: None,
    is_array: None,
    sibling: Some("logon_type"),
    attr_type: TypeNames::Integer,
};

pub const MAC: DictAttribute = DictAttribute {
    caption: "MAC Address",
    default: None,
    description:
        "The Media Access Control (MAC) address that is associated with the network interface.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "mac_t" },
};

pub const MALWARE: DictAttribute = DictAttribute {
    caption: "Malware",
    default: None,
    description: "The list of malware identified by a finding.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "malware" },
};

pub const MESSAGE: DictAttribute = DictAttribute {
    caption: "Message",
    default: None,
    description: "The description of the event, as defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const MESSAGE_UID: DictAttribute = DictAttribute {
    caption: "Message UID",
    default: None,
    description: "The email header Message-Id value, as defined by RFC 5322.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const METADATA: DictAttribute = DictAttribute {
    caption: "Metadata",
    default: None,
    description: "The metadata associated with the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "metadata" },
};

pub const METRICS: DictAttribute = DictAttribute {
    caption: "Metrics",
    default: None,
    description: "The general purpose metrics associated with the event. See specific usage.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "metric" },
};

pub const MFA: DictAttribute = DictAttribute {
    caption: "Multi Factor Authentication",
    default: None,
    description: "Indicates whether Multi Factor Authentication was used during authentication.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const MIME_TYPE: DictAttribute = DictAttribute {
    caption: "MIME type",
    default: None,
    description:
        "The Multipurpose Internet Mail Extensions (MIME) type of the file, if applicable.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const MODEL: DictAttribute = DictAttribute {
    caption: "Model",
    default: None,
    description: "The peripheral device model.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const MODIFIED_TIME: DictAttribute = DictAttribute {
    caption: "Modified Time",
    default: None,
    description: "The time when the object was last modified. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const MODIFIER: DictAttribute = DictAttribute {
    caption: "Modifier",
    default: None,
    description:
        "The user that last modified the object associated with the event. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "user" },
};

pub const MODULE: DictAttribute = DictAttribute {
    caption: "Module",
    default: None,
    description: "The module that pertains to the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "module" },
};

pub const NAME: DictAttribute = DictAttribute {
    caption: "Name",
    default: None,
    description: "The name of the entity. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const NAMESPACE: DictAttribute = DictAttribute {
    caption: "Namespace",
    default: None,
    description: "The namespace is useful in merger or acquisition situations. For example, when similar entities exists that you need to keep separate.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const NETWORK_DRIVER: DictAttribute = DictAttribute {
    caption: "Network Driver",
    default: None,
    description:
        "The network driver used by the container. For example, bridge, overlay, host, none, etc.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const NETWORK_INTERFACE: DictAttribute = DictAttribute {
    caption: "Network Interface",
    default: None,
    description: "The network interface that is associated with the device.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_interface",
    },
};

pub const NETWORK_INTERFACES: DictAttribute = DictAttribute {
    caption: "Network Interfaces",
    default: None,
    description: "The network interfaces that are associated with the device, one for each MAC address/IP address combination.<p><b>Note:</b> The first element of the array is the network information that pertains to the event.</p>",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "network_interface" },
};

pub const NEXT_RUN_TIME: DictAttribute = DictAttribute {
    caption: "Next Run",
    default: None,
    description: "The next run time. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const OBSERVABLES: DictAttribute = DictAttribute {
    caption: "Observables",
    default: None,
    description: "The observables associated with the event.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "observable" },
};

pub const OPCODE: DictAttribute = DictAttribute {
    caption: "DNS Opcode",
    default: None,
    description: "The DNS opcode specifies the type of the query message.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const OPCODE_ID: DictAttribute = DictAttribute {
    caption: "DNS Opcode ID",
    default: None,
    description: "The DNS opcode ID specifies the normalized query message type.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const OPEN_MASK: DictAttribute = DictAttribute {
    caption: "Open Mask",
    default: None,
    description: "The Windows options needed to open a registry key.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const OPEN_TYPE: DictAttribute = DictAttribute {
    caption: "Open Type",
    default: None,
    description: "The file open type.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const OPERATION: DictAttribute = DictAttribute {
    caption: "Operation",
    default: None,
    description: "Verb/Operation associated with the request",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const OPNUM: DictAttribute = DictAttribute {
    caption: "Opnum",
    default: None,
    description: "An operation number used to identify a specific remote procedure call (RPC) method or a method in an interface.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const ORCHESTRATOR: DictAttribute = DictAttribute {
    caption: "Orchestrator",
    default: None,
    description: "The orchestrator managing the container, such as ECS, EKS, K8s, OpenShift, None.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ORG_UID: DictAttribute = DictAttribute {
    caption: "Org ID",
    default: None,
    description: "The unique identifier of the organization to which the user belongs. For example, Active Directory or AWS Org ID.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ORG_UNIT: DictAttribute = DictAttribute {
    caption: "Org Unit",
    default: None,
    description: "The name of the organization to which the user belongs.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const ORIGINAL_TIME: DictAttribute = DictAttribute {
    caption: "Original Time",
    default: None,
    description: "The original event time as reported by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const OS: DictAttribute = DictAttribute {
    caption: "OS",
    default: None,
    description: "The device operation system.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "os" },
};

pub const OVERALL_SCORE: DictAttribute = DictAttribute {
    caption: "Overall Score",
    default: None,
    description: "The overall score as reported by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "float_t" },
};

pub const OWNER: DictAttribute = DictAttribute {
    caption: "Owner",
    default: None,
    description: "The user that owns the file/object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "user" },
};

pub const PACKAGES: DictAttribute = DictAttribute {
    caption: "Software Packages",
    default: None,
    description: "List of vulnerable packages as identified by the security product",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "package" },
};

pub const PACKET_UID: DictAttribute = DictAttribute {
    caption: "Packet UID",
    default: None,
    description: "The packet identifier assigned by the protocol.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PACKETS: DictAttribute = DictAttribute {
    caption: "Total Packets",
    default: Some(0),
    description: "The total number of packets (in and out).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const PACKETS_IN: DictAttribute = DictAttribute {
    caption: "Packets In",
    default: Some(0),
    description: "The number of packets sent from the destination to the source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const PACKETS_OUT: DictAttribute = DictAttribute {
    caption: "Packets Out",
    default: Some(0),
    description: "The number of packets sent from the source to the destination.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const PARENT_FOLDER: DictAttribute = DictAttribute {
    caption: "Parent Folder",
    default: None,
    description: "The parent folder in which the file resides. For example: <code>c:\\windows\\system32</code>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "path_t" },
};

pub const PARENT_PROCESS: DictAttribute = DictAttribute {
    caption: "Parent Process",
    default: None,
    description: "The parent process of this process object. It is recommended to only populate this field for the first process object, to prevent deep nesting.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "process" },
};

pub const PATH: DictAttribute = DictAttribute {
    caption: "Path",
    default: None,
    description: "The path that pertains to the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "path_t" },
};

pub const PERIPHERAL_DEVICE: DictAttribute = DictAttribute {
    caption: "Peripheral Device",
    default: None,
    description: "The peripheral device that triggered the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "peripheral_device",
    },
};

pub const PERMISSION: DictAttribute = DictAttribute {
    caption: "Permission",
    default: None,
    description: "The IAM permission related to an event",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PHYSICAL_HEIGHT: DictAttribute = DictAttribute {
    caption: "Physical Height",
    default: None,
    description: "The numeric physical height of display.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PHYSICAL_ORIENTATION: DictAttribute = DictAttribute {
    caption: "Physical Orientation",
    default: None,
    description: "The numeric physical orientation of display.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PHYSICAL_WIDTH: DictAttribute = DictAttribute {
    caption: "Physical Width",
    default: None,
    description: "The numeric physical width of display.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PID: DictAttribute = DictAttribute {
    caption: "Process ID",
    default: None,
    description: "The process identifier, as reported by the operating system. Process ID (PID) is a number used by the operating system to uniquely identify an active process.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PIXEL_BITS: DictAttribute = DictAttribute {
    caption: "Pixel Bits",
    default: None,
    description: "The number of bits per pixel.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const POLICY: DictAttribute = DictAttribute {
    caption: "Policy",
    default: None,
    description: "Describes details of a policy. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "policy" },
};

pub const PORT: DictAttribute = DictAttribute {
    caption: "Port",
    default: None,
    description: "The TCP/UDP port number associated with a connection. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "port_t" },
};

pub const POSTAL_CODE: DictAttribute = DictAttribute {
    caption: "Postal Code",
    default: None,
    description: "The postal code of the location.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PREFIX: DictAttribute = DictAttribute {
    caption: "Prefix",
    default: None,
    description: "The domain prefix.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PREV_REG_KEY: DictAttribute = DictAttribute {
    caption: "Previous Registry Key",
    default: None,
    description: "The registry key before the mutation",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "registry_key",
    },
};

pub const PREV_REG_VALUE: DictAttribute = DictAttribute {
    caption: "Previous Registry Value",
    default: None,
    description: "The registry value before the mutation",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "registry_value",
    },
};

pub const PRIORITY: DictAttribute = DictAttribute {
    caption: "Priority",
    default: None,
    description: "The priority, normalized to the caption of the priority_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PRIORITY_ID: DictAttribute = DictAttribute {
    caption: "Priority ID",
    default: None,
    description: "The normalized priority. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("priority"),
    attr_type: TypeNames::Integer,
};

pub const PRIVILEGES: DictAttribute = DictAttribute {
    caption: "Privileges",
    default: None,
    description: "The user or group privileges.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROCESS: DictAttribute = DictAttribute {
    caption: "Process",
    default: None,
    description: "The process object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "process" },
};

pub const PROCESSED_TIME: DictAttribute = DictAttribute {
    caption: "Processed Time",
    default: None,
    description: "The event processed time, such as an ETL operation.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const PRODUCT: DictAttribute = DictAttribute {
    caption: "Product",
    default: None,
    description: "The product that reported the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "product" },
};

pub const PRODUCT_UID: DictAttribute = DictAttribute {
    caption: "Product Identifier",
    default: None,
    description: "Unique Identifier of a product.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROFILES: DictAttribute = DictAttribute {
    caption: "Profiles",
    default: None,
    description: "The list of profiles used to create the event.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROJECT_UID: DictAttribute = DictAttribute {
    caption: "Project ID",
    default: None,
    description: "The unique identifier of a Cloud project.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROTOCOL_NAME: DictAttribute = DictAttribute {
    caption: "Protocol Name",
    default: None,
    description: "The TCP/IP protocol name in lowercase, as defined by the Internet Assigned Numbers Authority (IANA). See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>tcp</code> or <code>udp</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROTOCOL_NUM: DictAttribute = DictAttribute {
    caption: "Protocol Number",
    default: None,
    description: "The TCP/IP protocol number, as defined by the Internet Assigned Numbers Authority (IANA). Use -1 if the protocol is not defined by IANA. See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>6</code> for TCP and <code>17</code> for UDP.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const PROTOCOL_VER: DictAttribute = DictAttribute {
    caption: "Protocol Version",
    default: None,
    description: "The Protocol version, normalized to the caption of the protocol_ver_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROTOCOL_VER_ID: DictAttribute = DictAttribute {
    caption: "Protocol Version ID",
    default: None,
    description: "The normalized identifier of the Protocol version.",
    attr_enum: None,
    is_array: None,
    sibling: Some("protocol_ver"),
    attr_type: TypeNames::Integer,
};

pub const PROVIDER: DictAttribute = DictAttribute {
    caption: "Provider",
    default: None,
    description: "The origin of information associated with the event. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const PROXY: DictAttribute = DictAttribute {
    caption: "Proxy",
    default: None,
    description:
        "If a proxy connection is present, the connection from the client to the proxy server.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_proxy",
    },
};

pub const QUERY: DictAttribute = DictAttribute {
    caption: "DNS Query",
    default: None,
    description: "The Domain Name System (DNS) query.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "dns_query" },
};

pub const QUERY_STRING: DictAttribute = DictAttribute {
    caption: "HTTP Query String",
    default: None,
    description: "The query portion of the URL. For example: the query portion of the URL <code>http://www.example.com/search?q=bad&sort=date</code> is <code>q=bad&sort=date</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const QUERY_TIME: DictAttribute = DictAttribute {
    caption: "Query Time",
    default: None,
    description: "The Domain Name System (DNS) query time.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const RAM_SIZE: DictAttribute = DictAttribute {
    caption: "RAM Size",
    default: None,
    description:
        "The ctotal amount of installed RAM, in Megabytes. For example: <code>2048</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const RAW_DATA: DictAttribute = DictAttribute {
    caption: "Raw Data",
    default: None,
    description: "The event data as received from the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RAW_HEADER: DictAttribute = DictAttribute {
    caption: "Raw Header",
    default: None,
    description: "The email authentication header.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RCODE: DictAttribute = DictAttribute {
    caption: "Response Code",
    default: None,
    description: "The server response code, normalized to the caption of the rcode_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RCODE_ID: DictAttribute = DictAttribute {
    caption: "Response Code ID",
    default: None,
    description: "The normalized identifier of the server response code. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("rcode"),
    attr_type: TypeNames::Integer,
};

pub const RDATA: DictAttribute = DictAttribute {
    caption: "DNS RData",
    default: None,
    description: "The data describing the DNS resource. The meaning of this data depends on the type and class of the resource record.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const REFERENCES: DictAttribute = DictAttribute {
    caption: "References",
    default: None,
    description: "Supporting reference URLs",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const REFERRER: DictAttribute = DictAttribute {
    caption: "HTTP Referrer",
    default: None,
    description: "The request header that identifies the address of the previous web page, which is linked to the current web page or resource being requested.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const REG_KEY: DictAttribute = DictAttribute {
    caption: "Registry Key",
    default: None,
    description: "The registry key.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "registry_key",
    },
};

pub const REG_VALUE: DictAttribute = DictAttribute {
    caption: "Registry Value",
    default: None,
    description: "The registry value.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "registry_value",
    },
};

pub const REGION: DictAttribute = DictAttribute {
    caption: "Region",
    default: None,
    description: "The name or the code of a region. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const REGISTRAR: DictAttribute = DictAttribute {
    caption: "Domain Registrar",
    default: None,
    description: "The domain registrar.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RELATED_EVENTS: DictAttribute = DictAttribute {
    caption: "Related Events",
    default: None,
    description:
        "Describes events related to a finding or detection as identified by the security product.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "related_event",
    },
};

pub const RELATED_VULNERABILITIES: DictAttribute = DictAttribute {
    caption: "Related Vulnerabilities",
    default: None,
    description: "List of vulnerabilities that are related to this vulnerability.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RELAY: DictAttribute = DictAttribute {
    caption: "Relay",
    default: None,
    description: "The network relay that is associated with the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_interface",
    },
};

pub const RELEASE: DictAttribute = DictAttribute {
    caption: "Software Release Details",
    default: None,
    description: "Release is the number of times a version of the software has been packaged.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const REMEDIATION: DictAttribute = DictAttribute {
    caption: "Remediation",
    default: None,
    description: "The remediation recommendations on how to fix the identified issue(s).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "remediation",
    },
};

pub const REMOTE_DISPLAY: DictAttribute = DictAttribute {
    caption: "Remote Display",
    default: None,
    description: "The remote display affiliated with the event",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "display" },
};

pub const REPLY_TO: DictAttribute = DictAttribute {
    caption: "Reply To",
    default: None,
    description: "The email header Reply-To values, as defined by RFC 5322.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const REPUTATION: DictAttribute = DictAttribute {
    caption: "Reputation Scores",
    default: None,
    description: "Contains the original and normalized reputation scores.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "reputation" },
};

pub const REQUEST: DictAttribute = DictAttribute {
    caption: "API Request Details",
    default: None,
    description: "General Purpose API Request Object. See specific usage",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "request" },
};

pub const REQUESTED_PERMISSIONS: DictAttribute = DictAttribute {
    caption: "Requested Permissions",
    default: None,
    description: "The permissions mask that were requested by the process.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const REQUIREMENTS: DictAttribute = DictAttribute {
    caption: "Requirements",
    default: None,
    description:
        "A list of applicable compliance requirements for which this finding is related to.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RESOURCE: DictAttribute = DictAttribute {
    caption: "Resource",
    default: None,
    description: "The target resource.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "resource" },
};

pub const RESOURCE_TYPE: DictAttribute = DictAttribute {
    caption: "Resource Type",
    default: None,
    description: "The context in which a resource was retrieved in a web request.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RESOURCE_UID: DictAttribute = DictAttribute {
    caption: "Resource ID",
    default: None,
    description:
        "The unique identifier of a cloud resource. For example, S3 Bucket name, EC2 Instance Id.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "resource_uid_t",
    },
};

pub const RESOURCES: DictAttribute = DictAttribute {
    caption: "Resources Array",
    default: None,
    description: "Describes details about resources that were affected by the activity/event.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "resource" },
};

pub const RESPONSE: DictAttribute = DictAttribute {
    caption: "API Response Details",
    default: None,
    description: "General Purpose API Response Object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "response" },
};

pub const RESPONSE_TIME: DictAttribute = DictAttribute {
    caption: "Response Time",
    default: None,
    description: "The Domain Name System (DNS) response time.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const RISK_LEVEL: DictAttribute = DictAttribute {
    caption: "Risk Level",
    default: None,
    description: "The risk level, normalized to the caption of the risk_level_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RISK_LEVEL_ID: DictAttribute = DictAttribute {
    caption: "Risk Level ID",
    default: None,
    description: "The normalized risk level id.",
    attr_enum: None,
    is_array: None,
    sibling: Some("risk_level"),
    attr_type: TypeNames::Integer,
};

pub const RISK_SCORE: DictAttribute = DictAttribute {
    caption: "Risk Score",
    default: None,
    description: "The risk score as reported by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const RULE: DictAttribute = DictAttribute {
    caption: "Rule",
    default: None,
    description: "The rules that reported the events.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "rule" },
};

pub const RUN_STATE: DictAttribute = DictAttribute {
    caption: "Run State",
    default: None,
    description: "The state of the job or service, normalized to the caption of the run_state_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const RUN_STATE_ID: DictAttribute = DictAttribute {
    caption: "Run State ID",
    default: None,
    description:
        "The normalized identifier of the state of the job or service. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("run_state"),
    attr_type: TypeNames::Integer,
};

pub const RUNTIME: DictAttribute = DictAttribute {
    caption: "Runtime",
    default: None,
    description: "The runtime managing this container.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SAMESITE: DictAttribute = DictAttribute {
    caption: "SameSite",
    default: None,
    description: "The cookie attribute that lets servers specify whether/when cookies are sent with cross-site requests. Values are: Strict, Lax or None",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SANDBOX: DictAttribute = DictAttribute {
    caption: "Sandbox",
    default: None,
    description: "The name of the containment jail (i.e., sandbox). For example, hardened_ps, high_security_ps, oracle_ps, netsvcs_ps, or default_ps.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SANS: DictAttribute = DictAttribute {
    caption: "Subject Alternative Names",
    default: None,
    description:
        "The list of subject alternative names that are secured by a specific certificate.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "san" },
};

pub const SCALE_FACTOR: DictAttribute = DictAttribute {
    caption: "Scale Factor",
    default: None,
    description: "The numeric scale factor of display.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const SCHEME: DictAttribute = DictAttribute {
    caption: "Scheme",
    default: None,
    description: "The scheme portion of the URL. For example: <code>http</code>, <code>https</code>, <code>ftp</code>, or <code>sftp</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SCORE: DictAttribute = DictAttribute {
    caption: "Reputation Score",
    default: None,
    description: "The reputation score, normalized to the caption of the score_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SCORE_ID: DictAttribute = DictAttribute {
    caption: "Reputation Score ID",
    default: None,
    description: "The normalized reputation score identifier.",
    attr_enum: None,
    is_array: None,
    sibling: Some("score"),
    attr_type: TypeNames::Integer,
};

pub const SECURE: DictAttribute = DictAttribute {
    caption: "Secure",
    default: None,
    description: "The cookie attribute to only send cookies to the server with an encrypted request over the HTTPS protocol.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "boolean_t" },
};

pub const SECURITY_DESCRIPTOR: DictAttribute = DictAttribute {
    caption: "Security Descriptor",
    default: None,
    description: "The object security descriptor.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SEQUENCE: DictAttribute = DictAttribute {
    caption: "Sequence Number",
    default: None,
    description: "Sequence number of the event. The sequence number is a value available in some events, to make the exact ordering of events unambiguous, regardless of the event time precision.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const SERIAL_NUMBER: DictAttribute = DictAttribute {
    caption: "Serial Number",
    default: None,
    description: "The serial number that pertains to the object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SERVER_CIPHERS: DictAttribute = DictAttribute {
    caption: "Server Cipher Suites",
    default: None,
    description:
        "The server cipher suites that were exchanged during the TLS handshake negotiation.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SERVER_HASSH: DictAttribute = DictAttribute {
    caption: "Server HASSH",
    default: None,
    description: "The Server HASSH fingerprinting object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "hassh" },
};

pub const SERVICE: DictAttribute = DictAttribute {
    caption: "Service",
    default: None,
    description: "The service that pertains to the event.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "service" },
};

pub const SESSION: DictAttribute = DictAttribute {
    caption: "User Session",
    default: None,
    description: "The authenticated user session.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "session" },
};

pub const SEVERITY: DictAttribute = DictAttribute {
    caption: "Severity",
    default: None,
    description: "The event severity, normalized to the caption of the severity_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SEVERITY_ID: DictAttribute = DictAttribute {
    caption: "Severity ID",
    default: None,
    description: "The normalized identifier of the event severity.</p>The normalized severity is a measurement the effort and expense required to manage and resolve an event or incident. Smaller numerical values represent lower impact events, and larger numerical values represent higher impact events.",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "severity",
    ),
    attr_type: TypeNames::Integer,
};

pub const SHARE: DictAttribute = DictAttribute {
    caption: "Share",
    default: None,
    description: "The SMB share name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SHARE_TYPE: DictAttribute = DictAttribute {
    caption: "Share Type",
    default: None,
    description: "The SMB share type.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SIGNATURE: DictAttribute = DictAttribute {
    caption: "Digital Signature",
    default: None,
    description: "The digital signature of the file.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "digital_signature",
    },
};

pub const SIZE: DictAttribute = DictAttribute {
    caption: "Size",
    default: None,
    description: "The size of data, in bytes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "long_t" },
};

pub const SMTP_FROM: DictAttribute = DictAttribute {
    caption: "SMTP From",
    default: None,
    description: "The value of the SMTP MAIL FROM command.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const SMTP_HELLO: DictAttribute = DictAttribute {
    caption: "SMTP Hello",
    default: None,
    description: "The value of the SMTP HELO or EHLO command.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SMTP_TO: DictAttribute = DictAttribute {
    caption: "SMTP To",
    default: None,
    description: "The value of the SMTP envelope RCPT TO command.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const SNI: DictAttribute = DictAttribute {
    caption: "Server Name Indication",
    default: None,
    description: " The Server Name Indication (SNI) extension sent by the client.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SP_NAME: DictAttribute = DictAttribute {
    caption: "OS Service Pack",
    default: None,
    description: "The name of the latest Service Pack.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SP_VER: DictAttribute = DictAttribute {
    caption: "OS Service Pack Version",
    default: None,
    description: "The version number of the latest Service Pack.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const SPF: DictAttribute = DictAttribute {
    caption: "SPF Status",
    default: None,
    description: "The Sender Policy Framework (SPF) status of the email.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SRC_ENDPOINT: DictAttribute = DictAttribute {
    caption: "Source Endpoint",
    default: None,
    description: "The network source endpoint.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "network_endpoint",
    },
};

pub const SRC_URL: DictAttribute = DictAttribute {
    caption: "Source URL",
    default: None,
    description: "The URL pointing towards the source of an entity. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const START_ADDRESS: DictAttribute = DictAttribute {
    caption: "Start Address",
    default: None,
    description: "The start address of the execution.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const START_TIME: DictAttribute = DictAttribute {
    caption: "Start Time",
    default: None,
    description: "The start time of a time period. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const STATE: DictAttribute = DictAttribute {
    caption: "State",
    default: None,
    description: "The state of the event or object, normalized to the caption of the state_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const STATE_ID: DictAttribute = DictAttribute {
    caption: "State ID",
    default: None,
    description: "The normalized state ID of the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("state"),
    attr_type: TypeNames::Integer,
};

pub const STATUS: DictAttribute = DictAttribute {
    caption: "Status",
    default: None,
    description: "The event status, normalized to the caption of the status_id value. In the case of 'Other', it is defined by the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const STATUS_CODE: DictAttribute = DictAttribute {
    caption: "Status Code",
    default: None,
    description: "The event status code, as reported by the event source.<br /><br />For example, in a Windows Failed Authentication event, this would be the value of 'Failure Code', e.g. 0x18.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const STATUS_DETAIL: DictAttribute = DictAttribute {
    caption: "Status Details",
    default: None,
    description: "The status details contains additional information about the event outcome.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const STATUS_ID: DictAttribute = DictAttribute {
    caption: "Status ID",
    default: None,
    description: "The normalized identifier of the event status.",
    attr_enum: None,
    is_array: None,
    sibling: Some("status"),
    attr_type: TypeNames::Integer,
};

pub const SUBJECT: DictAttribute = DictAttribute {
    caption: "Subject",
    default: None,
    description: "The email header Subject value, as defined by RFC 5322.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SUBJECT_DN: DictAttribute = DictAttribute {
    caption: "Subject Distinguished Name",
    default: None,
    description: "The certificate subject distinguished name.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SUBNET: DictAttribute = DictAttribute {
    caption: "Subnet",
    default: None,
    description: "The subnet mask.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "subnet_t" },
};

pub const SUBNET_UID: DictAttribute = DictAttribute {
    caption: "Subnet UID",
    default: None,
    description: "The unique identifier of a virtual subnet.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SUPPORTING_DATA: DictAttribute = DictAttribute {
    caption: "Supporting Data",
    default: None,
    description: "Additional data supporting a finding as provided by security tool",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::Json,
};

pub const SVC_NAME: DictAttribute = DictAttribute {
    caption: "Service Name",
    default: None,
    description: "The service name in service-to-service connections. For example, AWS VPC logs the pkt-src-aws-service and pkt-dst-aws-service fields identify the connection is coming from or going to an AWS service.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const SYSTEM_CALL: DictAttribute = DictAttribute {
    caption: "System Call",
    default: None,
    description: "The system call that was invoked.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TACTICS: DictAttribute = DictAttribute {
    caption: "Tactics",
    default: None,
    description: "The a list of tactic ID's/names that are associated with the attack technique, as defined by <a target='_blank' href='https://attack.mitre.org/wiki/ATT&CK_Matrix'>ATT&CK Matrix<sup>TM</sup></a>.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "tactic" },
};

pub const TAG: DictAttribute = DictAttribute {
    caption: "Image Tag",
    default: None,
    description: "The image tag. For example: <code>1.11-alpine</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TCP_FLAGS: DictAttribute = DictAttribute {
    caption: "TCP Flags",
    default: None,
    description: "The network connection TCP header flags (i.e., control bits).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const TECHNIQUE: DictAttribute = DictAttribute {
    caption: "Technique",
    default: None,
    description: "The attack technique.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "technique" },
};

pub const TERMINATED_TIME: DictAttribute = DictAttribute {
    caption: "Terminated Time",
    default: None,
    description: "The time when the entity was terminated. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const TEXT: DictAttribute = DictAttribute {
    caption: "URL Text",
    default: None,
    description: "The URL. For example: <code>http://www.example.com/download/trouble.exe</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "url_t" },
};

pub const TID: DictAttribute = DictAttribute {
    caption: "Thread ID",
    default: None,
    description: "The Identifier of the thread associated with the event, as returned by the operating system.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const TIME: DictAttribute = DictAttribute {
    caption: "Event Time",
    default: None,
    description: "The normalized event occurrence time.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Timestamp,
};

pub const TIMEZONE_OFFSET: DictAttribute = DictAttribute {
    caption: "Timezone Offset",
    default: None,
    description: "The number of minutes that the reported event <code>time</code> is ahead or behind UTC, in the range -1,080 to +1,080.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const TITLE: DictAttribute = DictAttribute {
    caption: "Title",
    default: None,
    description: "The title of an entity. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TLS: DictAttribute = DictAttribute {
    caption: "TLS",
    default: None,
    description: "The Transport Layer Security (TLS) attributes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "tls" },
};

pub const TO: DictAttribute = DictAttribute {
    caption: "To",
    default: None,
    description: "The email header To values, as defined by RFC 5322.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "email_t" },
};

pub const TRAFFIC: DictAttribute = DictAttribute {
    caption: "Traffic",
    default: None,
    description: "The network traffic refers to the amount of data moving across a network at a given point of time. Intended to be used alongside Network Connection.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "network_traffic" },
};

pub const TRANSACTION_UID: DictAttribute = DictAttribute {
    caption: "Transaction UID",
    default: None,
    description: "The unique identifier of the transaction.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TREE_UID: DictAttribute = DictAttribute {
    caption: "Tree UID",
    default: None,
    description:
        "The tree id is a unique SMB identifier which represents an open connection to a share.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TRUE_COLOR: DictAttribute = DictAttribute {
    caption: "True Color",
    default: None,
    description:
        "A boolean indicating whether to extract pixel values through red/green/blue intensities.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "boolean_t" },
};

pub const TTL: DictAttribute = DictAttribute {
    caption: "TTL",
    default: None,
    description: "The time interval that the resource record may be cached. Zero value means that the resource record can only be used for the transaction in progress, and should not be cached.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::Integer,
};

pub const TYPE: DictAttribute = DictAttribute {
    caption: "Type",
    default: None,
    description: "The type of an object or value, normalized to the caption of the type_id value. In the case of 'Other', it is defined by the event source. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TYPE_ID: DictAttribute = DictAttribute {
    caption: "Type ID",
    default: Some(0),
    description: "The normalized type identifier of an object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: Some("type"),
    attr_type: TypeNames::Integer,
};

pub const TYPE_NAME: DictAttribute = DictAttribute {
    caption: "Type Name",
    default: None,
    description: "The event type name, as defined by the type_uid.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const TYPE_UID: DictAttribute = DictAttribute {
    caption: "Type ID",
    default: None,
    description: "The event type ID. It identifies the event's semantics and structure. The value is calculated by the logging system as: <code>class_uid * 100 + activity_id</code>.",
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "type_name",
    ),
    attr_type: TypeNames::Integer,
};

pub const TYPES: DictAttribute = DictAttribute {
    caption: "Types",
    default: None,
    description: "The type/s of an entity. See specific usage.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::String,
};

pub const UID: DictAttribute = DictAttribute {
    caption: "Unique ID",
    default: None,
    description: "The unique identifier. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const UNMAPPED: DictAttribute = DictAttribute {
    caption: "Unmapped Data",
    default: None,
    description: "The attributes that are not mapped to the event schema. The names and values of those attributes are specific to the event source.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "object" },
};

pub const URL: DictAttribute = DictAttribute {
    caption: "URL",
    default: None,
    description: "The URL object that pertains to the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "url" },
};

pub const USER: DictAttribute = DictAttribute {
    caption: "User",
    default: None,
    description: "The user that pertains to the event or object.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "user" },
};

pub const USER_AGENT: DictAttribute = DictAttribute {
    caption: "HTTP User-Agent",
    default: None,
    description: "The request header that identifies the operating system and web browser.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const USER_RESULT: DictAttribute = DictAttribute {
    caption: "User Result",
    default: None,
    description: "The result of the user account change. It should contain the new values of the changed attributes.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "user" },
};

pub const UUID: DictAttribute = DictAttribute {
    caption: "UUID",
    default: None,
    description: "The universally unique identifier. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VALUE: DictAttribute = DictAttribute {
    caption: "Value",
    default: None,
    description: "The value that pertains to the object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VECTOR_STRING: DictAttribute = DictAttribute {
    caption: "Vector String",
    default: None,
    description: "The CVSS vector string is a text representation of a set of CVSS metrics. It is commonly used to record or transfer CVSS metric information in a concise form. For example: <code>3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:N/A:H</code>.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VENDOR_NAME: DictAttribute = DictAttribute {
    caption: "Vendor Name",
    default: None,
    description: "The name of the vendor. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VERSION: DictAttribute = DictAttribute {
    caption: "Version",
    default: None,
    description: "The version that pertains to the event or object. See specific usage.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VLAN_UID: DictAttribute = DictAttribute {
    caption: "VLAN",
    default: None,
    description: "The Virtual LAN identifier.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VNC_AUTH: DictAttribute = DictAttribute {
    caption: "VNC Authentication",
    default: None,
    description: "The Virtual Network Computing (VNC) authentication object describes the VNC authentication values.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "vnc_auth" },
};

pub const VPC_UID: DictAttribute = DictAttribute {
    caption: "VPC UID",
    default: None,
    description: "The unique identifier of the Virtual Private Cloud (VPC).",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};

pub const VULNERABILITIES: DictAttribute = DictAttribute {
    caption: "Vulnerabilities",
    default: None,
    description: "This object describes vulnerabilities reported in a security finding.",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "vulnerability",
    },
};

pub const VULNERABILITY: DictAttribute = DictAttribute {
    caption: "Vulnerability",
    default: None,
    description: "The vulnerability object describes details related to the observed vulnerability",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "vulnerability",
    },
};

pub const WIN_RESOURCE: DictAttribute = DictAttribute {
    caption: "Windows Resource",
    default: None,
    description: "The Windows resource object that was accessed, such as a mutant or timer.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported {
        name: "win_resource",
    },
};

pub const X_FORWARDED_FOR: DictAttribute = DictAttribute {
    caption: "X-Forwarded-For",
    default: None,
    description: "The X-Forwarded-For header identifying the originating IP address(es) of a client connecting to a web server through an HTTP proxy or a load balancer.",
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "ip_t" },
};

pub const X_ORIGINATING_IP: DictAttribute = DictAttribute {
    caption: "X-Originating-IP",
    default: None,
    description: "The X-Originating-IP header identifying the emails originating IP address(es).",
    attr_enum: None,
    is_array: Some(true),
    sibling: None,
    attr_type: TypeNames::NotSupported { name: "ip_t" },
};

pub const XATTRIBUTES: DictAttribute = DictAttribute {
    caption: "Extended Attributes",
    default: None,
    description: "An unordered collection of zero or more name/value pairs where each pair represents a file or folder extended attribute.</p>For example: Windows alternate data stream attributes (ADS stream name, ADS size, etc.), user-defined or application-defined attributes, ACL, owner, primary group, etc. Examples from DCS: </p><ul><li><strong>ads_name</strong></li><li><strong>ads_size</strong></li><li><strong>dacl</strong></li><li><strong>owner</strong></li><li><strong>primary_group</strong></li><li><strong>link_name</strong> - name of the link associated to the file.</li><li><strong>hard_link_count</strong> - the number of links that are associated to the file.</li></ul>",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::NotSupported{ name: "object" },
};

pub const ZONE: DictAttribute = DictAttribute {
    caption: "Network Zone",
    default: None,
    description: "The network zone or LAN segment.",
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: TypeNames::String,
};
