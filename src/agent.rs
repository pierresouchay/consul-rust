//! Defines data-types relating to Agents.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    errors::Result,
    request::{get, put},
    sealed::Sealed,
    Client,
};

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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentService {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Address")]
    pub address: String,
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
/// For more information consult the API docs at https://www.consul.io/api-docs/agent.
#[async_trait]
pub trait Agent: Sealed {
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>>;
    async fn members(&self, wan: bool) -> Result<AgentMember>;
    async fn reload(&self) -> Result<()>;
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()>;
    async fn join(&self, address: &str, wan: bool) -> Result<()>;
    async fn leave(&self) -> Result<()>;
    async fn force_leave(&self) -> Result<()>;
}

#[async_trait]
impl Agent for Client {
    /// This method returns all checks that are registered with the local
    /// agent.
    ///
    /// For more information, consult https://www.consul.io/api/agent/check.html#list-checks.
    async fn checks(&self) -> Result<HashMap<String, AgentCheck>> {
        get("/v1/agent/checks", &self.config, HashMap::new(), None).await.map(|x| x.0)
    }
    /// This method returns the members the agent sees in the cluster gossip
    /// pool. Due to the nature of gossip, this is eventually consistent: the
    /// results may differ by agent.
    ///
    /// For more information, consult https://www.consul.io/api-docs/agent#list-members.
    async fn members(&self, wan: bool) -> Result<AgentMember> {
        let mut params = HashMap::new();
        if wan {
            params.insert(String::from("wan"), String::from("1"));
        }
        get("/v1/agent/members", &self.config, params, None).await.map(|x| x.0)
    }
    /// This method instructs the agent to reload its configuration.
    ///
    /// For more information, consult https://www.consul.io/api-docs/agent#reload-agent.
    async fn reload(&self) -> Result<()> {
        put("/v1/agent/reload", None as Option<&()>, &self.config, HashMap::new(), None)
            .await
            .map(|x| x.0)
    }

    /// This method places the agent into "maintenance mode". During maintenance
    /// mode, the node will be marked as unavailable and will not be present in
    /// DNS or API queries.
    ///
    /// For more information, consult https://www.consul.io/api-docs/agent#enable-maintenance-mode.
    async fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable { String::from("true") } else { String::from("false") };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        put("/v1/agent/maintenance", None as Option<&()>, &self.config, params, None)
            .await
            .map(|x| x.0)
    }
    /// This method instructs the agent to attempt to connect to a given
    /// address.
    ///
    /// For more information, consult https://www.consul.io/api-docs/agent#join-agent.
    async fn join(&self, address: &str, wan: bool) -> Result<()> {
        let mut params = HashMap::new();

        if wan {
            params.insert(String::from("wan"), String::from("true"));
        }
        let path = format!("/v1/agent/join/{}", address);
        put(&path, None as Option<&()>, &self.config, params, None).await.map(|x| x.0)
    }

    /// This endpoint triggers a graceful leave and shutdown of the agent. It is
    /// used to ensure other nodes see the agent as "left" instead of "failed".
    ///
    /// For more information, consult https://www.consul.io/api/agent.html#graceful-leave-and-shutdown.
    async fn leave(&self) -> Result<()> {
        put("/v1/agent/leave", None as Option<&()>, &self.config, HashMap::new(), None)
            .await
            .map(|x| x.0)
    }

    /// This endpoint instructs the agent to force a node into the left state in
    /// the LAN and WAN gossip pools. If a node fails unexpectedly, then it will
    /// be in a failed state.
    ///
    /// For more information, consult https://www.consul.io/api-docs/agent#force-leave-and-shutdown.
    async fn force_leave(&self) -> Result<()> {
        put("/v1/agent/force-leave", None as Option<&()>, &self.config, HashMap::new(), None)
            .await
            .map(|x| x.0)
    }
}
