use std::collections::HashMap;

use async_trait::async_trait;

use crate::{sealed::Sealed, AgentService, Client, ConsulResult, QueryOptions};

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

#[async_trait]
pub trait Health: Sealed {
    /// See the [API documentation] for more information.
    ///
    /// [API documentation]: https://www.consul.io/api-docs/health#list-nodes-for-service
    async fn list_service_instances(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<ServiceEntry>>;
}

#[async_trait]
impl Health for Client {
    async fn list_service_instances(
        &self,
        service: &str,
        tag: Option<&str>,
        passing_only: bool,
        options: Option<QueryOptions>,
    ) -> ConsulResult<Vec<ServiceEntry>> {
        let mut params = HashMap::new();
        let path = format!("/v1/health/service/{}", service);
        if passing_only {
            params.insert(String::from("passing"), String::from("1"));
        }
        if let Some(tag) = tag {
            params.insert(String::from("tag"), tag.to_owned());
        }
        self.get(&path, options).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{Client, Config, Health};

    #[tokio::test]
    async fn test_list_service_instances() {
        let config = Config::default();
        let client = Client::new(config);
        // An existing service for a agent in dev mode
        let snodes = client
            .list_service_instances("consul", Option::None, true, Option::None)
            .await
            .unwrap();
        {
            assert!(!snodes.is_empty(), "should have at least one Service Node");
        }
        // A non existing, should be empty
        let snodes = client
            .list_service_instances("non-existing-service", Option::None, true, Option::None)
            .await
            .unwrap();
        {
            assert_eq!(snodes.len(), 0);
        }
    }
}
