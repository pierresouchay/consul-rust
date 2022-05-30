use std::collections::HashMap;

use crate::{AgentCheck, AgentService};

/// Datatype containing payload data for the [crate::Catalog::register] method.
///
/// For more information, consult https://www.consul.io/api-docs/catalog#json-request-body-schema.
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
