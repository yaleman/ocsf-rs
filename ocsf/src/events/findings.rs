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
        Self { activity_id: Some(activity_id),
        }
    }

    /// No description available.
    pub fn new() -> Self {
        Self {
        activity_id: None,
        }
    }
}

/// Security Finding events describe findings, detections, anomalies, alerts and/or actions performed by security products
///
/// Sourced from: `events/findings/security_finding.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct SecurityFinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attacks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cis_csc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    pub finding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill_chain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malware: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_level_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub state_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<String>,
}

impl SecurityFinding {
    pub const UID: u16 = 2001;
    /// Set the value of analytic
    pub fn with_analytic(self, analytic: String) -> Self {
        Self { analytic: Some(analytic),
        ..self  
        }
    }

    /// Set the value of attacks
    pub fn with_attacks(self, attacks: String) -> Self {
        Self { attacks: Some(attacks),
        ..self  
        }
    }

    /// Set the value of cis_csc
    pub fn with_cis_csc(self, cis_csc: String) -> Self {
        Self { cis_csc: Some(cis_csc),
        ..self  
        }
    }

    /// Set the value of compliance
    pub fn with_compliance(self, compliance: String) -> Self {
        Self { compliance: Some(compliance),
        ..self  
        }
    }

    /// Set the value of confidence
    pub fn with_confidence(self, confidence: String) -> Self {
        Self { confidence: Some(confidence),
        ..self  
        }
    }

    /// Set the value of confidence_id
    pub fn with_confidence_id(self, confidence_id: String) -> Self {
        Self { confidence_id: Some(confidence_id),
        ..self  
        }
    }

    /// Set the value of confidence_score
    pub fn with_confidence_score(self, confidence_score: String) -> Self {
        Self { confidence_score: Some(confidence_score),
        ..self  
        }
    }

    /// Set the value of data_sources
    pub fn with_data_sources(self, data_sources: String) -> Self {
        Self { data_sources: Some(data_sources),
        ..self  
        }
    }

    /// Set the value of evidence
    pub fn with_evidence(self, evidence: String) -> Self {
        Self { evidence: Some(evidence),
        ..self  
        }
    }

    /// Set the value of impact
    pub fn with_impact(self, impact: String) -> Self {
        Self { impact: Some(impact),
        ..self  
        }
    }

    /// Set the value of impact_id
    pub fn with_impact_id(self, impact_id: String) -> Self {
        Self { impact_id: Some(impact_id),
        ..self  
        }
    }

    /// Set the value of impact_score
    pub fn with_impact_score(self, impact_score: String) -> Self {
        Self { impact_score: Some(impact_score),
        ..self  
        }
    }

    /// Set the value of kill_chain
    pub fn with_kill_chain(self, kill_chain: String) -> Self {
        Self { kill_chain: Some(kill_chain),
        ..self  
        }
    }

    /// Set the value of malware
    pub fn with_malware(self, malware: String) -> Self {
        Self { malware: Some(malware),
        ..self  
        }
    }

    /// Set the value of nist
    pub fn with_nist(self, nist: String) -> Self {
        Self { nist: Some(nist),
        ..self  
        }
    }

    /// Set the value of process
    pub fn with_process(self, process: String) -> Self {
        Self { process: Some(process),
        ..self  
        }
    }

    /// Set the value of resources
    pub fn with_resources(self, resources: String) -> Self {
        Self { resources: Some(resources),
        ..self  
        }
    }

    /// Set the value of risk_level
    pub fn with_risk_level(self, risk_level: String) -> Self {
        Self { risk_level: Some(risk_level),
        ..self  
        }
    }

    /// Set the value of risk_level_id
    pub fn with_risk_level_id(self, risk_level_id: String) -> Self {
        Self { risk_level_id: Some(risk_level_id),
        ..self  
        }
    }

    /// Set the value of risk_score
    pub fn with_risk_score(self, risk_score: String) -> Self {
        Self { risk_score: Some(risk_score),
        ..self  
        }
    }

    /// Set the value of state
    pub fn with_state(self, state: String) -> Self {
        Self { state: Some(state),
        ..self  
        }
    }

    /// Set the value of vulnerabilities
    pub fn with_vulnerabilities(self, vulnerabilities: String) -> Self {
        Self { vulnerabilities: Some(vulnerabilities),
        ..self  
        }
    }

    /// No description available. - optional
    pub fn new(finding: String, state_id: String) -> Self {
        Self {
        analytic: None,
        attacks: None,
        cis_csc: None,
        compliance: None,
        confidence: None,
        confidence_id: None,
        confidence_score: None,
        data_sources: None,
        evidence: None,
        finding,
        impact: None,
        impact_id: None,
        impact_score: None,
        kill_chain: None,
        malware: None,
        nist: None,
        process: None,
        resources: None,
        risk_level: None,
        risk_level_id: None,
        risk_score: None,
        state: None,
        state_id,
        vulnerabilities: None,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-06-06T03:33:41+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/79a55e1a7e6ab3fdb662d961c59066a530e7afae>