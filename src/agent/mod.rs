use std::collections::HashMap;

use async_trait::async_trait;

use crate::{errors::Result, sealed::Sealed, Client};

mod service;

pub use service::*;

/// A health check run on a service hosted on this node.
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentCheck {
    /// The node the service is running on.
    #[serde(rename = "Node")]
    pub node: String,
    /// The ID of the service within the agent.
    #[serde(rename = "CheckID")]
    pub check_id: String,
    /// The name of the service.
    #[serde(rename = "Name")]
    pub name: String,
    /// The status of the check.
    #[serde(rename = "Status")]
    pub status: String,
    /// Notes attached to this check.
    #[serde(rename = "Notes")]
    pub notes: String,
    /// Output of the check.
    #[serde(rename = "Output")]
    pub output: String,
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    /// The name of the service.
    #[serde(rename = "ServiceName")]
    pub service_name: String,
}

/// A member within the cluster gossip pool.
///
/// Due to the nature of gossip, this is eventually consistent: the results may
/// differ by agent.
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentMember {
    /// The name of the agent.
    #[serde(rename = "Name")]
    pub name: String,
    /// The address of the agent.
    #[serde(rename = "Addr")]
    pub addr: String,
    /// The port of the agent.
    #[serde(rename = "Port")]
    pub port: u16,
    /// The tags assigned to this agent.
    #[serde(rename = "Tags")]
    pub tags: HashMap<String, String>,
    /// The status of this agent.
    #[serde(rename = "Status")]
    pub status: usize,
    /// The minimum protocol version this agent supports.
    #[serde(rename = "ProtocolMin")]
    pub protocol_min: u8,
    /// The maximum protocol version this agent supports.
    #[serde(rename = "ProtocolMax")]
    pub protocol_max: u8,
    /// The version of the agent.
    #[serde(rename = "ProtocolCur")]
    pub protocol_cur: u8,
    #[serde(rename = "DelegateMin")]
    pub delegate_min: u8,
    #[serde(rename = "DelegateMax")]
    pub delegate_max: u8,
    #[serde(rename = "DelegateCur")]
    pub delegate_cur: u8,
}

/// A service hosted on this node.
///
/// For more information, see the [Agent:]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentService {
    /// The ID of the service in the agent.
    #[serde(rename = "ID")]
    /// The name of this service.
    pub id: String,
    #[serde(rename = "Service")]
    pub service: String,
    /// A list of tags assigned to this service.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    /// The port this service is running on.
    #[serde(rename = "Port")]
    pub port: u16,
    /// The address this service is running on.
    #[serde(rename = "Address")]
    pub address: String,
    /// Whether tags are being overridden.
    #[serde(rename = "EnableTagOverride")]
    pub enable_tag_override: bool,
    #[serde(rename = "CreateIndex")]
    pub create_index: u64,
    #[serde(rename = "ModifyIndex")]
    pub modify_index: u64,
}

/// This trait provides methods for interacting with the local Consul agent.
///
/// These methods are used to interact with the local Consul agent.
/// Usually, services and checks are registered with an agent which then takes
/// on the burden of keeping that data synchronized with the cluster. For
/// example, the agent registers services and checks with the Catalog and
/// performs anti-entropy to recover from outages.
///
/// For more information consult the [API documentation] for the `/agent`
/// endpoint on the Consul website.
///
/// [API documentation]: https://www.consul.io/api/agent/check.html#list-checks
#[async_trait]
pub trait Agent: Sealed {
    /// This method returns all checks that are registered with the local
    /// agent.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/agent/check.html#list-checks
    async fn list_checks(&self) -> Result<HashMap<String, AgentCheck>>;

    /// This method returns the members the agent sees in the cluster gossip
    /// pool. Due to the nature of gossip, this is eventually consistent: the
    /// results may differ by agent.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent#list-members.
    async fn list_members(&self, wan: bool) -> Result<AgentMember>;

    /// This method instructs the agent to reload its configuration.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent#reload-agent.
    async fn reload_agent(&self) -> Result<()>;

    /// This method places the agent into "maintenance mode". During maintenance
    /// mode, the node will be marked as unavailable and will not be present in
    /// DNS or API queries.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent#enable-maintenance-mode.
    async fn enable_maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()>;

    /// This method instructs the agent to attempt to connect to a given
    /// address.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api-docs/agent#join-agent.
    async fn join_cluster(&self, address: &str, wan: bool) -> Result<()>;

    /// This endpoint triggers a graceful leave and shutdown of the agent. It is
    /// used to ensure other nodes see the agent as "left" instead of "failed".
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]: https://www.consul.io/api/agent.html#graceful-leave-and-shutdown.
    async fn leave_cluster(&self) -> Result<()>;

    /// This endpoint instructs the agent to force a node into the left state in
    /// the LAN and WAN gossip pools. If a node fails unexpectedly, then it will
    /// be in a failed state.
    ///
    /// For more information, consult the relevant endpoint's [API
    /// documentation].
    ///
    /// [API documentation]:  https://www.consul.io/api-docs/agent#force-leave-and-shutdown.
    async fn force_leave_cluster(&self) -> Result<()>;
}

#[async_trait]
impl Agent for Client {
    async fn list_checks(&self) -> Result<HashMap<String, AgentCheck>> {
        self.get("/v1/agent/checks", None).await
    }

    async fn list_members(&self, wan: bool) -> Result<AgentMember> {
        let mut params = HashMap::new();
        if wan {
            params.insert(String::from("wan"), String::from("1"));
        }
        self.get("/v1/agent/members", None).await
    }

    async fn reload_agent(&self) -> Result<()> {
        self.put("/v1/agent/reload", (), None, None).await
    }

    async fn enable_maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable { String::from("true") } else { String::from("false") };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        self.put("/v1/agent/maintenance", (), Some(params), None).await
    }

    async fn join_cluster(&self, address: &str, wan: bool) -> Result<()> {
        let mut params = HashMap::new();

        if wan {
            params.insert(String::from("wan"), String::from("true"));
        }
        let path = format!("/v1/agent/join/{}", address);
        self.put(&path, (), Some(params), None).await
    }

    async fn leave_cluster(&self) -> Result<()> {
        self.put("/v1/agent/leave", (), None, None).await
    }

    async fn force_leave_cluster(&self) -> Result<()> {
        self.put("/v1/agent/force-leave", (), None, None).await
    }
}
