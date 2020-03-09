use std::collections::HashMap;

use crate::errors::Result;
use crate::request::{get, put};
use crate::Client;

#[serde(default)]
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentCheck {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
}

#[serde(default)]
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentMember {
    pub Name: String,
    pub Addr: String,
    pub Port: u16,
    pub Tags: HashMap<String, String>,
    pub pubStatus: usize,
    pub ProtocolMin: u8,
    pub ProtocolMax: u8,
    pub ProtocolCur: u8,
    pub DelegateMin: u8,
    pub DelegateMax: u8,
    pub DelegateCur: u8,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct AgentService {
    pub ID: String,
    pub Service: String,
    pub Tags: Option<Vec<String>>,
    pub Port: u16,
    pub Address: String,
    pub EnableTagOverride: bool,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
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
