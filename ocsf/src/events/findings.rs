/// Findings events report findings, detections, and possible resolutions of malware, anomalies, or actions performed by security products.
///
/// Sourced from: `events/events/findings/findings.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Findings {
    pub activity_id: Option<String>,
}

impl Findings {
    pub const UID: u16 = 2000;
}

/// Security Finding events describe findings, detections, anomalies, alerts and/or actions performed by security products
///
/// Sourced from: `events/events/findings/security_finding.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SecurityFinding {
    /// The attack object describes the technique and associated tactics as defined by <a target='_blank' href='https://attack.mitre.org/wiki/ATT&CK_Matrix'>ATT&CK Matrix<sup>TM</sup></a>.
    pub attacks: Option<String>,
    pub compliance: Option<String>,
    pub finding: String,
    pub malware: Option<String>,
    pub process: Option<String>,
    pub resources: Option<String>,
    /// The normalized state of a security finding.
    pub state: Option<String>,
    /// The normalized state identifier of a security finding.
    pub state_id: String,
    pub vulnerabilities: Option<String>,
}

impl SecurityFinding {
    pub const UID: u16 = 2001;
}

// This file was automatically generated by ocsf-codegen at 2023-03-28T11:36:12+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/c035bcbabbea474b72d2dfc1b4a316ad45549a19>
