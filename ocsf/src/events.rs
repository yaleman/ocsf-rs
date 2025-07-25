/// The base event is a generic and concrete event. It also defines a set of attributes available in most event classes. As a generic event that does not belong to any event category, it could be used to log events that are not otherwise defined by the schema.
///
/// Sourced from: `events/events/base_event.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BaseEvent {
    pub enrichments: Option<String>,
    pub message: Option<String>,
    pub metadata: String,
    pub observables: Option<String>,
    pub raw_data: Option<String>,
    pub severity: Option<String>,
    pub severity_id: String,
    pub status: Option<String>,
    pub status_code: Option<String>,
    pub status_detail: Option<String>,
    pub status_id: Option<String>,
    pub unmapped: Option<String>,
}

impl BaseEvent {}

pub mod application;

pub mod iam;

pub mod findings;

pub mod system;

pub mod network;

pub mod discovery;

// This file was automatically generated by ocsf-codegen at 2025-06-14T00:43:22+00:00 branch: "maintainer" link: <https://github.com/yaleman/ocsf-rs/commit/bf284b1bcb0ecdd744bfaeb425cd12ea5098ab7b> OCSF Schema version: 1.2.0
