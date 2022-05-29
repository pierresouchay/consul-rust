use std::collections::HashMap;

use crate::agent::AgentService;
use crate::errors::Result;
use crate::request::get;
use crate::{Client, QueryMeta, QueryOptions};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct HealthCheck {
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
    pub servicename: String,
    #[serde(rename = "ServiceTags")]
    pub servicetags: Option<Vec<String>>,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Node {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Node")]
    pub node: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Datacenter")]
    pub datacenter: Option<String>,
    #[serde(rename = "TaggedAddresses")]
    pub taggedaddresses: Option<HashMap<String, String>>,
    #[serde(rename = "Meta")]
    pub meta: Option<HashMap<String, String>>,
    #[serde(rename = "CreateIndex")]
    pub createindex: u64,
    #[serde(rename = "ModifyIndex")]
    pub modifyindex: u64,
}

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ServiceEntry {
    #[serde(rename = "Node")]
    pub node: Node,
    #[serde(rename = "Service")]
    pub service: AgentService,
    #[serde(rename = "Checks")]
    pub checks: Vec<HealthCheck>,
}

pub trait Health {
    fn service(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<ServiceEntry>, QueryMeta)>;
}

impl Health for Client {
    fn service(
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
