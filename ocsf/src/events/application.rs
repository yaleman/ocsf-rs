/// API events describe general CRUD (Create, Read, Update, Delete) API activities, e.g. (AWS Cloudtrail)
///
/// Sourced from: `events/application/api.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ApiActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    pub actor: String,
    pub api: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<String>,
    pub src_endpoint: String,
}

impl ApiActivity {
    pub const UID: u16 = 3;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self { dst_endpoint: Some(dst_endpoint),
        ..self  
        }
    }

    /// Set the value of http_request
    pub fn with_http_request(self, http_request: String) -> Self {
        Self { http_request: Some(http_request),
        ..self  
        }
    }

    /// Set the value of resources
    pub fn with_resources(self, resources: String) -> Self {
        Self { resources: Some(resources),
        ..self  
        }
    }

    /// Details about the source of the activity. - required
    pub fn new(actor: String, api: String, src_endpoint: String) -> Self {
        Self {
        activity_id: None,
        actor,
        api,
        dst_endpoint: None,
        http_request: None,
        resources: None,
        src_endpoint,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Application;

impl Application {
    pub const UID: u16 = 6000;
    /// Create a new instance of this event.
    pub fn new() -> Self {
        Self {
        }
    }
}

/// Application Lifecycle events report installation, removal, start, stop of an application or service.
///
/// Sourced from: `events/application/application_lifecycle.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct ApplicationLifecycle {
    pub activity_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub app: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl ApplicationLifecycle {
    pub const UID: u16 = 2;
    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self { device: Some(device),
        ..self  
        }
    }

    /// No description available. - recommended
    pub fn new(activity_id: String, app: String) -> Self {
        Self {
        activity_id,
        actor: None,
        app,
        device: None,
        }
    }
}

/// Web Resource Access Activity events describe successful/failed attempts to access a web resource over HTTP.
///
/// Sourced from: `events/application/web_resource_access_activity.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct WebResourceAccessActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    pub http_request: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_response: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<String>,
    pub web_resources: String,
}

impl WebResourceAccessActivity {
    pub const UID: u16 = 6004;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self { device: Some(device),
        ..self  
        }
    }

    /// Set the value of http_response
    pub fn with_http_response(self, http_response: String) -> Self {
        Self { http_response: Some(http_response),
        ..self  
        }
    }

    /// Set the value of proxy
    pub fn with_proxy(self, proxy: String) -> Self {
        Self { proxy: Some(proxy),
        ..self  
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self { src_endpoint: Some(src_endpoint),
        ..self  
        }
    }

    /// Set the value of tls
    pub fn with_tls(self, tls: String) -> Self {
        Self { tls: Some(tls),
        ..self  
        }
    }

    /// Details about the resource that is the target of the activity. - required
    pub fn new(http_request: String, web_resources: String) -> Self {
        Self {
        activity_id: None,
        actor: None,
        device: None,
        http_request,
        http_response: None,
        proxy: None,
        src_endpoint: None,
        tls: None,
        web_resources,
        }
    }
}

/// Web Resources Activity events describe actions executed on a set of Web Resources.
///
/// Sourced from: `events/application/web_resources_activity.json`
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct WebResourcesActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dst_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_endpoint: Option<String>,
    pub web_resources: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_resources_result: Option<String>,
}

impl WebResourcesActivity {
    pub const UID: u16 = 6001;
    /// Set the value of activity_id
    pub fn with_activity_id(self, activity_id: String) -> Self {
        Self { activity_id: Some(activity_id),
        ..self  
        }
    }

    /// Set the value of actor
    pub fn with_actor(self, actor: String) -> Self {
        Self { actor: Some(actor),
        ..self  
        }
    }

    /// Set the value of device
    pub fn with_device(self, device: String) -> Self {
        Self { device: Some(device),
        ..self  
        }
    }

    /// Set the value of dst_endpoint
    pub fn with_dst_endpoint(self, dst_endpoint: String) -> Self {
        Self { dst_endpoint: Some(dst_endpoint),
        ..self  
        }
    }

    /// Set the value of src_endpoint
    pub fn with_src_endpoint(self, src_endpoint: String) -> Self {
        Self { src_endpoint: Some(src_endpoint),
        ..self  
        }
    }

    /// Set the value of web_resources_result
    pub fn with_web_resources_result(self, web_resources_result: String) -> Self {
        Self { web_resources_result: Some(web_resources_result),
        ..self  
        }
    }

    /// No description available. - optional
    pub fn new(web_resources: String) -> Self {
        Self {
        activity_id: None,
        actor: None,
        device: None,
        dst_endpoint: None,
        src_endpoint: None,
        web_resources,
        web_resources_result: None,
        }
    }
}

// This file was automatically generated by ocsf-codegen at 2023-06-06T22:56:49+00:00 branch: "yaleman/issue9" link: <https://github.com/yaleman/ocsf-rs/commit/7c25539db4b4823979b3d4ee870fad2ab8f805cf>