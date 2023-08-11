/// Application Lifecycle events report installation, removal, start, stop of an application or service.
///
/// Sourced from: `events/events/application/application_lifecycle.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApplicationLifecycle {
    pub activity_id: String,
    /// The application that was affected by the lifecycle event.  This also applies to self-updating application systems.
    pub app: String,
}

impl ApplicationLifecycle {
    pub const UID: u16 = 2;
}

/// API events describe general CRUD (Create, Read, Update, Delete) API activities, e.g. (AWS Cloudtrail)
///
/// Sourced from: `events/events/application/api.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ApiActivity {
    pub activity_id: Option<String>,
    pub actor: String,
    pub api: String,
    pub dst_endpoint: Option<String>,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    /// Details about resources that were affected by the activity/event.
    pub resources: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: String,
}

impl ApiActivity {
    pub const UID: u16 = 3;
}

/// Web Resource Access Activity events describe successful/failed attempts to access a web resource over HTTP.
///
/// Sourced from: `events/events/application/web_resource_access_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WebResourceAccessActivity {
    pub activity_id: Option<String>,
    /// Details about the underlying HTTP request.
    pub http_request: String,
    /// Details about the HTTP response, if available.
    pub http_response: Option<String>,
    /// Details about the proxy service, if available.
    pub proxy: Option<String>,
    /// Details about the source endpoint of the request.
    pub src_endpoint: Option<String>,
    /// The Transport Layer Security (TLS) attributes, if available.
    pub tls: Option<String>,
    /// Details about the resource that is the target of the activity.
    pub web_resources: String,
}

impl WebResourceAccessActivity {
    pub const UID: u16 = 6004;
}

/// Web Resources Activity events describe actions executed on a set of Web Resources.
///
/// Sourced from: `events/events/application/web_resources_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WebResourcesActivity {
    pub activity_id: Option<String>,
    /// Details about server providing the web resources.
    pub dst_endpoint: Option<String>,
    /// Details about the endpoint from which the request originated.
    pub src_endpoint: Option<String>,
    pub web_resources: String,
    pub web_resources_result: Option<String>,
}

impl WebResourcesActivity {
    pub const UID: u16 = 6001;
}

/// Sourced from: `events/events/application/application.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Application;

impl Application {
    pub const UID: u16 = 6000;
}

// This file was automatically generated by ocsf-codegen at 2023-08-11T23:04:15+00:00 branch: "fixing-build-updating-schema" link: <https://github.com/yaleman/ocsf-rs/commit/c9ee180815729dcae5c937f32da30495d8aecf1a> OCSF Schema version: 1.1.0-dev
