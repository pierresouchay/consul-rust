use std::collections::HashMap;

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Weights {
    Passing: u32,
    Warning: u32,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Node {
    ID: String,
    Node: String,
    Address: String,
    Datacenter: String,
    TaggedAddresses: HashMap<String, String>,
    Meta: HashMap<String, String>,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogService {
    ID: String,
    Node: String,
    Address: String,
    Datacenter: String,
    TaggedAddresses: HashMap<String, String>,
    NodeMeta: HashMap<String, String>,
    ServiceID: String,
    ServiceName: String,
    ServiceAddress: String,
    ServiceTags: Vec<String>,
    ServiceMeta: HashMap<String, String>,
    ServicePort: u32,
    ServiceWeights: Weights,
    ServiceEnableTagOverride: bool,
    CreateIndex: u64,
    ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogNode {
    Node: Option<Node>,
    Services: HashMap<String, AgentService>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogRegistration {
    ID: String,
    Node: String,
    Address: String,
    TaggedAddresses: HashMap<String, String>,
    NodeMeta: HashMap<String, String>,
    Datacenter: String,
    Service: Option<AgentService>,
    Check: Option<AgentCheck>,
    SkipNodeUpdate: bool,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogDeregistration {
    Node: String,
    Address: String,
    Datacenter: String,
    ServiceID: String,
    CheckID: String,
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
