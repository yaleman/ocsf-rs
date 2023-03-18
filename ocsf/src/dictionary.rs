//* "The Attribute Dictionary defines attributes and includes references to the events and objects in which they are used."
use serde::{Deserialize, Serialize};

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
lazy_static! {
    static ref ACCESS_LIST: DictAttribute = DictAttribute {
        caption: Some("Access List".to_string(),),
        default: None,
        description: Some("The list of requested access rights.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCESS_MASK: DictAttribute = DictAttribute {
        caption: Some("Access Mask".to_string(),),
        default: None,
        description: Some("The access mask in a platform-native format.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCESS_RESULT: DictAttribute = DictAttribute {
        caption: Some("Access Check Result".to_string(),),
        default: None,
        description: Some("The list of access check results.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCESSED_TIME: DictAttribute = DictAttribute {
        caption: Some("Accessed Time".to_string(),),
        default: None,
        description: Some("The time when the file was last accessed.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCESSOR: DictAttribute = DictAttribute {
        caption: Some("Accessor".to_string(),),
        default: None,
        description: Some("The name of the user who last accessed the object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCOUNT_NAME: DictAttribute = DictAttribute {
        caption: Some("Account Name".to_string(),),
        default: None,
        description: Some("The name of the account (e.g. AWS Account Name).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ACCOUNT_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Account Type".to_string(),
    ),
    default: None,
    description: Some(
        "The user account type, normalized to the caption of 'account_type_id'. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ACCOUNT_TYPE_ID: DictAttribute = DictAttribute {
        caption: Some("Account Type ID".to_string(),),
        default: None,
        description: Some("The normalized user account type identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("account_type".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref ACCOUNT_UID: DictAttribute = DictAttribute {
        caption: Some("Account UID".to_string(),),
        default: None,
        description: Some(
            "The unique identifier of the account (e.g. AWS Account ID).".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ACTIVITY_ID: DictAttribute = DictAttribute {
        caption: Some("Activity ID".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the activity that triggered the event.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("activity_name".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref ACTIVITY_NAME: DictAttribute = DictAttribute {
        caption: Some("Activity".to_string(),),
        default: None,
        description: Some("The event activity name, as defined by the activity_id.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ACTOR: DictAttribute = DictAttribute {
    caption: Some(
        "Actor".to_string(),
    ),
    default: None,
    description: Some(
        "The actor object describes details about the user/role/process that was the source of the activity.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ACTUAL_PERMISSIONS: DictAttribute = DictAttribute {
        caption: Some("Actual Permissions".to_string(),),
        default: None,
        description: Some(
            "The permissions that were granted to the in a platform-native format.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ALERT: DictAttribute = DictAttribute {
    caption: Some(
        "Client TLS Alert".to_string(),
    ),
    default: None,
    description: Some(
        "The integer value of TLS alert if present. The alerts are defined in the TLS specification in <a target='_blank' href='https://datatracker.ietf.org/doc/html/rfc2246'>RFC-2246</a>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref ALGORITHM: DictAttribute = DictAttribute {
    caption: Some(
        "Algorithm".to_string(),
    ),
    default: None,
    description: Some(
        "The hash algorithm used to create the digital fingerprint, normalized to the caption of 'algorithm_id'. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref ALGORITHM_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Algorithm ID".to_string(),
    ),
    default: None,
    description: Some(
        "The identifier of the normalized hash algorithm, which was used to create the digital fingerprint.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "algorithm".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref ANSWERS: DictAttribute = DictAttribute {
        caption: Some("DNS Answer".to_string(),),
        default: None,
        description: Some("The Domain Name System (DNS) answers.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref API: DictAttribute = DictAttribute {
        caption: Some("API details".to_string(),),
        default: None,
        description: Some("API object details information pertaining to the API calls".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref APP_NAME: DictAttribute = DictAttribute {
        caption: Some("Application Name".to_string(),),
        default: None,
        description: Some(
            "The name of the application that is associated with the event or object.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ARCHITECTURE: DictAttribute = DictAttribute {
    caption: Some(
        "Architecture".to_string(),
    ),
    default: None,
    description: Some(
        "Architecture is a shorthand name describing the type of computer hardware the packaged software is meant to run on.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ARGS: DictAttribute = DictAttribute {
        caption: Some("HTTP Arguments".to_string(),),
        default: None,
        description: Some("The arguments sent along with the HTTP request.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ATTACKS: DictAttribute = DictAttribute {
        caption: Some("Attacks".to_string(),),
        default: None,
        description: Some("An array of attacks associated with an event.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ATTEMPT: DictAttribute = DictAttribute {
        caption: Some("Attempt".to_string(),),
        default: None,
        description: Some("The delivery attempt.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ATTRIBUTES: DictAttribute = DictAttribute {
        caption: Some("Attributes".to_string(),),
        default: None,
        description: Some("The bitmask value that represents the file attributes.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref AUTH_PROTOCOL: DictAttribute = DictAttribute {
    caption: Some(
        "Auth Protocol".to_string(),
    ),
    default: None,
    description: Some(
        "The authentication protocol as defined by the caption of 'auth_protocol_id'. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref AUTH_PROTOCOL_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Auth Protocol ID".to_string(),
    ),
    default: None,
    description: Some(
        "The normalized identifier of the authentication protocol used to create the user session.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "auth_protocol".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! { static ref AUTH_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Authentication Type".to_string(),
    ),
    default: None,
    description: Some(
        "The agreed upon authentication type, normalized to the caption of 'auth_type_id'. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref AUTH_TYPE_ID: DictAttribute = DictAttribute {
        caption: Some("Authentication Type ID".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the agreed upon authentication type. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("auth_type".to_string(),),
        attr_type: None,
    };
}

lazy_static! { static ref AUTHORIZATIONS: DictAttribute = DictAttribute {
    caption: Some(
        "Authorization Information".to_string(),
    ),
    default: None,
    description: Some(
        "This object provides details such as authorization outcome, associated policies related to activity/event.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref AUTOSCALE_UID: DictAttribute = DictAttribute {
        caption: Some("Autoscale UID".to_string(),),
        default: None,
        description: Some(
            "The unique identifier of the cloud autoscale configuration.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref BANNER: DictAttribute = DictAttribute {
    caption: Some(
        "SMTP Banner".to_string(),
    ),
    default: None,
    description: Some(
        "The initial SMTP connection response that a messaging server receives after it connects to a email server.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref BASE_ADDRESS: DictAttribute = DictAttribute {
        caption: Some("Base Address".to_string(),),
        default: None,
        description: Some("The memory address where the module was loaded.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BASE_SCORE: DictAttribute = DictAttribute {
        caption: Some("Base Score".to_string(),),
        default: None,
        description: Some(
            "The base score as reported by the event source. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BIG_ENDIAN: DictAttribute = DictAttribute {
        caption: Some("Big Endian".to_string(),),
        default: None,
        description: Some(
            "A boolean indicating whether the most significant byte is stored/transmitted first."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BIOS_DATE: DictAttribute = DictAttribute {
        caption: Some("BIOS Date".to_string(),),
        default: None,
        description: Some("The BIOS date. For example: <code>03/31/16</code>.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BIOS_MANUFACTURER: DictAttribute = DictAttribute {
        caption: Some("BIOS Manufacturer".to_string(),),
        default: None,
        description: Some("The BIOS manufacturer. For example: <code>LENOVO</code>.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BIOS_VER: DictAttribute = DictAttribute {
        caption: Some("BIOS Version".to_string(),),
        default: None,
        description: Some(
            "The BIOS version. For example: <code>LENOVO G5ETA2WW (2.62)</code>.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref BOUNDARY: DictAttribute = DictAttribute {
    caption: Some(
        "Boundary".to_string(),
    ),
    default: None,
    description: Some(
        "The boundary of the connection, normalized to the caption of 'boundary_id'. In the case of 'Other', it is defined by the event source. <p> For cloud connections, this translates to the traffic-boundary(same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref BOUNDARY_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Boundary ID".to_string(),
    ),
    default: None,
    description: Some(
        "The normalized identifier of the boundary of the connection. </p> For cloud connections, this translates to the traffic-boundary (same VPC, through IGW, etc.). For traditional networks, this is described as Local, Internal, or External.</p>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "boundary".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref BUILD: DictAttribute = DictAttribute {
        caption: Some("OS Build".to_string(),),
        default: None,
        description: Some("The operating system build number.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BYTES: DictAttribute = DictAttribute {
        caption: Some("Total Bytes".to_string(),),
        default: Some(0,),
        description: Some("The total number of bytes (in and out).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BYTES_IN: DictAttribute = DictAttribute {
        caption: Some("Bytes In".to_string(),),
        default: Some(0,),
        description: Some(
            "The number of bytes sent from the destination to the source.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref BYTES_OUT: DictAttribute = DictAttribute {
        caption: Some("Bytes Out".to_string(),),
        default: Some(0,),
        description: Some(
            "The number of bytes sent from the source to the destination.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CAPABILITIES: DictAttribute = DictAttribute {
        caption: Some("Capabilities".to_string(),),
        default: None,
        description: Some("A list of RDP capabilities.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CAPTION: DictAttribute = DictAttribute {
    caption: Some(
        "Caption".to_string(),
    ),
    default: None,
    description: Some(
        "A short description or caption of the device. For example: <code>Scanner 1</code> or <code>Database Manager</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CATEGORIES: DictAttribute = DictAttribute {
    caption: Some(
        "Website Categorization".to_string(),
    ),
    default: None,
    description: Some(
        "The Website categorization names, as defined by <code>category_ids</code> enum values.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CATEGORY: DictAttribute = DictAttribute {
        caption: Some("Category".to_string(),),
        default: None,
        description: Some("The object category. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CATEGORY_IDS: DictAttribute = DictAttribute {
        caption: Some("Website Categorization IDs".to_string(),),
        default: None,
        description: Some("The Website categorization identifies.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: Some("categories".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref CATEGORY_NAME: DictAttribute = DictAttribute {
        caption: Some("Category".to_string(),),
        default: None,
        description: Some("The event category name, as defined by category_uid value.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CATEGORY_UID: DictAttribute = DictAttribute {
        caption: Some("Category ID".to_string(),),
        default: Some(0,),
        description: Some("The category unique identifier of the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("category_name".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref CC: DictAttribute = DictAttribute {
        caption: Some("Cc".to_string(),),
        default: None,
        description: Some("The email header Cc values, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CERTIFICATE: DictAttribute = DictAttribute {
        caption: Some("Certificate".to_string(),),
        default: None,
        description: Some(
            "The certificate object containing information about the digital certificate."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CERTIFICATE_CHAIN: DictAttribute = DictAttribute {
    caption: Some(
        "Certificate Chain".to_string(),
    ),
    default: None,
    description: Some(
        "The Chain of Certificate Serial Numbers field provides a chain of Certificate Issuer Serial Numbers leading to the Root Certificate Issuer.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CHALLENGE: DictAttribute = DictAttribute {
        caption: Some("Challenge".to_string(),),
        default: None,
        description: Some("The VNC challenge".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CHALLENGE_RESPONSE: DictAttribute = DictAttribute {
        caption: Some("Challenge Response".to_string(),),
        default: None,
        description: Some("The VNC challenge response".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CHASSIS: DictAttribute = DictAttribute {
    caption: Some(
        "Chassis".to_string(),
    ),
    default: None,
    description: Some(
        "The chassis type describes the system enclosure or physical form factor. Such as the following examples for Windows <a target='_blank' href='https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-systemenclosure'>Windows Chassis Types</a>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CIPHER: DictAttribute = DictAttribute {
        caption: Some("Cipher Suite".to_string(),),
        default: None,
        description: Some("The negotiated cipher suite.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CIS_BENCHMARK_RESULT: DictAttribute = DictAttribute {
        caption: Some("CIS Benchmark Result".to_string(),),
        default: None,
        description: Some("The CIS benchmark result.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CITY: DictAttribute = DictAttribute {
        caption: Some("City".to_string(),),
        default: None,
        description: Some("The name of the city.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CLASS: DictAttribute = DictAttribute {
        caption: Some("Class".to_string(),),
        default: None,
        description: Some("The class name of the object. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CLASS_NAME: DictAttribute = DictAttribute {
        caption: Some("Class".to_string(),),
        default: None,
        description: Some("The event class name, as defined by class_uid value.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CLASS_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Class ID".to_string(),
    ),
    default: Some(
        0,
    ),
    description: Some(
        "The unique identifier of a class. A Class describes the attributes available in an event.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "class_name".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! { static ref CLASSIFICATION_IDS: DictAttribute = DictAttribute {
    caption: Some(
        "Classification IDs".to_string(),
    ),
    default: None,
    description: Some(
        "The list of normalzied identifiers of the malware classifications. Reference: <a target='_blank' href='https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html#_oxlc4df65spl'>STIX Malware Types</a> ".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: Some(
        "classifications".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! { static ref CLASSIFICATIONS: DictAttribute = DictAttribute {
    caption: Some(
        "Classifications".to_string(),
    ),
    default: None,
    description: Some(
        "The list of malware classifications, normalized to the captions of the classifcation_id values. In the case of 'Other', they are defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CLIENT_CIPHERS: DictAttribute = DictAttribute {
        caption: Some("Client Cipher Suites".to_string(),),
        default: None,
        description: Some(
            "The client cipher suites that were exchanged during the TLS handshake negotiation."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CLIENT_DIALECTS: DictAttribute = DictAttribute {
        caption: Some("Client Dialects".to_string(),),
        default: None,
        description: Some("The list of SMB dialects that the client speaks.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CLIENT_HASSH: DictAttribute = DictAttribute {
        caption: Some("Client HASSH".to_string(),),
        default: None,
        description: Some("The Client HASSH fingerprinting object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CLOUD: DictAttribute = DictAttribute {
    caption: Some(
        "Cloud".to_string(),
    ),
    default: None,
    description: Some(
        "Describes details about the Cloud enviroment where the event was originally created or logged.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CLOUD_PARTITION: DictAttribute = DictAttribute {
    caption: Some(
        "Cloud Partition".to_string(),
    ),
    default: None,
    description: Some(
        "The canonical cloud partition name to which the region is assigned (e.g. AWS Partitions: aws, aws-cn, aws-us-gov).".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CMD_LINE: DictAttribute = DictAttribute {
    caption: Some(
        "Command Line".to_string(),
    ),
    default: None,
    description: Some(
        "The full command line used to launch an application, service, process, or job. For example: <code>ssh user@10.0.0.10</code>. If the command line is unavailable or missing, the empty string <code>''</code> is to be used".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CODE: DictAttribute = DictAttribute {
        caption: Some("Response Code".to_string(),),
        default: None,
        description: Some("The numeric response sent to a request.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CODES: DictAttribute = DictAttribute {
        caption: Some("Response Codes".to_string(),),
        default: None,
        description: Some("The list of numeric responses sent to a request.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref COLOR_DEPTH: DictAttribute = DictAttribute {
        caption: Some("Color Depth".to_string(),),
        default: None,
        description: Some("The numeric color depth.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref COLOR_MAX: DictAttribute = DictAttribute {
        caption: Some("Color Maximum".to_string(),),
        default: None,
        description: Some(
            "The maximum color value (with 'n' bits this would result in a 2^n-1 maximum value)."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref COLOR_SHIFT: DictAttribute = DictAttribute {
    caption: Some(
        "Color Shift".to_string(),
    ),
    default: None,
    description: Some(
        "The color shift value represents the number of shifts needed in order to get the color value in a pixel to the least significant bit.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref COMMAND: DictAttribute = DictAttribute {
        caption: Some("Command".to_string(),),
        default: None,
        description: Some("The command name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref COMMAND_RESPONSE: DictAttribute = DictAttribute {
        caption: Some("Command Response".to_string(),),
        default: None,
        description: Some("The response to the command.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref COMMAND_RESPONSES: DictAttribute = DictAttribute {
        caption: Some("Command Responses".to_string(),),
        default: None,
        description: Some("The responses to the command.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref COMMENT: DictAttribute = DictAttribute {
        caption: Some("Comment".to_string(),),
        default: None,
        description: Some("The user-provided comment.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref COMPANY_NAME: DictAttribute = DictAttribute {
    caption: Some(
        "Company Name".to_string(),
    ),
    default: None,
    description: Some(
        "The name of the company that published the file. For example: <code>Microsoft Corporation</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref COMPLIANCE: DictAttribute = DictAttribute {
    caption: Some(
        "Compliance".to_string(),
    ),
    default: None,
    description: Some(
        "The complaince object provides context to compliance findings (e.g., a check against a specific regulatory or best practice framework such as CIS or NIST) and contains compliance related details.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref COMPONENT: DictAttribute = DictAttribute {
    caption: Some(
        "Component".to_string(),
    ),
    default: None,
    description: Some(
        "The name or relative pathname of a sub-component of the data object, if applicable. </p>For example: <code>attachment.doc</code>, <code>attachment.zip/bad.doc</code>, or <code>part.mime/part.cab/part.uue/part.doc</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CONFIDENCE: DictAttribute = DictAttribute {
        caption: Some("Confidence".to_string(),),
        default: None,
        description: Some(
            "The confidence of the reported event severity as a percentage: 0%-100%.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CONFIDENTIALITY: DictAttribute = DictAttribute {
    caption: Some(
        "Confidentiality".to_string(),
    ),
    default: None,
    description: Some(
        "The file content confidentiality, normalized to the confidentiality_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CONFIDENTIALITY_ID: DictAttribute = DictAttribute {
        caption: Some("Confidentiality ID".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the file content confidentiality indicator.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("confidentiality".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref CONNECTION_INFO: DictAttribute = DictAttribute {
        caption: Some("Connection Info".to_string(),),
        default: None,
        description: Some("The network connection information.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CONNECTION_UID: DictAttribute = DictAttribute {
        caption: Some("Connection Identifier".to_string(),),
        default: None,
        description: Some("The network connection identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CONTAINER: DictAttribute = DictAttribute {
        caption: Some("Container".to_string(),),
        default: None,
        description: Some("The container information.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CONTENT_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Content Type".to_string(),
    ),
    default: None,
    description: Some(
        "The request header that identifies the original <a target='_blank' href='https://www.iana.org/assignments/media-types/media-types.xhtml'>media type </a> of the resource (prior to any content encoding applied for sending).".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CONTINENT: DictAttribute = DictAttribute {
        caption: Some("Continent".to_string(),),
        default: None,
        description: Some("The name of the continent.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref COORDINATES: DictAttribute = DictAttribute {
    caption: Some(
        "Coordinates".to_string(),
    ),
    default: None,
    description: Some(
        "A two-element array, containing a longitude/latitude pair. The format conforms with <a target='_blank' href='https://geojson.org'>GeoJSON</a>. For example: <code>[-73.983, 40.719]</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CORRELATION_UID: DictAttribute = DictAttribute {
        caption: Some("Correlation UID".to_string(),),
        default: None,
        description: Some("The unique identifier used to correlate events.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref COUNT: DictAttribute = DictAttribute {
    caption: Some(
        "Count".to_string(),
    ),
    default: Some(
        1,
    ),
    description: Some(
        "The number of times that events in the same logical group occurred during the event <strong>Start Time</strong> to <strong>End Time</strong> period.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref COUNTRY: DictAttribute = DictAttribute {
    caption: Some(
        "Country".to_string(),
    ),
    default: None,
    description: Some(
        "The ISO 3166-1 Alpha-2 country code. For the complete list of country codes see <a target='_blank' href='https://www.iso.org/obp/ui/#iso:pub:PUB500001:en' >ISO 3166-1 alpha-2 codes</a>.<p><b>Note:</b> The two letter country code should be capitalized. For example: <code>US</code> or <code>CA</code>.</p>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CPU_BITS: DictAttribute = DictAttribute {
    caption: Some(
        "CPU Bits".to_string(),
    ),
    default: None,
    description: Some(
        "The cpu architecture, the number of bits used for addressing in memory. For example: <code>32</code> or <code>64</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CPU_CORES: DictAttribute = DictAttribute {
    caption: Some(
        "CPU Cores".to_string(),
    ),
    default: None,
    description: Some(
        "The number of processor cores in all installed processors. For Example: <code>42</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref CPU_COUNT: DictAttribute = DictAttribute {
        caption: Some("CPU Count".to_string(),),
        default: None,
        description: Some(
            "The number of physical processors on a system. For example: <code>1</code>."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CPU_SPEED: DictAttribute = DictAttribute {
        caption: Some("Processor Type".to_string(),),
        default: None,
        description: Some(
            "The speed of the processor in Mhz. For Example: <code>4200</code>.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CPU_TYPE: DictAttribute = DictAttribute {
        caption: Some("Processor Type".to_string(),),
        default: None,
        description: Some(
            "The processor type. For example: <code>x86 Family 6 Model 37 Stepping 5</code>."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CREATE_MASK: DictAttribute = DictAttribute {
        caption: Some("Create Mask".to_string(),),
        default: None,
        description: Some(
            "The original Windows mask that is required to create the object.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CREATED_TIME: DictAttribute = DictAttribute {
        caption: Some("Created Time".to_string(),),
        default: None,
        description: Some("The time when the object was created. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CREATOR: DictAttribute = DictAttribute {
        caption: Some("Creator".to_string(),),
        default: None,
        description: Some(
            "The user that created the object associated with event. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CREDENTIAL_UID: DictAttribute = DictAttribute {
        caption: Some("User Credential ID".to_string(),),
        default: None,
        description: Some(
            "The unique identifier of the user's credential. For example, AWS Access Key ID."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CRITICALITY: DictAttribute = DictAttribute {
        caption: Some("Criticality".to_string(),),
        default: None,
        description: Some("Criticality of a resource/object in question".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref CUSTOMER_UID: DictAttribute = DictAttribute {
        caption: Some("Customer UID".to_string(),),
        default: None,
        description: Some("The unique customer identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref CVE: DictAttribute = DictAttribute {
    caption: Some(
        "CVE".to_string(),
    ),
    default: None,
    description: Some(
        "The Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CVES: DictAttribute = DictAttribute {
    caption: Some(
        "CVE List".to_string(),
    ),
    default: None,
    description: Some(
        "List of Common Vulnerabilities and Exposures (<a target='_blank' href='https://cve.mitre.org/'>CVE</a>).".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CVSS: DictAttribute = DictAttribute {
    caption: Some(
        "CVSS Score".to_string(),
    ),
    default: None,
    description: Some(
        "The CVSS object details Common Vulnerability Scoring System (<a target='_blank' href='https://www.first.org/cvss/'>CVSS</a>) scores from the advisory that are related to the vulnerability.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CWE_UID: DictAttribute = DictAttribute {
    caption: Some(
        "CWE UID".to_string(),
    ),
    default: None,
    description: Some(
        "The <a target='_blank' href='https://cwe.mitre.org/'>Common Weakness Enumeration (CWE)</a> unique identifier. For example: <code>CWE-787</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref CWE_URL: DictAttribute = DictAttribute {
    caption: Some(
        "CWE URL".to_string(),
    ),
    default: None,
    description: Some(
        "Common Weakness Enumiration (CWE) definition URL. For example: <code>https://cwe.mitre.org/data/definitions/787.html</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref DATA: DictAttribute = DictAttribute {
        caption: Some("Data".to_string(),),
        default: None,
        description: Some(
            "The additional data that is associated with the event or object. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DATA_BUCKET: DictAttribute = DictAttribute {
        caption: Some("Data Bucket".to_string(),),
        default: None,
        description: Some(
            "The name of the data bucket (e.g. bucket name for AWS/GCP and blob name for Azure)."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref DCE_RPC: DictAttribute = DictAttribute {
    caption: Some(
        "Distributed Computing Environment/Remote Procedure Call (DCE/RPC)".to_string(),
    ),
    default: None,
    description: Some(
        "The DCE/RPC object describes the remote procedure call system for distributed computing environments.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref DECISION: DictAttribute = DictAttribute {
        caption: Some("Authorization Decision/Outcome".to_string(),),
        default: None,
        description: Some(
            "Decision/outcome of the authorization mechanism (e.g. Approved, Denied)".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DELIVERED_TO: DictAttribute = DictAttribute {
        caption: Some("Delivered To".to_string(),),
        default: None,
        description: Some("The <strong>Delivered-To</strong> email header field.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DEPTH: DictAttribute = DictAttribute {
        caption: Some("CVSS Depth".to_string(),),
        default: None,
        description: Some(
            "The CVSS depth represents a depth of the equation used to calculate CVSS score."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DESC: DictAttribute = DictAttribute {
        caption: Some("Description".to_string(),),
        default: None,
        description: Some(
            "The description that pertains to the object or event. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DESKTOP_DISPLAY: DictAttribute = DictAttribute {
        caption: Some("Desktop Display".to_string(),),
        default: None,
        description: Some("The desktop display affiliated with the event".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DETAILS: DictAttribute = DictAttribute {
        caption: Some("Details".to_string(),),
        default: None,
        description: Some("Details of an entity. See specific usage".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref DETECTION_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Detection UID".to_string(),
    ),
    default: None,
    description: Some(
        "The associated unique detection event identifier. For example: detection response events include the <b>Detection ID</b> of the original event.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref DEVELOPER_UID: DictAttribute = DictAttribute {
        caption: Some("Developer UID".to_string(),),
        default: None,
        description: Some("The developer ID on the certificate that signed the file.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DEVICE: DictAttribute = DictAttribute {
        caption: Some("Device".to_string(),),
        default: None,
        description: Some("The device that reported the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DEVICES: DictAttribute = DictAttribute {
        caption: Some("Devices".to_string(),),
        default: None,
        description: Some(
            "The object describes details related to the list of devices.".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DIALECT: DictAttribute = DictAttribute {
        caption: Some("Dialect".to_string(),),
        default: None,
        description: Some("The negotiated protocol dialect.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref DIRECTION: DictAttribute = DictAttribute {
    caption: Some(
        "Direction".to_string(),
    ),
    default: None,
    description: Some(
        "The direction of the initiated connection, traffic, or email, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref DIRECTION_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Direction ID".to_string(),
    ),
    default: None,
    description: Some(
        "The normalized identifier of the direction of the initiated connection, traffic, or email.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "direction".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! { static ref DISPOSITION: DictAttribute = DictAttribute {
    caption: Some(
        "Disposition".to_string(),
    ),
    default: None,
    description: Some(
        "The event disposition name, normalized to the caption of the disposition_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref DISPOSITION_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Disposition ID".to_string(),
    ),
    default: None,
    description: Some(
        "When security issues, such as malware or policy violations, are detected and possibly corrected, then <code>disposition_id</code> describes the action taken by the security product.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "disposition".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref DKIM: DictAttribute = DictAttribute {
        caption: Some("DKIM Status".to_string(),),
        default: None,
        description: Some("The DomainKeys Identified Mail (DKIM) status of the email.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DKIM_DOMAIN: DictAttribute = DictAttribute {
        caption: Some("DKIM Domain".to_string(),),
        default: None,
        description: Some(
            "The DomainKeys Identified Mail (DKIM) signing domain of the email.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DKIM_SIGNATURE: DictAttribute = DictAttribute {
        caption: Some("DKIM Signature".to_string(),),
        default: None,
        description: Some(
            "The DomainKeys Identified Mail (DKIM) signature used by the sending/receiving system."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref DMARC: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Status".to_string(),
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) status of the email.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref DMARC_OVERRIDE: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Override".to_string(),
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) override action.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref DMARC_POLICY: DictAttribute = DictAttribute {
    caption: Some(
        "DMARC Policy".to_string(),
    ),
    default: None,
    description: Some(
        "The Domain-based Message Authentication, Reporting and Conformance (DMARC) policy status.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref DOMAIN: DictAttribute = DictAttribute {
        caption: Some("Domain".to_string(),),
        default: None,
        description: Some("The name of the domain.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DOMAIN_INFO: DictAttribute = DictAttribute {
        caption: Some("Domain Information".to_string(),),
        default: None,
        description: Some("The registration information pertaining to a domain.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DRIVER: DictAttribute = DictAttribute {
        caption: Some("Kernel Driver".to_string(),),
        default: None,
        description: Some("The driver that was loaded/unloaded into the kernel".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref DST_ENDPOINT: DictAttribute = DictAttribute {
        caption: Some("Destination Endpoint".to_string(),),
        default: None,
        description: Some("The network destination endpoint.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref DURATION: DictAttribute = DictAttribute {
    caption: Some(
        "Duration".to_string(),
    ),
    default: None,
    description: Some(
        "The event duration or aggregate time, the amount of time the event covers from <code>start_time</code> to <code>end_time</code> in milliseconds.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref EDITION: DictAttribute = DictAttribute {
        caption: Some("OS Edition".to_string(),),
        default: None,
        description: Some(
            "The operating system edition. For example: <code>Professional</code>.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref EMAIL: DictAttribute = DictAttribute {
        caption: Some("Email".to_string(),),
        default: None,
        description: Some("The email object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref EMAIL_ADDR: DictAttribute = DictAttribute {
        caption: Some("Email Address".to_string(),),
        default: None,
        description: Some("The user's email address.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref EMAIL_AUTH: DictAttribute = DictAttribute {
        caption: Some("Email Authentication".to_string(),),
        default: None,
        description: Some("The SPF, DKIM and DMARC attributes of an email.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref END_TIME: DictAttribute = DictAttribute {
        caption: Some("End Time".to_string(),),
        default: None,
        description: Some("The end time of a time period. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ENRICHMENTS: DictAttribute = DictAttribute {
    caption: Some(
        "Enrichments".to_string(),
    ),
    default: None,
    description: Some(
        "The additional information from an external data source, which is associated with the event. For example add location information for the IP address in the DNS answers:</p><code>[{\"name\": \"answers.ip\", \"value\": \"92.24.47.250\", \"type\": \"location\", \"data\": {\"city\": \"Socotra\", \"continent\": \"Asia\", \"coordinates\": [-25.4153, 17.0743], \"country\": \"YE\", \"desc\": \"Yemen\"}}]</code>".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ENTITY: DictAttribute = DictAttribute {
        caption: Some("Entity".to_string(),),
        default: None,
        description: Some("The managed entity that is being acted upon.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ENTITY_RESULT: DictAttribute = DictAttribute {
        caption: Some("Entity Result".to_string(),),
        default: None,
        description: Some("The updated managed entity.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref EPOCH: DictAttribute = DictAttribute {
    caption: Some(
        "Epoch".to_string(),
    ),
    default: None,
    description: Some(
        "The software package epoch. Epoch is a way to define weighted dependencies based on version numbers.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ERROR: DictAttribute = DictAttribute {
        caption: Some("Error Code".to_string(),),
        default: None,
        description: Some("Error Code".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ERROR_MESSAGE: DictAttribute = DictAttribute {
        caption: Some("Error Message".to_string(),),
        default: None,
        description: Some("Error Message".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref EXIT_CODE: DictAttribute = DictAttribute {
    caption: Some(
        "Exit Code".to_string(),
    ),
    default: None,
    description: Some(
        "The exit code reported by a process when it terminates. The convention is that zero indicates success and any non-zero exit code indicates that some error occurred.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref EXPIRATION_TIME: DictAttribute = DictAttribute {
        caption: Some("Expiration Time".to_string(),),
        default: None,
        description: Some("The expiration time. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref EXPOSED_PORT: DictAttribute = DictAttribute {
    caption: Some(
        "Port".to_string(),
    ),
    default: None,
    description: Some(
        "The IP port number exposed by container. For example 0.0.0.0:49155-> <<exposed_port>>/tcp".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref EXTENSION_LIST: DictAttribute = DictAttribute {
        caption: Some("Extension List".to_string(),),
        default: None,
        description: Some("The list of TLS extensions".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FEATURE: DictAttribute = DictAttribute {
        caption: Some("Feature".to_string(),),
        default: None,
        description: Some("The feature that reported the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FILE: DictAttribute = DictAttribute {
        caption: Some("File".to_string(),),
        default: None,
        description: Some(
            "The file that pertains to the event or object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref FILE_DIFF: DictAttribute = DictAttribute {
    caption: Some(
        "File Diff".to_string(),
    ),
    default: None,
    description: Some(
        "File content differences used for change detection. For example, a common use case is to identify itemized changes within INI or configuration/property setting values.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref FILE_RESULT: DictAttribute = DictAttribute {
    caption: Some(
        "File Result".to_string(),
    ),
    default: None,
    description: Some(
        "The result of the file change. It should contain the new values of the changed attributes.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref FINDING: DictAttribute = DictAttribute {
        caption: Some("Finding".to_string(),),
        default: None,
        description: Some(
            "Finding object provides details related to a finding generated by security tool"
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FINGERPRINT: DictAttribute = DictAttribute {
        caption: Some("Fingerprint".to_string(),),
        default: None,
        description: Some("The digital fingerprint associated with an object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FINGERPRINTS: DictAttribute = DictAttribute {
        caption: Some("Fingerprints".to_string(),),
        default: None,
        description: Some("An array of digital fingerprint objects.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FIRST_SEEN_TIME: DictAttribute = DictAttribute {
        caption: Some("First Seen".to_string(),),
        default: None,
        description: Some(
            "The initial detection time of the activity or object. See specific usage".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FLAG_IDS: DictAttribute = DictAttribute {
        caption: Some("Communication Flag IDs".to_string(),),
        default: None,
        description: Some(
            "The list of normalized identifiers of the communication flag IDs.".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: Some("flags".to_string(),),
        attr_type: None,
    };
}

lazy_static! { static ref FLAGS: DictAttribute = DictAttribute {
    caption: Some(
        "Flags".to_string(),
    ),
    default: None,
    description: Some(
        "The list of communication flags, normalized to the captions of the flag_ids values. In the case of 'Other', they are defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref FOLDER: DictAttribute = DictAttribute {
        caption: Some("Folder".to_string(),),
        default: None,
        description: Some("The folder that pertains to the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FROM: DictAttribute = DictAttribute {
        caption: Some("From".to_string(),),
        default: None,
        description: Some("The email header From values, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref FUNCTION_KEYS: DictAttribute = DictAttribute {
        caption: Some("Function Keys".to_string(),),
        default: None,
        description: Some("The number of function keys on client keyboard.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref FUNCTION_NAME: DictAttribute = DictAttribute {
    caption: Some(
        "Function Name".to_string(),
    ),
    default: None,
    description: Some(
        "The entry-point function of the module. The system calls the entry-point function whenever a process or thread loads or unloads the module.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref GROUP: DictAttribute = DictAttribute {
        caption: Some("Group".to_string(),),
        default: None,
        description: Some(
            "The group object associated with an entity such as user, policy, or rule.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref GROUP_NAME: DictAttribute = DictAttribute {
        caption: Some("Group Name".to_string(),),
        default: None,
        description: Some("The name of the group that the resource belongs to.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref GROUPS: DictAttribute = DictAttribute {
        caption: Some("Groups".to_string(),),
        default: None,
        description: Some("The groups to which an entity belongs. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref HANDSHAKE_DUR: DictAttribute = DictAttribute {
    caption: Some(
        "Handshake Duration".to_string(),
    ),
    default: None,
    description: Some(
        "The amount of total time for the TLS handshake to complete after the TCP connection is established, including client-side delays, in milliseconds.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref HOSTNAME: DictAttribute = DictAttribute {
        caption: Some("Hostname".to_string(),),
        default: None,
        description: Some("The hostname of an endpoint or a device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref HTTP_COOKIES: DictAttribute = DictAttribute {
        caption: Some("HTTP Cookies".to_string(),),
        default: None,
        description: Some("The cookies object describes details about HTTP cookies".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref HTTP_HEADERS: DictAttribute = DictAttribute {
        caption: Some("HTTP Headers".to_string(),),
        default: None,
        description: Some("Additional HTTP headers of an HTTP request or response.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref HTTP_METHOD: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Method".to_string(),
    ),
    default: None,
    description: Some(
        "The HTTP request method indicates the desired action to be performed for a given resource. Expected values: <ul> <li>TRACE</li> <li>CONNECT</li> <li>OPTIONS</li> <li>HEAD</li> <li>DELETE</li> <li>POST</li> <li>PUT</li> <li>GET</li></ul>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref HTTP_ONLY: DictAttribute = DictAttribute {
        caption: Some("HTTP Only".to_string(),),
        default: None,
        description: Some("A cookie attribute to make it inaccessible via JavaScript".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref HTTP_REQUEST: DictAttribute = DictAttribute {
        caption: Some("HTTP Request".to_string(),),
        default: None,
        description: Some("The HTTP Request made to a web server.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref HTTP_RESPONSE: DictAttribute = DictAttribute {
        caption: Some("HTTP Response".to_string(),),
        default: None,
        description: Some("The HTTP Response from a web server to a requester.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref HTTP_STATUS: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Status".to_string(),
    ),
    default: None,
    description: Some(
        "The Hypertext Transfer Protocol (HTTP) <a target='_blank' href='https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml'>status code</a> returned to the client.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref HW_INFO: DictAttribute = DictAttribute {
        caption: Some("Hardware Info".to_string(),),
        default: None,
        description: Some("The device hardware information.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref HYPERVISOR: DictAttribute = DictAttribute {
    caption: Some(
        "Hypervisor".to_string(),
    ),
    default: None,
    description: Some(
        "The name of the hypervisor running on the device. For example, <code>Xen</code>, <code>VMware</code>, <code>Hyper-V</code>, <code>VirtualBox</code>, etc.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref IDENTIFIER_COOKIE: DictAttribute = DictAttribute {
        caption: Some("Identifier Cookie".to_string(),),
        default: None,
        description: Some(
            "The client identifier cookie during client/server exchange.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IDP: DictAttribute = DictAttribute {
        caption: Some("Identity Provider".to_string(),),
        default: None,
        description: Some(
            "This object describes details about the Identity Provider used.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IMAGE: DictAttribute = DictAttribute {
        caption: Some("Image".to_string(),),
        default: None,
        description: Some(
            "The image used as a template to run a container or virtual machine.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IME: DictAttribute = DictAttribute {
        caption: Some("IME".to_string(),),
        default: None,
        description: Some("The Input Method Editor (IME) file name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref IMEI: DictAttribute = DictAttribute {
    caption: Some(
        "IMEI".to_string(),
    ),
    default: None,
    description: Some(
        "The International Mobile Station Equipment Identifier that is associated with the device.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref INJECTION_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Injection Type".to_string(),
    ),
    default: None,
    description: Some(
        "The process injection method, normalized to the caption of the injection_type_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref INJECTION_TYPE_ID: DictAttribute = DictAttribute {
        caption: Some("Injection Type ID".to_string(),),
        default: None,
        description: Some("The normalized identifier of the process injection method.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("injection_type".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref INSTANCE_UID: DictAttribute = DictAttribute {
        caption: Some("Instance ID".to_string(),),
        default: None,
        description: Some("The unique identifier of a VM instance.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref INTEGRITY: DictAttribute = DictAttribute {
    caption: Some(
        "Integrity".to_string(),
    ),
    default: None,
    description: Some(
        "The process integrity level, normalized to the caption of the direction_id value. In the case of 'Other', it is defined by the event source (Windows only).".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref INTEGRITY_ID: DictAttribute = DictAttribute {
        caption: Some("Integrity Level".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the process integrity level (Windows only).".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("integrity".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref INTERFACE_NAME: DictAttribute = DictAttribute {
        caption: Some("Network Interface Name".to_string(),),
        default: None,
        description: Some("The name of the network interface (e.g. eth2).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref INTERFACE_UID: DictAttribute = DictAttribute {
        caption: Some("Network Interface ID".to_string(),),
        default: None,
        description: Some("The unique identifier of the network interface.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref INTERMEDIATE_IPS: DictAttribute = DictAttribute {
    caption: Some(
        "Intermediate IP Addresses".to_string(),
    ),
    default: None,
    description: Some(
        "The intermediate IP Addresses. For example, the IP addresses in the HTTP X-Forwarded-For header.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref INVOKED_BY: DictAttribute = DictAttribute {
        caption: Some("Invoked by".to_string(),),
        default: None,
        description: Some(
            "The name of the service that invoked the activity as described in the event."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IP: DictAttribute = DictAttribute {
        caption: Some("IP Address".to_string(),),
        default: None,
        description: Some("The IP address, in either IPv4 or IPv6 format.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref IS_CLEARTEXT: DictAttribute = DictAttribute {
    caption: Some(
        "Cleartext Credentials".to_string(),
    ),
    default: None,
    description: Some(
        "Indicates whether the credentials were passed in clear text.<p><b>Note:</b> True if the credentials were passed in a clear text protocol such as FTP or TELNET, or if Windows detected that a user's logon password was passed to the authentication package in clear text.</p>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref IS_COMPLIANT: DictAttribute = DictAttribute {
        caption: Some("Compliant Device".to_string(),),
        default: None,
        description: Some("The event occurred on a compliant device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref IS_DEFAULT: DictAttribute = DictAttribute {
    caption: Some(
        "Default Value".to_string(),
    ),
    default: None,
    description: Some(
        "The indication of whether the value is from a default value name. For example, the value name could be missing.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref IS_MANAGED: DictAttribute = DictAttribute {
        caption: Some("Managed Device".to_string(),),
        default: None,
        description: Some("The event occurred on a managed device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_ON_PREMISES: DictAttribute = DictAttribute {
        caption: Some("On Premises".to_string(),),
        default: None,
        description: Some("The indication of whether the location is on premises.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_PERSONAL: DictAttribute = DictAttribute {
        caption: Some("Personal Device".to_string(),),
        default: None,
        description: Some("The event occurred on a personal device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_REMOTE: DictAttribute = DictAttribute {
        caption: Some("Remote".to_string(),),
        default: None,
        description: Some("The indication of whether the session is remote.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_RENEWAL: DictAttribute = DictAttribute {
        caption: Some("Renewal".to_string(),),
        default: None,
        description: Some(
            "The indication of whether this is a lease/session renewal event.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_SYSTEM: DictAttribute = DictAttribute {
        caption: Some("System".to_string(),),
        default: None,
        description: Some(
            "The indication of whether the object is part of the operating system.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref IS_TRUSTED: DictAttribute = DictAttribute {
        caption: Some("Trusted Device".to_string(),),
        default: None,
        description: Some("The event occurred on a trusted device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ISP: DictAttribute = DictAttribute {
        caption: Some("ISP".to_string(),),
        default: None,
        description: Some("The name of the Internet Service Provider (ISP).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ISSUER: DictAttribute = DictAttribute {
        caption: Some("Issuer Details".to_string(),),
        default: None,
        description: Some("The identifier of the session issuer.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ISSUER_DN: DictAttribute = DictAttribute {
        caption: Some("Issuer Distinguished Name".to_string(),),
        default: None,
        description: Some("The certificate issuer distinguished name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ISSUER_NAME: DictAttribute = DictAttribute {
        caption: Some("Issuer Name".to_string(),),
        default: None,
        description: Some("The certificate issuer name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref JA3_FINGERPRINT: DictAttribute = DictAttribute {
        caption: Some("JA3 Fingerprint".to_string(),),
        default: None,
        description: Some("The fingerprint of JA3 string.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref JA3_STRING: DictAttribute = DictAttribute {
        caption: Some("JA3 String".to_string(),),
        default: None,
        description: Some("The JA3 string.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref JA3S_FINGERPRINT: DictAttribute = DictAttribute {
        caption: Some("JAS3 Fingerprint".to_string(),),
        default: None,
        description: Some("The fingerprint of JAS3 string.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref JA3S_STRING: DictAttribute = DictAttribute {
        caption: Some("JAS3 String".to_string(),),
        default: None,
        description: Some("The JAS3 string.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref JOB: DictAttribute = DictAttribute {
        caption: Some("Job".to_string(),),
        default: None,
        description: Some("The job object that pertains to the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KB_ARTICLES: DictAttribute = DictAttribute {
        caption: Some("Knowledgebase Articles".to_string(),),
        default: None,
        description: Some("The KB article/s related to the entity".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KERNEL: DictAttribute = DictAttribute {
        caption: Some("Kernel".to_string(),),
        default: None,
        description: Some("The kernel resource object that pertains to the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KEY_LENGTH: DictAttribute = DictAttribute {
        caption: Some("Key Length".to_string(),),
        default: None,
        description: Some("The length of the encryption key.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KEYBOARD_INFO: DictAttribute = DictAttribute {
        caption: Some("Keyboard Information".to_string(),),
        default: None,
        description: Some("The keyboard detailed information.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KEYBOARD_LAYOUT: DictAttribute = DictAttribute {
        caption: Some("Keyboard Layout".to_string(),),
        default: None,
        description: Some("The keyboard locale identifier name (e.g., en-US).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KEYBOARD_SUBTYPE: DictAttribute = DictAttribute {
        caption: Some("Keyboard Subtype".to_string(),),
        default: None,
        description: Some("The keyboard numeric code.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref KEYBOARD_TYPE: DictAttribute = DictAttribute {
        caption: Some("Keyboard Type".to_string(),),
        default: None,
        description: Some("The keyboard type (e.g., xt, ico).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref LABELS: DictAttribute = DictAttribute {
        caption: Some("Labels".to_string(),),
        default: None,
        description: Some(
            "The list of labels attached to an event, object, or attribute.".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref LANG: DictAttribute = DictAttribute {
    caption: Some(
        "Language".to_string(),
    ),
    default: None,
    description: Some(
        "The two letter lower case language codes, as defined by <a target='_blank' href='https://en.wikipedia.org/wiki/ISO_639-1'>ISO 639-1</a>. For example: <code>en</code> (English), <code>de</code> (German), or <code>fr</code> (French).".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref LAST_RUN_TIME: DictAttribute = DictAttribute {
        caption: Some("Last Run".to_string(),),
        default: None,
        description: Some(
            "The last run time of application or service. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref LAST_SEEN_TIME: DictAttribute = DictAttribute {
        caption: Some("Last Seen".to_string(),),
        default: None,
        description: Some(
            "The most recent detection time of the activity or object. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref LATENCY: DictAttribute = DictAttribute {
        caption: Some("Latency".to_string(),),
        default: None,
        description: Some(
            "TODO: The HTTP response latency. In seconds, milliseconds, etc.?".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref LEASE_DUR: DictAttribute = DictAttribute {
    caption: Some(
        "Lease Duration".to_string(),
    ),
    default: None,
    description: Some(
        "This represents the length of the DHCP lease in seconds. This is present in DHCP Ack events. (activity_id = 1)".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref LENGTH: DictAttribute = DictAttribute {
        caption: Some("Response Length".to_string(),),
        default: None,
        description: Some("The HTTP response length, in number of bytes.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref LICENSE: DictAttribute = DictAttribute {
    caption: Some(
        "Software License".to_string(),
    ),
    default: None,
    description: Some(
        "The name or identifier of the license applied on package or software. See <a target='_blank' href='https://spdx.org/licenses/'>SPDX License List</a>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref LINEAGE: DictAttribute = DictAttribute {
    caption: Some(
        "Lineage".to_string(),
    ),
    default: None,
    description: Some(
        "The lineage of the process, represented by a list of paths for each ancestor process. For example: <code>['/usr/sbin/sshd', '/usr/bin/bash', '/usr/bin/whoami']</code>".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref LOAD_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Load Type".to_string(),
    ),
    default: None,
    description: Some(
        "The load type, normalized to the caption of the load_type_id value. In the case of 'Other', it is defined by the event source. It describes how the module was loaded in memory.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref LOAD_TYPE_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Load Type ID".to_string(),
    ),
    default: None,
    description: Some(
        "The normalized identifier of the load type. It identifies how the module was loaded in memory.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "load_type".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref LOADED_MODULES: DictAttribute = DictAttribute {
        caption: Some("Loaded Modules".to_string(),),
        default: None,
        description: Some("The list of loaded module names.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref LOCATION: DictAttribute = DictAttribute {
        caption: Some("Detailed Geo Location".to_string(),),
        default: None,
        description: Some(
            "The detailed geographical location usually associated with an IP address.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref LOGGED_TIME: DictAttribute = DictAttribute {
    caption: Some(
        "Logged Time".to_string(),
    ),
    default: None,
    description: Some(
        "The time when the logging system collected and logged the event.</p>This attribute is distinct from the event time in that event time typically contain the time extracted from the original event. Most of the time, these two times will be different.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref LOGON_PROCESS: DictAttribute = DictAttribute {
        caption: Some("Logon Process".to_string(),),
        default: None,
        description: Some(
            "The trusted process that validated the authentication credentials.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref LOGON_TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Logon Type".to_string(),
    ),
    default: None,
    description: Some(
        "The logon type, normalized to the caption of the logon_type_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref LOGON_TYPE_ID: DictAttribute = DictAttribute {
        caption: Some("Logon Type ID".to_string(),),
        default: None,
        description: Some("The normalized logon type identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("logon_type".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref MAC: DictAttribute = DictAttribute {
        caption: Some("MAC Address".to_string(),),
        default: None,
        description: Some(
            "The Media Access Control (MAC) address that is associated with the network interface."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MALWARE: DictAttribute = DictAttribute {
        caption: Some("Malware".to_string(),),
        default: None,
        description: Some("The list of malware identified by a finding.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MESSAGE: DictAttribute = DictAttribute {
        caption: Some("Message".to_string(),),
        default: None,
        description: Some(
            "The description of the event, as defined by the event source.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MESSAGE_UID: DictAttribute = DictAttribute {
        caption: Some("Message UID".to_string(),),
        default: None,
        description: Some("The email header Message-Id value, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref METADATA: DictAttribute = DictAttribute {
        caption: Some("Metadata".to_string(),),
        default: None,
        description: Some("The metadata associated with the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref METRICS: DictAttribute = DictAttribute {
        caption: Some("Metrics".to_string(),),
        default: None,
        description: Some(
            "The general purpose metrics associated with the event. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MFA: DictAttribute = DictAttribute {
        caption: Some("Multi Factor Authentication".to_string(),),
        default: None,
        description: Some(
            "Indicates whether Multi Factor Authentication was used during authentication."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MIME_TYPE: DictAttribute = DictAttribute {
        caption: Some("MIME type".to_string(),),
        default: None,
        description: Some(
            "The Multipurpose Internet Mail Extensions (MIME) type of the file, if applicable."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MODEL: DictAttribute = DictAttribute {
        caption: Some("Model".to_string(),),
        default: None,
        description: Some("The peripheral device model.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MODIFIED_TIME: DictAttribute = DictAttribute {
        caption: Some("Modified Time".to_string(),),
        default: None,
        description: Some(
            "The time when the object was last modified. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MODIFIER: DictAttribute = DictAttribute {
        caption: Some("Modifier".to_string(),),
        default: None,
        description: Some(
            "The user that last modified the object associated with the event. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref MODULE: DictAttribute = DictAttribute {
        caption: Some("Module".to_string(),),
        default: None,
        description: Some("The module that pertains to the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref NAME: DictAttribute = DictAttribute {
        caption: Some("Name".to_string(),),
        default: None,
        description: Some("The name of the entity. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref NAMESPACE: DictAttribute = DictAttribute {
    caption: Some(
        "Namespace".to_string(),
    ),
    default: None,
    description: Some(
        "The namespace is useful in merger or acquisition situations. For example, when similar entities exists that you need to keep separate.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref NETWORK_DRIVER: DictAttribute = DictAttribute {
    caption: Some(
        "Network Driver".to_string(),
    ),
    default: None,
    description: Some(
        "The network driver used by the container. For example, bridge, overlay, host, none, etc.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref NETWORK_INTERFACE: DictAttribute = DictAttribute {
        caption: Some("Network Interface".to_string(),),
        default: None,
        description: Some("The network interface that is associated with the device.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref NETWORK_INTERFACES: DictAttribute = DictAttribute {
    caption: Some(
        "Network Interfaces".to_string(),
    ),
    default: None,
    description: Some(
        "The network interfaces that are associated with the device, one for each MAC address/IP address combination.<p><b>Note:</b> The first element of the array is the network information that pertains to the event.</p>".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref NEXT_RUN_TIME: DictAttribute = DictAttribute {
        caption: Some("Next Run".to_string(),),
        default: None,
        description: Some("The next run time. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OBSERVABLES: DictAttribute = DictAttribute {
        caption: Some("Observables".to_string(),),
        default: None,
        description: Some("The observables associated with the event.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OPCODE: DictAttribute = DictAttribute {
        caption: Some("DNS Opcode".to_string(),),
        default: None,
        description: Some("The DNS opcode specifies the type of the query message.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OPCODE_ID: DictAttribute = DictAttribute {
        caption: Some("DNS Opcode ID".to_string(),),
        default: None,
        description: Some(
            "The DNS opcode ID specifies the normalized query message type.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OPEN_MASK: DictAttribute = DictAttribute {
        caption: Some("Open Mask".to_string(),),
        default: None,
        description: Some("The Windows options needed to open a registry key.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OPEN_TYPE: DictAttribute = DictAttribute {
        caption: Some("Open Type".to_string(),),
        default: None,
        description: Some("The file open type.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OPERATION: DictAttribute = DictAttribute {
        caption: Some("Operation".to_string(),),
        default: None,
        description: Some("Verb/Operation associated with the request".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref OPNUM: DictAttribute = DictAttribute {
    caption: Some(
        "Opnum".to_string(),
    ),
    default: None,
    description: Some(
        "An operation number used to identify a specific remote procedure call (RPC) method or a method in an interface.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ORCHESTRATOR: DictAttribute = DictAttribute {
        caption: Some("Orchestrator".to_string(),),
        default: None,
        description: Some(
            "The orchestrator managing the container, such as ECS, EKS, K8s, OpenShift, None."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref ORG_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Org ID".to_string(),
    ),
    default: None,
    description: Some(
        "The unique identifier of the organization to which the user belongs. For example, Active Directory or AWS Org ID.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ORG_UNIT: DictAttribute = DictAttribute {
        caption: Some("Org Unit".to_string(),),
        default: None,
        description: Some("The name of the organization to which the user belongs.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref ORIGINAL_TIME: DictAttribute = DictAttribute {
        caption: Some("Original Time".to_string(),),
        default: None,
        description: Some("The original event time as reported by the event source.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OS: DictAttribute = DictAttribute {
        caption: Some("OS".to_string(),),
        default: None,
        description: Some("The device operation system.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OVERALL_SCORE: DictAttribute = DictAttribute {
        caption: Some("Overall Score".to_string(),),
        default: None,
        description: Some(
            "The overall score as reported by the event source. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref OWNER: DictAttribute = DictAttribute {
        caption: Some("Owner".to_string(),),
        default: None,
        description: Some("The user that owns the file/object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PACKAGES: DictAttribute = DictAttribute {
        caption: Some("Software Packages".to_string(),),
        default: None,
        description: Some(
            "List of vulnerable packages as identified by the security product".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PACKET_UID: DictAttribute = DictAttribute {
        caption: Some("Packet UID".to_string(),),
        default: None,
        description: Some("The packet identifier assigned by the protocol.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PACKETS: DictAttribute = DictAttribute {
        caption: Some("Total Packets".to_string(),),
        default: Some(0,),
        description: Some("The total number of packets (in and out).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PACKETS_IN: DictAttribute = DictAttribute {
        caption: Some("Packets In".to_string(),),
        default: Some(0,),
        description: Some(
            "The number of packets sent from the destination to the source.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PACKETS_OUT: DictAttribute = DictAttribute {
        caption: Some("Packets Out".to_string(),),
        default: Some(0,),
        description: Some(
            "The number of packets sent from the source to the destination.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref PARENT_FOLDER: DictAttribute = DictAttribute {
    caption: Some(
        "Parent Folder".to_string(),
    ),
    default: None,
    description: Some(
        "The parent folder in which the file resides. For example: <code>c:\\windows\\system32</code>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref PARENT_PROCESS: DictAttribute = DictAttribute {
    caption: Some(
        "Parent Process".to_string(),
    ),
    default: None,
    description: Some(
        "The parent process of this process object. It is recommended to only populate this field for the first process object, to prevent deep nesting.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref PATH: DictAttribute = DictAttribute {
        caption: Some("Path".to_string(),),
        default: None,
        description: Some(
            "The path that pertains to the event or object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PERIPHERAL_DEVICE: DictAttribute = DictAttribute {
        caption: Some("Peripheral Device".to_string(),),
        default: None,
        description: Some("The peripheral device that triggered the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PERMISSION: DictAttribute = DictAttribute {
        caption: Some("Permission".to_string(),),
        default: None,
        description: Some("The IAM permission related to an event".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PHYSICAL_HEIGHT: DictAttribute = DictAttribute {
        caption: Some("Physical Height".to_string(),),
        default: None,
        description: Some("The numeric physical height of display.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PHYSICAL_ORIENTATION: DictAttribute = DictAttribute {
        caption: Some("Physical Orientation".to_string(),),
        default: None,
        description: Some("The numeric physical orientation of display.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PHYSICAL_WIDTH: DictAttribute = DictAttribute {
        caption: Some("Physical Width".to_string(),),
        default: None,
        description: Some("The numeric physical width of display.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref PID: DictAttribute = DictAttribute {
    caption: Some(
        "Process ID".to_string(),
    ),
    default: None,
    description: Some(
        "The process identifier, as reported by the operating system. Process ID (PID) is a number used by the operating system to uniquely identify an active process.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref PIXEL_BITS: DictAttribute = DictAttribute {
        caption: Some("Pixel Bits".to_string(),),
        default: None,
        description: Some("The number of bits per pixel.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref POLICY: DictAttribute = DictAttribute {
        caption: Some("Policy".to_string(),),
        default: None,
        description: Some("Describes details of a policy. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PORT: DictAttribute = DictAttribute {
        caption: Some("Port".to_string(),),
        default: None,
        description: Some(
            "The TCP/UDP port number associated with a connection. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref POSTAL_CODE: DictAttribute = DictAttribute {
        caption: Some("Postal Code".to_string(),),
        default: None,
        description: Some("The postal code of the location.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PREFIX: DictAttribute = DictAttribute {
        caption: Some("Prefix".to_string(),),
        default: None,
        description: Some("The domain prefix.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PREV_REG_KEY: DictAttribute = DictAttribute {
        caption: Some("Previous Registry Key".to_string(),),
        default: None,
        description: Some("The registry key before the mutation".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PREV_REG_VALUE: DictAttribute = DictAttribute {
        caption: Some("Previous Registry Value".to_string(),),
        default: None,
        description: Some("The registry value before the mutation".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref PRIORITY: DictAttribute = DictAttribute {
    caption: Some(
        "Priority".to_string(),
    ),
    default: None,
    description: Some(
        "The priority, normalized to the caption of the priority_id value. In the case of 'Other', it is defined by the event source. See specific usage.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref PRIORITY_ID: DictAttribute = DictAttribute {
        caption: Some("Priority ID".to_string(),),
        default: None,
        description: Some("The normalized priority. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("priority".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref PRIVILEGES: DictAttribute = DictAttribute {
        caption: Some("Privileges".to_string(),),
        default: None,
        description: Some("The user or group privileges.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PROCESS: DictAttribute = DictAttribute {
        caption: Some("Process".to_string(),),
        default: None,
        description: Some("The process object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PROCESSED_TIME: DictAttribute = DictAttribute {
        caption: Some("Processed Time".to_string(),),
        default: None,
        description: Some("The event processed time, such as an ETL operation.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PRODUCT: DictAttribute = DictAttribute {
        caption: Some("Product".to_string(),),
        default: None,
        description: Some("The product that reported the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PRODUCT_UID: DictAttribute = DictAttribute {
        caption: Some("Product Identifier".to_string(),),
        default: None,
        description: Some("Unique Identifier of a product.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PROFILES: DictAttribute = DictAttribute {
        caption: Some("Profiles".to_string(),),
        default: None,
        description: Some("The list of profiles used to create the event.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PROJECT_UID: DictAttribute = DictAttribute {
        caption: Some("Project ID".to_string(),),
        default: None,
        description: Some("The unique identifier of a Cloud project.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref PROTOCOL_NAME: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Name".to_string(),
    ),
    default: None,
    description: Some(
        "The TCP/IP protocol name in lowercase, as defined by the Internet Assigned Numbers Authority (IANA). See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>tcp</code> or <code>udp</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref PROTOCOL_NUM: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Number".to_string(),
    ),
    default: None,
    description: Some(
        "The TCP/IP protocol number, as defined by the Internet Assigned Numbers Authority (IANA). Use -1 if the protocol is not defined by IANA. See <a target='_blank' href='https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml'>Protocol Numbers</a>. For example: <code>6</code> for TCP and <code>17</code> for UDP.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref PROTOCOL_VER: DictAttribute = DictAttribute {
    caption: Some(
        "Protocol Version".to_string(),
    ),
    default: None,
    description: Some(
        "The Protocol version, normalized to the caption of the protocol_ver_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref PROTOCOL_VER_ID: DictAttribute = DictAttribute {
        caption: Some("Protocol Version ID".to_string(),),
        default: None,
        description: Some("The normalized identifier of the Protocol version.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("protocol_ver".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref PROVIDER: DictAttribute = DictAttribute {
        caption: Some("Provider".to_string(),),
        default: None,
        description: Some(
            "The origin of information associated with the event. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref PROXY: DictAttribute = DictAttribute {
        caption: Some("Proxy".to_string(),),
        default: None,
        description: Some(
            "If a proxy connection is present, the connection from the client to the proxy server."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref QUERY: DictAttribute = DictAttribute {
        caption: Some("DNS Query".to_string(),),
        default: None,
        description: Some("The Domain Name System (DNS) query.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref QUERY_STRING: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Query String".to_string(),
    ),
    default: None,
    description: Some(
        "The query portion of the URL. For example: the query portion of the URL <code>http://www.example.com/search?q=bad&sort=date</code> is <code>q=bad&sort=date</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref QUERY_TIME: DictAttribute = DictAttribute {
        caption: Some("Query Time".to_string(),),
        default: None,
        description: Some("The Domain Name System (DNS) query time.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RAM_SIZE: DictAttribute = DictAttribute {
        caption: Some("RAM Size".to_string(),),
        default: None,
        description: Some(
            "The ctotal amount of installed RAM, in Megabytes. For example: <code>2048</code>."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RAW_DATA: DictAttribute = DictAttribute {
        caption: Some("Raw Data".to_string(),),
        default: None,
        description: Some("The event data as received from the event source.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RAW_HEADER: DictAttribute = DictAttribute {
        caption: Some("Raw Header".to_string(),),
        default: None,
        description: Some("The email authentication header.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref RCODE: DictAttribute = DictAttribute {
    caption: Some(
        "Response Code".to_string(),
    ),
    default: None,
    description: Some(
        "The server response code, normalized to the caption of the rcode_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref RCODE_ID: DictAttribute = DictAttribute {
        caption: Some("Response Code ID".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the server response code. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("rcode".to_string(),),
        attr_type: None,
    };
}

lazy_static! { static ref RDATA: DictAttribute = DictAttribute {
    caption: Some(
        "DNS RData".to_string(),
    ),
    default: None,
    description: Some(
        "The data describing the DNS resource. The meaning of this data depends on the type and class of the resource record.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref REFERENCES: DictAttribute = DictAttribute {
        caption: Some("References".to_string(),),
        default: None,
        description: Some("Supporting reference URLs".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref REFERRER: DictAttribute = DictAttribute {
    caption: Some(
        "HTTP Referrer".to_string(),
    ),
    default: None,
    description: Some(
        "The request header that identifies the address of the previous web page, which is linked to the current web page or resource being requested.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref REG_KEY: DictAttribute = DictAttribute {
        caption: Some("Registry Key".to_string(),),
        default: None,
        description: Some("The registry key.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REG_VALUE: DictAttribute = DictAttribute {
        caption: Some("Registry Value".to_string(),),
        default: None,
        description: Some("The registry value.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REGION: DictAttribute = DictAttribute {
        caption: Some("Region".to_string(),),
        default: None,
        description: Some("The name or the code of a region. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REGISTRAR: DictAttribute = DictAttribute {
        caption: Some("Domain Registrar".to_string(),),
        default: None,
        description: Some("The domain registrar.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref RELATED_EVENTS: DictAttribute = DictAttribute {
    caption: Some(
        "Related Events".to_string(),
    ),
    default: None,
    description: Some(
        "Describes events related to a finding or detection as identified by the security product.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref RELATED_VULNERABILITIES: DictAttribute = DictAttribute {
        caption: Some("Related Vulnerabilities".to_string(),),
        default: None,
        description: Some(
            "List of vulnerabilities that are related to this vulnerability.".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RELAY: DictAttribute = DictAttribute {
        caption: Some("Relay".to_string(),),
        default: None,
        description: Some("The network relay that is associated with the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RELEASE: DictAttribute = DictAttribute {
        caption: Some("Software Release Details".to_string(),),
        default: None,
        description: Some(
            "Release is the number of times a version of the software has been packaged."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REMEDIATION: DictAttribute = DictAttribute {
        caption: Some("Remediation".to_string(),),
        default: None,
        description: Some(
            "The remediation recommendations on how to fix the identified issue(s).".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REMOTE_DISPLAY: DictAttribute = DictAttribute {
        caption: Some("Remote Display".to_string(),),
        default: None,
        description: Some("The remote display affiliated with the event".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REPLY_TO: DictAttribute = DictAttribute {
        caption: Some("Reply To".to_string(),),
        default: None,
        description: Some("The email header Reply-To values, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REPUTATION: DictAttribute = DictAttribute {
        caption: Some("Reputation Scores".to_string(),),
        default: None,
        description: Some("Contains the original and normalized reputation scores.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REQUEST: DictAttribute = DictAttribute {
        caption: Some("API Request Details".to_string(),),
        default: None,
        description: Some("General Purpose API Request Object. See specific usage".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REQUESTED_PERMISSIONS: DictAttribute = DictAttribute {
        caption: Some("Requested Permissions".to_string(),),
        default: None,
        description: Some("The permissions mask that were requested by the process.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref REQUIREMENTS: DictAttribute = DictAttribute {
        caption: Some("Requirements".to_string(),),
        default: None,
        description: Some(
            "A list of applicable compliance requirements for which this finding is related to."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RESOURCE: DictAttribute = DictAttribute {
        caption: Some("Resource".to_string(),),
        default: None,
        description: Some("The target resource.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RESOURCE_TYPE: DictAttribute = DictAttribute {
        caption: Some("Resource Type".to_string(),),
        default: None,
        description: Some(
            "The context in which a resource was retrieved in a web request.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref RESOURCE_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Resource ID".to_string(),
    ),
    default: None,
    description: Some(
        "The unique identifier of a cloud resource. For example, S3 Bucket name, EC2 Instance Id.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref RESOURCES: DictAttribute = DictAttribute {
        caption: Some("Resources Array".to_string(),),
        default: None,
        description: Some(
            "Describes details about resources that were affected by the activity/event."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RESPONSE: DictAttribute = DictAttribute {
        caption: Some("API Response Details".to_string(),),
        default: None,
        description: Some("General Purpose API Response Object. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RESPONSE_TIME: DictAttribute = DictAttribute {
        caption: Some("Response Time".to_string(),),
        default: None,
        description: Some("The Domain Name System (DNS) response time.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref RISK_LEVEL: DictAttribute = DictAttribute {
    caption: Some(
        "Risk Level".to_string(),
    ),
    default: None,
    description: Some(
        "The risk level, normalized to the caption of the risk_level_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref RISK_LEVEL_ID: DictAttribute = DictAttribute {
        caption: Some("Risk Level ID".to_string(),),
        default: None,
        description: Some("The normalized risk level id.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("risk_level".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref RISK_SCORE: DictAttribute = DictAttribute {
        caption: Some("Risk Score".to_string(),),
        default: None,
        description: Some("The risk score as reported by the event source.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref RULE: DictAttribute = DictAttribute {
        caption: Some("Rule".to_string(),),
        default: None,
        description: Some("The rules that reported the events.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref RUN_STATE: DictAttribute = DictAttribute {
    caption: Some(
        "Run State".to_string(),
    ),
    default: None,
    description: Some(
        "The state of the job or service, normalized to the caption of the run_state_id value. In the case of 'Other', it is defined by the event source. See specific usage.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref RUN_STATE_ID: DictAttribute = DictAttribute {
        caption: Some("Run State ID".to_string(),),
        default: None,
        description: Some(
            "The normalized identifier of the state of the job or service. See specific usage."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("run_state".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref RUNTIME: DictAttribute = DictAttribute {
        caption: Some("Runtime".to_string(),),
        default: None,
        description: Some("The runtime managing this container.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref SAMESITE: DictAttribute = DictAttribute {
    caption: Some(
        "SameSite".to_string(),
    ),
    default: None,
    description: Some(
        "The cookie attribute that lets servers specify whether/when cookies are sent with cross-site requests. Values are: Strict, Lax or None".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref SANDBOX: DictAttribute = DictAttribute {
    caption: Some(
        "Sandbox".to_string(),
    ),
    default: None,
    description: Some(
        "The name of the containment jail (i.e., sandbox). For example, hardened_ps, high_security_ps, oracle_ps, netsvcs_ps, or default_ps.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref SANS: DictAttribute = DictAttribute {
        caption: Some("Subject Alternative Names".to_string(),),
        default: None,
        description: Some(
            "The list of subject alternative names that are secured by a specific certificate."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SCALE_FACTOR: DictAttribute = DictAttribute {
        caption: Some("Scale Factor".to_string(),),
        default: None,
        description: Some("The numeric scale factor of display.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref SCHEME: DictAttribute = DictAttribute {
    caption: Some(
        "Scheme".to_string(),
    ),
    default: None,
    description: Some(
        "The scheme portion of the URL. For example: <code>http</code>, <code>https</code>, <code>ftp</code>, or <code>sftp</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref SCORE: DictAttribute = DictAttribute {
    caption: Some(
        "Reputation Score".to_string(),
    ),
    default: None,
    description: Some(
        "The reputation score, normalized to the caption of the score_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref SCORE_ID: DictAttribute = DictAttribute {
        caption: Some("Reputation Score ID".to_string(),),
        default: None,
        description: Some("The normalized reputation score identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("score".to_string(),),
        attr_type: None,
    };
}

lazy_static! { static ref SECURE: DictAttribute = DictAttribute {
    caption: Some(
        "Secure".to_string(),
    ),
    default: None,
    description: Some(
        "The cookie attribute to only send cookies to the server with an encrypted request over the HTTPS protocol.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref SECURITY_DESCRIPTOR: DictAttribute = DictAttribute {
        caption: Some("Security Descriptor".to_string(),),
        default: None,
        description: Some("The object security descriptor.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref SEQUENCE: DictAttribute = DictAttribute {
    caption: Some(
        "Sequence Number".to_string(),
    ),
    default: None,
    description: Some(
        "Sequence number of the event. The sequence number is a value available in some events, to make the exact ordering of events unambiguous, regardless of the event time precision.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref SERIAL_NUMBER: DictAttribute = DictAttribute {
        caption: Some("Serial Number".to_string(),),
        default: None,
        description: Some(
            "The serial number that pertains to the object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SERVER_CIPHERS: DictAttribute = DictAttribute {
        caption: Some("Server Cipher Suites".to_string(),),
        default: None,
        description: Some(
            "The server cipher suites that were exchanged during the TLS handshake negotiation."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SERVER_HASSH: DictAttribute = DictAttribute {
        caption: Some("Server HASSH".to_string(),),
        default: None,
        description: Some("The Server HASSH fingerprinting object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SERVICE: DictAttribute = DictAttribute {
        caption: Some("Service".to_string(),),
        default: None,
        description: Some("The service that pertains to the event.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SESSION: DictAttribute = DictAttribute {
        caption: Some("User Session".to_string(),),
        default: None,
        description: Some("The authenticated user session.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref SEVERITY: DictAttribute = DictAttribute {
    caption: Some(
        "Severity".to_string(),
    ),
    default: None,
    description: Some(
        "The event severity, normalized to the caption of the severity_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref SEVERITY_ID: DictAttribute = DictAttribute {
    caption: Some(
        "Severity ID".to_string(),
    ),
    default: None,
    description: Some(
        "The normalized identifier of the event severity.</p>The normalized severity is a measurement the effort and expense required to manage and resolve an event or incident. Smaller numerical values represent lower impact events, and larger numerical values represent higher impact events.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "severity".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref SHARE: DictAttribute = DictAttribute {
        caption: Some("Share".to_string(),),
        default: None,
        description: Some("The SMB share name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SHARE_TYPE: DictAttribute = DictAttribute {
        caption: Some("Share Type".to_string(),),
        default: None,
        description: Some("The SMB share type.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SIGNATURE: DictAttribute = DictAttribute {
        caption: Some("Digital Signature".to_string(),),
        default: None,
        description: Some("The digital signature of the file.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SIZE: DictAttribute = DictAttribute {
        caption: Some("Size".to_string(),),
        default: None,
        description: Some("The size of data, in bytes.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SMTP_FROM: DictAttribute = DictAttribute {
        caption: Some("SMTP From".to_string(),),
        default: None,
        description: Some("The value of the SMTP MAIL FROM command.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SMTP_HELLO: DictAttribute = DictAttribute {
        caption: Some("SMTP Hello".to_string(),),
        default: None,
        description: Some("The value of the SMTP HELO or EHLO command.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SMTP_TO: DictAttribute = DictAttribute {
        caption: Some("SMTP To".to_string(),),
        default: None,
        description: Some("The value of the SMTP envelope RCPT TO command.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SNI: DictAttribute = DictAttribute {
        caption: Some("Server Name Indication".to_string(),),
        default: None,
        description: Some(
            " The Server Name Indication (SNI) extension sent by the client.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SP_NAME: DictAttribute = DictAttribute {
        caption: Some("OS Service Pack".to_string(),),
        default: None,
        description: Some("The name of the latest Service Pack.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SP_VER: DictAttribute = DictAttribute {
        caption: Some("OS Service Pack Version".to_string(),),
        default: None,
        description: Some("The version number of the latest Service Pack.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SPF: DictAttribute = DictAttribute {
        caption: Some("SPF Status".to_string(),),
        default: None,
        description: Some("The Sender Policy Framework (SPF) status of the email.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SRC_ENDPOINT: DictAttribute = DictAttribute {
        caption: Some("Source Endpoint".to_string(),),
        default: None,
        description: Some("The network source endpoint.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SRC_URL: DictAttribute = DictAttribute {
        caption: Some("Source URL".to_string(),),
        default: None,
        description: Some(
            "The URL pointing towards the source of an entity. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref START_ADDRESS: DictAttribute = DictAttribute {
        caption: Some("Start Address".to_string(),),
        default: None,
        description: Some("The start address of the execution.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref START_TIME: DictAttribute = DictAttribute {
        caption: Some("Start Time".to_string(),),
        default: None,
        description: Some("The start time of a time period. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref STATE: DictAttribute = DictAttribute {
    caption: Some(
        "State".to_string(),
    ),
    default: None,
    description: Some(
        "The state of the event or object, normalized to the caption of the state_id value. In the case of 'Other', it is defined by the event source. See specific usage.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref STATE_ID: DictAttribute = DictAttribute {
        caption: Some("State ID".to_string(),),
        default: None,
        description: Some(
            "The normalized state ID of the event or object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("state".to_string(),),
        attr_type: None,
    };
}

lazy_static! { static ref STATUS: DictAttribute = DictAttribute {
    caption: Some(
        "Status".to_string(),
    ),
    default: None,
    description: Some(
        "The event status, normalized to the caption of the status_id value. In the case of 'Other', it is defined by the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref STATUS_CODE: DictAttribute = DictAttribute {
    caption: Some(
        "Status Code".to_string(),
    ),
    default: None,
    description: Some(
        "The event status code, as reported by the event source.<br /><br />For example, in a Windows Failed Authentication event, this would be the value of 'Failure Code', e.g. 0x18.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref STATUS_DETAIL: DictAttribute = DictAttribute {
        caption: Some("Status Details".to_string(),),
        default: None,
        description: Some(
            "The status details contains additional information about the event outcome."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref STATUS_ID: DictAttribute = DictAttribute {
        caption: Some("Status ID".to_string(),),
        default: None,
        description: Some("The normalized identifier of the event status.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: Some("status".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref SUBJECT: DictAttribute = DictAttribute {
        caption: Some("Subject".to_string(),),
        default: None,
        description: Some("The email header Subject value, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SUBJECT_DN: DictAttribute = DictAttribute {
        caption: Some("Subject Distinguished Name".to_string(),),
        default: None,
        description: Some("The certificate subject distinguished name.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SUBNET: DictAttribute = DictAttribute {
        caption: Some("Subnet".to_string(),),
        default: None,
        description: Some("The subnet mask.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SUBNET_UID: DictAttribute = DictAttribute {
        caption: Some("Subnet UID".to_string(),),
        default: None,
        description: Some("The unique identifier of a virtual subnet.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref SUPPORTING_DATA: DictAttribute = DictAttribute {
        caption: Some("Supporting Data".to_string(),),
        default: None,
        description: Some(
            "Additional data supporting a finding as provided by security tool".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref SVC_NAME: DictAttribute = DictAttribute {
    caption: Some(
        "Service Name".to_string(),
    ),
    default: None,
    description: Some(
        "The service name in service-to-service connections. For example, AWS VPC logs the pkt-src-aws-service and pkt-dst-aws-service fields identify the connection is coming from or going to an AWS service.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref SYSTEM_CALL: DictAttribute = DictAttribute {
        caption: Some("System Call".to_string(),),
        default: None,
        description: Some("The system call that was invoked.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TACTICS: DictAttribute = DictAttribute {
    caption: Some(
        "Tactics".to_string(),
    ),
    default: None,
    description: Some(
        "The a list of tactic ID's/names that are associated with the attack technique, as defined by <a target='_blank' href='https://attack.mitre.org/wiki/ATT&CK_Matrix'>ATT&CK Matrix<sup>TM</sup></a>.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref TAG: DictAttribute = DictAttribute {
        caption: Some("Image Tag".to_string(),),
        default: None,
        description: Some("The image tag. For example: <code>1.11-alpine</code>.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TCP_FLAGS: DictAttribute = DictAttribute {
        caption: Some("TCP Flags".to_string(),),
        default: None,
        description: Some(
            "The network connection TCP header flags (i.e., control bits).".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TECHNIQUE: DictAttribute = DictAttribute {
        caption: Some("Technique".to_string(),),
        default: None,
        description: Some("The attack technique.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TERMINATED_TIME: DictAttribute = DictAttribute {
        caption: Some("Terminated Time".to_string(),),
        default: None,
        description: Some(
            "The time when the entity was terminated. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TEXT: DictAttribute = DictAttribute {
        caption: Some("URL Text".to_string(),),
        default: None,
        description: Some(
            "The URL. For example: <code>http://www.example.com/download/trouble.exe</code>."
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TID: DictAttribute = DictAttribute {
    caption: Some(
        "Thread ID".to_string(),
    ),
    default: None,
    description: Some(
        "The Identifier of the thread associated with the event, as returned by the operating system.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref TIME: DictAttribute = DictAttribute {
        caption: Some("Event Time".to_string(),),
        default: None,
        description: Some("The normalized event occurrence time.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TIMEZONE_OFFSET: DictAttribute = DictAttribute {
    caption: Some(
        "Timezone Offset".to_string(),
    ),
    default: None,
    description: Some(
        "The number of minutes that the reported event <code>time</code> is ahead or behind UTC, in the range -1,080 to +1,080.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref TITLE: DictAttribute = DictAttribute {
        caption: Some("Title".to_string(),),
        default: None,
        description: Some("The title of an entity. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TLS: DictAttribute = DictAttribute {
        caption: Some("TLS".to_string(),),
        default: None,
        description: Some("The Transport Layer Security (TLS) attributes.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref TO: DictAttribute = DictAttribute {
        caption: Some("To".to_string(),),
        default: None,
        description: Some("The email header To values, as defined by RFC 5322.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TRAFFIC: DictAttribute = DictAttribute {
    caption: Some(
        "Traffic".to_string(),
    ),
    default: None,
    description: Some(
        "The network traffic refers to the amount of data moving across a network at a given point of time. Intended to be used alongside Network Connection.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref TRANSACTION_UID: DictAttribute = DictAttribute {
        caption: Some("Transaction UID".to_string(),),
        default: None,
        description: Some("The unique identifier of the transaction.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TREE_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Tree UID".to_string(),
    ),
    default: None,
    description: Some(
        "The tree id is a unique SMB identifier which represents an open connection to a share.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref TRUE_COLOR: DictAttribute = DictAttribute {
    caption: Some(
        "True Color".to_string(),
    ),
    default: None,
    description: Some(
        "A boolean indicating whether to extract pixel values through red/green/blue intensities.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref TTL: DictAttribute = DictAttribute {
    caption: Some(
        "TTL".to_string(),
    ),
    default: None,
    description: Some(
        "The time interval that the resource record may be cached. Zero value means that the resource record can only be used for the transaction in progress, and should not be cached.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! { static ref TYPE: DictAttribute = DictAttribute {
    caption: Some(
        "Type".to_string(),
    ),
    default: None,
    description: Some(
        "The type of an object or value, normalized to the caption of the type_id value. In the case of 'Other', it is defined by the event source. See specific usage.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref TYPE_ID: DictAttribute = DictAttribute {
        caption: Some("Type ID".to_string(),),
        default: Some(0,),
        description: Some(
            "The normalized type identifier of an object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: Some("type".to_string(),),
        attr_type: None,
    };
}

lazy_static! {
    static ref TYPE_NAME: DictAttribute = DictAttribute {
        caption: Some("Type Name".to_string(),),
        default: None,
        description: Some("The event type name, as defined by the type_uid.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref TYPE_UID: DictAttribute = DictAttribute {
    caption: Some(
        "Type ID".to_string(),
    ),
    default: None,
    description: Some(
        "The event type ID. It identifies the event's semantics and structure. The value is calculated by the logging system as: <code>class_uid * 100 + activity_id</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: Some(
        "type_name".to_string(),
    ),
    attr_type: None,
};
}

lazy_static! {
    static ref TYPES: DictAttribute = DictAttribute {
        caption: Some("Types".to_string(),),
        default: None,
        description: Some("The type/s of an entity. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref UID: DictAttribute = DictAttribute {
        caption: Some("Unique ID".to_string(),),
        default: None,
        description: Some("The unique identifier. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref UNMAPPED: DictAttribute = DictAttribute {
    caption: Some(
        "Unmapped Data".to_string(),
    ),
    default: None,
    description: Some(
        "The attributes that are not mapped to the event schema. The names and values of those attributes are specific to the event source.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref URL: DictAttribute = DictAttribute {
        caption: Some("URL".to_string(),),
        default: None,
        description: Some(
            "The URL object that pertains to the event or object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref USER: DictAttribute = DictAttribute {
        caption: Some("User".to_string(),),
        default: None,
        description: Some("The user that pertains to the event or object.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref USER_AGENT: DictAttribute = DictAttribute {
        caption: Some("HTTP User-Agent".to_string(),),
        default: None,
        description: Some(
            "The request header that identifies the operating system and web browser.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref USER_RESULT: DictAttribute = DictAttribute {
    caption: Some(
        "User Result".to_string(),
    ),
    default: None,
    description: Some(
        "The result of the user account change. It should contain the new values of the changed attributes.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref UUID: DictAttribute = DictAttribute {
        caption: Some("UUID".to_string(),),
        default: None,
        description: Some("The universally unique identifier. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref VALUE: DictAttribute = DictAttribute {
        caption: Some("Value".to_string(),),
        default: None,
        description: Some("The value that pertains to the object. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref VECTOR_STRING: DictAttribute = DictAttribute {
    caption: Some(
        "Vector String".to_string(),
    ),
    default: None,
    description: Some(
        "The CVSS vector string is a text representation of a set of CVSS metrics. It is commonly used to record or transfer CVSS metric information in a concise form. For example: <code>3.1/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:N/A:H</code>.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref VENDOR_NAME: DictAttribute = DictAttribute {
        caption: Some("Vendor Name".to_string(),),
        default: None,
        description: Some("The name of the vendor. See specific usage.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref VERSION: DictAttribute = DictAttribute {
        caption: Some("Version".to_string(),),
        default: None,
        description: Some(
            "The version that pertains to the event or object. See specific usage.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref VLAN_UID: DictAttribute = DictAttribute {
        caption: Some("VLAN".to_string(),),
        default: None,
        description: Some("The Virtual LAN identifier.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref VNC_AUTH: DictAttribute = DictAttribute {
    caption: Some(
        "VNC Authentication".to_string(),
    ),
    default: None,
    description: Some(
        "The Virtual Network Computing (VNC) authentication object describes the VNC authentication values.".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref VPC_UID: DictAttribute = DictAttribute {
        caption: Some("VPC UID".to_string(),),
        default: None,
        description: Some("The unique identifier of the Virtual Private Cloud (VPC).".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref VULNERABILITIES: DictAttribute = DictAttribute {
        caption: Some("Vulnerabilities".to_string(),),
        default: None,
        description: Some(
            "This object describes vulnerabilities reported in a security finding.".to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref VULNERABILITY: DictAttribute = DictAttribute {
        caption: Some("Vulnerability".to_string(),),
        default: None,
        description: Some(
            "The vulnerability object describes details related to the observed vulnerability"
                .to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! {
    static ref WIN_RESOURCE: DictAttribute = DictAttribute {
        caption: Some("Windows Resource".to_string(),),
        default: None,
        description: Some(
            "The Windows resource object that was accessed, such as a mutant or timer.".to_string(),
        ),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref X_FORWARDED_FOR: DictAttribute = DictAttribute {
    caption: Some(
        "X-Forwarded-For".to_string(),
    ),
    default: None,
    description: Some(
        "The X-Forwarded-For header identifying the originating IP address(es) of a client connecting to a web server through an HTTP proxy or a load balancer.".to_string(),
    ),
    attr_enum: None,
    is_array: Some(
        true,
    ),
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref X_ORIGINATING_IP: DictAttribute = DictAttribute {
        caption: Some("X-Originating-IP".to_string(),),
        default: None,
        description: Some(
            "The X-Originating-IP header identifying the emails originating IP address(es)."
                .to_string(),
        ),
        attr_enum: None,
        is_array: Some(true,),
        sibling: None,
        attr_type: None,
    };
}

lazy_static! { static ref XATTRIBUTES: DictAttribute = DictAttribute {
    caption: Some(
        "Extended Attributes".to_string(),
    ),
    default: None,
    description: Some(
        "An unordered collection of zero or more name/value pairs where each pair represents a file or folder extended attribute.</p>For example: Windows alternate data stream attributes (ADS stream name, ADS size, etc.), user-defined or application-defined attributes, ACL, owner, primary group, etc. Examples from DCS: </p><ul><li><strong>ads_name</strong></li><li><strong>ads_size</strong></li><li><strong>dacl</strong></li><li><strong>owner</strong></li><li><strong>primary_group</strong></li><li><strong>link_name</strong> - name of the link associated to the file.</li><li><strong>hard_link_count</strong> - the number of links that are associated to the file.</li></ul>".to_string(),
    ),
    attr_enum: None,
    is_array: None,
    sibling: None,
    attr_type: None,
};
}

lazy_static! {
    static ref ZONE: DictAttribute = DictAttribute {
        caption: Some("Network Zone".to_string(),),
        default: None,
        description: Some("The network zone or LAN segment.".to_string(),),
        attr_enum: None,
        is_array: None,
        sibling: None,
        attr_type: None,
    };
}
