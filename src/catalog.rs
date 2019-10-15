use std::collections::HashMap;

use crate::agent::{AgentCheck, AgentService};
use crate::errors::Result;
use crate::request::{get, put};
use crate::{Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions};

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Weights {
    pub Passing: u32,
    pub Warning: u32,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Node {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub Meta: HashMap<String, String>,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}
impl Node {
    pub fn id(&self) -> &String {
        &self.ID
    }
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogService {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub NodeMeta: HashMap<String, String>,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceAddress: String,
    pub ServiceTags: Vec<String>,
    pub ServiceMeta: HashMap<String, String>,
    pub ServicePort: u32,
    pub ServiceWeights: Weights,
    pub ServiceEnableTagOverride: bool,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogNode {
    pub Node: Option<Node>,
    pub Services: HashMap<String, AgentService>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogRegistration {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub TaggedAddresses: HashMap<String, String>,
    pub NodeMeta: HashMap<String, String>,
    pub Datacenter: String,
    pub Service: Option<AgentService>,
    pub Check: Option<AgentCheck>,
    pub SkipNodeUpdate: bool,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct CatalogDeregistration {
    pub Node: String,
    pub Address: String,
    pub Datacenter: String,
    pub ServiceID: String,
    pub CheckID: String,
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
    fn node(&self, node_id:&str, q: Option<&QueryOptions>) -> Result<(CatalogNode, QueryMeta)>;
    fn services(&self, q: Option<&QueryOptions>) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
    fn service(&self, service_id:&str, q: Option<&QueryOptions>) -> Result<(Vec<CatalogService>, QueryMeta)>;
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

    fn node(&self, node_id:&str, q: Option<&QueryOptions>) -> Result<(CatalogNode, QueryMeta)> {
        get(format!("/v1/catalog/node/{}",node_id).as_str(), &self.config, HashMap::new(), q)
    }

    fn services(&self, q: Option<&QueryOptions>) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q)
    }

    fn service(&self, service_id: &str, q: Option<&QueryOptions>) -> Result<(Vec<CatalogService>, QueryMeta)> {
        get(format!("/v1/catalog/service/{}", service_id).as_str(), &self.config, HashMap::new(), q)
    }
}
