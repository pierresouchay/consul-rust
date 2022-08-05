use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    agent::AgentService, sealed::Sealed, AgentCheck, Client, ConsulResult, QueryOptions,
    ServiceWeights,
};

/// A node within the cluster gossip pool.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Node {
    /// The ID of the node.
    #[serde(rename = "ID")]
    id: String,
    /// The name of the node.
    #[serde(rename = "Node")]
    node: String,
    /// The address of the node.
    #[serde(rename = "Address")]
    address: String,
    /// The datacenter of the node.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// The tags of the node.
    #[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
    /// The meta data of the node.
    #[serde(rename = "Meta")]
    meta: HashMap<String, String>,
    create_index: u64,
    modify_index: u64,
}

/// A service defined within the Agent catalog.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogService {
    /// The ID of the service.
    #[serde(rename = "ID")]
    id: String,
    /// The node the service is associated with.
    #[serde(rename = "Node")]
    node: String,
    /// The address of the node.
    #[serde(rename = "Address")]
    address: String,
    /// The datacenter of the node running the service.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// A map of addresses tagged to the node hosting the service.
    #[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
    /// Metadata attached to the node this service is hosted on.
    #[serde(rename = "NodeMeta")]
    node_meta: HashMap<String, String>,
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    service_id: String,
    /// The name of the service.
    #[serde(rename = "ServiceName")]
    service_name: String,
    /// The address of the service.
    #[serde(rename = "ServiceAddress")]
    service_address: String,
    /// Tags assigned to the service.
    #[serde(rename = "ServiceTags")]
    service_tags: Vec<String>,
    /// Metadata assigned to the service.
    #[serde(rename = "ServiceMeta")]
    service_meta: HashMap<String, String>,
    /// The port of the service.
    #[serde(rename = "ServicePort")]
    service_port: u32,
    #[serde(rename = "ServiceWeights")]
    service_weights: ServiceWeights,
    #[serde(rename = "ServiceEnableTagOverride")]
    service_enable_tag_override: bool,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

/// A response datatype containing a [Node] and its services.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogNode {
    /// The node stored in the catalog.
    #[serde(rename = "Node")]
    node: Option<Node>,
    /// The services associated with the node.
    #[serde(rename = "Services")]
    services: HashMap<String, AgentService>,
}

/// Datatype containing payload data for the [crate::Catalog::register] method.
///
/// For more information, consult the [API documentation](https://www.consul.io/api-docs/catalog#json-request-body-schema).
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogRegistrationPayload {
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

/// Request payload datatype for the [crate::Catalog::deregister] method.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogDeregistrationPayload {
    /// Specifies the node ID to deregister.
    #[serde(rename = "Node")]
    node: String,
    /// The address of the node.
    #[serde(rename = "Address")]
    address: String,
    /// Specifies the datacenter, which defaults to the agent's datacenter if
    /// not provided.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// Specifies the service ID to deregister.
    #[serde(rename = "ServiceID")]
    service_id: String,
    /// Specifies the check ID to deregister.
    #[serde(rename = "CheckID")]
    check_id: String,
}

/// This trait provides methods for interacting with the Agent catalogue.
#[async_trait]
pub trait Catalog: Sealed {
    /// This method is a low-level mechanism for registering or updating
    /// entries in the catalog. It is usually preferable to instead use methods
    /// defined in the `Agent` trait for registration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult the [API documentation](https://www.consul.io/api-docs/catalog#register-entity).
    async fn register(
        &self,
        reg: CatalogRegistrationPayload,
        q: Option<QueryOptions>,
    ) -> ConsulResult<()>;

    /// This method is a low-level mechanism for directly removing entries from
    /// the Catalog. It is usually preferable to instead use methods defined
    /// in the `Agent` trait for deregistration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult the [API documentation](https://www.consul.io/api/catalog.html#deregister-entity).
    async fn deregister(
        &self,
        payload: CatalogDeregistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()>;

    /// This method returns the list of all known datacenters. The datacenters
    /// will be sorted in ascending order based on the estimated median round
    /// trip time from the server to the servers in that datacenter.
    ///
    /// For more information, consult the [API documentation](https://www.consul.io/api/catalog.html#list-datacenters).
    async fn list_datacenters(&self) -> ConsulResult<Vec<String>>;

    /// This endpoint and returns the nodes registered in a given datacenter.
    ///
    /// For more information, consult the [API documentation](https://www.consul.io/api/catalog.html#list-nodes).
    async fn list_datacenter_nodes(
        &self,
        q: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<Node>>>;

    /// This endpoint returns the services registered in a given datacenter.
    ///
    /// For more information, consult the [API documentation](https://www.consul.io/api-docs/catalog#list-services).
    async fn list_datacenter_services(
        &self,
        q: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<String>>>;
}

#[async_trait]
impl Catalog for Client {
    async fn register(
        &self,
        payload: CatalogRegistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/session/create", payload, None, options).await
    }

    async fn deregister(
        &self,
        payload: CatalogDeregistrationPayload,
        options: Option<QueryOptions>,
    ) -> ConsulResult<()> {
        self.put("/v1/catalog/deregister", payload, None, options).await
    }

    async fn list_datacenters(&self) -> ConsulResult<Vec<String>> {
        self.get("/v1/catalog/datacenters", None).await
    }

    async fn list_datacenter_nodes(
        &self,
        q: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<Node>>> {
        self.get("/v1/catalog/nodes", q).await
    }

    async fn list_datacenter_services(
        &self,
        options: Option<QueryOptions>,
    ) -> ConsulResult<HashMap<String, Vec<String>>> {
        self.get("/v1/catalog/services", options).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Catalog, Client, Config};

    #[tokio::test]
    async fn test_list_datacenters() {
        let config = Config::new_from_env();
        let client = Client::new(config);
        let r = client.list_datacenters().await.unwrap();
        assert_eq!(r, ["dc1"]);
    }

    #[tokio::test]
    async fn test_list_datacenter_services() {
        let config = Config::default();
        let client = Client::new(config);
        let r = client.list_datacenter_services(None).await.unwrap();
        assert_ne!(r.len(), 0);
        match r.get("consul") {
            None => panic!("Should have a Consul service"),
            Some(val) => assert_eq!(val.len(), 0), // consul has no tags
        }
    }
}
