use std::collections::HashMap;

use crate::{Client, QueryMeta, QueryOptions};
use crate::agent::AgentService;
use crate::errors::Result;
use crate::request::get;

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    pub Node: String,
    pub CheckID: String,
    pub Name: String,
    pub Status: String,
    pub Notes: String,
    pub Output: String,
    pub ServiceID: String,
    pub ServiceName: String,
    pub ServiceTags: Option<Vec<String>>,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct Node {
    pub ID: String,
    pub Node: String,
    pub Address: String,
    pub Datacenter: Option<String>,
    pub TaggedAddresses: Option<HashMap<String, String>>,
    pub Meta: Option<HashMap<String, String>>,
    pub CreateIndex: u64,
    pub ModifyIndex: u64,
}

#[serde(default)]
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ServiceEntry {
    pub Node: Node,
    pub Service: AgentService,
    pub Checks: Vec<HealthCheck>,
}

pub trait Health {
    fn check_service(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<ServiceEntry>, QueryMeta)>;
}

impl Health for Client {
    fn check_service(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<ServiceEntry>, QueryMeta)> {
        let mut params = HashMap::new();
        let path = format!("/v1/health/service/{}", service);
        if passing_only {
            params.insert(String::from("passing"), String::from("1"));
        }
        if let Some(tag) = tag {
            params.insert(String::from("tag"), tag.to_owned());
        }
        get(&path, &self.config, params, options)
    }
}
