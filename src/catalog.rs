use std::collections::HashMap;

use agent::{AgentCheck, AgentService};
use error::*;
use request::{Method, Request, StatusCode};
use Client;

// Types
#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Weights {
    pub passing: i64,
    pub warning: i64,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub node: String,
    pub address: String,
    pub datacenter: String,
    pub tagged_addresses: HashMap<String, String>,
    pub meta: HashMap<String, String>,
    pub create_index: u64,
    pub modify_index: u64,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CatalogService {
    pub id: String,
    pub node: String,
    pub address: String,
    pub datacenter: String,
    pub tagged_addresses: HashMap<String, String>,
    pub node_meta: HashMap<String, String>,
    pub service_id: String,
    pub service_name: String,
    pub service_address: String,
    pub service_tags: Vec<String>,
    pub service_meta: HashMap<String, String>,
    pub service_port: u64,
    pub service_weights: Weights,
    pub service_enabled_tag_override: bool,
    pub create_index: u64,
    pub modify_index: u64,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CatalogNode {
    pub node: Node,
    pub services: HashMap<String, AgentService>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct CatalogRegistration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub node: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_addresses: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<AgentService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<AgentCheck>,
    pub skip_node_update: Option<bool>,
}

// API
#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct DeregisterOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct NodeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct NodesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ServiceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub near: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
}

#[serde(default, rename_all = "PascalCase")]
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ServicesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_meta: Option<String>,
}

pub trait Catalog {
    fn register(&self, catalog: &CatalogRegistration) -> Result<bool>;
    fn deregister(&self, node: &str, options: Option<&DeregisterOptions>) -> Result<bool>;
    fn datacenters(&self) -> Result<Vec<String>>;
    // TODO: blocking
    fn nodes(&self, options: Option<&NodesOptions>) -> Result<Vec<CatalogNode>>;
    // TODO: blocking
    fn services(&self, options: Option<&ServicesOptions>) -> Result<HashMap<String, Vec<String>>>;
    // TODO: blocking
    fn service(&self, service: &str, options: Option<&ServiceOptions>) -> Result<CatalogService>;
    // TODO: blocking
    fn connect(&self, service: &str, options: Option<&ServiceOptions>) -> Result<CatalogService>;
    // TODO: blocking
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<CatalogNode>;
}

impl Catalog for Client {
    /// https://www.consul.io/api/catalog.html#register-entity
    fn register(&self, catalog: &CatalogRegistration) -> Result<bool> {
        let mut r = Request::new(&self, Method::PUT, "catalog/register")
            .json(catalog)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#deregister-entity
    fn deregister(&self, node: &str, options: Option<&DeregisterOptions>) -> Result<bool> {
        let mut body: HashMap<String, String> = HashMap::new();
        body.insert(String::from("node"), node.to_string());
        if let Some(options) = options {
            if let Some(datacenter) = &options.datacenter {
                body.insert(String::from("datacenter"), datacenter.to_string());
            }
            if let Some(check_id) = &options.check_id {
                body.insert(String::from("check_id"), check_id.to_string());
            }
            if let Some(service_id) = &options.service_id {
                body.insert(String::from("service_id"), service_id.to_string());
            }
        }
        let mut r = Request::new(&self, Method::PUT, "catalog/deregister")
            .json(&body)
            .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-datacenters
    fn datacenters(&self) -> Result<Vec<String>> {
        let mut r = Request::new(&self, Method::GET, "catalog/datacenters").send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-nodes
    fn nodes(&self, options: Option<&NodesOptions>) -> Result<Vec<CatalogNode>> {
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
        let mut r = Request::new_with_params(&self, Method::GET, "catalog/nodes", params).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-services
    fn services(&self, options: Option<&ServicesOptions>) -> Result<HashMap<String, Vec<String>>> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
        }
        let mut r =
            Request::new_with_params(&self, Method::GET, "catalog/services", params).send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-nodes-for-service
    fn service(&self, service: &str, options: Option<&ServiceOptions>) -> Result<CatalogService> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(tag) = &options.tag {
                params.insert(String::from("tag"), tag.to_string());
            }
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::GET,
            &format!("catalog/service/{}", service),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-nodes-for-connect-capable-service
    fn connect(&self, service: &str, options: Option<&ServiceOptions>) -> Result<CatalogService> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        if let Some(options) = options {
            if let Some(tag) = &options.tag {
                params.insert(String::from("tag"), tag.to_string());
            }
            if let Some(near) = &options.near {
                params.insert(String::from("near"), near.to_string());
            }
            if let Some(node_meta) = &options.node_meta {
                params.insert(String::from("node-meta"), node_meta.to_string());
            }
        }
        let mut r = Request::new_with_params(
            &self,
            Method::GET,
            &format!("catalog/connect/{}", service),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }

    /// https://www.consul.io/api/catalog.html#list-services-for-node
    fn node(&self, node: &str, options: Option<&NodeOptions>) -> Result<CatalogNode> {
        let mut params: HashMap<String, String> = HashMap::new();
        if let Some(dc) = options
            .and_then(|o| o.dc.as_ref())
            .or_else(|| self.config.datacenter.as_ref())
        {
            params.insert(String::from("dc"), dc.to_string());
        }
        let mut r = Request::new_with_params(
            &self,
            Method::GET,
            &format!("catalog/node/{}", node),
            params,
        )
        .send()?;
        if r.status() != StatusCode::OK {
            Err(ErrorKind::UnexpectedResponse(r.text()?))?
        }
        Ok(r.json().context(ErrorKind::InvalidResponse)?)
    }
}
