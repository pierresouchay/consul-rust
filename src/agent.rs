use std::collections::HashMap;

use crate::errors::Result;
use crate::request::{get, put};
use crate::Client;

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentCheck {
    #[serde(rename = "Node")]
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Notes")]
    pub notes: String,
    #[serde(rename = "Output")]
    pub output: String,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    #[serde(rename = "ServiceName")]
    pub service_name: String,
}

#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct AgentMember {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Addr")]
    pub addr: String,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Tags")]
    pub tags: HashMap<String, String>,
    #[serde(rename = "PubStatus")]
    pub pub_status: usize,
    #[serde(rename = "ProtocolMin")]
    pub protocol_min: u8,
    #[serde(rename = "ProtocolMax")]
    pub protocol_max: u8,
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

//I haven't implemetned https://www.consul.io/api/agent.html#read-configuration
//I haven't implemetned https://www.consul.io/api/agent.html#stream-logs
pub trait Agent {
    fn checks(&self) -> Result<HashMap<String, AgentCheck>>;
    fn members(&self, wan: bool) -> Result<AgentMember>;
    fn reload(&self) -> Result<()>;
    fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()>;
    fn join(&self, address: &str, wan: bool) -> Result<()>;
    fn leave(&self) -> Result<()>;
    fn force_leave(&self) -> Result<()>;
}

impl Agent for Client {
    /// https://www.consul.io/api/agent/check.html#list-checks
    fn checks(&self) -> Result<HashMap<String, AgentCheck>> {
        get("/v1/agent/checks", &self.config, HashMap::new(), None).map(|x| x.0)
    }
    /// https://www.consul.io/api/agent.html#list-members
    fn members(&self, wan: bool) -> Result<AgentMember> {
        let mut params = HashMap::new();
        if wan {
            params.insert(String::from("wan"), String::from("1"));
        }
        get("/v1/agent/members", &self.config, params, None).map(|x| x.0)
    }
    /// https://www.consul.io/api/agent.html#reload-agent
    fn reload(&self) -> Result<()> {
        put(
            "/v1/agent/reload",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#reload-agent
    fn maintenance_mode(&self, enable: bool, reason: Option<&str>) -> Result<()> {
        let mut params = HashMap::new();
        let enable_str = if enable {
            String::from("true")
        } else {
            String::from("false")
        };
        params.insert(String::from("enabled"), enable_str);
        if let Some(r) = reason {
            params.insert(String::from("reason"), r.to_owned());
        }
        put(
            "/v1/agent/maintenance",
            None as Option<&()>,
            &self.config,
            params,
            None,
        )
        .map(|x| x.0)
    }
    ///https://www.consul.io/api/agent.html#join-agent
    fn join(&self, address: &str, wan: bool) -> Result<()> {
        let mut params = HashMap::new();

        if wan {
            params.insert(String::from("wan"), String::from("true"));
        }
        let path = format!("/v1/agent/join/{}", address);
        put(&path, None as Option<&()>, &self.config, params, None).map(|x| x.0)
    }

    /// https://www.consul.io/api/agent.html#graceful-leave-and-shutdown
    fn leave(&self) -> Result<()> {
        put(
            "/v1/agent/leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .map(|x| x.0)
    }

    ///https://www.consul.io/api/agent.html#force-leave-and-shutdown
    fn force_leave(&self) -> Result<()> {
        put(
            "/v1/agent/force-leave",
            None as Option<&()>,
            &self.config,
            HashMap::new(),
            None,
        )
        .map(|x| x.0)
    }
}
