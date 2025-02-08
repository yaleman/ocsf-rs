/// Compliance Finding events describe results of evaluations performed against resources, to check compliance with various Industry Frameworks or Security Standards such as <code>NIST SP 800-53, CIS AWS Foundations Benchmark v1.4.0, ISO/IEC 27001</code> etc.
///
/// Sourced from: `events/events/findings/compliance_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ComplianceFinding {
    pub compliance: String,
    pub remediation: Option<String>,
    /// Describes details about the resource that is the subject of the compliance check.
    pub resource: Option<String>,
}

impl ComplianceFinding {
    pub const UID: u16 = 2003;
}

/// Security Finding events describe findings, detections, anomalies, alerts and/or actions performed by security products
///
/// Sourced from: `events/events/findings/security_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SecurityFinding {
    pub activity_id: Option<String>,
    pub analytic: Option<String>,
    /// The attack object describes the technique and associated tactics as defined by <a target='_blank' href='https://attack.mitre.org/wiki/ATT&CK_Matrix'>ATT&CK Matrix<sup>TM</sup></a>.
    pub attacks: Option<String>,
    pub cis_csc: Option<String>,
    pub compliance: Option<String>,
    pub confidence: Option<String>,
    pub confidence_id: Option<String>,
    pub confidence_score: Option<String>,
    pub data_sources: Option<String>,
    pub evidence: Option<String>,
    pub finding: String,
    pub impact: Option<String>,
    pub impact_id: Option<String>,
    pub impact_score: Option<String>,
    pub kill_chain: Option<String>,
    pub malware: Option<String>,
    pub nist: Option<String>,
    pub process: Option<String>,
    pub resources: Option<String>,
    pub risk_level: Option<String>,
    pub risk_level_id: Option<String>,
    pub risk_score: Option<String>,
    /// The normalized state of a security finding.
    pub state: Option<String>,
    /// The normalized state identifier of a security finding.
    pub state_id: String,
    pub vulnerabilities: Option<String>,
}

impl SecurityFinding {
    pub const UID: u16 = 2001;
}

/// An Incident Finding reports the creation, update, or closure of security incidents as a result of detections and/or analytics.
///
/// Sourced from: `events/events/findings/incident_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IncidentFinding {
    /// The normalized identifier of the Incident activity.
    pub activity_id: String,
    /// The Incident activity name, as defined by the `activity_id`.
    pub activity_name: Option<String>,
    pub assignee: Option<String>,
    pub assignee_group: Option<String>,
    /// An array of <a target='_blank' href='https://attack.mitre.org'>MITRE ATT&CK®</a> objects describing the tactics, techniques & sub-techniques associated to the Incident.
    pub attacks: Option<String>,
    /// Additional user supplied details for updating or closing the incident.
    pub comment: Option<String>,
    pub confidence: Option<String>,
    pub confidence_id: Option<String>,
    pub confidence_score: Option<String>,
    /// The short description of the Incident.
    pub desc: Option<String>,
    /// The time of the most recent event included in the incident.
    pub end_time: Option<String>,
    pub finding_info_list: String,
    pub impact: Option<String>,
    pub impact_id: Option<String>,
    pub impact_score: Option<String>,
    pub is_suspected_breach: Option<String>,
    pub priority: Option<String>,
    pub priority_id: Option<String>,
    /// A Url link used to access the original incident.
    pub src_url: Option<String>,
    /// The time of the least recent event included in the incident.
    pub start_time: Option<String>,
    /// The normalized status of the Incident normalized to the caption of the status_id value. In the case of 'Other', it is defined by the source.
    pub status: Option<String>,
    /// The normalized status identifier of the Incident.
    pub status_id: String,
    pub verdict: Option<String>,
    pub verdict_id: Option<String>,
}

impl IncidentFinding {
    pub const UID: u16 = 2005;
}

/// The Finding event is a generic event that defines a set of attributes available in the Findings category.
///
/// Sourced from: `events/events/findings/finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Finding {
    /// The normalized identifier of the finding activity.
    pub activity_id: Option<String>,
    /// The finding activity name, as defined by the `activity_id`.
    pub activity_name: Option<String>,
    /// A user provided comment about the finding.
    pub comment: Option<String>,
    pub confidence: Option<String>,
    pub confidence_id: Option<String>,
    pub confidence_score: Option<String>,
    /// Describes the affected device/host. It can be used in conjunction with `Affected Resource(s)`.
    /// ///  e.g. Specific details about an AWS EC2 instance, that is affected by the Finding.
    /// ///
    ///  ///
    pub device: Option<String>,
    /// The time of the most recent event included in the finding.
    pub end_time: Option<String>,
    pub finding_info: String,
    /// The time of the least recent event included in the finding.
    pub start_time: Option<String>,
    /// The normalized status of the Finding set by the consumer normalized to the caption of the status_id value. In the case of 'Other', it is defined by the source.
    pub status: Option<String>,
    /// The normalized status identifier of the Finding, set by the consumer.
    pub status_id: Option<String>,
}

impl Finding {
    pub const UID: u16 = 2000;
}

/// The Vulnerability Finding event is a notification about weakness in an information system, system security procedures, internal controls, or implementation that could be exploited or triggered by a threat source.
///
/// Sourced from: `events/events/findings/vulnerability_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct VulnerabilityFinding {
    /// Describes details about the resource that is affected by the vulnerability/vulnerabilities.
    pub resource: Option<String>,
    pub vulnerabilities: String,
}

impl VulnerabilityFinding {
    pub const UID: u16 = 2;
}

/// A Detection Finding describes detections or alerts generated by security products using correlation engines, detection engines or other methodologies. Note: if the product is a security control, the <code>security_control</code> profile should be applied and its <code>attacks</code> information should be duplicated into the <code>finding_info</code> object.
///
/// Sourced from: `events/events/findings/detection_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DetectionFinding {
    pub evidences: Option<String>,
    pub impact: Option<String>,
    pub impact_id: Option<String>,
    pub impact_score: Option<String>,
    pub remediation: Option<String>,
    /// Describes details about resources that were the target of the activity that triggered the finding.
    pub resources: Option<String>,
    pub risk_details: Option<String>,
    pub risk_level: Option<String>,
    pub risk_level_id: Option<String>,
    pub risk_score: Option<String>,
    /// Describes vulnerabilities reported in a Detection Finding.
    pub vulnerabilities: Option<String>,
}

impl DetectionFinding {
    pub const UID: u16 = 2004;
}

/// A Data Security Finding describes detections or alerts generated by various data security products such as Data Loss Prevention (DLP), Data Classification, Secrets Management, Digital Rights Management (DRM), Data Security Posture Management (DSPM), and similar tools. These detections or alerts can be created using fingerprinting, statistical analysis, machine learning or other methodologies. The finding describes the actors and endpoints who accessed or own the sensitive data, as well as the resources which store the sensitive data.
///
/// Sourced from: `events/events/findings/data_security_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DataSecurityFinding {
    /// The normalized identifier of the Data Security Finding activity.
    pub activity_id: String,
    /// The Data Security finding activity name, as defined by the `activity_id`.
    pub activity_name: Option<String>,
    /// Describes details about the actor implicated in the data security finding. Either an actor that owns a particular digital file or information store, or an actor which accessed classified or sensitive data.
    pub actor: Option<String>,
    pub data_security: Option<String>,
    /// Describes the database where classified or sensitive data is stored in, or was accessed from. Databases are typically datastore services that contain an organized collection of structured and/or semi-structured data.
    pub database: Option<String>,
    /// Describes the databucket where classified or sensitive data is stored in, or was accessed from. The data bucket object is a basic container that holds data, typically organized through the use of data partitions.
    pub databucket: Option<String>,
    /// Describes the device where classified or sensitive data is stored in, or was accessed from.
    pub device: Option<String>,
    /// Describes the endpoint where classified or sensitive data is stored in, or was accessed from.
    pub dst_endpoint: Option<String>,
    /// Describes a file that contains classified or sensitive data.
    pub file: Option<String>,
    pub impact: Option<String>,
    pub impact_id: Option<String>,
    pub impact_score: Option<String>,
    /// Describes details about resources twhere classified or sensitive data is stored in, or was accessed from.
    pub resources: Option<String>,
    pub risk_level: Option<String>,
    pub risk_level_id: Option<String>,
    pub risk_score: Option<String>,
    /// Details about the source endpoint where classified or sensitive data was accessed from.
    pub src_endpoint: Option<String>,
    /// Describes the table where classified or sensitive data is stored in, or was accessed from. The table object represents a table within a structured relational database, warehouse, lake, or similar.
    pub table: Option<String>,
}

impl DataSecurityFinding {
    pub const UID: u16 = 2006;
}

// This file was automatically generated by ocsf-codegen at 2025-02-08T00:22:36+00:00 branch: "more-fixes" link: <https://github.com/yaleman/ocsf-rs/commit/e3c933d060233d645bbfb4b9ab8f230ab9ba725e> OCSF Schema version: 1.2.0
