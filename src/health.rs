use std::collections::HashMap;

use crate::agent::AgentService;
use crate::errors::Result;
use crate::request::get;
use crate::{Client, QueryMeta, QueryOptions};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
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

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ServiceEntry {
    pub Node: Node,
    pub Service: AgentService,
    pub Checks: Vec<HealthCheck>,
}

pub trait Health {
    fn service(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<ServiceEntry>, QueryMeta)>;

    fn node(
        &self,
        node: &str,
        service_name: Option<&str>,
        check_id: Option<&str>,
        tag: Option<&str>,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<HealthCheck>, QueryMeta)>;
}

impl Health for Client {
    fn service(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<ServiceEntry>, QueryMeta)> {
        let mut qoptions = options.map_or_else(QueryOptions::default, |qo| qo.to_owned());
        let mut params = HashMap::new();
        let path = format!("/v1/health/service/{}", service);
        if passing_only {
            params.insert(String::from("passing"), String::from("1"));
        }
        if let Some(tag) = tag {
            let mut filter = qoptions.filter.unwrap_or_default();
            if !filter.is_empty() {
                filter = format!("({filter}) and ");
            }
            filter.push_str(&format!("\"{tag}\" in Service.Tags"));
            qoptions.filter = Some(filter);
        }
        get(&path, &self.config, params, Some(&qoptions))
    }

    fn node(
        &self,
        node: &str,
        check_id: Option<&str>,
        service_name: Option<&str>,
        tag: Option<&str>,
        options: Option<&QueryOptions>,
    ) -> Result<(Vec<HealthCheck>, QueryMeta)> {
        let mut qoptions = options.map_or_else(QueryOptions::default, |qo| qo.to_owned());
        let params = HashMap::new();
        let path = format!("/v1/health/node/{}", node);
        if let Some(service_name) = service_name {
            let mut filter = qoptions.filter.unwrap_or_default();
            if !filter.is_empty() {
                filter = format!("({filter}) and ");
            }
            filter.push_str(&format!("ServiceName == \"{service_name}\""));
            qoptions.filter = Some(filter);
        } else if let Some(check_id) = check_id {
            let mut filter = qoptions.filter.unwrap_or_default();
            if !filter.is_empty() {
                filter = format!("({filter}) and ");
            }
            filter.push_str(&format!("CheckID == \"{check_id}\""));
            qoptions.filter = Some(filter);
        }
        if let Some(tag) = tag {
            let mut filter = qoptions.filter.unwrap_or_default();
            if !filter.is_empty() {
                filter = format!("({filter}) and ");
            }
            filter.push_str(&format!("\"{tag}\" in ServiceTags"));
            qoptions.filter = Some(filter);
        }
        get(&path, &self.config, params, Some(&qoptions))
    }
}
