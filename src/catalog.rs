use std::collections::HashMap;

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Weights {
    passing: u32,
    warning: u32,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Node {
    id: String,
    node: String,
    address: String,
    datacenter: String,
    tagged_addresses: HashMap<String, String>,
    meta: HashMap<String, String>,
    create_index: u64,
    modify_index: u64,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogService {
	#[serde(rename = "ID")]
    id: String,
	#[serde(rename = "Node")]
    node: String,
	#[serde(rename = "Address")]
    address: String,
	#[serde(rename = "Datacenter")]
    datacenter: String,
	#[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
	#[serde(rename = "NodeMeta")]
    node_meta: HashMap<String, String>,
	#[serde(rename = "ServiceID")]
    service_id: String,
	#[serde(rename = "ServiceName")]
    service_name: String,
	#[serde(rename = "ServiceAddress")]
    service_address: String,
	#[serde(rename = "ServiceTags")]
    service_tags: Vec<String>,
	#[serde(rename = "ServiceMeta")]
    service_meta: HashMap<String, String>,
	#[serde(rename = "ServicePort")]
    service_port: u32,
	#[serde(rename = "ServiceWeights")]
    service_weights: Weights,
	#[serde(rename = "ServiceEnableTagOverride")]
    service_enable_tag_override: bool,
	#[serde(rename = "CreateIndex")]
    create_index: u64,
	#[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogNode {
	#[serde(rename = "Node")]
    node: Option<Node>,
	#[serde(rename = "Services")]
    services: HashMap<String, AgentService>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogRegistration {
	#[serde(rename = "Node")]
    id: String,
	#[serde(rename = "Address")]
    node: String,
	#[serde(rename = "Datacenter")]
    address: String,
	#[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
	#[serde(rename = "NodeMeta")]
    node_meta: HashMap<String, String>,
	#[serde(rename = "Service")]
    datacenter: String,
	#[serde(rename = "Service")]
    service: Option<AgentService>,
	#[serde(rename = "Check")]
    check: Option<AgentCheck>,
	#[serde(rename = "SkipNodeUpdate")]
    skip_node_update: bool,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogDeregistration {
	#[serde(rename = "Node")]
    node: String,
	#[serde(rename = "Address")]
    address: String,
	#[serde(rename = "Datacenter")]
    datacenter: String,
	#[serde(rename = "ServiceID")]
    service_id: String,
	#[serde(rename = "CheckID")]
    check_id: String,
}

pub trait Catalog {
    fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)>;
    fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
}

impl Catalog for Client {
    /// https://www.consul.io/api/catalog.html#register-entity
    fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/session/create",
            Some(reg),
            &self.config,
            HashMap::new(),
            q,
        )
    }

    /// https://www.consul.io/api/catalog.html#deregister-entity
    fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put(
            "/v1/catalog/deregister",
            Some(dereg),
            &self.config,
            HashMap::new(),
            q,
        )
    }

    /// https://www.consul.io/api/catalog.html#list-datacenters
    fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get(
            "/v1/catalog/datacenters",
            &self.config,
            HashMap::new(),
            None,
        )
    }

    /// https://www.consul.io/api/catalog.html#list-nodes
    fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)> {
        get("/v1/catalog/nodes", &self.config, HashMap::new(), q)
    }

    fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q)
    }
}
