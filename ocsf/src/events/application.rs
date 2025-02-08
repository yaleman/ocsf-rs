/// File Hosting Activity events report the actions taken by file management applications, including file sharing servers like Sharepoint and services such as Box, MS OneDrive, or Google Drive.
///
/// Sourced from: `events/events/application/file_hosting.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct FileHosting {
    pub activity_id: Option<String>,
    /// The actor that performed the activity on the target file.
    pub actor: String,
    pub connection_info: Option<String>,
    /// The endpoint that received the activity on the target file.
    pub dst_endpoint: Option<String>,
    /// The share expiration time.
    pub expiration_time: Option<String>,
    /// The file that is the target of the activity.
    pub file: String,
    /// The endpoint that performed the activity on the target file.
    pub src_endpoint: String,
}

impl FileHosting {
    pub const UID: u16 = 6006;
}

/// Datastore events describe general activities (Read, Update, Query, Delete, etc.) which affect datastores or data within those datastores, e.g. (AWS RDS, AWS S3).
///
/// Sourced from: `events/events/application/datastore_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DatastoreActivity {
    pub activity_id: Option<String>,
    pub actor: String,
    pub database: Option<String>,
    pub databucket: Option<String>,
    /// Details about the endpoint hosting the datastore application or service.
    pub dst_endpoint: Option<String>,
    /// Details about the underlying http request.
    pub http_request: Option<String>,
    pub query_info: Option<String>,
    /// Details about the source of the activity.
    pub src_endpoint: String,
    pub table: Option<String>,
    /// The datastore resource type (e.g. database, datastore, or table).
    pub type_name: Option<String>,
    /// The normalized datastore resource type identifier.
    pub type_id: Option<String>,
}

impl DatastoreActivity {
    pub const UID: u16 = 5;
}

/// Scan events report the start, completion, and results of a scan job. The scan event includes the number of items that were scanned and the number of detections that were resolved.
///
/// Sourced from: `events/events/application/scan_activity.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ScanActivity {
    pub activity_id: Option<String>,
    /// The command identifier that is associated with this scan event.  This ID uniquely identifies the proactive scan command, e.g., if remotely initiated.
    pub command_uid: Option<String>,
    /// The duration of the scan
    pub duration: Option<String>,
    /// The end time of the scan job.
    pub end_time: Option<String>,
    pub num_detections: Option<String>,
    pub num_files: Option<String>,
    pub num_folders: Option<String>,
    pub num_network_items: Option<String>,
    pub num_processes: Option<String>,
    pub num_registry_items: Option<String>,
    pub num_resolutions: Option<String>,
    pub num_skipped_items: Option<String>,
    pub num_trusted_items: Option<String>,
    /// The policy associated with this Scan event; required if the scan was initiated by a policy.
    pub policy: Option<String>,
    /// The Scan object describes characteristics of the scan job.
    pub scan: String,
    pub schedule_uid: Option<String>,
    /// The start time of the scan job.
    pub start_time: Option<String>,
    /// The total number of items that were scanned; zero if no items were scanned.
    pub total: Option<String>,
}

impl ScanActivity {
    pub const UID: u16 = 6007;
}

/// Sourced from: `events/events/application/application.json`
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Application;

impl Application {
    pub const UID: u16 = 6000;
}

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
    /// Details about the underlying HTTP request.
    pub http_request: Option<String>,
    /// Details about the HTTP response, if available.
    pub http_response: Option<String>,
    /// Details about the endpoint from which the request originated.
    pub src_endpoint: Option<String>,
    /// The Transport Layer Security (TLS) attributes, if available.
    pub tls: Option<String>,
    pub web_resources: String,
    pub web_resources_result: Option<String>,
}

impl WebResourcesActivity {
    pub const UID: u16 = 6001;
}

// This file was automatically generated by ocsf-codegen at 2025-02-08T00:22:36+00:00 branch: "more-fixes" link: <https://github.com/yaleman/ocsf-rs/commit/e3c933d060233d645bbfb4b9ab8f230ab9ba725e> OCSF Schema version: 1.2.0
