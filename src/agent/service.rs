use async_trait::async_trait;

use crate::{sealed::Sealed, AgentService, Client, ConsulResult, HealthCheck, ServiceEntry, payload::CatalogRegistrationPayload};

/// The `/agent/service` endpoints interact with services on the local agent in
/// Consul. These should not be confused with services in the catalog.
#[async_trait]
pub trait AgentServices: Sealed {
    async fn list_local_services(&self) -> ConsulResult<Vec<AgentService>>;
    async fn get_local_service_config<S: AsRef<str> + Send>(
        &self,
        id: S,
    ) -> ConsulResult<ServiceEntry>;
    async fn get_local_service_health<S: AsRef<str> + Send>(
        &self,
        name: S,
    ) -> ConsulResult<HealthCheck>;
    async fn get_local_service_health_by_id<S: AsRef<str> + Send>(
        &self,
        id: S,
    ) -> ConsulResult<HealthCheck>;
    async fn register_service(&self, payload: CatalogRegistrationPayload) -> ConsulResult<()>;
}

#[async_trait]
impl AgentServices for Client {
    async fn list_local_services(&self) -> ConsulResult<Vec<AgentService>> {
        self.get("/v1/agent/services", None).await
    }
    async fn get_local_service_config<S: AsRef<str> + Send>(
        &self,
        name: S,
    ) -> ConsulResult<ServiceEntry> {
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
    async fn register_service(&self, payload: CatalogRegistrationPayload) -> ConsulResult<()> {
        self.put("/v1/agent/service/register", payload, None, None).await
    }
}
