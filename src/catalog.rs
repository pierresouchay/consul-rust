use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    agent::{AgentCheck, AgentService},
    errors::Result,
    request::{get, put},
    sealed::Sealed,
    Client, QueryMeta, QueryOptions, WriteMeta, WriteOptions,
};

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

/// Datatype containing payload data for the `register` method.
///
/// For more information, consult https://www.consul.io/api-docs/catalog#json-request-body-schema.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogRegistration {
    /// An optional UUID to assign to the node. This must be a 36-character
    /// UUID-formatted string.
    #[serde(rename = "Node")]
    id: String,
    /// Specifies the node ID to register.
    #[serde(rename = "Address")]
    node: String,
    /// Specifies the address to register.
    #[serde(rename = "Datacenter")]
    address: String,
    /// Specifies the tagged addresses.
    #[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
    /// Specifies arbitrary KV metadata pairs for filtering purposes.
    #[serde(rename = "NodeMeta")]
    node_meta: HashMap<String, String>,
    /// Specifies the datacenter, which defaults to the agent's datacenter if
    /// not provided.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// Specifies to register a service. If `id` is not provided, it will be
    /// defaulted to the value of the Service.Service property. Only one service
    /// with a given ID may be present per node.
    #[serde(rename = "Service")]
    service: Option<AgentService>,
    /// Specifies to register a check.
    #[serde(rename = "Check")]
    check: Option<AgentCheck>,
    /// Specifies whether to skip updating the node's information in the
    /// registration.
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

#[async_trait]
pub trait Catalog: Sealed {
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)>;
    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
}

#[async_trait]
impl Catalog for Client {
    /// This method is a low-level mechanism for registering or updating
    /// entries in the catalog. It is usually preferable to instead use methods
    /// defined in the `Agent` trait for registration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult https://www.consul.io/api-docs/catalog#register-entity.
    async fn register(
        &self,
        reg: &CatalogRegistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put("/v1/session/create", Some(reg), &self.config, HashMap::new(), q).await
    }

    /// This method is a low-level mechanism for directly removing entries from
    /// the Catalog. It is usually preferable to instead use methods defined
    /// in the `Agent` trait for deregistration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#deregister-entity.
    async fn deregister(
        &self,
        dereg: &CatalogDeregistration,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put("/v1/catalog/deregister", Some(dereg), &self.config, HashMap::new(), q).await
    }

    /// This method returns the list of all known datacenters. The datacenters
    /// will be sorted in ascending order based on the estimated median round
    /// trip time from the server to the servers in that datacenter.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#list-datacenters
    async fn datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get("/v1/catalog/datacenters", &self.config, HashMap::new(), None).await
    }

    /// This endpoint and returns the nodes registered in a given datacenter.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#list-nodes.
    async fn nodes(&self, q: Option<&QueryOptions>) -> Result<(Vec<Node>, QueryMeta)> {
        get("/v1/catalog/nodes", &self.config, HashMap::new(), q).await
    }

    /// This endpoint returns the services registered in a given datacenter.
    ///
    /// For more information, consult https://www.consul.io/api-docs/catalog#list-services.
    async fn services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q).await
    }
}
