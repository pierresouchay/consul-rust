use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    sealed::Sealed, Client, ConsulResult, HealthCheck, ServiceWeights, TaggedAddress,
};

/// A service registered with the local agent.
///
/// This service was either provided through configuration files or added
/// dynamically using the HTTP API.
///
/// See the [List Services] endpoint documentation for more information.
///
/// [List Services]: https://www.consul.io/api-docs/agent/service#list-services
#[derive(Deserialize, Debug)]
pub struct Service {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "TaggedAddresses")]
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    #[serde(rename = "Meta")]
    pub meta: HashMap<String, String>,
    #[serde(rename = "Port")]
    pub port: u16,
    #[serde(rename = "Weights")]
    pub weights: ServiceWeights,
    #[serde(rename = "EnableTagOverride")]
    pub enable_tag_override: bool,
    #[serde(rename = "Address")]
    pub address: String,
}

/// The full service definition for a single service instance registered on the
/// local agent.
#[derive(Deserialize, Debug)]
pub struct ServiceConfig {
    #[serde(rename = "Kind")]
    pub kind: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Service")]
    pub service: Service,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Meta")]
    pub meta: Option<HashMap<String, String>>,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "TaggedAddresses")]
    pub tagged_addresses: HashMap<String, TaggedAddress>,
    #[serde(rename = "Port")]
    pub port: u16,
}

#[derive(Serialize)]
pub struct ServiceRegistrationPayload {
    #[serde(rename = "Name")]
    /// Specifies the logical name of the service.
    pub name: String,
    /// Specifies a unique ID for this service. This must be unique per agent.
    /// This defaults to the Name parameter if not provided.
    #[serde(rename = "ID")]
    pub id: Option<String>,
    ///  Specifies a list of tags to assign to the service.
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    /// Specifies the port on which the service is exposed.
    #[serde(rename = "Port")]
    pub port: u16,
    /// Specifies the address on which the service is exposed.
    #[serde(rename = "Address")]
    pub address: Option<String>,
    ///Specifies to disable the anti-entropy feature for this service's tags.
    #[serde(rename = "EnableTagOverride")]
    pub enable_tag_override: bool,
}
/// The `/agent/service` endpoints interact with services on the local agent in
/// Consul. These should not be confused with services in the catalog.
#[async_trait]
pub trait AgentServices: Sealed {
    async fn list_local_services(&self) -> ConsulResult<Vec<Service>>;
    async fn get_local_service_config<S: AsRef<str> + Send>(
        &self,
        id: S,
    ) -> ConsulResult<ServiceConfig>;
    async fn get_local_service_health<S: AsRef<str> + Send>(
        &self,
        name: S,
    ) -> ConsulResult<HealthCheck>;
    async fn get_local_service_health_by_id<S: AsRef<str> + Send>(
        &self,
        id: S,
    ) -> ConsulResult<HealthCheck>;
    async fn register_service(&self, payload: ServiceRegistrationPayload) -> ConsulResult<()>;
}

#[async_trait]
impl AgentServices for Client {
    async fn list_local_services(&self) -> ConsulResult<Vec<Service>> {
        self.get("/v1/agent/services", None).await
    }
    async fn get_local_service_config<S: AsRef<str> + Send>(
        &self,
        name: S,
    ) -> ConsulResult<ServiceConfig> {
        self.get(format!("/v1/agent/services/{}", name.as_ref()), None).await
    }
    async fn get_local_service_health<S: AsRef<str> + Send>(
        &self,
        name: S,
    ) -> ConsulResult<HealthCheck> {
        self.get(format!("/v1/agent/health/service/{}", name.as_ref()), None).await
    }
    async fn get_local_service_health_by_id<S: AsRef<str> + Send>(
        &self,
        id: S,
    ) -> ConsulResult<HealthCheck> {
        self.get(format!("/v1/agent/health/service/id/{}", id.as_ref()), None).await
    }
    async fn register_service(&self, payload: ServiceRegistrationPayload) -> ConsulResult<()> {
        self.put("/v1/agent/service/register", payload, None, None).await
    }
}
