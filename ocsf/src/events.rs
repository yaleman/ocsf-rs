/// The base event is a generic and concrete event. It also defines a set of attributes available in most event classes. As a generic event that does not belong to any event category, it could be used to log events that are not otherwise defined by the schema.
///
/// Sourced from: `events/events/base_event.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BaseEvent {
    pub confidence: Option<String>,
    /// Additional data that is associated with the event.
    pub data: Option<String>,
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

pub mod audit;

pub mod inventory;

pub mod system;

pub mod findings;

pub mod network;

// This file was automatically generated by ocsf-codegen at 2023-03-28T11:36:12+00:00 branch: "yaleman/issue8" link: <https://github.com/yaleman/ocsf-rs/commit/c035bcbabbea474b72d2dfc1b4a316ad45549a19>
