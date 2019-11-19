use std::collections::HashMap;

use agent::AgentService;
use catalog::Node;
use error::*;
use request::{Method, Request};
use response::ResponseHelper;
use Client;

// Types
#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: String,
    pub name: String,
    pub status: String,
    pub notes: String,
    pub output: String,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    pub service_name: String,
    pub service_tags: Vec<String>,
    pub definition: HealthCheckDefinition,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct HealthCheckDefinition {
    #[serde(rename = "HTTP")]
    pub http: String,
    pub header: HashMap<String, String>,
    pub method: String,
    pub tls_skip_verify: bool,
    #[serde(rename = "TCP")]
    pub tcp: String,
    pub interval: String,
    pub timeout: String,
    pub deregister_critical_service_after: String,
}
type HealthChecks = Vec<HealthCheck>;

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ServiceEntry {
    pub node: Node,
    pub service: AgentService,
    pub checks: HealthChecks,
}

// API
#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct NodeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CheckOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ServiceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passing: Option<bool>,
}

#[serde(default)]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct StateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
}

pub trait Health {
    // TODO: blocking
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<HealthChecks>;
    // TODO: blocking
    fn checks(&self, service: &str, options: Option<&CheckOptions>) -> Result<HealthChecks>;
    // TODO: blocking
    fn service(&self, service: &str, options: Option<&ServiceOptions>)
        -> Result<Vec<ServiceEntry>>;
    // TODO: blocking
    fn connect(&self, service: &str, options: Option<&ServiceOptions>)
        -> Result<Vec<ServiceEntry>>;
    // TODO: blocking
    fn state(&self, state: &str, options: Option<&StateOptions>) -> Result<HealthChecks>;
}

impl Health for Client {
    /// https://www.consul.io/api/health.html#list-checks-for-node
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<HealthChecks> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        Request::new_with_params(&self, Method::GET, &format!("health/node/{}", node), params)
            .send()?
            .parse_json()
    }

    /// https://www.consul.io/api/health.html#list-checks-for-service
    fn checks(&self, service: &str, options: Option<&CheckOptions>) -> Result<HealthChecks> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
        }
        Request::new_with_params(
            &self,
            Method::GET,
            &format!("health/checks/{}", service),
            params,
        )
        .send()?
        .parse_json()
    }

    /// https://www.consul.io/api/health.html#list-nodes-for-service
    fn service(
        &self,
        service: &str,
        options: Option<&ServiceOptions>,
    ) -> Result<Vec<ServiceEntry>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(tag) = &options.tag {
                params.insert(String::from("tag"), tag.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
            if let Some(passing) = options.passing {
                params.insert(String::from("passing"), passing.to_string());
            }
        }
        Request::new_with_params(
            &self,
            Method::GET,
            &format!("health/service/{}", service),
            params,
        )
        .send()?
        .parse_json()
    }

    /// https://www.consul.io/api/health.html#list-nodes-for-connect-capable-service
    fn connect(
        &self,
        service: &str,
        options: Option<&ServiceOptions>,
    ) -> Result<Vec<ServiceEntry>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(tag) = &options.tag {
                params.insert(String::from("tag"), tag.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
            if let Some(passing) = options.passing {
                params.insert(String::from("passing"), passing.to_string());
            }
        }
        Request::new_with_params(
            &self,
            Method::GET,
            &format!("health/connect/{}", service),
            params,
        )
        .send()?
        .parse_json()
    }

    /// https://www.consul.io/api/health.html#list-checks-in-state
    fn state(&self, state: &str, options: Option<&StateOptions>) -> Result<HealthChecks> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
        }
        Request::new_with_params(
            &self,
            Method::GET,
            &format!("health/state/{}", state),
            params,
        )
        .send()?
        .parse_json()
    }
}
