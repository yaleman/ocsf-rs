/// Findings events report findings, detections, and possible resolutions of malware, anomalies, or actions performed by security products.
///
/// Sourced from: `events/findings/findings.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Findings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}

impl Findings {
    pub const UID: u16 = 2000;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self {
            activity_id: Some(activity_id),
        }
    }

    /// No description available.
    pub fn new() -> Self {
        Self { activity_id: None }
    }
}

/// Security Finding events describe findings, detections, anomalies, alerts and/or actions performed by security products
///
/// Sourced from: `events/findings/security_finding.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct SecurityFinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
    pub disposition_id: String,
    pub finding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub state_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<String>,
}

impl SecurityFinding {
    pub const UID: u16 = 2001;
    /// Set the value of attacks
    pub fn with_attacks(self, attacks: String) -> Self {
        Self {
            attacks: Some(attacks),
            ..self
        }
    }

    /// Set the value of compliance
    pub fn with_compliance(self, compliance: String) -> Self {
        Self {
            compliance: Some(compliance),
            ..self
        }
    }

    /// Set the value of disposition
    pub fn with_disposition(self, disposition: String) -> Self {
        Self {
            disposition: Some(disposition),
            ..self
        }
    }

    /// Set the value of malware
    pub fn with_malware(self, malware: String) -> Self {
        Self {
            malware: Some(malware),
            ..self
        }
    }

    /// Set the value of process
    pub fn with_process(self, process: String) -> Self {
        Self {
            process: Some(process),
            ..self
        }
    }

    /// Set the value of resources
    pub fn with_resources(self, resources: String) -> Self {
        Self {
            resources: Some(resources),
            ..self
        }
    }

    /// Set the value of state
    pub fn with_state(self, state: String) -> Self {
        Self {
            state: Some(state),
            ..self
        }
    }

    /// Set the value of vulnerabilities
    pub fn with_vulnerabilities(self, vulnerabilities: String) -> Self {
        Self {
            vulnerabilities: Some(vulnerabilities),
            ..self
        }
    }

    /// No description available. - optional
    pub fn new(disposition_id: String, finding: String, state_id: String) -> Self {
        Self {
            attacks: None,
            compliance: None,
            disposition: None,
            disposition_id,
            finding,
            malware: None,
            process: None,
            resources: None,
            state: None,
            state_id,
            vulnerabilities: None,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-03-30T12:52:14+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/08bc45b0dda1acfa1bed0034f0784684b63c3635>
